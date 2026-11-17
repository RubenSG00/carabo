// The engine snapshot (05 §10.3): what must sound, with the lanes already resolved by the
// covering rule. It has no view state: metronome, loop and position go with their own commands.
import { resolveLane } from "./lanes";
import type {
  AudioTrack,
  EngineSnapshot,
  ProjectDoc,
  RhythmTrack,
  SnapshotAudioTrack,
  SnapshotRhythmTrack,
  TimeSignature,
} from "./model";
import { barToStep, stepsPerBar } from "./time";

function rhythmTrack(track: RhythmTrack, timeSignature: TimeSignature): SnapshotRhythmTrack {
  const segments = resolveLane(
    track.blocks.map((block) => ({
      id: block,
      start: barToStep(block.startBar, timeSignature),
      end: barToStep(block.startBar + block.lengthBars, timeSignature),
      z: block.z,
    })),
  ).map(({ id: block, start, end }) => ({
    patternId: block.patternId,
    startStep: start,
    lengthSteps: end - start,
    phaseSteps: start - barToStep(block.startBar, timeSignature),
  }));

  return {
    id: track.id,
    type: "rhythm",
    strip: track.strip,
    kitId: track.kitId,
    rows: track.rows,
    patterns: track.patterns.map((pattern) => ({
      id: pattern.id,
      lengthSteps: pattern.bars * stepsPerBar(timeSignature),
      steps: pattern.steps,
    })),
    segments,
  };
}

// TODO(T-018): the segments of the clips, without those of missing files (RF-028).
function audioTrack(track: AudioTrack): SnapshotAudioTrack {
  return { id: track.id, type: "audio", strip: track.strip, segments: [] };
}

/**
 * The snapshot of `doc` saved in the project folder `projectDir`, leaving out the audio files
 * of `missingFiles` (RF-028). `strip`, `rows`, `kitId`, `master`, `tempoBpm` and
 * `timeSignature` are copied as they are.
 */
export function buildEngineSnapshot(
  doc: ProjectDoc,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars -- TODO(T-018): audio segments.
  projectDir: string,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars -- TODO(T-018): audio segments.
  missingFiles: readonly string[],
): EngineSnapshot {
  return {
    snapshotVersion: 1,
    tempoBpm: doc.tempoBpm,
    timeSignature: doc.timeSignature,
    tracks: doc.tracks.map((track) =>
      track.type === "rhythm" ? rhythmTrack(track, doc.timeSignature) : audioTrack(track),
    ),
    master: doc.master,
  };
}
