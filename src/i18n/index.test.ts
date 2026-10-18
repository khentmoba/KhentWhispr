import { afterEach, describe, expect, it, vi } from "vitest";
import { LANGUAGE_STORAGE_KEY } from "@/lib/constants";
import en from "./en";
import { COMMON_LANGUAGE_OPTIONS } from "@/lib/languages";

async function loadEnglish() {
  vi.resetModules();
  const { default: i18n } = await import("./index");
  return i18n;
}

afterEach(() => {
  vi.restoreAllMocks();
  localStorage.removeItem(LANGUAGE_STORAGE_KEY);
});

describe("English-only UI", () => {
  it.each(["zh", "zh-CN", "fr", "en"])("migrates the stored %s locale and ignores the browser language", async (locale) => {
    localStorage.setItem(LANGUAGE_STORAGE_KEY, locale);
    vi.spyOn(navigator, "language", "get").mockReturnValue("zh-CN");
    vi.spyOn(navigator, "languages", "get").mockReturnValue(["zh-CN"]);
    const i18n = await loadEnglish();

    expect(i18n.language).toBe("en");
    expect(i18n.t("settings.title")).toBe("Settings");
    expect(localStorage.getItem(LANGUAGE_STORAGE_KEY)).toBe("en");
    expect(Object.keys(i18n.options.resources ?? {})).toEqual(["en"]);
  });

  it("cannot switch to Chinese even through a stale locale request", async () => {
    const i18n = await loadEnglish();
    await i18n.changeLanguage("zh-CN");
    expect(i18n.language).toBe("en");
    expect(i18n.resolvedLanguage).toBe("en");
    expect(i18n.t("settings.title")).toBe("Settings");
  });

  it("starts in English when storage is unavailable", async () => {
    vi.spyOn(Storage.prototype, "setItem").mockImplementation(() => { throw new Error("Storage unavailable"); });
    const i18n = await loadEnglish();
    expect(i18n.language).toBe("en");
    expect(i18n.t("app.title")).toBe("SpeakFlow");
  });

  it("ships no Chinese UI copy in the English resource", () => {
    expect(JSON.stringify(en)).not.toMatch(/\p{Script=Han}/u);
    expect(COMMON_LANGUAGE_OPTIONS.filter(({ code }) => code === "zh").map(({ label }) => label))
      .toEqual(["Chinese (Simplified)", "Chinese (Traditional)"]);
  });
});
