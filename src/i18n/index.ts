import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import en from "./en";
import { LANGUAGE_STORAGE_KEY } from "@/lib/constants";
import { writeLocalStorage } from "@/lib/storage";

// Replace legacy locale preferences without touching recognition or translation languages.
writeLocalStorage(LANGUAGE_STORAGE_KEY, "en");

i18n
  .use(initReactI18next)
  .init({
    resources: {
      en: { translation: en },
    },
    lng: "en",
    fallbackLng: "en",
    supportedLngs: ["en"],
    interpolation: {
      escapeValue: false,
    },
  });

export default i18n;
