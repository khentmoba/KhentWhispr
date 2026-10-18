import { useLiveTranscript } from "@/hooks/useLiveTranscript";
import { Monitor, Moon, Sun } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useTheme } from "@/hooks/useTheme";

const themeOptions = [
  { mode: "light" as const, icon: Sun, labelKey: "settings.themeLight" },
  { mode: "dark" as const, icon: Moon, labelKey: "settings.themeDark" },
  { mode: "system" as const, icon: Monitor, labelKey: "settings.themeSystem" },
] as const;

export default function AppearanceSettingsSection() {
  const [showLiveTranscript, setShowLiveTranscript] = useLiveTranscript();
  const { t } = useTranslation();
  const { isDark, theme, setTheme } = useTheme();

  return (
    <section className="settings-card" data-nav-id="appearance">
      <div className="settings-section-header">
        {isDark ? <Moon size={15} className="icon-accent" /> : <Sun size={15} className="icon-accent" />}
        <h2 className="settings-section-title">{t("settings.appearance")}</h2>
      </div>
      <div className="settings-grid-3">
        {themeOptions.map(({ mode, icon: Icon, labelKey }) => (
          <button
            key={mode}
            className="theme-btn settings-option-btn theme-option"
            aria-label={t("settings.switchToTheme", { label: t(labelKey) })}
            aria-pressed={theme === mode}
            onClick={() => setTheme(mode)}
          >
            <Icon size={20} strokeWidth={1.5} />
            <span className="settings-option-label">{t(labelKey)}</span>
          </button>
        ))}
      </div>
      <div className="settings-row">
        <div className="settings-column">
          <span className="settings-option-desc">{t("settings.showLiveTranscript")}</span>
          <p id="live-transcript-hint" className="settings-hint">{t("settings.showLiveTranscriptHint")}</p>
        </div>
        <button type="button" role="switch" className="toggle-switch"
          aria-label={t("settings.showLiveTranscript")} aria-describedby="live-transcript-hint"
          aria-checked={showLiveTranscript} data-active={showLiveTranscript}
          onClick={() => setShowLiveTranscript(!showLiveTranscript)}>
          <div className="toggle-knob" />
        </button>
      </div>
    </section>
  );
}
