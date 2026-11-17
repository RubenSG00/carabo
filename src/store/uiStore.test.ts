import { useProjectStore } from "./projectStore";
import { useUiStore } from "./uiStore";

describe("uiStore", () => {
  it("starts in the Rhythm tab with nothing selected", () => {
    const { tab, selectedRhythmTrackId, selectedPatternId } = useUiStore.getState();
    expect(tab).toBe("rhythm");
    expect(selectedRhythmTrackId).toBeNull();
    expect(selectedPatternId).toBeNull();
  });

  it("changes the active tab", () => {
    useUiStore.getState().setTab("song");
    expect(useUiStore.getState().tab).toBe("song");
    useUiStore.getState().setTab("rhythm");
    expect(useUiStore.getState().tab).toBe("rhythm");
  });

  it("selects a rhythm track and a pattern, and nothing again, without touching the project", () => {
    const { doc, past } = useProjectStore.getState();
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    expect(useUiStore.getState()).toMatchObject({
      selectedRhythmTrackId: "trk-1",
      selectedPatternId: "pat-2",
    });
    useUiStore.getState().selectPattern(null, null);
    expect(useUiStore.getState()).toMatchObject({
      selectedRhythmTrackId: null,
      selectedPatternId: null,
    });
    expect(useProjectStore.getState().doc).toBe(doc);
    expect(useProjectStore.getState().past).toBe(past);
  });
});
