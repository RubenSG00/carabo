// The global constants of 00 §4. This is the only TypeScript file that holds these values
// (07 §7); engine/src/constants.rs holds the same names on the Rust side, and both are
// checked against fixtures/constants.json (08 §7).

export const APP_NAME = "Carabo";
export const PROJECT_FILE_NAME = "project.carabo";
export const FORMAT_VERSION = 1;

/** Internal sample rate, in Hz. */
export const SAMPLE_RATE = 48000;
/** Ticks per quarter note. */
export const PPQ = 960;
/** Ticks per step (a sixteenth note). */
export const TICKS_PER_STEP = 240;

/** Tempo in quarter notes per minute, always an integer. */
export const TEMPO_MIN = 40;
export const TEMPO_MAX = 240;
export const TEMPO_DEFAULT = 120;

/** Allowed time signatures; the initial one is "4/4". */
export const TIME_SIGNATURES = ["4/4", "3/4", "6/8"] as const;
export const STEPS_PER_BAR = { "4/4": 16, "3/4": 12, "6/8": 12 } as const satisfies Record<
  (typeof TIME_SIGNATURES)[number],
  number
>;

export const MAX_TRACKS = 16;
/** Maximum song length: 10 minutes. */
export const MAX_SONG_FRAMES = 28800000;
export const MAX_PATTERNS_PER_TRACK = 32;
/** Allowed pattern lengths, in bars. */
export const PATTERN_BARS = [1, 2, 4] as const;
export const KIT_ROWS = 8;
/** Length of the block placed on its own when a rhythm track is created. */
export const FIRST_BLOCK_BARS = 16;
/** Minimum audio clip length: 50 ms. */
export const MIN_CLIP_FRAMES = 2400;
/** Micro-fade length: 5 ms. */
export const MICROFADE_FRAMES = 240;
export const COUNT_IN_BARS = 1;
/** Round-trip delay assumed while there is no calibration: 50 ms. */
export const DEFAULT_LATENCY_FRAMES = 2400;
export const RECOVERY_INTERVAL_S = 120;
export const HISTORY_LIMIT = 200;
export const RECENTS_MAX = 10;
/** Maximum length of a track, pattern or project name. */
export const NAME_MAX_CHARS = 40;

/** Volume fader range; below FADER_MIN_DB the gain is 0. */
export const FADER_MIN_DB = -60;
export const FADER_MAX_DB = 6;
export const ROW_GAIN_MIN_DB = -48;
export const ROW_GAIN_MAX_DB = 6;
export const EQ_GAIN_MIN_DB = -12;
export const EQ_GAIN_MAX_DB = 12;
export const EQ_LOW_HZ = 150;
export const EQ_MID_HZ = 1000;
export const EQ_HIGH_HZ = 6000;

/** Gain of a hit by level: 0 off, 1 soft, 2 normal, 3 loud. */
export const LEVEL_GAINS = [0, 0.45, 0.75, 1.0] as const;
/** Simultaneous voices per rhythm track. */
export const MAX_VOICES = 32;
/** How many times per second the engine reports to the interface. */
export const TELEMETRY_HZ = 30;
/** Tail added when exporting: 2 seconds. */
export const EXPORT_TAIL_FRAMES = 96000;

/** Pixels per quarter note in the Song tab; the initial one is 24. */
export const ZOOM_LEVELS = [6, 12, 24, 48, 96, 192] as const;
export const WINDOW_MIN_WIDTH = 1280;
export const WINDOW_MIN_HEIGHT = 720;

/** HTTPS address of the Git `origin` remote when the repository was created (07 §7). */
export const REPOSITORY_URL = "https://github.com/RubenSG00/carabo";
