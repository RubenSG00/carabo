// The data model of 05 §3.1 (the project) and 05 §10.1 (the engine snapshot), exactly as the
// specification writes them. The project file is the direct serialisation of these types.

// ---- Basic types ----
export type TimeSignature = "4/4" | "3/4" | "6/8";
export type KitId = "acoustic" | "electronic" | "percussion";
export type Zoom = 6 | 12 | 24 | 48 | 96 | 192;

// ---- Project ----
export interface ProjectFile extends ProjectDoc {
  view: ViewState;
}

export interface ProjectDoc {
  formatVersion: 1;
  tempoBpm: number; // integer, TEMPO_MIN..TEMPO_MAX
  timeSignature: TimeSignature;
  tracks: Track[]; // 0..MAX_TRACKS, in track order
  sources: AudioSource[]; // exactly the audio files that some clip uses
  master: MasterStrip;
  nextId: number; // integer ≥ 1: next number for an identifier
  nextZ: number; // integer ≥ 1: next value of z
}

export interface ViewState {
  metronomeEnabled: boolean;
  loopEnabled: boolean;
  loopStartBar: number; // integer ≥ 0
  loopEndBar: number; // integer > loopStartBar (the range is [loopStartBar, loopEndBar))
  snapEnabled: boolean;
  zoom: Zoom; // pixels per quarter note
}

// ---- Tracks ----
export type Track = RhythmTrack | AudioTrack;

export interface TrackBase {
  id: string; // "trk-<n>"
  type: "rhythm" | "audio";
  name: string; // 1..NAME_MAX_CHARS characters
  colorIndex: number; // integer 0..7: colour of the track palette (03)
  strip: ChannelStrip;
}

export interface RhythmTrack extends TrackBase {
  type: "rhythm";
  kitId: KitId;
  rows: RowSettings[]; // exactly KIT_ROWS (8), in the row order of the kit
  patterns: Pattern[]; // 1..MAX_PATTERNS_PER_TRACK, in creation order
  blocks: PatternBlock[]; // in any order
}

export interface AudioTrack extends TrackBase {
  type: "audio";
  clips: AudioClip[]; // in any order
}

// ---- Rhythm ----
export interface RowSettings {
  gainDb: number; // integer, ROW_GAIN_MIN_DB..ROW_GAIN_MAX_DB
  mute: boolean;
}

export interface Pattern {
  id: string; // "pat-<n>"
  name: string; // 1..NAME_MAX_CHARS characters
  bars: 1 | 2 | 4;
  steps: number[][]; // 8 rows × (bars · stepsPerBar) steps; each value is a level 0..3
}

export interface PatternBlock {
  id: string; // "blk-<n>"
  patternId: string; // a pattern of the same track
  startBar: number; // integer ≥ 0
  lengthBars: number; // integer ≥ 1
  z: number; // integer ≥ 1; greater = further on top
}

// ---- Audio ----
export interface AudioClip {
  id: string; // "clip-<n>"
  sourceId: string; // an element of ProjectDoc.sources
  startTick: number; // integer ≥ 0
  lengthFrames: number; // integer ≥ MIN_CLIP_FRAMES: what the clip takes up in the song
  regionStartFrame: number; // integer ≥ 0: first frame of the file that sounds
  regionLengthFrames: number; // integer ≥ MIN_CLIP_FRAMES: length of the region
  repeat: boolean; // true: the region repeats until it fills lengthFrames
  z: number; // integer ≥ 1; greater = further on top
}

export interface AudioSource {
  id: string; // "src-<n>"
  kind: "take" | "import";
  file: string; // path relative to the project folder, with "/": "audio/take-0001.wav"
  name: string; // visible name: "Toma 1" or the name of the imported file without extension
  frames: number; // integer ≥ 1: length of the file in frames
  channels: 1 | 2;
}

// ---- Mix ----
export interface ChannelStrip {
  gainDb: number; // FADER_MIN_DB..FADER_MAX_DB in steps of 0.5; -60 means gain 0 ("−∞")
  pan: number; // integer -100..100
  mute: boolean;
  solo: boolean;
  effects: EffectUnit[]; // in 1.0, exactly [EqUnit, ReverbUnit], in that order
}

export type EffectUnit = EqUnit | ReverbUnit;

export interface EqUnit {
  type: "eq";
  enabled: boolean;
  lowDb: number; // EQ_GAIN_MIN_DB..EQ_GAIN_MAX_DB in steps of 0.5
  midDb: number;
  highDb: number;
}

export interface ReverbUnit {
  type: "reverb";
  enabled: boolean;
  amount: number; // integer 0..100
  size: number; // integer 0..100
}

export interface MasterStrip {
  gainDb: number; // like ChannelStrip.gainDb
}

// ---- Engine snapshot (05 §10.1) ----
export interface EngineSnapshot {
  snapshotVersion: 1;
  tempoBpm: number;
  timeSignature: TimeSignature;
  tracks: SnapshotTrack[]; // in track order
  master: MasterStrip;
}

export type SnapshotTrack = SnapshotRhythmTrack | SnapshotAudioTrack;

export interface SnapshotRhythmTrack {
  id: string;
  type: "rhythm";
  strip: ChannelStrip; // the same as in the project
  kitId: KitId;
  rows: RowSettings[]; // 8
  patterns: SnapshotPattern[]; // every pattern of the track
  segments: RhythmSegment[]; // without overlaps, sorted by startStep
}

export interface SnapshotPattern {
  id: string;
  lengthSteps: number; // bars · stepsPerBar
  steps: number[][]; // 8 × lengthSteps, levels 0..3
}

export interface RhythmSegment {
  patternId: string;
  startStep: number; // song step where the segment starts
  lengthSteps: number; // ≥ 1
  phaseSteps: number; // steps elapsed since the start of its block
}

export interface SnapshotAudioTrack {
  id: string;
  type: "audio";
  strip: ChannelStrip;
  segments: AudioSegment[]; // without overlaps, sorted by startFrame
}

export interface AudioSegment {
  file: string; // absolute path of the audio file
  channels: 1 | 2;
  startFrame: number; // song frame where the segment starts
  lengthFrames: number; // ≥ 1
  regionStartFrame: number; // of the clip
  regionLengthFrames: number; // of the clip
  phaseFrames: number; // frames elapsed since the start of its clip
}

// ---- Result of an edit operation ----

/**
 * Why a function of core/commands/ did not apply its operation (04 §3.1, rule 2; 04 §10).
 * The function leaves the document untouched and returns this instead of throwing (DI-34).
 */
export type RejectionReason = "maxTracks" | "notFound";

export interface Rejection {
  rejected: RejectionReason;
}
