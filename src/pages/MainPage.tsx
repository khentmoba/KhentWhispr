import { useState, useEffect, useLayoutEffect, useRef, useCallback } from "react";
import { ArrowUpRight, FileText, Keyboard, Minus, Settings, X } from "lucide-react";
import { toast } from "sonner";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useRecordingContext } from "@/contexts/RecordingContext";
import { copyToClipboard, hideMainWindow, submitUserCorrection } from "@/api/tauri";
import TitleBar from "@/components/TitleBar";
import RecordingButton from "@/components/RecordingButton";
import StatusIndicator from "@/components/StatusIndicator";
import TranscriptionResult from "@/components/TranscriptionResult";
import TranscriptionHistory from "@/components/TranscriptionHistory";
import { useDebouncedCallback } from "@/hooks/useDebouncedCallback";
import { useTranslation } from "react-i18next";
import { PADDING, ONBOARDING_DISMISSED_KEY, RECORDING_MODE_KEY } from "@/lib/constants";
import { readLocalStorage, writeLocalStorage } from "@/lib/storage";
import { prefersReducedMotion } from "@/lib/motion";

export default function MainPage({ onNavigate, animClass = "" }: {
  onNavigate: (v: "main" | "settings" | "history") => void;
  animClass?: string;
}) {
  const { t } = useTranslation();
  const {
    isStarting, isRecording, isProcessing, startRecording, stopRecording,
    recordingError, transcriptionResult, originalAsrText, editBaselineText, setEditBaselineText, setTranscriptionResult, polishAuditWarning,
    durationSec, charCount, detectedLanguage, editGrabStatus, timing, history, recordingMode, stage, isReady,
    device, gpuName, downloadProgress, downloadMessage,
    isDownloading, modelError, hotkeyDisplay,
    downloadModels: triggerDownload, cancelDownload, retryModel,
  } = useRecordingContext();

  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [errorDismissed, setErrorDismissed] = useState(false);
  const [onboardingDismissed, setOnboardingDismissed] = useState(() => readLocalStorage(ONBOARDING_DISMISSED_KEY) === "true");
  const isToggleMode = readLocalStorage(RECORDING_MODE_KEY) === "toggle";
  const copyTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const resultsRef = useRef<HTMLDivElement>(null);
  const previousResult = useRef<{ id: string; top: number } | null>(null);

  useLayoutEffect(() => {
    const root = resultsRef.current;
    const current = history[0];
    const card = root?.querySelector<HTMLElement>(".result-card");
    if (!root || !card || !current || current.text !== transcriptionResult) return;
    const previous = previousResult.current;
    if (previous && previous.id !== current.id && !prefersReducedMotion()) {
      const oldItem = Array.from(root.querySelectorAll<HTMLElement>(".history-item"))
        .find((item) => item.dataset.historyId === previous.id);
      if (oldItem?.animate) {
        oldItem.animate([
          { transform: `translateY(${previous.top - oldItem.getBoundingClientRect().top}px)`, opacity: 0.5 },
          { transform: "translateY(0)", opacity: 1 },
        ], { duration: 220, easing: "cubic-bezier(0.16, 1, 0.3, 1)" });
      }
      card.querySelector("textarea")?.animate?.([{ opacity: 0 }, { opacity: 1 }], { duration: 160, easing: "ease-out" });
    }
    previousResult.current = { id: current.id, top: card.getBoundingClientRect().top };
  }, [history, transcriptionResult]);

  useEffect(() => { setErrorDismissed(false); }, [recordingError, modelError]);
  // Auto-dismiss onboarding after first successful transcription
  useEffect(() => {
    if (transcriptionResult && !onboardingDismissed) {
      setOnboardingDismissed(true);
      writeLocalStorage(ONBOARDING_DISMISSED_KEY, "true");
    }
  }, [transcriptionResult, onboardingDismissed]);
  useEffect(() => () => { if (copyTimerRef.current) clearTimeout(copyTimerRef.current); }, []);

  const pageContentClass = `page-content ${animClass || ""}`.trim();

  const correctionSubmit = useDebouncedCallback((previousText: string, nextText: string, rawOriginalText: string | null) => {
    submitUserCorrection(previousText, nextText, rawOriginalText)
      .then(() => toast.success(t("toast.correctionRecorded"), { duration: 1500 }))
      .catch(() => toast.error(t("toast.correctionFailed")));
  }, 900, { onUnmount: "flush" });

  const handleCopy = useCallback(async (text: string, id: string) => {
    try {
      await copyToClipboard(text);
      setCopiedId(id);
      toast.success(t("common.copiedToClipboard"));
      if (copyTimerRef.current) clearTimeout(copyTimerRef.current);
      copyTimerRef.current = setTimeout(() => {
        setCopiedId(null);
        copyTimerRef.current = null;
      }, 1500);
    } catch {
      toast.error(t("common.copyFailed"));
    }
  }, [t]);

  const handleDraftChange = useCallback((newText: string) => {
    setTranscriptionResult(newText);
  }, [setTranscriptionResult]);

  const handleTextChange = useCallback((newText: string) => {
    if (recordingMode !== "dictation") {
      setTranscriptionResult(newText);
      return;
    }
    if (editBaselineText && newText !== editBaselineText) {
      const prevText = editBaselineText;
      setEditBaselineText(newText);
      setTranscriptionResult(newText);
      correctionSubmit.schedule(prevText, newText, originalAsrText);
    }
  }, [
    correctionSubmit,
    editBaselineText,
    originalAsrText,
    recordingMode,
    setTranscriptionResult,
    setEditBaselineText,
  ]);

  const flushPendingEditAndNavigate = useCallback((target: "main" | "settings" | "history") => {
    const active = document.activeElement;
    if (active instanceof HTMLElement && (active.isContentEditable || active instanceof HTMLTextAreaElement || active instanceof HTMLInputElement)) {
      active.blur();
    }
    correctionSubmit.flush();
    onNavigate(target);
  }, [correctionSubmit, onNavigate]);

  const handleToggleRecording = useCallback(() => {
    if (!isReady) return;
    if (isStarting || isRecording) {
      stopRecording();
    } else {
      startRecording();
    }
  }, [isReady, isStarting, isRecording, stopRecording, startRecording]);

  return (
    <div className="page-root main-page">
      <TitleBar
        title="Dictation"
        rightActions={
          <>
            <button aria-label={t("common.minimize")} className="icon-btn" onClick={() => getCurrentWindow().minimize()}>
              <Minus size={12} strokeWidth={1.5} />
            </button>
            <button aria-label={t("common.close")} className="icon-btn" onClick={() => hideMainWindow()}>
              <X size={12} strokeWidth={1.5} />
            </button>
          </>
        }
      />

      <div className={`${pageContentClass} dictation-content`}>
        <header className="workspace-heading">
          <div>
            <h1>Make room for your words.</h1>
            <p>Speak naturally. SpeakFlow takes care of the typing.</p>
          </div>
          <button className="workspace-settings-link" onClick={() => flushPendingEditAndNavigate("settings")}>
            <Settings size={15} aria-hidden="true" /> Configure
          </button>
        </header>
        <div className="dictation-grid">
        <section className="recording-panel" data-recording={isRecording || isStarting} aria-label="Recording controls">
        <div className="recording-panel-heading">
          <h2>{isRecording ? "Listening to you" : isStarting ? "Connecting" : isProcessing ? "Working on your words" : isReady ? "Ready when you are" : stage === "error" ? "Let’s get you set up" : "Getting ready"}</h2>
          <p>{isRecording ? "Finish speaking, then stop the recording." : isReady ? "A thought, a message, a first draft." : "Your engine status appears below."}</p>
        </div>
        <div className="recording-zone">
          <StatusIndicator
            stage={stage}
            isReady={isReady}
            isStarting={isStarting}
            isRecording={isRecording}
            isProcessing={isProcessing}
            device={device}
            gpuName={gpuName}
            downloadProgress={downloadProgress}
            downloadMessage={downloadMessage}
            isDownloading={isDownloading}
            downloadModels={triggerDownload}
            cancelDownload={cancelDownload}
          >
            <RecordingButton
              isStarting={isStarting}
              isRecording={isRecording}
              isProcessing={isProcessing}
              isReady={isReady}
              onToggle={handleToggleRecording}
            />
          </StatusIndicator>

          {/* Tactile Hotkey Bar */}
          {isReady && !recordingError && !modelError && (
            <div className="tactile-hotkey-bar" aria-hidden="true">
              <span className={`tactile-hotkey-badge${isRecording ? " is-active" : ""}`}>
                <kbd className="tactile-key">{hotkeyDisplay || "F2"}</kbd>
                <span className="tactile-hotkey-label">
                  {isToggleMode ? t("main.hotkeyHintToggle") : t("main.hotkeyHintHold")}
                </span>
              </span>
            </div>
          )}
        </div>
        <div className="recording-panel-footer">
          <Keyboard size={16} aria-hidden="true" />
          <p>Works in your active app.<br /><span>No need to switch windows.</span></p>
        </div>
        </section>

        {/* Error */}
        {(recordingError || modelError) && !errorDismissed && (
          <div className="error-section animate-shake" style={{ margin: `0 ${PADDING}px 8px` }}>
            <div className="error-banner">
              <div className="error-banner-inner">
                <p className="settings-error" style={{ lineHeight: 1.6, flex: 1 }}>{recordingError || modelError}</p>
                <button onClick={() => setErrorDismissed(true)} aria-label={t("common.close")} className="error-dismiss-btn">
                  <X size={12} />
                </button>
              </div>
              {stage === "error" && <button onClick={retryModel} className="error-retry-link">{t("common.retry")}</button>}
            </div>
          </div>
        )}

        {/* Results */}
        <section ref={resultsRef} className="results-area" aria-label="Transcription workspace">
          <div className="transcript-heading">
            <h2>Your transcript</h2>
            <button onClick={() => flushPendingEditAndNavigate("history")}>History <ArrowUpRight size={14} aria-hidden="true" /></button>
          </div>
          {!transcriptionResult && !isProcessing && (
            <div className="transcript-empty">
              <FileText size={32} strokeWidth={1.25} aria-hidden="true" />
              <h3>Your words will appear here.</h3>
              <p>Start a recording to turn your voice into text. You can review, edit, and copy the result here.</p>
            </div>
          )}
          {polishAuditWarning && (
            <div className="polish-audit-notice" role="status">
              <p>{t("settings.jevAuditWarning")}</p>
              {originalAsrText && <details><summary>{t("settings.jevAuditOriginal")}</summary><p>{originalAsrText}</p></details>}
            </div>
          )}
          <TranscriptionResult
            text={transcriptionResult}
            originalText={editBaselineText}
            isProcessing={isProcessing}
            copiedId={copiedId}
            onCopy={handleCopy}
            onDraftChange={handleDraftChange}
            onTextChange={handleTextChange}
            durationSec={durationSec}
            charCount={charCount}
            detectedLanguage={detectedLanguage}
            editGrabStatus={editGrabStatus}
            timing={timing}
          />
          <TranscriptionHistory
            history={history}
            currentResult={transcriptionResult}
            copiedId={copiedId}
            onCopy={handleCopy}
          />
          {/* First-use onboarding hint */}
          {!onboardingDismissed && !transcriptionResult && !isProcessing && isReady && history.length === 0 && (
            <div className="onboarding-hero-card">
                <div className="result-card-header">
                  <span className="result-card-title">
                    <span className="result-dot" />
                    {t("main.quickStart")}
                  </span>
                  <button
                    aria-label={t("common.close")}
                    className="icon-btn icon-btn-sm"
                    onClick={() => {
                      setOnboardingDismissed(true);
                      writeLocalStorage(ONBOARDING_DISMISSED_KEY, "true");
                    }}
                  >
                    <X size={12} strokeWidth={1.5} />
                  </button>
                </div>
                <div className="onboarding-body">
                  <div className="onboarding-step-row">
                    <span className="onboarding-step-pill">1</span>
                    <p>
                      {isToggleMode ? "Press" : "Hold"} <strong className="onboarding-accent">{hotkeyDisplay || "F2"}</strong> {isToggleMode ? "to start. Press again to finish." : "and speak. Release to finish."}
                    </p>
                  </div>
                  <div className="onboarding-step-row">
                    <span className="onboarding-step-pill">2</span>
                    <p>Your text is inserted at the cursor in your active app.</p>
                  </div>
                  <p className="onboarding-subtext">Change your microphone, hotkey, and engine in Settings.</p>
                </div>
            </div>
          )}
        </section>
        </div>
      </div>
    </div>
  );
}
