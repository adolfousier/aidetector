export type Platform = "twitter" | "instagram" | "linkedin";

export interface AnalyzeRequest {
  content: string;
  platform: Platform;
  post_id?: string;
  author?: string;
}

export interface AnalyzeResponse {
  score: number;
  confidence: number;
  label: "human" | "mixed" | "likely_ai" | "ai";
  breakdown: {
    llm_score: number | null;
    heuristic_score: number;
    signals: string[];
  };
}

export interface HistoryItem {
  id: string;
  content: string;
  content_preview: string;
  platform: string;
  post_id: string | null;
  author: string | null;
  score: number;
  confidence: number;
  label: string;
  llm_score: number | null;
  heuristic_score: number;
  signals: string;
  created_at: string;
}

export interface HistoryResponse {
  items: HistoryItem[];
  total: number;
}

export interface ExtensionSettings {
  apiUrl: string;
  apiKey: string;
  enabled: boolean;
  enabledPlatforms: {
    twitter: boolean;
    instagram: boolean;
    linkedin: boolean;
  };
}

export interface HistoryQuery {
  limit?: number;
  offset?: number;
  author?: string;
}

// Chrome message types
export type MessageType =
  | { type: "ANALYZE"; payload: AnalyzeRequest }
  | { type: "GET_SETTINGS" }
  | { type: "UPDATE_SETTINGS"; payload: Partial<ExtensionSettings> }
  | { type: "GET_HISTORY"; payload?: HistoryQuery }
  | { type: "GET_AUTHORS" }
  | { type: "LOG"; payload: string };

export type MessageResponse =
  | { success: true; data: AnalyzeResponse }
  | { success: true; data: ExtensionSettings }
  | { success: true; data: HistoryResponse }
  | { success: true; data: string[] }
  | { success: false; error: string };
