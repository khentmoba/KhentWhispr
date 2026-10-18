// Shared common presets; recognition uses the model's language codes.
export const COMMON_LANGUAGE_OPTIONS = [
  { code: "zh", label: "Chinese (Simplified)" },
  { code: "zh", label: "Chinese (Traditional)" },
  { code: "en", label: "English" },
  { code: "ja", label: "Japanese" },
  { code: "ko", label: "Korean" },
  { code: "fr", label: "French" },
  { code: "de", label: "German" },
  { code: "es", label: "Spanish" },
  { code: "ru", label: "Russian" },
  { code: "pt", label: "Portuguese" },
] as const;
