// The texts of the interface (07 §7): `es.json` and `en.json` are flat objects whose keys are the
// keys of 03, dots included, and whose data go between braces, as in `{name}` (03 §14.1).
// Importing this module sets i18next up, so the texts are ready before anything is drawn.
// TODO(T-020): the rest of the configuration and of the keys of 03, with their test.
import i18next from "i18next";
import { initReactI18next, useTranslation } from "react-i18next";
import en from "./en.json";
import es from "./es.json";

await i18next.use(initReactI18next).init({
  resources: { es: { translation: es }, en: { translation: en } },
  // TODO(T-052): take the language from the settings (RF-701).
  lng: "es",
  keySeparator: false,
  nsSeparator: false,
  interpolation: { prefix: "{", suffix: "}", escapeValue: false },
  initAsync: false,
});

export { useTranslation };

/** The active language of the interface (RF-701). */
export function useLanguage(): "es" | "en" {
  const { i18n } = useTranslation();
  return i18n.language === "en" ? "en" : "es";
}
