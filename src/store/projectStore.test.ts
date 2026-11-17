import { applyPatches } from "immer";
import { createRhythmTrack } from "../core/commands/tracks";
import { toggleStep } from "../core/commands/patterns";
import { HISTORY_LIMIT } from "../core/constants";
import { newProject } from "../core/factory";
import type { ProjectDoc } from "../core/model";
import { useProjectStore } from "./projectStore";

const rhythmName = (n: number) => `Ritmo ${n}`;
const patternName = (n: number) => `Patrón ${n}`;

function state() {
  return useProjectStore.getState();
}

function addRhythmTrack(draft: ProjectDoc) {
  return createRhythmTrack(draft, rhythmName, patternName);
}

beforeEach(() => {
  state().load(newProject(), "C:\\Temp\\carabo-1", true, []);
});

describe("projectStore", () => {
  it("starts with an empty project and no history", () => {
    const { view, ...doc } = newProject();
    expect(state().doc).toEqual(doc);
    expect(state().view).toEqual(view);
    expect(state().past).toEqual([]);
    expect(state().future).toEqual([]);
    expect(state().savedIndex).toBe(0);
    expect(state().gesture).toBeNull();
  });
});

describe("commit", () => {
  it("applies the operation and keeps its patches and inverse patches as one history entry", () => {
    const before = state().doc;
    state().commit("history.createRhythmTrack", addRhythmTrack);

    const after = state().doc;
    expect(after.tracks.map((t) => t.name)).toEqual(["Ritmo 1"]);
    expect(state().past).toHaveLength(1);
    const [entry] = state().past;
    expect(entry!.label).toBe("history.createRhythmTrack");
    expect(applyPatches(before, entry!.patches)).toEqual(after);
    expect(applyPatches(after, entry!.inversePatches)).toEqual(before);
  });

  it("does not change the previous document", () => {
    const before = state().doc;
    const copy = structuredClone(before);
    state().commit("history.createRhythmTrack", addRhythmTrack);
    expect(state().doc).not.toBe(before);
    expect(before).toEqual(copy);
  });

  it("returns what the operation returns", () => {
    expect(state().commit("history.createRhythmTrack", addRhythmTrack)).toBe("trk-1");
    expect(
      state().commit("history.toggleStep", (d) => toggleStep(d, "trk-9", "pat-2", 0, 0)),
    ).toEqual({
      rejected: "notFound",
    });
  });

  it("does nothing when the operation changes nothing", () => {
    const before = state().doc;
    state().commit("history.toggleStep", (d) => toggleStep(d, "trk-9", "pat-2", 0, 0));
    expect(state().doc).toBe(before);
    expect(state().past).toEqual([]);
  });

  it("adds one entry per change and empties the redo list", () => {
    state().commit("history.createRhythmTrack", addRhythmTrack);
    const [entry] = state().past;
    useProjectStore.setState({ future: [entry!] });

    state().commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    state().commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    expect(state().past.map((e) => e.label)).toEqual([
      "history.createRhythmTrack",
      "history.toggleStep",
      "history.toggleStep",
    ]);
    expect(state().future).toEqual([]);
  });

  it("keeps the last HISTORY_LIMIT entries and moves savedIndex with them", () => {
    state().commit("history.createRhythmTrack", addRhythmTrack);
    useProjectStore.setState({ savedIndex: 1 });
    for (let i = 1; i < HISTORY_LIMIT; i += 1) {
      state().commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    }
    expect(state().past).toHaveLength(HISTORY_LIMIT);
    expect(state().savedIndex).toBe(1);

    // One more: the oldest entry goes, and the saved point moves one place down.
    state().commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    expect(state().past).toHaveLength(HISTORY_LIMIT);
    expect(state().past[0]!.label).toBe("history.toggleStep");
    expect(state().savedIndex).toBe(0);

    // Once the saved point leaves the history it can no longer be reached.
    state().commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    expect(state().savedIndex).toBeNull();
    state().commit("history.toggleStep", (d) => toggleStep(d, "trk-1", "pat-2", 0, 0));
    expect(state().savedIndex).toBeNull();
  });

  it("is a programming error during a gesture", () => {
    const { doc } = state();
    useProjectStore.setState({
      gesture: { label: "history.moveBlock", baseDoc: doc, pending: null },
    });
    expect(() => state().commit("history.createRhythmTrack", addRhythmTrack)).toThrow();
    expect(state().doc).toBe(doc);
    expect(state().past).toEqual([]);
  });
});

describe("load", () => {
  it("replaces the project, its folder and its missing files, and empties the history", () => {
    state().commit("history.createRhythmTrack", addRhythmTrack);
    useProjectStore.setState({ future: [...state().past], savedIndex: null });

    const file = newProject();
    file.tempoBpm = 90;
    file.view.metronomeEnabled = true;
    state().load(file, "C:\\Users\\ana\\Documents\\Carabo\\Demo", false, ["audio/take-0001.wav"]);

    const { view, ...doc } = file;
    expect(state().doc).toEqual(doc);
    expect(state().view).toEqual(view);
    expect(state().dir).toBe("C:\\Users\\ana\\Documents\\Carabo\\Demo");
    expect(state().isTemp).toBe(false);
    expect(state().missingFiles).toEqual(["audio/take-0001.wav"]);
    expect(state().past).toEqual([]);
    expect(state().future).toEqual([]);
    expect(state().savedIndex).toBe(0);
    expect(state().gesture).toBeNull();
  });
});
