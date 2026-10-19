import { AudioWaveform, Cloud, Globe, Zap } from "lucide-react";

export const engineOptions = [
  { key: "qwen3-asr-0.6b", icon: Zap, label: "Qwen3-ASR 0.6B Q8", labelKey: undefined, descKey: "settings.qwen3Asr06Desc" },
  { key: "confucius4-r2t2", icon: AudioWaveform, label: "Confucius4-R2T2 Q8", labelKey: undefined, descKey: "settings.r2t2Desc" },
  { key: "glm-asr", icon: Globe, label: "GLM-ASR", labelKey: undefined, descKey: "settings.glmAsrDesc" },
  { key: "alibaba-asr", icon: Cloud, label: "Alibaba DashScope", labelKey: "settings.alibabaAsrLabel", descKey: "settings.alibabaAsrDesc" },
] as const;

export const engineOptionLabel = (
  option: (typeof engineOptions)[number],
  translate: (key: string) => string,
) => option.labelKey ? translate(option.labelKey) : option.label;
