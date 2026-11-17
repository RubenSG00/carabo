import { newProject } from "../factory";
import type { ProjectDoc } from "../model";
import { toggleStep } from "./patterns";
import { createRhythmTrack } from "./tracks";

/** A project with one rhythm track (trk-1) and its empty pattern (pat-2). */
function projectWithRhythmTrack(): ProjectDoc {
  const doc = newProject();
  createRhythmTrack(
    doc,
    (n) => `Ritmo ${n}`,
    (n) => `Patrón ${n}`,
  );
  return doc;
}

function stepsOf(doc: ProjectDoc): number[][] {
  const track = doc.tracks[0];
  if (track?.type !== "rhythm") throw new Error("the first track is not a rhythm track");
  return track.patterns[0]!.steps;
}

describe("toggleStep", () => {
  it("turns an empty step into a level 2 hit", () => {
    const doc = projectWithRhythmTrack();
    expect(toggleStep(doc, "trk-1", "pat-2", 3, 5)).toBeUndefined();
    expect(stepsOf(doc)[3]![5]).toBe(2);
  });

  it("removes a hit, and toggling twice leaves the step as it was", () => {
    const doc = projectWithRhythmTrack();
    const before = structuredClone(doc);
    toggleStep(doc, "trk-1", "pat-2", 0, 0);
    toggleStep(doc, "trk-1", "pat-2", 0, 0);
    expect(doc).toEqual(before);
  });

  it("removes a hit of any other level", () => {
    const doc = projectWithRhythmTrack();
    stepsOf(doc)[1]![2] = 1;
    stepsOf(doc)[1]![3] = 3;
    toggleStep(doc, "trk-1", "pat-2", 1, 2);
    toggleStep(doc, "trk-1", "pat-2", 1, 3);
    expect(stepsOf(doc)[1]).toEqual(new Array<number>(16).fill(0));
  });

  it("only changes the step it is given", () => {
    const doc = projectWithRhythmTrack();
    toggleStep(doc, "trk-1", "pat-2", 7, 15);
    const expected = Array.from({ length: 8 }, () => new Array<number>(16).fill(0));
    expected[7]![15] = 2;
    expect(stepsOf(doc)).toEqual(expected);
  });

  it.each([
    ["an unknown track", "trk-9", "pat-2", 0, 0],
    ["a pattern of no track", "trk-1", "pat-9", 0, 0],
    ["the id of a block", "trk-1", "blk-3", 0, 0],
    ["a row below 0", "trk-1", "pat-2", -1, 0],
    ["a row past the kit", "trk-1", "pat-2", 8, 0],
    ["a step below 0", "trk-1", "pat-2", 0, -1],
    ["a step past the pattern", "trk-1", "pat-2", 0, 16],
    ["a fractional step", "trk-1", "pat-2", 0, 1.5],
  ])("rejects %s without changing anything", (_case, trackId, patternId, row, step) => {
    const doc = projectWithRhythmTrack();
    const before = structuredClone(doc);
    expect(toggleStep(doc, trackId, patternId, row, step)).toEqual({ rejected: "notFound" });
    expect(doc).toEqual(before);
  });
});
