//! The transport: states, position and the events that end a subblock (06 §6).

use crate::constants::MICROFADE_FRAMES;
use crate::project::EngineProject;
use crate::rt::commands::PlayMode;
use crate::rt::sequencer::step_at;
use crate::telemetry::PublishedTransport;
use crate::time::step_to_frame;

/// What the transport computes while it plays (06 §6.1).
// TODO(T-043): song mode; TODO(T-047): the count-in; TODO(T-083): the recording.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Running {
    /// Pattern mode: pattern `pattern_index` of the rhythm track in `slot`, in a loop.
    Pattern { slot: usize, pattern_index: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Stopped,
    Playing(Running),
    /// Computes as in `from` while the master fades out; `faded` frames of the fade are done.
    Stopping {
        from: Running,
        faded: u64,
    },
}

/// Whether the transport has just reached `Stopped`, when the output thread switches every
/// voice off (06 §6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Change {
    None,
    Stopped,
}

/// The state of the transport and its position (06 §6.1, §6.2).
#[derive(Debug)]
pub(crate) struct Transport {
    state: State,
    /// In pattern mode, frames from the start of the pattern.
    position: u64,
}

impl Transport {
    /// A transport in `Stopped`.
    pub(crate) fn new() -> Self {
        Self {
            state: State::Stopped,
            position: 0,
        }
    }

    /// `Play` (06 §6.3): from `Stopped`, it plays from position 0. In `Stopping`, the fade is
    /// over: the transport goes through `Stopped`, which the result reports, and then plays.
    pub(crate) fn play(&mut self, mode: PlayMode) -> Change {
        let PlayMode::Pattern {
            slot,
            pattern_index,
        } = mode
        else {
            // TODO(T-043): song mode.
            return Change::None;
        };
        let change = match self.state {
            State::Stopped => Change::None,
            State::Stopping { .. } => Change::Stopped,
            // TODO(T-033): change the track or the pattern without stopping, and relocate the
            // position (06 §6.3).
            State::Playing(_) => return Change::None,
        };
        self.state = State::Playing(Running::Pattern {
            slot,
            pattern_index,
        });
        self.position = 0;
        // TODO(T-080): the counter of audio dropouts goes back to 0.
        change
    }

    /// `Stop` (06 §6.3): from `Playing` to `Stopping`. Ignored in any other state.
    pub(crate) fn stop(&mut self) {
        if let State::Playing(from) = self.state {
            self.state = State::Stopping { from, faded: 0 };
        }
    }

    /// What is computed: what plays, in `Playing` and in `Stopping`; `None` in `Stopped`.
    pub(crate) fn running(&self) -> Option<Running> {
        match self.state {
            State::Stopped => None,
            State::Playing(running) | State::Stopping { from: running, .. } => Some(running),
        }
    }

    pub(crate) fn position(&self) -> u64 {
        self.position
    }

    /// In `Stopping`, the frame of the fade where the next subblock starts (0 to 239).
    pub(crate) fn stop_fade(&self) -> Option<u64> {
        match self.state {
            State::Stopping { faded, .. } => Some(faded),
            _ => None,
        }
    }

    /// Length of the next subblock: at most `limit` frames, up to the next event (06 §6.4).
    pub(crate) fn next_subblock(&mut self, limit: u64, project: Option<&EngineProject>) -> u64 {
        let mut frames = limit;
        if let Some(end) = self.pattern_end(project) {
            if self.position >= end {
                // The pattern has become shorter than the position: it starts again.
                // TODO(T-033): relocate the position instead (06 §6.3, §6.6).
                self.position = 0;
            }
            frames = frames.min(end - self.position);
        }
        if let State::Stopping { faded, .. } = self.state {
            frames = frames.min(MICROFADE_FRAMES.saturating_sub(faded));
        }
        frames
    }

    /// Advances the position `frames` frames and runs the event that ended the subblock
    /// (06 §6.4; 06 §7, step 2.6): at the end of the pattern the position goes back to 0, and
    /// at the end of the stop fade the state becomes `Stopped`.
    pub(crate) fn advance(&mut self, frames: u64, project: Option<&EngineProject>) -> Change {
        if self.running().is_some() {
            self.position += frames;
            if Some(self.position) == self.pattern_end(project) {
                self.position = 0;
            }
        }
        if let State::Stopping { from, faded } = self.state {
            let faded = faded + frames;
            if faded >= MICROFADE_FRAMES {
                self.state = State::Stopped;
                self.position = 0;
                return Change::Stopped;
            }
            self.state = State::Stopping { from, faded };
        }
        Change::None
    }

    /// State, mode, position and step for the telemetry (06 §18.1). `Stopping` is reported
    /// as stopped (06 §6.1), whose position is reported as 0.
    pub(crate) fn published(
        &self,
        project: Option<&EngineProject>,
    ) -> (PublishedTransport, u64, i32) {
        match self.state {
            State::Playing(Running::Pattern { .. }) => {
                let step = project
                    .and_then(|p| i32::try_from(step_at(self.position, p.tempo_bpm)).ok())
                    .unwrap_or(-1);
                (PublishedTransport::PlayingPattern, self.position, step)
            }
            State::Stopped | State::Stopping { .. } => (PublishedTransport::Stopped, 0, -1),
        }
    }

