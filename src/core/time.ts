// Unit conversions of 05 §2. engine/src/time.rs implements the same formulas; both are
// checked against fixtures/formulas/time.json (08 §7). Every conversion is computed from
// the start of the song, never by accumulating increments (05 §2.2).

import { MAX_SONG_FRAMES, PPQ, SAMPLE_RATE, STEPS_PER_BAR, TICKS_PER_STEP } from "./constants";

type TimeSignature = keyof typeof STEPS_PER_BAR;

/** Frames per tick at one quarter note per minute: 48000 · 60 / 960 = 3000. */
const TICK_FRAMES_AT_1_BPM = (SAMPLE_RATE * 60) / PPQ;
/** Frames per step at one quarter note per minute: 3000 · 240 = 720000. */
const STEP_FRAMES_AT_1_BPM = TICK_FRAMES_AT_1_BPM * TICKS_PER_STEP;

// Glossary (00 §7): the beat is a quarter note in 4/4 and 3/4, and an eighth note in 6/8.
const TICKS_PER_BEAT = { "4/4": PPQ, "3/4": PPQ, "6/8": PPQ / 2 } as const satisfies Record<
  TimeSignature,
  number
>;

/** Frames per tick; not an integer in general. */
export function framesPerTick(tempoBpm: number): number {
  return TICK_FRAMES_AT_1_BPM / tempoBpm;
}

// Math.round sends halves up, as 05 §2.2 requires for non-negative operands.

export function tickToFrame(tick: number, tempoBpm: number): number {
  return Math.round((tick * TICK_FRAMES_AT_1_BPM) / tempoBpm);
}

export function frameToTick(frame: number, tempoBpm: number): number {
  return Math.round((frame * tempoBpm) / TICK_FRAMES_AT_1_BPM);
}

/** Frame of step `step`, counted from the start of the song. */
export function stepToFrame(step: number, tempoBpm: number): number {
  return Math.round((step * STEP_FRAMES_AT_1_BPM) / tempoBpm);
}

export function stepsPerBar(timeSignature: TimeSignature): number {
  return STEPS_PER_BAR[timeSignature];
}

export function ticksPerBar(timeSignature: TimeSignature): number {
  return STEPS_PER_BAR[timeSignature] * TICKS_PER_STEP;
}

export function ticksPerBeat(timeSignature: TimeSignature): number {
  return TICKS_PER_BEAT[timeSignature];
}

export function beatsPerBar(timeSignature: TimeSignature): number {
  return ticksPerBar(timeSignature) / ticksPerBeat(timeSignature);
}

export function barToTick(bar: number, timeSignature: TimeSignature): number {
  return bar * ticksPerBar(timeSignature);
}

export function barToStep(bar: number, timeSignature: TimeSignature): number {
  return bar * stepsPerBar(timeSignature);
}

/** Last tick that fits in the song at this tempo. */
export function maxSongTick(tempoBpm: number): number {
  return Math.floor((MAX_SONG_FRAMES * tempoBpm) / TICK_FRAMES_AT_1_BPM);
}

/** The two texts of the position display (RF-113). */
export interface PositionText {
  /** `bar.beat`, both counted from 1: "5.3". */
  barBeat: string;
  /** `minutes:seconds.tenths`: "0:09.5". */
  time: string;
}

/**
 * Position display of 05 §2.2 (RF-113). In pattern mode `tick` is the position within the
 * pattern. The texts use a dot whatever the language (DI-9), and the tenths are those already
 * reached, not rounded (DI-8).
 */
export function formatPosition(
  tick: number,
  tempoBpm: number,
  timeSignature: TimeSignature,
): PositionText {
  const barTicks = ticksPerBar(timeSignature);
  const bar = Math.floor(tick / barTicks) + 1;
  const beat = Math.floor((tick % barTicks) / ticksPerBeat(timeSignature)) + 1;
  const tenths = Math.floor((tickToFrame(tick, tempoBpm) * 10) / SAMPLE_RATE);
  const minutes = Math.floor(tenths / 600);
  const seconds = String(Math.floor((tenths % 600) / 10)).padStart(2, "0");
  return { barBeat: `${bar}.${beat}`, time: `${minutes}:${seconds}.${tenths % 10}` };
}
