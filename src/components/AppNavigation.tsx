import { AudioLines, History, Settings, RefreshCw } from "lucide-react";
import { SpeakFlowBrandIcon } from "./TitleBar";

export type AppView = "main" | "history" | "settings" | "updates";

const destinations = [
  { view: "main", label: "Dictation", icon: AudioLines },
  { view: "history", label: "History", icon: History },
  { view: "settings", label: "Settings", icon: Settings },
  { view: "updates", label: "Updates", icon: RefreshCw },
] as const;

export default function AppNavigation({ view, onNavigate, updateAvailable = false, updateInstalling = false }: {
  view: AppView;
  onNavigate: (view: AppView) => void;
  updateAvailable?: boolean;
  updateInstalling?: boolean;
}) {
  return (
    <aside className="app-sidebar">
      <div className="sidebar-brand">
        <SpeakFlowBrandIcon size={30} />
        <span>SpeakFlow</span>
      </div>
      <nav className="app-navigation" aria-label="Main navigation">
        {destinations.map(({ view: destination, label, icon: Icon }) => (
          <button
            key={destination}
            type="button"
            className="app-nav-item"
            data-destination={destination}
            aria-current={view === destination ? "page" : undefined}
            onClick={() => onNavigate(destination)}
          >
            <Icon size={18} strokeWidth={1.7} aria-hidden="true" className={destination === "updates" && updateInstalling ? "update-spinner" : undefined} />
            <span>{label}</span>
            {destination === "updates" && updateAvailable && <span className="update-nav-dot" role="img" aria-label={updateInstalling ? "Update in progress" : "New release available"} />}
          </button>
        ))}
      </nav>
      <div className="sidebar-note"><span className="sidebar-note-rule" /><p>Speak freely.<br /><span>Stay in your flow.</span></p></div>
    </aside>
  );
}
