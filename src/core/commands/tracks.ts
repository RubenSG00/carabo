// Edit operations on tracks (04 §3.1). Each one changes the draft it receives and applies its
// own limits: if it cannot apply, it changes nothing and returns the reason.
// TODO(T-014): createAudioTrack, renameTrack, moveTrack, deleteTrack, setKit, setRowGain and
// setRowMute.
import { FIRST_BLOCK_BARS, MAX_SONG_FRAMES, MAX_TRACKS } from "../constants";
import { newPattern, newPatternBlock, newRhythmTrack } from "../factory";
import { newId, newZ } from "../ids";
import type { ProjectDoc, Rejection } from "../model";
import { barToStep, stepToFrame } from "../time";

/** Number of colours of the track palette: `colorIndex` is 0..7 (05 §3.1). */
const TRACK_COLORS = 8;

/**
 * A default name as the caller translates it (03 §14.2): `core/` does not know the languages,
 * so it receives the function that writes the name for a number, such as `(n) => "Ritmo " + n`
 * (DI-35).
 */
export type DefaultName = (n: number) => string;

/** The default name with the smallest positive N that is not among `taken` (RF-201). */
function freeName(defaultName: DefaultName, taken: readonly string[]): string {
  let n = 1;
  while (taken.includes(defaultName(n))) n += 1;
  return defaultName(n);
}

/**
 * RF-201: a rhythm track at the end of the list, with its empty «Pattern 1» and a block of it
 * from bar 1, `FIRST_BLOCK_BARS` long or as many whole bars as fit before the song limit.
 * Returns the id of the track.
 */
export function createRhythmTrack(
  doc: ProjectDoc,
  trackName: DefaultName,
  patternName: DefaultName,
): string | Rejection {
  if (doc.tracks.length >= MAX_TRACKS) return { rejected: "maxTracks" };

  const trackId = newId(doc, "trk");
  const pattern = newPattern(newId(doc, "pat"), patternName(1), doc.timeSignature);

  // RF-112 and INV-09: the block may not end after the song limit (05 §2.3).
  let lengthBars = FIRST_BLOCK_BARS;
  while (stepToFrame(barToStep(lengthBars, doc.timeSignature), doc.tempoBpm) > MAX_SONG_FRAMES) {
    lengthBars -= 1;
  }
  const block = newPatternBlock(newId(doc, "blk"), pattern.id, 0, lengthBars, newZ(doc));

  const name = freeName(
    trackName,
    doc.tracks.map((track) => track.name),
  );
  const colorIndex = doc.tracks.length % TRACK_COLORS;
  doc.tracks.push(newRhythmTrack(trackId, name, colorIndex, pattern, block));
  return trackId;
}
