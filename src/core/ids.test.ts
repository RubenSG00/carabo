import { newProject } from "./factory";
import { newId, newZ } from "./ids";
import type { ProjectDoc } from "./model";

function emptyDoc(): ProjectDoc {
  return newProject();
}

describe("newId", () => {
  it("gives <prefix>-<nextId> and increases nextId by 1", () => {
    const doc = emptyDoc();
    expect(newId(doc, "trk")).toBe("trk-1");
    expect(doc.nextId).toBe(2);
  });

  it("shares one counter between every prefix, so numbers never repeat", () => {
    const doc = emptyDoc();
    const ids = [
      newId(doc, "trk"),
      newId(doc, "pat"),
      newId(doc, "blk"),
      newId(doc, "clip"),
      newId(doc, "src"),
    ];
    expect(ids).toEqual(["trk-1", "pat-2", "blk-3", "clip-4", "src-5"]);
    expect(doc.nextId).toBe(6);
  });
});

describe("newZ", () => {
  it("gives nextZ and increases it by 1", () => {
    const doc = emptyDoc();
    expect([newZ(doc), newZ(doc), newZ(doc)]).toEqual([1, 2, 3]);
    expect(doc.nextZ).toBe(4);
    expect(doc.nextId).toBe(1);
  });
});
