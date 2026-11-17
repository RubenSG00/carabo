import { mockControl } from "../bridge/mockBridge";
import { useUiStore } from "../store/uiStore";
import { play, stop } from "./playback";

beforeEach(() => {
  mockControl.clearCalls();
  useUiStore.getState().selectPattern(null, null);
});

describe("playback", () => {
  it("plays the selected pattern of the selected rhythm track in pattern mode", async () => {
    useUiStore.getState().selectPattern("trk-1", "pat-2");
    await play();
    expect(mockControl.calls).toEqual([
      { method: "enginePlay", args: [{ mode: "pattern", trackId: "trk-1", patternId: "pat-2" }] },
    ]);
  });

  it("does not play when no pattern is selected", async () => {
    await play();
    useUiStore.getState().selectPattern("trk-1", null);
    await play();
    expect(mockControl.calls).toEqual([]);
  });

  it("stops", async () => {
    await stop();
    expect(mockControl.calls).toEqual([{ method: "engineStop", args: [] }]);
  });
});
