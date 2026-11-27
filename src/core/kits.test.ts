import { kitName, ROLES, soundNames } from "./kits";
import type { KitId } from "./model";

// Every `kit.json` and every WAV file of assets/kits/, as Vite sees them when it builds.
const kitFiles = import.meta.glob<unknown>("../../assets/kits/*/kit.json", {
  eager: true,
  import: "default",
});
const wavFiles = Object.keys(import.meta.glob("../../assets/kits/*/*.wav"));

const kitIds: KitId[] = ["acoustic", "electronic", "percussion"];

// The roles of 05 §8.1, in row order, and the fixed file name of each one (05 §8.3).
const roles = ["low", "snare", "clap", "hatClosed", "hatOpen", "tom", "accent", "extra"];
const files = [
  "low.wav",
  "snare.wav",
  "clap.wav",
  "hat-closed.wav",
  "hat-open.wav",
  "tom.wav",
  "accent.wav",
  "extra.wav",
];

// Names of the kits and of their sounds, by row (05 §8.2).
const names = {
  acoustic: {
    kit: { es: "Batería acústica", en: "Acoustic drums" },
    es: ["Bombo", "Caja", "Aro", "Charles cerrado", "Charles abierto", "Tom", "Crash", "Ride"],
    en: ["Kick", "Snare", "Rimshot", "Closed hi-hat", "Open hi-hat", "Tom", "Crash", "Ride"],
  },
  electronic: {
    kit: { es: "Batería electrónica", en: "Electronic drums" },
    es: [
      "Bombo",
      "Caja",
      "Palmada",
      "Charles cerrado",
      "Charles abierto",
      "Tom",
      "Platillo",
      "Cencerro",
    ],
    en: ["Kick", "Snare", "Clap", "Closed hi-hat", "Open hi-hat", "Tom", "Cymbal", "Cowbell"],
  },
  percussion: {
    kit: { es: "Percusión", en: "Percussion" },
    es: [
      "Cajón grave",
      "Cajón agudo",
      "Palmas",
      "Shaker",
      "Pandereta",
      "Bongó",
      "Campana",
      "Claves",
    ],
    en: ["Cajón low", "Cajón high", "Handclaps", "Shaker", "Tambourine", "Bongo", "Bell", "Claves"],
  },
} as const;

// Choke groups by row (05 §8.2): rows 3 and 4 share "hat" in acoustic and electronic.
const hat = [null, null, null, "hat", "hat", null, null, null];
const none = [null, null, null, null, null, null, null, null];
const chokeGroups: Record<KitId, (string | null)[]> = {
  acoustic: hat,
  electronic: hat,
  percussion: none,
};

describe("kits (contract with assets/kits/*/kit.json, 08 §7)", () => {
  it("are exactly the three kits of 05 §8.2, one folder each", () => {
    expect(Object.keys(kitFiles).sort()).toEqual(
      kitIds.map((id) => `../../assets/kits/${id}/kit.json`).sort(),
    );
  });

  it("have the kit.json of 05 §8.3: id, names, and 8 sounds in the order of 05 §8.1", () => {
    for (const id of kitIds) {
      expect(kitFiles[`../../assets/kits/${id}/kit.json`]).toEqual({
        id,
        name: names[id].kit,
        sounds: roles.map((role, row) => ({
          role,
          file: files[row],
          name: { es: names[id].es[row], en: names[id].en[row] },
          chokeGroup: chokeGroups[id][row],
        })),
      });
    }
  });

  it("have a WAV file for every sound, and no other", () => {
    expect(wavFiles.sort()).toEqual(
      kitIds.flatMap((id) => files.map((file) => `../../assets/kits/${id}/${file}`)).sort(),
    );
  });
});

describe("ROLES", () => {
  it("are the roles of 05 §8.1 in row order", () => {
    expect(ROLES).toEqual(roles);
  });
});

describe("kitName", () => {
  it("gives the name of each kit in both languages", () => {
    for (const id of kitIds) {
      expect(kitName(id, "es")).toBe(names[id].kit.es);
      expect(kitName(id, "en")).toBe(names[id].kit.en);
    }
  });
});

describe("soundNames", () => {
  it("give the names of the rows in both languages", () => {
    for (const id of kitIds) {
      expect(soundNames(id, "es")).toEqual(names[id].es);
      expect(soundNames(id, "en")).toEqual(names[id].en);
    }
  });
});
