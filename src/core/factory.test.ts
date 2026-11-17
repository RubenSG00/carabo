import {
  newChannelStrip,
  newPattern,
  newPatternBlock,
  newProject,
  newRhythmTrack,
  newRowSettings,
  newViewState,
} from "./factory";

// Initial values of 05 §3.5.

describe("newProject", () => {
  it("has the initial values of a new project and of the view state", () => {
    expect(newProject()).toEqual({
      formatVersion: 1,
      tempoBpm: 120,
      timeSignature: "4/4",
      tracks: [],
      sources: [],
      master: { gainDb: 0 },
      nextId: 1,
      nextZ: 1,
      view: {
        metronomeEnabled: false,
        loopEnabled: false,
        loopStartBar: 0,
        loopEndBar: 4,
        snapEnabled: true,
        zoom: 24,
      },
    });
  });

  it("returns a new object each time", () => {
    const a = newProject();
    const b = newProject();
    expect(a).not.toBe(b);
    expect(a.view).not.toBe(b.view);
    expect(newViewState()).toEqual(a.view);
  });
});

describe("newChannelStrip", () => {
  it("is centred, at 0 dB, with the EQ and the reverb off", () => {
    expect(newChannelStrip()).toEqual({
      gainDb: 0,
      pan: 0,
      mute: false,
      solo: false,
      effects: [
        { type: "eq", enabled: false, lowDb: 0, midDb: 0, highDb: 0 },
        { type: "reverb", enabled: false, amount: 20, size: 50 },
      ],
    });
  });
});

describe("newRowSettings", () => {
  it("is at 0 dB and not muted", () => {
    expect(newRowSettings()).toEqual({ gainDb: 0, mute: false });
  });
});

describe("newPattern", () => {
  it("has one bar of 16 empty steps per row in 4/4", () => {
    const pattern = newPattern("pat-2", "Patrón 1", "4/4");
    expect(pattern).toEqual({
      id: "pat-2",
      name: "Patrón 1",
      bars: 1,
      steps: Array.from({ length: 8 }, () => new Array<number>(16).fill(0)),
    });
  });

  it("has 12 steps per row in 3/4 and in 6/8", () => {
    for (const timeSignature of ["3/4", "6/8"] as const) {
      const { steps } = newPattern("pat-1", "Pattern 1", timeSignature);
      expect(steps).toHaveLength(8);
      expect(steps.every((row) => row.length === 12)).toBe(true);
    }
  });

  it("does not share rows between them", () => {
    const { steps } = newPattern("pat-1", "Patrón 1", "4/4");
    steps[0]![0] = 2;
    expect(steps[1]![0]).toBe(0);
  });
});

describe("newPatternBlock", () => {
  it("keeps the values it receives", () => {
    expect(newPatternBlock("blk-3", "pat-2", 0, 16, 1)).toEqual({
      id: "blk-3",
      patternId: "pat-2",
      startBar: 0,
      lengthBars: 16,
      z: 1,
    });
  });
});

describe("newRhythmTrack", () => {
  it("has the acoustic kit, 8 rows, the initial channel and one pattern and one block", () => {
    const pattern = newPattern("pat-2", "Patrón 1", "4/4");
    const block = newPatternBlock("blk-3", "pat-2", 0, 16, 1);
    expect(newRhythmTrack("trk-1", "Ritmo 1", 0, pattern, block)).toEqual({
      id: "trk-1",
      type: "rhythm",
      name: "Ritmo 1",
      colorIndex: 0,
      strip: newChannelStrip(),
      kitId: "acoustic",
      rows: Array.from({ length: 8 }, () => ({ gainDb: 0, mute: false })),
      patterns: [pattern],
      blocks: [block],
    });
  });
});
