import { AudioLines, History, Settings } from "lucide-react";
import { SpeakFlowBrandIcon } from "./TitleBar";

export type AppView = "main" | "history" | "settings";

const destinations = [
  { view: "main", label: "Dictation", icon: AudioLines },
  { view: "history", label: "History", icon: History },
  { view: "settings", label: "Settings", icon: Settings },
] as const;

export default function AppNavigation({ view, onNavigate }: {
  view: AppView;
  onNavigate: (view: AppView) => void;
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
            aria-current={view === destination ? "page" : undefined}
            onClick={() => onNavigate(destination)}
          >
            <Icon size={18} strokeWidth={1.7} aria-hidden="true" />
            <span>{label}</span>
          </button>
        ))}
      </nav>
      <div className="sidebar-note">
        <span className="sidebar-note-rule" />
        <p>A little less typing.<br />A little more flow.</p>
      </div>
    </aside>
  );
}
