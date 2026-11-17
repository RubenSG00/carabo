// Writes the 24 placeholder kit sounds of 05 §8.4 (07 §13.1).
// Usage: node tools/gen-placeholder-kits.mjs   (npm run kits)
// Each sound is assets/kits/<kit>/<file>: mono, 48 000 Hz, 16 bits. The output is deterministic:
// two runs write byte-identical files. The kit.json files are written by hand and not touched.

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { SAMPLE_RATE } from "../src/core/constants.ts";

/** Kits in the order of their index, which seeds the noise (05 §8.4). */
const KITS = ["acoustic", "electronic", "percussion"];

/** File of each row, in the order of roles of 05 §8.1 (05 §8.3). */
const FILES = [
  "low.wav",
  "snare.wav",
  "clap.wav",
  "hat-closed.wav",
  "hat-open.wav",
  "tom.wav",
  "accent.wav",
  "extra.wav",
];

/** Peak of every sound after normalizing. */
const PEAK = 0.8;
/** Linear fade-out at the end of every sound: 5 ms. */
const FADE_FRAMES = (SAMPLE_RATE * 5) / 1000;
/** Full scale of a 16-bit sample: x is written as round(x · 32767), as in 06 §16.2. */
const FULL_SCALE_16 = 32767;

// ---- Building blocks of the recipes. `t` is the time in seconds; `i`, the frame. ----

/** Envelope e^(−t/τa). */
const env = (tau) => (t) => Math.exp(-t / tau);

/** Sine wave of frequency f, starting at phase 0. */
const sine = (f) => (t) => Math.sin(2 * Math.PI * f * t);

/**
 * Sweep: a sine whose frequency at time t is fEnd + (fStart − fEnd) · e^(−t/τf). Its phase is
 * the integral of that frequency from 0, so it starts at phase 0.
 */
const sweep = (fStart, fEnd, tauF) => (t) =>
  Math.sin(2 * Math.PI * (fEnd * t + (fStart - fEnd) * tauF * (1 - Math.exp(-t / tauF))));

/** Square wave of integer frequency f: +1 in the first half of each period, −1 in the second. */
const square = (f) => (_t, i) => ((f * i) % SAMPLE_RATE < SAMPLE_RATE / 2 ? 1 : -1);

/** Linear rise from 0 to 1 over `seconds`. */
const rise = (seconds) => (t) => Math.min(1, t / seconds);

/** Frame of a time given in milliseconds. */
const msToFrames = (ms) => (ms * SAMPLE_RATE) / 1000;

/**
 * Envelope of a clap: three bursts at 0, 12 and 24 ms (τa 0.004) and a tail from 36 ms with the
 * given τa. Each part starts at amplitude 1 and the parts add up.
 */
const clapEnv = (tailTau) => (_t, i) => {
  const part = (startMs, tau) => {
    const start = msToFrames(startMs);
    return i < start ? 0 : Math.exp(-(i - start) / SAMPLE_RATE / tau);
  };
  return part(0, 0.004) + part(12, 0.004) + part(24, 0.004) + part(36, tailTau);
};

/** The high noise of the sound, read at frame i. */
const noise = (_t, _i, highNoise) => highNoise;

/** Product of several signals. */
const mul =
  (...parts) =>
  (t, i, n) =>
    parts.reduce((product, part) => product * part(t, i, n), 1);

/** Sum of several signals, each one scaled: sum([0.5, a], [0.7, b]). */
const sum =
  (...terms) =>
  (t, i, n) =>
    terms.reduce((total, [gain, part]) => total + gain * part(t, i, n), 0);

