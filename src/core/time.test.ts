import { TEMPO_MAX, TEMPO_MIN, TICKS_PER_STEP, TIME_SIGNATURES } from "./constants";
import {
  barToStep,
  barToTick,
  beatsPerBar,
  formatPosition,
  frameToTick,
  framesPerTick,
  maxSongTick,
  stepToFrame,
  stepsPerBar,
  tickToFrame,
  ticksPerBar,
  ticksPerBeat,
} from "./time";
import cases from "../../fixtures/formulas/time.json";

type Signature = (typeof TIME_SIGNATURES)[number];

function asNumber(value: number | string | undefined): number {
  if (typeof value !== "number") throw new Error(`expected a number, got ${String(value)}`);
  return value;
}

function asSignature(value: number | string | undefined): Signature {
  const signature = TIME_SIGNATURES.find((s) => s === value);
  if (signature === undefined) throw new Error(`unknown time signature ${String(value)}`);
  return signature;
}

function evaluate(fn: string, args: readonly (number | string)[]): number {
  const [first, second] = args;
  switch (fn) {
    case "tickToFrame":
      return tickToFrame(asNumber(first), asNumber(second));
    case "frameToTick":
      return frameToTick(asNumber(first), asNumber(second));
    case "stepToFrame":
      return stepToFrame(asNumber(first), asNumber(second));
    case "barToTick":
      return barToTick(asNumber(first), asSignature(second));
    case "barToStep":
      return barToStep(asNumber(first), asSignature(second));
    case "maxSongTick":
      return maxSongTick(asNumber(first));
    default:
      throw new Error(`unknown function ${fn}`);
  }
}

const allTempos = Array.from({ length: TEMPO_MAX - TEMPO_MIN + 1 }, (_, i) => TEMPO_MIN + i);

describe("time formulas of 05 §2.2 (contract with fixtures/formulas/time.json, 08 §7)", () => {
  it("the fixture covers every formula", () => {
    const covered = new Set(cases.map((c) => c.fn));
    expect([...covered].sort()).toEqual(
      ["barToStep", "barToTick", "frameToTick", "maxSongTick", "stepToFrame", "tickToFrame"].sort(),
    );
  });

  it.each(cases)("$fn($args) = $result", ({ fn, args, result }) => {
    expect(evaluate(fn, args)).toBe(result);
  });
});

describe("time formulas of 05 §2.2", () => {
  it("at 97 bpm, stepToFrame(1) is 7423 and stepToFrame(2) is 14845", () => {
    expect(stepToFrame(1, 97)).toBe(7423);
    expect(stepToFrame(2, 97)).toBe(14845);
  });

  it("at 120 bpm a step lasts 6000 frames", () => {
    expect(stepToFrame(1, 120)).toBe(6000);
  });

  it("framesPerTick is 3000 / tempoBpm", () => {
    expect(framesPerTick(120)).toBe(25);
    expect(framesPerTick(100)).toBe(30);
    expect(framesPerTick(97)).toBeCloseTo(30.927835, 6);
  });

  it("rounds halves up", () => {
    // 1 tick at 240 bpm is 12.5 frames.
    expect(tickToFrame(1, 240)).toBe(13);
    // 75 frames at 60 bpm are 1.5 ticks.
    expect(frameToTick(75, 60)).toBe(2);
  });

  it("matches the example of 05 §5.3: at 100 bpm tick 15360 is frame 460800", () => {
    expect(tickToFrame(15360, 100)).toBe(460800);
  });
});

describe("units of 05 §2.1", () => {
  it.each([
    ["4/4", 3840, 16, 4, 960],
    ["3/4", 2880, 12, 3, 960],
    ["6/8", 2880, 12, 6, 480],
  ] as const)(
    "%s has %i ticks and %i steps per bar, and %i beats of %i ticks",
    (signature, ticks, steps, beats, beatTicks) => {
      expect(ticksPerBar(signature)).toBe(ticks);
      expect(stepsPerBar(signature)).toBe(steps);
      expect(beatsPerBar(signature)).toBe(beats);
      expect(ticksPerBeat(signature)).toBe(beatTicks);
    },
  );
});

describe("round trips between ticks, steps, bars and frames", () => {
  it("bars give the same tick through steps and directly", () => {
    const mismatches: string[] = [];
    for (const signature of TIME_SIGNATURES) {
      for (let bar = 0; bar <= 1000; bar++) {
        if (barToStep(bar, signature) * TICKS_PER_STEP !== barToTick(bar, signature)) {
          mismatches.push(`${signature} bar ${bar}`);
        }
      }
    }
    expect(mismatches).toEqual([]);
  });

  it("a step gives the same frame as its first tick", () => {
    const mismatches: string[] = [];
    for (const tempo of allTempos) {
      for (let step = 0; step <= 2000; step++) {
        if (stepToFrame(step, tempo) !== tickToFrame(step * TICKS_PER_STEP, tempo)) {
          mismatches.push(`${tempo} bpm step ${step}`);
        }
      }
    }
    expect(mismatches).toEqual([]);
  });

  it("a tick converted to frames and back is the same tick", () => {
    const mismatches: string[] = [];
    for (const tempo of allTempos) {
      const ticks = [...Array.from({ length: 4001 }, (_, i) => i), maxSongTick(tempo)];
      for (const tick of ticks) {
        if (frameToTick(tickToFrame(tick, tempo), tempo) !== tick) {
          mismatches.push(`${tempo} bpm tick ${tick}`);
        }
      }
    }
    expect(mismatches).toEqual([]);
  });
});

describe("position display (05 §2.2, RF-113)", () => {
  it("RF-113 CA1: at 120 bpm in 4/4 the start of bar 5 shows as 5.1 and 0:08.0", () => {
    const startOfBar5 = barToTick(4, "4/4");
    expect(formatPosition(startOfBar5, 120, "4/4")).toEqual({ barBeat: "5.1", time: "0:08.0" });
  });

  it("the start of the song shows as 1.1 and 0:00.0", () => {
    expect(formatPosition(0, 120, "4/4")).toEqual({ barBeat: "1.1", time: "0:00.0" });
  });

  it("shows the beat within the bar and the tenths of a second", () => {
    // 120 bpm, 4/4: 9.5 s are 18240 ticks, beat 4 of bar 5.
    expect(formatPosition(18240, 120, "4/4")).toEqual({ barBeat: "5.4", time: "0:09.5" });
  });

  it("counts quarter-note beats in 3/4 and eighth-note beats in 6/8", () => {
    expect(formatPosition(2880 + 2 * 960, 90, "3/4").barBeat).toBe("2.3");
    expect(formatPosition(5 * 480, 90, "6/8").barBeat).toBe("1.6");
    expect(formatPosition(2880, 90, "6/8").barBeat).toBe("2.1");
  });

  it("shows minutes and pads the seconds to two digits", () => {
    // 120 bpm: 70.25 s are 3 372 000 frames, that is 134 880 ticks.
    expect(formatPosition(134880, 120, "4/4")).toEqual({ barBeat: "36.1", time: "1:10.2" });
  });

  it("truncates to the tenth of a second already reached", () => {
    // 120 bpm: 19160 ticks are 479 000 frames, 9.979 s.
    expect(formatPosition(19160, 120, "4/4")).toEqual({ barBeat: "5.4", time: "0:09.9" });
  });

  it("shows the end of the song as 10:00.0", () => {
    expect(formatPosition(maxSongTick(120), 120, "4/4")).toEqual({
      barBeat: "301.1",
      time: "10:00.0",
    });
  });
});
