import { createRhythmTrack } from "./commands/tracks";
import { newPattern, newPatternBlock, newProject } from "./factory";
import type { ProjectDoc, RhythmTrack } from "./model";
import { buildEngineSnapshot } from "./snapshot";

const DIR = "C:\\Users\\ana\\Documents\\Carabo\\Demo";

function addRhythmTrack(doc: ProjectDoc): RhythmTrack {
  createRhythmTrack(
    doc,
    (n) => `Ritmo ${n}`,
    (n) => `Patrón ${n}`,
  );
  const track = doc.tracks.at(-1);
  if (track?.type !== "rhythm") throw new Error("no rhythm track was created");
  return track;
}

describe("buildEngineSnapshot", () => {
  it("copies tempo, time signature and master of an empty project", () => {
    const doc = newProject();
    doc.tempoBpm = 100;
    doc.timeSignature = "3/4";
    doc.master.gainDb = -3;
    expect(buildEngineSnapshot(doc, DIR, [])).toEqual({
      snapshotVersion: 1,
      tempoBpm: 100,
      timeSignature: "3/4",
      tracks: [],
      master: { gainDb: -3 },
    });
  });

  it("gives a new rhythm track its pattern and one segment of 16 bars, as in 05 §10.4", () => {
    const doc = newProject();
    const track = addRhythmTrack(doc);
    track.patterns[0]!.steps[0]![0] = 2;
    track.strip.gainDb = -3;
    track.rows[2]!.mute = true;

    expect(buildEngineSnapshot(doc, DIR, [])).toEqual({
      snapshotVersion: 1,
      tempoBpm: 120,
      timeSignature: "4/4",
      tracks: [
        {
          id: "trk-1",
          type: "rhythm",
          strip: track.strip,
          kitId: "acoustic",
          rows: track.rows,
          patterns: [{ id: "pat-2", lengthSteps: 16, steps: track.patterns[0]!.steps }],
          segments: [{ patternId: "pat-2", startStep: 0, lengthSteps: 256, phaseSteps: 0 }],
        },
      ],
      master: { gainDb: 0 },
    });
  });

  it("measures patterns and blocks in steps of the time signature", () => {
    const doc = newProject();
    doc.timeSignature = "6/8";
    const track = addRhythmTrack(doc);
    track.patterns[0]!.bars = 2;
    track.patterns[0]!.steps = track.patterns[0]!.steps.map(() => new Array<number>(24).fill(0));
    track.blocks[0]!.startBar = 3;
    track.blocks[0]!.lengthBars = 5;

    const [snapshotTrack] = buildEngineSnapshot(doc, DIR, []).tracks;
    expect(snapshotTrack).toMatchObject({
      patterns: [{ id: "pat-2", lengthSteps: 24 }],
      segments: [{ patternId: "pat-2", startStep: 36, lengthSteps: 60, phaseSteps: 0 }],
    });
  });

  it("lists every pattern of the track, also those without a block", () => {
    const doc = newProject();
    const track = addRhythmTrack(doc);
    track.patterns.push(newPattern("pat-9", "Patrón 2", "4/4"));
    const [snapshotTrack] = buildEngineSnapshot(doc, DIR, []).tracks;
    expect(snapshotTrack).toMatchObject({
      patterns: [
        { id: "pat-2", lengthSteps: 16 },
        { id: "pat-9", lengthSteps: 16 },
      ],
      segments: [{ patternId: "pat-2", startStep: 0, lengthSteps: 256, phaseSteps: 0 }],
    });
  });

  it("splits a covered block and keeps the phase of each visible piece (05 §10.3)", () => {
    const doc = newProject();
    const track = addRhythmTrack(doc);
    track.patterns.push(newPattern("pat-9", "Patrón 2", "4/4"));
    // pat-2 covers bars 1 to 16 (z 1); pat-9 goes on top in bars 5 and 6 (z 2).
    track.blocks.push(newPatternBlock("blk-10", "pat-9", 4, 2, 2));

    const [snapshotTrack] = buildEngineSnapshot(doc, DIR, []).tracks;
    expect(snapshotTrack).toMatchObject({
      segments: [
        { patternId: "pat-2", startStep: 0, lengthSteps: 64, phaseSteps: 0 },
        { patternId: "pat-9", startStep: 64, lengthSteps: 32, phaseSteps: 0 },
        { patternId: "pat-2", startStep: 96, lengthSteps: 160, phaseSteps: 96 },
      ],
    });
  });

  it("gives a block whose start is covered the phase from its own start", () => {
    const doc = newProject();
    const track = addRhythmTrack(doc);
    track.blocks[0]!.startBar = 2;
    track.blocks[0]!.lengthBars = 4;
    track.blocks.push(newPatternBlock("blk-10", "pat-2", 0, 3, 2));

    const [snapshotTrack] = buildEngineSnapshot(doc, DIR, []).tracks;
    expect(snapshotTrack).toMatchObject({
      segments: [
        { patternId: "pat-2", startStep: 0, lengthSteps: 48, phaseSteps: 0 },
        { patternId: "pat-2", startStep: 48, lengthSteps: 48, phaseSteps: 16 },
      ],
    });
  });

  it("keeps the order of the tracks", () => {
    const doc = newProject();
    addRhythmTrack(doc);
    addRhythmTrack(doc);
    addRhythmTrack(doc);
    expect(buildEngineSnapshot(doc, DIR, []).tracks.map((t) => t.id)).toEqual([
      "trk-1",
      "trk-4",
      "trk-7",
    ]);
  });
});
