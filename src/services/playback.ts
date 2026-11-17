// Play and stop (04 §3.4, §7.3). For now it always plays in pattern mode, the pattern selected
// in the Rhythm tab.
// TODO(T-034): choose the mode from the active tab (RF-102) and follow the selection while
// playing.
// TODO(T-064): song mode, the cursor and the position it returns to.
import { bridge } from "../bridge";
import { useUiStore } from "../store/uiStore";

/** RF-101: plays the selected pattern in a loop. Without a selected pattern it does nothing. */
export async function play(): Promise<void> {
  const { selectedRhythmTrackId: trackId, selectedPatternId: patternId } = useUiStore.getState();
  if (trackId === null || patternId === null) return;
  // TODO(T-024): turn NO_OUTPUT_DEVICE into E-110 (04 §6.5).
  await bridge.enginePlay({ mode: "pattern", trackId, patternId });
}

export async function stop(): Promise<void> {
  await bridge.engineStop();
}
