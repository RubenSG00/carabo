import { MAX_TRACKS } from "../constants";
import { newChannelStrip, newProject } from "../factory";
import type { ProjectDoc, RhythmTrack } from "../model";
import { createRhythmTrack } from "./tracks";

// Default names as the interface gives them in each language (03 §14.2).
const rhythmEs = (n: number) => `Ritmo ${n}`;
const patternEs = (n: number) => `Patrón ${n}`;
const rhythmEn = (n: number) => `Rhythm ${n}`;
const patternEn = (n: number) => `Pattern ${n}`;

function rhythmTrack(doc: ProjectDoc, index: number): RhythmTrack {
  const track = doc.tracks[index];
  if (track?.type !== "rhythm") throw new Error(`track ${index} is not a rhythm track`);
  return track;
}

describe("createRhythmTrack", () => {
  it("RF-201 CA1: in an empty project it creates «Ritmo 1» with a block of «Patrón 1» in bars 1 to 16", () => {
    const doc = newProject();
    createRhythmTrack(doc, rhythmEs, patternEs);

    expect(doc.tracks).toHaveLength(1);
    const track = rhythmTrack(doc, 0);
    expect(track.name).toBe("Ritmo 1");
    expect(track.blocks).toHaveLength(1);
    const [block] = track.blocks;
    const pattern = track.patterns.find((p) => p.id === block!.patternId);
    expect(pattern?.name).toBe("Patrón 1");
    // Bar 1 on screen is bar 0 in the data (00 §3.5): bars 1 to 16 are [0, 16).
    expect(block!.startBar).toBe(0);
    expect(block!.lengthBars).toBe(16);
  });

  it("creates the track, its empty pattern and its block with the values of 05 §3.5", () => {
    const doc = newProject();
    const id = createRhythmTrack(doc, rhythmEs, patternEs);

    expect(id).toBe("trk-1");
    expect(doc.tracks).toEqual([
      {
        id: "trk-1",
        type: "rhythm",
        name: "Ritmo 1",
        colorIndex: 0,
        strip: newChannelStrip(),
        kitId: "acoustic",
        rows: Array.from({ length: 8 }, () => ({ gainDb: 0, mute: false })),
        patterns: [
          {
            id: "pat-2",
            name: "Patrón 1",
            bars: 1,
            steps: Array.from({ length: 8 }, () => new Array<number>(16).fill(0)),
          },
        ],
        blocks: [{ id: "blk-3", patternId: "pat-2", startBar: 0, lengthBars: 16, z: 1 }],
      },
    ]);
    expect(doc.nextId).toBe(4);
    expect(doc.nextZ).toBe(2);
  });

  it("uses the names it receives, in the language of the caller", () => {
    const doc = newProject();
    createRhythmTrack(doc, rhythmEn, patternEn);
    const track = rhythmTrack(doc, 0);
    expect(track.name).toBe("Rhythm 1");
    expect(track.patterns[0]!.name).toBe("Pattern 1");
  });

  it("makes the pattern as long as one bar of the time signature", () => {
    const doc = newProject();
    doc.timeSignature = "6/8";
    createRhythmTrack(doc, rhythmEs, patternEs);
    const { steps } = rhythmTrack(doc, 0).patterns[0]!;
    expect(steps).toHaveLength(8);
    expect(steps.every((row) => row.length === 12)).toBe(true);
  });

  it("adds the track at the end, with N the smallest positive integer whose name is free", () => {
    const doc = newProject();
    createRhythmTrack(doc, rhythmEs, patternEs);
    createRhythmTrack(doc, rhythmEs, patternEs);
    createRhythmTrack(doc, rhythmEs, patternEs);
    expect(doc.tracks.map((t) => t.name)).toEqual(["Ritmo 1", "Ritmo 2", "Ritmo 3"]);

    // With «Ritmo 1» renamed, «Ritmo 1» is free again and is the next name.
    doc.tracks[0]!.name = "Base";
    createRhythmTrack(doc, rhythmEs, patternEs);
    expect(doc.tracks.map((t) => t.name)).toEqual(["Base", "Ritmo 2", "Ritmo 3", "Ritmo 1"]);
  });

  it("gives each new track the number of tracks there were before, modulo 8, as colour", () => {
    const doc = newProject();
    for (let i = 0; i < 10; i += 1) createRhythmTrack(doc, rhythmEs, patternEs);
    expect(doc.tracks.map((t) => t.colorIndex)).toEqual([0, 1, 2, 3, 4, 5, 6, 7, 0, 1]);
  });

  it("gives unique identifiers and growing z values", () => {
    const doc = newProject();
    createRhythmTrack(doc, rhythmEs, patternEs);
    createRhythmTrack(doc, rhythmEs, patternEs);
    const second = rhythmTrack(doc, 1);
    expect(second.id).toBe("trk-4");
    expect(second.patterns[0]!.id).toBe("pat-5");
    expect(second.blocks[0]).toEqual({
      id: "blk-6",
      patternId: "pat-5",
      startBar: 0,
      lengthBars: 16,
      z: 2,
    });
    expect(doc.nextId).toBe(7);
    expect(doc.nextZ).toBe(3);
  });

  it("allows MAX_TRACKS tracks and rejects one more without changing anything", () => {
    const doc = newProject();
    for (let i = 0; i < MAX_TRACKS; i += 1) {
      expect(createRhythmTrack(doc, rhythmEs, patternEs)).toBe(`trk-${3 * i + 1}`);
    }
    expect(doc.tracks).toHaveLength(MAX_TRACKS);

    const before = structuredClone(doc);
    expect(createRhythmTrack(doc, rhythmEs, patternEs)).toEqual({ rejected: "maxTracks" });
    expect(doc).toEqual(before);
  });

  it("places the first block at the lowest tempo without passing the song limit", () => {
    const doc = newProject();
    doc.tempoBpm = 40;
    doc.timeSignature = "3/4";
    createRhythmTrack(doc, rhythmEs, patternEs);
    expect(rhythmTrack(doc, 0).blocks[0]!.lengthBars).toBe(16);
  });
});
