import { Store } from "@tauri-apps/plugin-store";

import en from "./en.json";
import fr from "./fr.json";

export const languages = {
  en,
  fr,
};

export const fallbackLocale = "en";

function getDefaultLocale() {
  const browserLanguages = navigator.languages || [navigator.language];

  for (const lang of browserLanguages) {
    if (lang in languages) return lang as keyof typeof languages;
  }

  return fallbackLocale;
}

async function getLocale() {
  const store = await Store.load("settings.json");
  return (await store.get<keyof typeof languages>("lang")) || getDefaultLocale();
}

export { getLocale };
