// Static check «Sonidos sustitutos» (08 §10): the placeholder kit sounds in assets/kits/ are
// exactly what tools/gen-placeholder-kits.mjs writes, and they follow 05 §8.3 and §8.4.

import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { generateKits } from "./gen-placeholder-kits.mjs";

const KITS_DIR = join(dirname(fileURLToPath(import.meta.url)), "..", "assets", "kits");

/** Length of each sound, in milliseconds, as its recipe in 05 §8.4 says. */
const RECIPE_MS = {
  acoustic: {
    "low.wav": 400,
    "snare.wav": 300,
    "clap.wav": 120,
    "hat-closed.wav": 80,
    "hat-open.wav": 500,
    "tom.wav": 400,
    "accent.wav": 1500,
    "extra.wav": 800,
  },
  electronic: {
    "low.wav": 600,
    "snare.wav": 350,
    "clap.wav": 300,
    "hat-closed.wav": 60,
    "hat-open.wav": 400,
    "tom.wav": 450,
    "accent.wav": 1200,
    "extra.wav": 400,
  },
  percussion: {
    "low.wav": 300,
    "snare.wav": 200,
    "clap.wav": 250,
    "hat-closed.wav": 120,
    "hat-open.wav": 350,
    "tom.wav": 200,
    "accent.wav": 1000,
    "extra.wav": 120,
  },
};

const SOUNDS = Object.entries(RECIPE_MS).flatMap(([kit, files]) =>
  Object.entries(files).map(([file, ms]) => ({ kit, file, ms })),
);

/** Reads the format and the samples of a PCM WAV file, walking its chunks. */
function readWav(bytes) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const tag = (offset) => bytes.subarray(offset, offset + 4).toString("ascii");
  expect(tag(0)).toBe("RIFF");
  expect(view.getUint32(4, true)).toBe(bytes.length - 8);
  expect(tag(8)).toBe("WAVE");

  let format = null;
  let samples = null;
  for (let offset = 12; offset + 8 <= bytes.length;) {
    const size = view.getUint32(offset + 4, true);
    const body = offset + 8;
    if (tag(offset) === "fmt ") {
      format = {
        audioFormat: view.getUint16(body, true),
        channels: view.getUint16(body + 2, true),
        sampleRate: view.getUint32(body + 4, true),
        byteRate: view.getUint32(body + 8, true),
        blockAlign: view.getUint16(body + 12, true),
        bitsPerSample: view.getUint16(body + 14, true),
      };
    } else if (tag(offset) === "data") {
      samples = Array.from({ length: size / 2 }, (_, i) => view.getInt16(body + 2 * i, true));
    }
    offset = body + size + (size % 2);
  }
  return { format, samples };
}

describe("placeholder kit sounds", () => {
  let runs;

  beforeAll(() => {
    runs = [0, 1].map(() => mkdtempSync(join(tmpdir(), "carabo-kits-")));
    for (const dir of runs) generateKits(dir);
  });

  afterAll(() => {
    for (const dir of runs) rmSync(dir, { recursive: true, force: true });
  });

  it("writes the 24 files of 05 §8.3 and nothing else", () => {
    for (const dir of runs) {
      for (const kit of Object.keys(RECIPE_MS)) {
        expect(readdirSync(join(dir, kit)).sort()).toEqual(Object.keys(RECIPE_MS[kit]).sort());
      }
    }
  });

  it("two runs write byte-identical files, identical to those in assets/kits/", () => {
    for (const { kit, file } of SOUNDS) {
      const committed = readFileSync(join(KITS_DIR, kit, file));
      for (const dir of runs) {
        expect(readFileSync(join(dir, kit, file)).equals(committed), `${kit}/${file}`).toBe(true);
      }
    }
  });

  it("assets/kits/ holds only kit.json and the eight sounds of each kit", () => {
    for (const kit of Object.keys(RECIPE_MS)) {
      const expected = ["kit.json", ...Object.keys(RECIPE_MS[kit])].sort();
      expect(readdirSync(join(KITS_DIR, kit)).sort()).toEqual(expected);
    }
  });

  it.each(SOUNDS)("$kit/$file is mono, 48 000 Hz, 16 bits and lasts what its recipe says", (s) => {
    const { format, samples } = readWav(readFileSync(join(KITS_DIR, s.kit, s.file)));
    expect(format).toEqual({
      audioFormat: 1,
      channels: 1,
      sampleRate: 48000,
      byteRate: 96000,
      blockAlign: 2,
      bitsPerSample: 16,
    });
    expect(samples).toHaveLength(s.ms * 48);
    expect(samples.length).toBeLessThanOrEqual(4 * 48000);
  });

  it.each(SOUNDS)("$kit/$file has its peak at 0.8 and fades out to silence", (s) => {
    const { samples } = readWav(readFileSync(join(KITS_DIR, s.kit, s.file)));
    const peak = Math.max(...samples.map(Math.abs));
    expect(peak).toBe(Math.round(0.8 * 32767));
    expect(samples.at(-1)).toBe(0);
    // Linear fade over the last 5 ms (240 frames): each sample is at most the peak scaled by its
    // remaining share of the fade.
    const fade = samples.slice(-240);
    fade.forEach((value, i) => {
      expect(Math.abs(value)).toBeLessThanOrEqual(Math.ceil((peak * (239 - i)) / 240));
    });
  });

  it("generates into a folder that does not exist yet", () => {
    const dir = join(runs[0], "new", "kits");
    generateKits(dir);
    expect(existsSync(join(dir, "percussion", "extra.wav"))).toBe(true);
  });
});
