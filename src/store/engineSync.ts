// Keeps the engine up to date (04 §3.2): whenever what must sound changes, it sends a new
// snapshot. The changes made during a gesture are sent too, so the user hears the result while
// dragging.
// TODO(T-066): forward the metronome and the loop of the view state.
import { bridge } from "../bridge";
import type { Bridge, Unsubscribe } from "../bridge/bridge";
import { buildEngineSnapshot } from "../core/snapshot";
import { useProjectStore } from "./projectStore";

/** 04 §3.2: the snapshot is sent at most once every 33 ms. */
const MIN_INTERVAL_MS = 33;

/**
 * Sends a snapshot to `target` on the animation frame after `doc`, `dir` or `missingFiles`
 * change, and never twice within `MIN_INTERVAL_MS`. Returns the function that stops it.
 */
export function startEngineSync(target: Pick<Bridge, "engineSetProject">): Unsubscribe {
  // The animation frame requested for a pending change; null when nothing is pending.
  let frame: number | null = null;
  let lastSentAt = -Infinity;

  const onFrame = (time: number) => {
    if (time - lastSentAt < MIN_INTERVAL_MS) {
      frame = requestAnimationFrame(onFrame);
      return;
    }
    frame = null;
    lastSentAt = time;
    const { doc, dir, missingFiles } = useProjectStore.getState();
    target.engineSetProject(buildEngineSnapshot(doc, dir, missingFiles)).catch(() => {
      // TODO(T-024): write the detail in the log and show E-150 (04 §6.5).
    });
  };

  const unsubscribe = useProjectStore.subscribe((state, previous) => {
    const changed =
      state.doc !== previous.doc ||
      state.dir !== previous.dir ||
      state.missingFiles !== previous.missingFiles;
    if (changed) frame ??= requestAnimationFrame(onFrame);
  });

  return () => {
    unsubscribe();
    if (frame !== null) cancelAnimationFrame(frame);
    frame = null;
  };
}

// The engine follows the project for as long as the interface runs (DI-37).
startEngineSync(bridge);
