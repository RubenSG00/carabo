//! The telemetry thread (04 §8, 07 §6.3): `TELEMETRY_HZ` times per second it reads the
//! telemetry of the engine and emits it as `engine://telemetry`, and then handles each pending
//! engine event.

use std::io;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use carabo_engine::constants::TELEMETRY_HZ;
use carabo_engine::{EngineEvent, EngineHandle, Telemetry};
use tauri::{AppHandle, Emitter, Runtime};

use crate::logging::{self, Level, Log};

/// Name of the telemetry event (04 §6.4).
pub const TELEMETRY_EVENT: &str = "engine://telemetry";
/// Time between two emissions: `1000 / TELEMETRY_HZ` milliseconds (07 §6.3).
pub const PERIOD: Duration = Duration::from_nanos(1_000_000_000 / TELEMETRY_HZ as u64);
/// Module name of the lines this thread writes to the log (05 §7.3).
const MODULE: &str = "telemetry";

/// Starts the `carabo-telemetry` thread, which runs as long as the application.
pub fn spawn<R: Runtime>(app: AppHandle<R>, engine: EngineHandle) -> io::Result<JoinHandle<()>> {
    thread::Builder::new()
        .name("carabo-telemetry".to_string())
        .spawn(move || {
            // Emitting only fails while the application closes: there is no one left to tell.
            let mut emit = |event: &str, telemetry: &Telemetry| {
                let _ = app.emit(event, telemetry);
            };
            let mut planned = Instant::now();
            loop {
                publish(
                    engine.telemetry(),
                    engine.poll_events(),
                    &mut emit,
                    logging::log(),
                );
                planned = next_emission(planned, Instant::now());
                thread::sleep(planned.saturating_duration_since(Instant::now()));
            }
        })
}

/// Emits `telemetry` and then handles `events`, in order.
///
/// The interface does not handle any engine event yet, so each one is written to the log and
/// not emitted.
// TODO(T-076): emit `EngineEvent::AudioUnavailable` as `engine://audio-unavailable`.
// TODO(T-086): emit `RecordingFinished` and `DevicesChanged` as `engine://recording-finished`
// and `engine://devices-changed`.
// TODO(T-111): emit `InternalError` as `engine://error`.
pub fn publish(
    telemetry: Telemetry,
    events: Vec<EngineEvent>,
    emit: &mut impl FnMut(&str, &Telemetry),
    log: &Log,
) {
    emit(TELEMETRY_EVENT, &telemetry);
    for event in events {
        log.write(
            Level::Warn,
            MODULE,
            &format!("engine event not delivered to the interface yet: {event:?}"),
        );
    }
}

/// When to emit next, after the emission planned for `planned` has finished at `now`. The
/// emissions keep a steady pace; after a delay of more than a period, the pace starts again
/// from `now` instead of emitting the missed ones in a burst.
pub fn next_emission(planned: Instant, now: Instant) -> Instant {
    let next = planned + PERIOD;
    if next < now {
        now
    } else {
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::fs;

    use carabo_engine::{ActiveDevices, RecordEndReason, TelemetryState};

    fn stopped() -> Telemetry {
        Telemetry {
            state: TelemetryState::Stopped,
            mode: None,
            position_frame: 0,
            pattern_step: None,
            track_peaks: BTreeMap::new(),
            master_peak: [0.0, 0.0],
            input_peak: None,
            xruns: 0,
            recorded_frames: None,
            output_available: false,
        }
    }

    /// One event of each kind.
    fn every_event() -> Vec<EngineEvent> {
        vec![
            EngineEvent::RecordingFinished {
                file: "audio/take-0001.wav".to_string(),
                frames: 48_000,
                track_id: "track-1".to_string(),
                start_frame: 0,
                reason: RecordEndReason::Stopped,
            },
            EngineEvent::DevicesChanged {
                lost_output_name: Some("Speakers".to_string()),
                lost_input_name: None,
                active: ActiveDevices::default(),
            },
            EngineEvent::AudioUnavailable {
                files: vec!["C:/project/audio/take-0002.wav".to_string()],
            },
            EngineEvent::InternalError {
                detail: "impossible state".to_string(),
            },
        ]
    }

    #[test]
    fn emits_telemetry_hz_times_per_second() {
        let per_second = Duration::from_secs(1).as_secs_f64() / PERIOD.as_secs_f64();
        assert!(
            (per_second - f64::from(TELEMETRY_HZ)).abs() < 1e-6,
            "{per_second}"
        );
        assert_eq!(TELEMETRY_HZ, 30);
    }

    #[test]
    fn emissions_keep_a_steady_pace_of_one_period() {
        let start = Instant::now();
        let mut planned = start;
        for _ in 0..TELEMETRY_HZ {
            // The work of each emission takes part of the period.
            let now = planned + PERIOD / 3;
            planned = next_emission(planned, now);
        }
        assert_eq!(planned - start, PERIOD * TELEMETRY_HZ);
    }

    #[test]
    fn after_a_long_delay_the_pace_starts_again_without_a_burst() {
        let start = Instant::now();
        let late = start + PERIOD * 5;
        let next = next_emission(start, late);
        assert_eq!(next, late);
        assert_eq!(next_emission(next, next), late + PERIOD);
    }

    #[test]
    fn emits_the_telemetry_as_engine_telemetry() {
        let mut emitted = Vec::new();
        let mut emit = |event: &str, telemetry: &Telemetry| {
            emitted.push((event.to_string(), telemetry.clone()));
        };
        publish(stopped(), Vec::new(), &mut emit, &Log::disabled());
        assert_eq!(emitted, [("engine://telemetry".to_string(), stopped())]);
    }

    #[test]
    fn engine_events_are_written_to_the_log_and_not_emitted() {
        let data = tempfile::tempdir().expect("a temporary folder");
        let log = Log::open(data.path()).expect("the log opens");
        let mut emitted = Vec::new();
        let mut emit = |event: &str, _: &Telemetry| emitted.push(event.to_string());

        publish(stopped(), every_event(), &mut emit, &log);

        assert_eq!(emitted, ["engine://telemetry"]);
        let text = fs::read_to_string(data.path().join("carabo.log")).expect("the log exists");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 4);
        for (line, kind) in lines.iter().zip([
            "RecordingFinished",
            "DevicesChanged",
            "AudioUnavailable",
            "InternalError",
        ]) {
            assert!(line.contains(" WARN telemetry "), "{line}");
            assert!(line.contains(kind), "{line}");
        }
    }
}
