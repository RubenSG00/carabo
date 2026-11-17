// The covering rule (05 §10.2): turns the elements of a lane, which may overlap, into segments
// that do not. It is the same for clips (in frames) and blocks (in steps), and also serves to
// draw the lane.

/**
 * An element of a lane: the half-open range [start, end), integers with start < end. `id` is
 * whatever identifies the element to the caller: its `id` text or the element itself.
 */
export interface LaneItem<Id = string> {
  id: Id;
  start: number;
  end: number;
  z: number;
}

/** A visible piece of a lane element. */
export interface LaneSegment<Id = string> {
  id: Id;
  start: number;
  end: number;
}

/** A half-open range [start, end). */
type Range = readonly [start: number, end: number];

/** The parts of [start, end) that no range of `taken` covers. `taken` is sorted and disjoint. */
function uncovered(start: number, end: number, taken: readonly Range[]): Range[] {
  const free: Range[] = [];
  let from = start;
  for (const [takenStart, takenEnd] of taken) {
    if (takenStart >= end) break;
    if (takenEnd <= from) continue;
    if (takenStart > from) free.push([from, takenStart]);
    from = takenEnd;
  }
  if (from < end) free.push([from, end]);
  return free;
}

/** `taken` plus [start, end), still sorted and disjoint: the ranges it touches are merged. */
function take(taken: readonly Range[], start: number, end: number): Range[] {
  const result: Range[] = [];
  let merged: Range = [start, end];
  for (const range of taken) {
    if (range[1] < merged[0] || range[0] > merged[1]) {
      result.push(range);
    } else {
      merged = [Math.min(merged[0], range[0]), Math.max(merged[1], range[1])];
    }
  }
  result.push(merged);
  return result.sort((a, b) => a[0] - b[0]);
}

/** RF-309: in an overlap, only the element on top (the greatest z) sounds and is seen. */
export function resolveLane<Id>(items: readonly LaneItem<Id>[]): LaneSegment<Id>[] {
  const byZ = [...items].sort((a, b) => b.z - a.z);
  let taken: Range[] = [];
  const segments: LaneSegment<Id>[] = [];
  for (const item of byZ) {
    for (const [start, end] of uncovered(item.start, item.end, taken)) {
      segments.push({ id: item.id, start, end });
    }
    taken = take(taken, item.start, item.end);
  }
  return segments.sort((a, b) => a.start - b.start);
}