    /// Frame where the pattern that plays ends: `stepToFrame(lengthSteps)`.
    fn pattern_end(&self, project: Option<&EngineProject>) -> Option<u64> {
        let Running::Pattern {
            slot,
            pattern_index,
        } = self.running()?;
        let project = project?;
        let pattern = project.rhythm_track(slot)?.patterns.get(pattern_index)?;
        Some(step_to_frame(pattern.length_steps, project.tempo_bpm)).filter(|&end| end > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::KIT_ROWS;
    use crate::project::{Pattern, RhythmTrack, Row, Strip};

    /// A project at 120 with one rhythm track in slot 2, whose only pattern has 16 steps:
    /// it ends on frame 96 000.
    fn project() -> EngineProject {
        EngineProject {
            tempo_bpm: 120,
            rhythm_tracks: vec![RhythmTrack {
                slot: 2,
                kit: 0,
                rows: [Row {
                    gain: 1.0,
                    mute: false,
                }; KIT_ROWS],
                patterns: vec![Pattern {
                    length_steps: 16,
                    steps: vec![[0; KIT_ROWS]; 16],
                }],
                strip: Strip {
                    volume: 1.0,
                    pan_left: 1.0,
                    pan_right: 1.0,
                },
            }],
        }
    }

    const PATTERN: PlayMode = PlayMode::Pattern {
        slot: 2,
        pattern_index: 0,
    };

    #[test]
    fn subblocks_end_at_the_end_of_the_pattern_and_the_position_goes_back_to_0() {
        let project = project();
        let mut t = Transport::new();
        assert_eq!(t.next_subblock(4_096, Some(&project)), 4_096);
        t.play(PATTERN);
        t.advance(95_000, Some(&project));
        assert_eq!(t.next_subblock(4_096, Some(&project)), 1_000);
        assert_eq!(t.advance(1_000, Some(&project)), Change::None);
        assert_eq!(t.position(), 0);
        assert_eq!(
            t.published(Some(&project)),
            (PublishedTransport::PlayingPattern, 0, 0)
        );
        t.advance(6_000, Some(&project));
        assert_eq!(
            t.published(Some(&project)),
            (PublishedTransport::PlayingPattern, 6_000, 1)
        );
    }

    #[test]
    fn stop_fades_out_for_240_frames_and_then_reaches_stopped() {
        let project = project();
        let mut t = Transport::new();
        t.play(PATTERN);
        t.advance(1_000, Some(&project));
        t.stop();
        assert_eq!(t.stop_fade(), Some(0));
        assert_eq!(
            t.running(),
            Some(Running::Pattern {
                slot: 2,
                pattern_index: 0
            })
        );
        assert_eq!(t.next_subblock(4_096, Some(&project)), 240);
        assert_eq!(t.advance(100, Some(&project)), Change::None);
        assert_eq!(t.position(), 1_100);
        assert_eq!(t.stop_fade(), Some(100));
        assert_eq!(t.next_subblock(4_096, Some(&project)), 140);
        assert_eq!(
            t.published(Some(&project)),
            (PublishedTransport::Stopped, 0, -1)
        );
        assert_eq!(t.advance(140, Some(&project)), Change::Stopped);
        assert_eq!(t.running(), None);
        assert_eq!(t.stop_fade(), None);
        assert_eq!(t.position(), 0);
        // Stop when stopped is ignored.
        t.stop();
        assert_eq!(t.running(), None);
    }

    #[test]
    fn play_while_stopping_goes_through_stopped_and_plays_from_0() {
        let project = project();
        let mut t = Transport::new();
        assert_eq!(t.play(PATTERN), Change::None);
        t.advance(5_000, Some(&project));
        t.stop();
        t.advance(10, Some(&project));
        assert_eq!(t.play(PATTERN), Change::Stopped);
        assert_eq!(t.position(), 0);
        assert_eq!(t.stop_fade(), None);
    }

    #[test]
    fn a_pattern_shorter_than_the_position_starts_again() {
        let mut project = project();
        let mut t = Transport::new();
        t.play(PATTERN);
        t.advance(50_000, Some(&project));
        project.rhythm_tracks[0].patterns[0].length_steps = 4;
        assert_eq!(t.next_subblock(4_096, Some(&project)), 4_096);
        assert_eq!(t.position(), 0);
    }

    #[test]
    fn without_the_pattern_there_is_no_end_of_pattern() {
        let mut t = Transport::new();
        t.play(PlayMode::Pattern {
            slot: 5,
            pattern_index: 0,
        });
        assert_eq!(t.next_subblock(4_096, Some(&project())), 4_096);
        assert_eq!(t.next_subblock(4_096, None), 4_096);
        t.advance(200_000, None);
        assert_eq!(t.position(), 200_000);
        assert_eq!(
            t.published(None),
            (PublishedTransport::PlayingPattern, 200_000, -1)
        );
    }

    #[test]
    fn song_mode_and_a_second_play_are_ignored_for_now() {
        let project = project();
        let mut t = Transport::new();
        t.play(PlayMode::Song { start_frame: 0 });
        assert_eq!(t.running(), None);
        t.play(PATTERN);
        t.advance(1_000, Some(&project));
        t.play(PlayMode::Pattern {
            slot: 2,
            pattern_index: 1,
        });
        assert_eq!(
            t.running(),
            Some(Running::Pattern {
                slot: 2,
                pattern_index: 0
            })
        );
        assert_eq!(t.position(), 1_000);
    }
}
