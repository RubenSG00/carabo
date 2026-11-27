import { newProject } from "./factory";
import { newId, newZ, type IdPrefix } from "./ids";
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

describe("identifiers and z values together (05 §3.2)", () => {
  it("are unique, carry their prefix and grow, whatever the order of the calls", () => {
    const doc = emptyDoc();
    const prefixes: IdPrefix[] = ["trk", "pat", "blk", "clip", "src"];
    const ids: string[] = [];
    const zs: number[] = [];
    for (let i = 0; i < 100; i += 1) {
      const prefix = prefixes[(i * 3) % prefixes.length]!;
      const id = newId(doc, prefix);
      expect(id).toMatch(new RegExp(`^${prefix}-[1-9][0-9]*$`));
      ids.push(id);
      if (i % 2 === 0) zs.push(newZ(doc));
    }
    expect(new Set(ids).size).toBe(ids.length);
    const numbers = ids.map((id) => Number(id.split("-")[1]));
    expect(numbers).toEqual(Array.from({ length: 100 }, (_, i) => i + 1));
    expect(zs).toEqual(Array.from({ length: 50 }, (_, i) => i + 1));
    expect(doc.nextId).toBe(101);
    expect(doc.nextZ).toBe(51);
  });
});