/** Recipes of 05 §8.4: for each kit, one { ms, signal } per row, in row order. */
const RECIPES = {
  acoustic: [
    { ms: 400, signal: mul(sweep(160, 55, 0.03), env(0.12)) },
    { ms: 300, signal: sum([0.5, mul(sine(190), env(0.06))], [0.7, mul(noise, env(0.09))]) },
    { ms: 120, signal: sum([0.6, mul(sine(800), env(0.02))], [0.5, mul(noise, env(0.01))]) },
    { ms: 80, signal: mul(noise, env(0.015)) },
    { ms: 500, signal: mul(noise, env(0.12)) },
    { ms: 400, signal: mul(sweep(180, 110, 0.04), env(0.15)) },
    { ms: 1500, signal: mul(noise, env(0.4)) },
    { ms: 800, signal: sum([0.4, mul(noise, env(0.25))], [0.3, mul(sine(5000), env(0.2))]) },
  ],
  electronic: [
    { ms: 600, signal: mul(sweep(220, 40, 0.05), env(0.25)) },
    { ms: 350, signal: sum([0.5, mul(sine(220), env(0.06))], [0.7, mul(noise, env(0.12))]) },
    { ms: 300, signal: mul(noise, clapEnv(0.07)) },
    { ms: 60, signal: mul(noise, env(0.012)) },
    { ms: 400, signal: mul(noise, env(0.1)) },
    { ms: 450, signal: mul(sweep(150, 70, 0.04), env(0.2)) },
    { ms: 1200, signal: mul(noise, env(0.3)) },
    { ms: 400, signal: mul(sum([0.5, square(540)], [0.5, square(800)]), env(0.12)) },
  ],
  percussion: [
    { ms: 300, signal: mul(sweep(140, 80, 0.02), env(0.08)) },
    { ms: 200, signal: sum([0.5, mul(sine(320), env(0.04))], [0.6, mul(noise, env(0.05))]) },
    { ms: 250, signal: mul(noise, clapEnv(0.05)) },
    { ms: 120, signal: mul(noise, rise(0.01), env(0.04)) },
    { ms: 350, signal: mul(noise, env(0.1)) },
    { ms: 200, signal: mul(sweep(400, 330, 0.02), env(0.07)) },
    { ms: 1000, signal: mul(sum([0.6, sine(2000)], [0.4, sine(2960)]), env(0.3)) },
    { ms: 120, signal: mul(sine(2500), env(0.03)) },
  ],
};

/**
 * High noise of 05 §8.4: the difference between each value of the congruential generator and
 * the previous one. The state starts at the seed, and the value of the seed itself is the
 * "previous" value of the first frame.
 */
export function highNoise(seed, frames) {
  const value = (state) => state / 2 ** 31 - 1;
  const out = new Float64Array(frames);
  let state = seed >>> 0;
  for (let i = 0; i < frames; i++) {
    const previous = value(state);
    // (state · 1664525 + 1013904223) mod 2^32, exact in 32-bit integer arithmetic.
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    out[i] = value(state) - previous;
  }
  return out;
}

/**
 * Samples of one sound, between −1 and 1: the recipe, then the linear fade over the last 5 ms
 * (the last frame is 0), then the peak normalized to 0.8.
 */
export function renderSound(kitIndex, row) {
  const { ms, signal } = RECIPES[KITS[kitIndex]][row];
  const frames = msToFrames(ms);
  const noiseValues = highNoise(1 + row + 10 * kitIndex, frames);
  const samples = new Float64Array(frames);
  for (let i = 0; i < frames; i++) {
    const fade = Math.min(1, (frames - 1 - i) / FADE_FRAMES);
    samples[i] = signal(i / SAMPLE_RATE, i, noiseValues[i]) * fade;
  }
  const peak = samples.reduce((max, x) => Math.max(max, Math.abs(x)), 0);
  return samples.map((x) => (x * PEAK) / peak);
}

/** A mono 16-bit PCM WAV file with these samples. */
export function encodeWav(samples) {
  const dataBytes = samples.length * 2;
  const bytes = Buffer.alloc(44 + dataBytes);
  bytes.write("RIFF", 0, "ascii");
  bytes.writeUInt32LE(36 + dataBytes, 4);
  bytes.write("WAVE", 8, "ascii");
  bytes.write("fmt ", 12, "ascii");
  bytes.writeUInt32LE(16, 16); // size of the fmt chunk
  bytes.writeUInt16LE(1, 20); // PCM
  bytes.writeUInt16LE(1, 22); // mono
  bytes.writeUInt32LE(SAMPLE_RATE, 24);
  bytes.writeUInt32LE(SAMPLE_RATE * 2, 28); // bytes per second
  bytes.writeUInt16LE(2, 32); // bytes per frame
  bytes.writeUInt16LE(16, 34); // bits per sample
  bytes.write("data", 36, "ascii");
  bytes.writeUInt32LE(dataBytes, 40);
  samples.forEach((x, i) => bytes.writeInt16LE(Math.round(x * FULL_SCALE_16), 44 + 2 * i));
  return bytes;
}

/** Writes the 24 sounds into `<dir>/<kit>/<file>` and returns the paths written. */
export function generateKits(dir) {
  const written = [];
  KITS.forEach((kit, kitIndex) => {
    mkdirSync(join(dir, kit), { recursive: true });
    FILES.forEach((file, row) => {
      const path = join(dir, kit, file);
      writeFileSync(path, encodeWav(renderSound(kitIndex, row)));
      written.push(path);
    });
  });
  return written;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    const dir = join(dirname(fileURLToPath(import.meta.url)), "..", "assets", "kits");
    for (const path of generateKits(dir)) console.log(`wrote ${path}`);
  } catch (error) {
    console.error(error);
    process.exitCode = 1;
  }
}
