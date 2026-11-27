import presetsFile from "../../assets/presets/presets.json";
import type { TimeSignature } from "./model";
import { presetSteps, presetsFor, type RhythmPreset } from "./presets";

// The fifteen ready-made rhythms of 05 §9.2, copied from the specification in their order.
const expected: RhythmPreset[] = [
  {
    id: "rock-basic",
    name: { es: "Rock básico", en: "Basic rock" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x-------x-------",
      snare: "----x-------x---",
      hatClosed: "x-x-x-x-x-x-x-x-",
    },
  },
  {
    id: "rock-driving",
    name: { es: "Rock a corcheas", en: "Driving rock" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x-----x-x-x-----",
      snare: "----x-------x---",
      hatClosed: "x-x-x-x-x-x-x-x-",
      accent: "x---------------",
    },
  },
  {
    id: "pop",
    name: { es: "Pop", en: "Pop" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x------x--x-----",
      snare: "----x-------x---",
      clap: "----x-------x---",
      hatClosed: "x-x-x-x-x-x-x-x-",
    },
  },
  {
    id: "ballad",
    name: { es: "Balada", en: "Ballad" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x---------x-----",
      clap: "----x-------x---",
      hatClosed: "x-x-x-x-x-x-x-x-",
    },
  },
  {
    id: "funk",
    name: { es: "Funk", en: "Funk" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x--x--x---x--x--",
      snare: "----x--x-x--x---",
      hatClosed: "xxxxxxxxxxxxxxxx",
    },
  },
  {
    id: "disco",
    name: { es: "Disco", en: "Disco" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x---x---x---x---",
      snare: "----x-------x---",
      hatClosed: "x---x---x---x---",
      hatOpen: "--x---x---x---x-",
    },
  },
  {
    id: "hiphop",
    name: { es: "Hip-hop", en: "Hip-hop" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x-----x---x--x--",
      snare: "----x-------x---",
      hatClosed: "x-x-x-x-x-x-x-x-",
    },
  },
  {
    id: "reggae",
    name: { es: "Reggae", en: "Reggae" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "--------x-------",
      clap: "--------x-------",
      hatClosed: "x-x-x-x-x-x-x-x-",
    },
  },
  {
    id: "rumba",
    name: { es: "Rumba", en: "Rumba" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x-----x-----x---",
      snare: "----x-----x---x-",
      clap: "--x---x---x---x-",
      hatClosed: "x-x-x-x-x-x-x-x-",
    },
  },
  {
    id: "bossa",
    name: { es: "Bossa nova", en: "Bossa nova" },
    timeSignature: "4/4",
    bars: 2,
    rows: {
      low: "x-----x-x-----x-x-----x-x-----x-",
      clap: "x-----x-----x-------x-----x-----",
      hatClosed: "x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-",
    },
  },
  {
    id: "country",
    name: { es: "Country", en: "Country" },
    timeSignature: "4/4",
    bars: 1,
    rows: {
      low: "x-------x-------",
      snare: "----x-------x---",
      hatClosed: "x-xxx-xxx-xxx-xx",
    },
  },
  {
    id: "waltz",
    name: { es: "Vals", en: "Waltz" },
    timeSignature: "3/4",
    bars: 1,
    rows: {
      low: "x-----------",
      snare: "----x---x---",
      hatClosed: "x-x-x-x-x-x-",
    },
  },
  {
    id: "waltz-slow",
    name: { es: "Vals lento", en: "Slow waltz" },
    timeSignature: "3/4",
    bars: 1,
    rows: {
      low: "x-----------",
      clap: "----x---x---",
      hatClosed: "x---x---x---",
    },
  },
  {
    id: "ballad-68",
    name: { es: "Balada 6/8", en: "6/8 ballad" },
    timeSignature: "6/8",
    bars: 1,
    rows: {
      low: "x---------x-",
      snare: "------x-----",
      hatClosed: "x-x-x-x-x-x-",
    },
  },
  {
    id: "blues-68",
    name: { es: "Blues 6/8", en: "6/8 blues" },
    timeSignature: "6/8",
    bars: 1,
    rows: {
      low: "x-----------",
      snare: "------x-----",
      hatClosed: "x---x-x---x-",
    },
  },
];

const timeSignatures: TimeSignature[] = ["4/4", "3/4", "6/8"];

