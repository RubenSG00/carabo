// New project elements with the initial values of 05 §3.5. The functions receive everything
// that is not an initial value (identifiers, names, positions), so they know nothing about
// counters or languages.
// TODO(T-011): audio tracks, clips and sources.
import { KIT_ROWS, TEMPO_DEFAULT } from "./constants";
import type {
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
