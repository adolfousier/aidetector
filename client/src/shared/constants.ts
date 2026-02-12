import type { ExtensionSettings } from "./types";

export const DEFAULT_API_URL = "http://localhost:3000";

export const DEFAULT_SETTINGS: ExtensionSettings = {
  apiUrl: DEFAULT_API_URL,
  apiKey: "",
  enabled: true,
  enabledPlatforms: {
    twitter: true,
    instagram: true,
    linkedin: true,
  },
};

export const SCORE_THRESHOLDS = {
  human: { max: 3, color: "#22c55e", bg: "#f0fdf4", label: "Human" },
  mixed: { max: 6, color: "#eab308", bg: "#fefce8", label: "Mixed" },
  uncertain: { max: 6, color: "#eab308", bg: "#fefce8", label: "Uncertain" },
  ai: { max: 10, color: "#ef4444", bg: "#fef2f2", label: "AI" },
} as const;

export function getScoreStyle(score: number, heuristicsOnly = false) {
  if (score <= SCORE_THRESHOLDS.human.max) return SCORE_THRESHOLDS.human;
  if (score <= SCORE_THRESHOLDS.mixed.max)
    return heuristicsOnly ? SCORE_THRESHOLDS.uncertain : SCORE_THRESHOLDS.mixed;
  return SCORE_THRESHOLDS.ai;
}
