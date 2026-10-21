import { AudioWaveform, Cloud, Globe, Languages, Layers, Mic, Sparkles, Waves, Zap } from "lucide-react";

export const engineOptions = [
  { key: "parakeet-tdt-0.6b-v2", icon: Mic, label: "Parakeet TDT 0.6B v2 Q8", labelKey: undefined, descKey: "settings.parakeetTdtV2Desc" },
  { key: "qwen3-asr-0.6b", icon: Zap, label: "Qwen3-ASR 0.6B Q8", labelKey: undefined, descKey: "settings.qwen3Asr06Desc" },
  { key: "confucius4-r2t2", icon: AudioWaveform, label: "Confucius4-R2T2 Q8", labelKey: undefined, descKey: "settings.r2t2Desc" },
  { key: "qwen3-asr-1.7b", icon: Layers, label: "Qwen3-ASR 1.7B Q8", labelKey: undefined, descKey: "settings.qwen3Asr17Desc" },
  { key: "parakeet-unified-en-0.6b", icon: Languages, label: "Parakeet Unified EN 0.6B Q8", labelKey: undefined, descKey: "settings.parakeetUnifiedDesc" },
  { key: "canary-qwen-2.5b", icon: Sparkles, label: "Canary-Qwen 2.5B Q8", labelKey: undefined, descKey: "settings.canaryQwen25Desc" },
  { key: "cohere-transcribe-03-2026", icon: Waves, label: "Cohere Transcribe Q8", labelKey: undefined, descKey: "settings.cohereTranscribeDesc" },
  { key: "glm-asr", icon: Globe, label: "GLM-ASR", labelKey: undefined, descKey: "settings.glmAsrDesc" },
  { key: "alibaba-asr", icon: Cloud, label: "Alibaba DashScope", labelKey: "settings.alibabaAsrLabel", descKey: "settings.alibabaAsrDesc" },
] as const;

export const engineOptionLabel = (
  option: (typeof engineOptions)[number],
  translate: (key: string) => string,
) => option.labelKey ? translate(option.labelKey) : option.label;
