import { resolveLane } from "./lanes";

describe("resolveLane", () => {
  it("resolves the example of 05 §10.2: B on top of A splits A in two", () => {
    const segments = resolveLane([
      { id: "A", start: 0, end: 400, z: 1 },
      { id: "B", start: 100, end: 300, z: 2 },
    ]);
    expect(segments).toEqual([
      { id: "A", start: 0, end: 100 },
      { id: "B", start: 100, end: 300 },
      { id: "A", start: 300, end: 400 },
    ]);
  });

  it("returns nothing for an empty lane", () => {
    expect(resolveLane([])).toEqual([]);
  });

  it("keeps elements that do not overlap, sorted by start", () => {
    const segments = resolveLane([
      { id: "B", start: 200, end: 300, z: 1 },
      { id: "A", start: 0, end: 100, z: 2 },
      { id: "C", start: 100, end: 200, z: 3 },
    ]);
    expect(segments).toEqual([
      { id: "A", start: 0, end: 100 },
      { id: "C", start: 100, end: 200 },
      { id: "B", start: 200, end: 300 },
    ]);
  });

  it("cuts the covered part of a partial overlap", () => {
    expect(
      resolveLane([
        { id: "A", start: 0, end: 200, z: 2 },
        { id: "B", start: 100, end: 300, z: 1 },
      ]),
    ).toEqual([
      { id: "A", start: 0, end: 200 },
      { id: "B", start: 200, end: 300 },
    ]);
    expect(
      resolveLane([
        { id: "A", start: 0, end: 200, z: 1 },
        { id: "B", start: 100, end: 300, z: 2 },
      ]),
    ).toEqual([
      { id: "A", start: 0, end: 100 },
      { id: "B", start: 100, end: 300 },
    ]);
  });

  it("hides an element entirely covered by another one", () => {
    expect(
      resolveLane([
        { id: "A", start: 100, end: 200, z: 1 },
        { id: "B", start: 0, end: 300, z: 2 },
      ]),
    ).toEqual([{ id: "B", start: 0, end: 300 }]);
  });

  it("resolves three levels of z", () => {
    const segments = resolveLane([
      { id: "A", start: 0, end: 1000, z: 1 },
      { id: "B", start: 200, end: 800, z: 2 },
      { id: "C", start: 400, end: 600, z: 3 },
    ]);
    expect(segments).toEqual([
      { id: "A", start: 0, end: 200 },
      { id: "B", start: 200, end: 400 },
      { id: "C", start: 400, end: 600 },
      { id: "B", start: 600, end: 800 },
      { id: "A", start: 800, end: 1000 },
    ]);
  });

  it("lets a lower element show through the gap between two higher ones", () => {
    const segments = resolveLane([
      { id: "A", start: 0, end: 100, z: 3 },
      { id: "B", start: 200, end: 300, z: 2 },
      { id: "C", start: 50, end: 250, z: 1 },
    ]);
    expect(segments).toEqual([
      { id: "A", start: 0, end: 100 },
      { id: "C", start: 100, end: 200 },
      { id: "B", start: 200, end: 300 },
    ]);
  });

  it("does not depend on the order of the input", () => {
    const items = [
      { id: "A", start: 0, end: 1000, z: 1 },
      { id: "B", start: 200, end: 800, z: 2 },
      { id: "C", start: 400, end: 600, z: 3 },
    ];
    expect(resolveLane([...items].reverse())).toEqual(resolveLane(items));
  });
});
