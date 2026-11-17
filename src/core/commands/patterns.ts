// Edit operations on patterns (04 §3.1). Each one changes the draft it receives and applies its
// own limits: if it cannot apply, it changes nothing and returns the reason.
// TODO(T-014): createPattern, duplicatePattern, renamePattern, deletePattern, setPatternBars,
// loadPreset and clearPattern.
import type { ProjectDoc, Rejection } from "../model";

/** Level of the hit that a click creates (RF-205). */
const CLICK_LEVEL = 2;

/** RF-205: a click on an empty step creates a level 2 hit; on a step with a hit, removes it. */
export function toggleStep(
  doc: ProjectDoc,
  trackId: string,
  patternId: string,
  row: number,
  step: number,
): Rejection | undefined {
  const track = doc.tracks.find((t) => t.id === trackId);
  const pattern =
    track?.type === "rhythm" ? track.patterns.find((p) => p.id === patternId) : undefined;
  const steps = pattern?.steps[row];
  const level = Number.isInteger(step) ? steps?.[step] : undefined;
  if (steps === undefined || level === undefined) return { rejected: "notFound" };

  steps[step] = level === 0 ? CLICK_LEVEL : 0;
  return undefined;
}
