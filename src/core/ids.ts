// Identifiers and z values of 05 §3.2. Both counters are part of the document, so undoing an
// operation also restores them. New identifiers are only created here (07 §8.2).
import type { ProjectDoc } from "./model";

/** Prefix of each kind of identifier (05 §3.2). */
export type IdPrefix = "trk" | "pat" | "blk" | "clip" | "src";

/** A new identifier `<prefix>-<n>`, with `n = nextId`; `nextId` then grows by 1. */
export function newId(doc: ProjectDoc, prefix: IdPrefix): string {
  const id = `${prefix}-${doc.nextId}`;
  doc.nextId += 1;
  return id;
}

/** The z of an element just created, moved or resized: `nextZ`, which then grows by 1 (RF-309). */
export function newZ(doc: ProjectDoc): number {
  const z = doc.nextZ;
  doc.nextZ += 1;
  return z;
}
