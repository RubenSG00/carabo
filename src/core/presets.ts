// The ready-made rhythms (05 §9): `assets/presets/presets.json` is read when the interface is
// built. A preset gives the steps of each role, so it suits every kit (05 §8.1).
import presetsFile from "../../assets/presets/presets.json";
import { ROLES, type Role } from "./kits";
import type { TimeSignature } from "./model";
import { stepsPerBar } from "./time";

/** The content of `presets.json` (05 §9.1). */
export interface PresetsFile {
  presets: RhythmPreset[];
}

/** A ready-made rhythm (05 §9.1). */
export interface RhythmPreset {
  id: string; // stable identifier, in lower case
  name: { es: string; en: string };
  timeSignature: TimeSignature;
  bars: 1 | 2;
  rows: Partial<Record<Role, string>>; // by role: a string of (bars · stepsPerBar) characters
}

/** Level of each `x` of a preset when it is loaded (05 §9.1, RF-209). */
const PRESET_LEVEL = 2;

const PRESETS: readonly RhythmPreset[] = (presetsFile as PresetsFile).presets;

/** The presets of a time signature, in the order in which they are shown (05 §9.2, RF-209). */
export function presetsFor(timeSignature: TimeSignature): RhythmPreset[] {
  return PRESETS.filter((preset) => preset.timeSignature === timeSignature);
}

/**
 * The steps of a preset: 8 rows in row order, of `bars · stepsPerBar` steps each. Each `x` is a
 * level 2 hit and each `-` an empty step; a role that does not appear is an empty row (05 §9.1).
 */
export function presetSteps(preset: RhythmPreset): number[][] {
  const length = preset.bars * stepsPerBar(preset.timeSignature);
  return ROLES.map((role) => {
    const row = preset.rows[role] ?? "";
    return Array.from({ length }, (_, step) => (row[step] === "x" ? PRESET_LEVEL : 0));
  });
}
