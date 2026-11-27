// New project elements with the initial values of 05 §3.5. The functions receive everything
// that is not an initial value (identifiers, names, positions), so they know nothing about
// counters or languages: a default name arrives already translated (03 §14.2).
import { KIT_ROWS, TEMPO_DEFAULT } from "./constants";
import type {
  AudioClip,
  AudioSource,
  AudioTrack,
  ChannelStrip,
  Pattern,
  PatternBlock,
  ProjectFile,
  RhythmTrack,
  RowSettings,
  TimeSignature,
  ViewState,
} from "./model";
import { stepsPerBar } from "./time";

/** An empty project (RF-005) with its initial view state. */
export function newProject(): ProjectFile {
  return {
    formatVersion: 1,
    tempoBpm: TEMPO_DEFAULT,
    timeSignature: "4/4",
    tracks: [],
    sources: [],
    master: { gainDb: 0 },
    nextId: 1,
    nextZ: 1,
    view: newViewState(),
  };
}

export function newViewState(): ViewState {
  return {
    metronomeEnabled: false,
    loopEnabled: false,
    loopStartBar: 0,
    loopEndBar: 4,
    snapEnabled: true,
    zoom: 24,
  };
}

export function newChannelStrip(): ChannelStrip {
  return {
    gainDb: 0,
    pan: 0,
    mute: false,
    solo: false,
    effects: [
      { type: "eq", enabled: false, lowDb: 0, midDb: 0, highDb: 0 },
      { type: "reverb", enabled: false, amount: 20, size: 50 },
    ],
  };
}

export function newRowSettings(): RowSettings {
  return { gainDb: 0, mute: false };
}

/** An empty pattern of one bar: 8 rows of `stepsPerBar` zeros. */
export function newPattern(id: string, name: string, timeSignature: TimeSignature): Pattern {
  const steps = Array.from({ length: KIT_ROWS }, () =>
    new Array<number>(stepsPerBar(timeSignature)).fill(0),
  );
  return { id, name, bars: 1, steps };
}

export function newPatternBlock(
  id: string,
  patternId: string,
  startBar: number,
  lengthBars: number,
  z: number,
): PatternBlock {
  return { id, patternId, startBar, lengthBars, z };
}

/** A rhythm track with the acoustic kit, its first pattern and its first block (RF-201). */
export function newRhythmTrack(
  id: string,
  name: string,
  colorIndex: number,
  pattern: Pattern,
  block: PatternBlock,
): RhythmTrack {
  return {
    id,
    type: "rhythm",
    name,
    colorIndex,
    strip: newChannelStrip(),
    kitId: "acoustic",
    rows: Array.from({ length: KIT_ROWS }, newRowSettings),
    patterns: [pattern],
    blocks: [block],
  };
}

/** An audio track with no clips and the initial channel. */
export function newAudioTrack(id: string, name: string, colorIndex: number): AudioTrack {
  return { id, type: "audio", name, colorIndex, strip: newChannelStrip(), clips: [] };
}

/**
 * A recorded or imported clip: it does not repeat and its region is as long as the clip
 * (`regionLengthFrames = lengthFrames`).
 */
export function newAudioClip(
  id: string,
  sourceId: string,
  startTick: number,
  regionStartFrame: number,
  lengthFrames: number,
  z: number,
): AudioClip {
  return {
    id,
    sourceId,
    startTick,
    lengthFrames,
    regionStartFrame,
    regionLengthFrames: lengthFrames,
    repeat: false,
    z,
  };
}

/** An audio file of the project folder (05 §6.2). */
export function newAudioSource(
  id: string,
  kind: AudioSource["kind"],
  file: string,
  name: string,
  frames: number,
  channels: AudioSource["channels"],
): AudioSource {
  return { id, kind, file, name, frames, channels };
}