/** Steps per bar of each time signature (00 §4), written here from the specification. */
const stepsPerBar: Record<TimeSignature, number> = { "4/4": 16, "3/4": 12, "6/8": 12 };

describe("presets.json (05 §9)", () => {
  it("contains exactly the fifteen ready-made rhythms of 05 §9.2, in their order", () => {
    expect(presetsFile).toEqual({ presets: expected });
  });

  it("has only the keys of 05 §9.1 and identifiers that are lower case and unique", () => {
    const ids = presetsFile.presets.map((preset) => preset.id);
    expect(new Set(ids).size).toBe(ids.length);
    for (const preset of presetsFile.presets) {
      expect(Object.keys(preset).sort()).toEqual(["bars", "id", "name", "rows", "timeSignature"]);
      expect(Object.keys(preset.name).sort()).toEqual(["en", "es"]);
      expect(preset.id).toBe(preset.id.toLowerCase());
    }
  });

  it("measures bars · stepsPerBar characters of x and - in every row, with known roles", () => {
    const roles = ["low", "snare", "clap", "hatClosed", "hatOpen", "tom", "accent", "extra"];
    for (const preset of presetsFor("4/4").concat(presetsFor("3/4"), presetsFor("6/8"))) {
      expect([1, 2]).toContain(preset.bars);
      for (const [role, row] of Object.entries(preset.rows)) {
        expect(roles).toContain(role);
        expect(row).toMatch(/^[x-]+$/);
        expect(row).toHaveLength(preset.bars * stepsPerBar[preset.timeSignature]);
      }
    }
  });
});

describe("presetsFor", () => {
  it("RF-209 CA1: in 3/4 the list has exactly 2 presets", () => {
    expect(presetsFor("3/4").map((preset) => preset.id)).toEqual(["waltz", "waltz-slow"]);
  });

  it("gives 11 presets for 4/4, 2 for 3/4 and 2 for 6/8, in the order of the file (RF-209)", () => {
    expect(timeSignatures.map((timeSignature) => presetsFor(timeSignature).length)).toEqual([
      11, 2, 2,
    ]);
    for (const timeSignature of timeSignatures) {
      const list = presetsFor(timeSignature);
      expect(list.every((preset) => preset.timeSignature === timeSignature)).toBe(true);
      expect(list).toEqual(expected.filter((preset) => preset.timeSignature === timeSignature));
    }
  });
});

describe("presetSteps", () => {
  /** A row of `length` zeros with level 2 at the given steps. */
  function row(length: number, hits: number[]): number[] {
    return Array.from({ length }, (_, step) => (hits.includes(step) ? 2 : 0));
  }

  it("turns each x into a level 2 hit, in the row of its role, and leaves other roles empty", () => {
    const rockBasic = presetsFor("4/4")[0]!;
    expect(presetSteps(rockBasic)).toEqual([
      row(16, [0, 8]), // low
      row(16, [4, 12]), // snare
      row(16, []), // clap
      row(16, [0, 2, 4, 6, 8, 10, 12, 14]), // hatClosed
      row(16, []), // hatOpen
      row(16, []), // tom
      row(16, []), // accent
      row(16, []), // extra
    ]);
  });

  it("gives 8 rows of bars · stepsPerBar steps for every preset", () => {
    for (const preset of expected) {
      const steps = presetSteps(preset);
      expect(steps).toHaveLength(8);
      for (const stepsOfRow of steps) {
        expect(stepsOfRow).toHaveLength(preset.bars * stepsPerBar[preset.timeSignature]);
      }
    }
  });

  it("gives the two bars of a two-bar preset and the 12 steps of a 6/8 preset", () => {
    const bossa = presetsFor("4/4").find((preset) => preset.id === "bossa")!;
    expect(presetSteps(bossa)[3]).toEqual(
      row(32, [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30]),
    );
    const blues = presetsFor("6/8").find((preset) => preset.id === "blues-68")!;
    expect(presetSteps(blues)[1]).toEqual(row(12, [6]));
  });

  it("returns new arrays each time", () => {
    const preset = presetsFor("4/4")[0]!;
    const first = presetSteps(preset);
    first[0]![0] = 3;
    expect(presetSteps(preset)[0]![0]).toBe(2);
  });
});
