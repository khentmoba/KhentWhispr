import { getCurrentWindow } from "@tauri-apps/api/window";
import { PADDING } from "@/lib/constants";

export function SpeakFlowBrandIcon({ size = 15, className }: { size?: number; className?: string }) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 512 512"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
      aria-hidden="true"
    >
      <defs>
        <linearGradient id="sf-tb-flow" x1="120" y1="140" x2="380" y2="370" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="#00F5A0" />
          <stop offset="35%" stopColor="#00E5C9" />
          <stop offset="70%" stopColor="#00D2F7" />
          <stop offset="100%" stopColor="#0284C7" />
        </linearGradient>
      </defs>
      <g transform="translate(-10 -20)">
        <path
          transform="translate(512 0) scale(-1 1)"
          d="M 166 220 C 156 176, 194 144, 256 144 C 324 144, 360 178, 352 220 C 344 252, 310 266, 256 276 C 202 286, 168 300, 160 332 C 152 374, 188 408, 256 408 C 310 408, 344 382, 350 344"
          fill="none"
          stroke="url(#sf-tb-flow)"
          strokeWidth="56"
          strokeLinecap="round"
          strokeLinejoin="round"
        />
        <path
          transform="translate(512 0) scale(-1 1)"
          d="M 164 216 Q 140 236 116 244 Q 142 246 170 230 Z"
          fill="#00F5A0"
        />
        <rect x="414" y="296" width="22" height="64" rx="11" fill="#00D2F7" />
      </g>
    </svg>
  );
}

interface TitleBarProps {
  title: string;
  leftAction?: React.ReactNode;
  rightActions?: React.ReactNode;
  showLogo?: boolean;
}

export default function TitleBar({ title, leftAction, rightActions, showLogo }: TitleBarProps) {
  const startDrag = (e: React.MouseEvent) => {
    if (e.button !== 0) return;

    const target = e.target as HTMLElement;
    if (
      target.closest("button") ||
      target.closest("input") ||
      target.closest("select") ||
      target.closest("textarea") ||
      target.closest("a") ||
      target.closest("[data-no-drag]")
    ) {
      return;
    }

    e.preventDefault();
    getCurrentWindow().startDragging();
  };

  return (
    // Window dragging is a pointer-only desktop affordance; keyboard actions
    // remain on the real buttons rendered in the action slots.
    // eslint-disable-next-line jsx-a11y/no-static-element-interactions
    <header
      onMouseDown={startDrag}
      className="title-bar"
      style={{ padding: `0 ${PADDING - 8}px`, justifyContent: rightActions ? "space-between" : "flex-start" }}
    >
      <div style={{ display: "flex", alignItems: "center", gap: 7 }}>
        {leftAction}
        {showLogo && <SpeakFlowBrandIcon size={15} className="title-bar-logo" />}
        <span className={`title-text${!leftAction && !showLogo ? " title-text-indented" : ""}`}>
          {title}
        </span>
      </div>
      {rightActions && (
        <div style={{ display: "flex", alignItems: "center", gap: 2 }}>
          {rightActions}
        </div>
      )}
    </header>
  );
}
