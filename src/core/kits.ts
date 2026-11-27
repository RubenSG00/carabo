// The kits as the interface needs them (05 §8): their `kit.json` files are read when the
// interface is built. The interface only uses the names; the engine loads the sounds.
import acoustic from "../../assets/kits/acoustic/kit.json";
import electronic from "../../assets/kits/electronic/kit.json";
import percussion from "../../assets/kits/percussion/kit.json";
import type { KitId } from "./model";

/** The languages of the interface (RF-701). */
export type Language = "es" | "en";

/** The role of each row of a kit (05 §8.1): every kit has its 8 sounds in this order. */
export type Role = "low" | "snare" | "clap" | "hatClosed" | "hatOpen" | "tom" | "accent" | "extra";

/** The roles in row order (05 §8.1): `ROLES[row]` is the role of that row. */
export const ROLES: readonly Role[] = [
  "low",
  "snare",
  "clap",
  "hatClosed",
  "hatOpen",
  "tom",
  "accent",
  "extra",
];

type Names = Record<Language, string>;

/** The content of a `kit.json` file (05 §8.3). */
interface KitFile {
  id: string;
  name: Names;
  sounds: { role: string; file: string; name: Names; chokeGroup: string | null }[];
}

const KITS: Record<KitId, KitFile> = { acoustic, electronic, percussion };

/** The names of the sounds of a kit, in row order: the names of the rows of the grid. */
export function soundNames(kitId: KitId, language: Language): string[] {
  return KITS[kitId].sounds.map((sound) => sound.name[language]);
}

/** The visible name of a kit (05 §8.2). */
export function kitName(kitId: KitId, language: Language): string {
  return KITS[kitId].name[language];
}
