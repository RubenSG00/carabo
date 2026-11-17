import acoustic from "../../assets/kits/acoustic/kit.json";
import electronic from "../../assets/kits/electronic/kit.json";
import percussion from "../../assets/kits/percussion/kit.json";
import { soundNames } from "./kits";

// Names of the sounds of the three kits, by row (05 §8.2).
const names = {
  acoustic: {
    es: ["Bombo", "Caja", "Aro", "Charles cerrado", "Charles abierto", "Tom", "Crash", "Ride"],
    en: ["Kick", "Snare", "Rimshot", "Closed hi-hat", "Open hi-hat", "Tom", "Crash", "Ride"],
  },
  electronic: {
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

describe("kits", () => {
  it("have 8 sounds each, in the order of roles of 05 §8.1", () => {
    const roles = ["low", "snare", "clap", "hatClosed", "hatOpen", "tom", "accent", "extra"];
    for (const kit of [acoustic, electronic, percussion]) {
      expect(kit.sounds.map((sound) => sound.role)).toEqual(roles);
    }
  });

  it("give the names of the rows in both languages", () => {
    for (const kitId of ["acoustic", "electronic", "percussion"] as const) {
      expect(soundNames(kitId, "es")).toEqual(names[kitId].es);
      expect(soundNames(kitId, "en")).toEqual(names[kitId].en);
    }
  });
});
