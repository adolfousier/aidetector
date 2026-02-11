import { DEFAULT_SETTINGS } from "../shared/constants";
import type {
  AnalyzeRequest,
  AnalyzeResponse,
  ExtensionSettings,
  HistoryResponse,
  MessageType,
  MessageResponse,
} from "../shared/types";

// In-memory cache for recent analyses
const analysisCache = new Map<string, AnalyzeResponse>();
const CACHE_MAX_SIZE = 200;

async function getSettings(): Promise<ExtensionSettings> {
  const result = await chrome.storage.local.get("settings");
  return { ...DEFAULT_SETTINGS, ...result.settings };
}

async function saveSettings(
  updates: Partial<ExtensionSettings>
): Promise<ExtensionSettings> {
  const current = await getSettings();
  const merged = { ...current, ...updates };
  await chrome.storage.local.set({ settings: merged });
  return merged;
}

async function callApi(
  settings: ExtensionSettings,
  request: AnalyzeRequest
): Promise<AnalyzeResponse> {
  // Check cache first
  const cacheKey = request.content.slice(0, 200);
  const cached = analysisCache.get(cacheKey);
  if (cached) return cached;

  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };

  if (settings.apiKey) {
    headers["x-api-key"] = settings.apiKey;
  }

  const response = await fetch(`${settings.apiUrl}/api/analyze`, {
    method: "POST",
    headers,
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    const body = await response.text();
    throw new Error(`API error ${response.status}: ${body}`);
  }

  const data: AnalyzeResponse = await response.json();

  // Cache the result
  if (analysisCache.size >= CACHE_MAX_SIZE) {
    const firstKey = analysisCache.keys().next().value;
    if (firstKey) analysisCache.delete(firstKey);
  }
  analysisCache.set(cacheKey, data);

  return data;
}

async function fetchHistory(
  settings: ExtensionSettings
): Promise<HistoryResponse> {
  const headers: Record<string, string> = {};
  if (settings.apiKey) {
    headers["x-api-key"] = settings.apiKey;
  }

  const response = await fetch(
    `${settings.apiUrl}/api/history?limit=20&offset=0`,
    { headers }
  );

  if (!response.ok) {
    throw new Error(`API error ${response.status}`);
  }

  return response.json();
}

// Message handler
chrome.runtime.onMessage.addListener(
  (
    message: MessageType,
    _sender: chrome.runtime.MessageSender,
    sendResponse: (response: MessageResponse) => void
  ) => {
    handleMessage(message)
      .then(sendResponse)
      .catch((err) =>
        sendResponse({ success: false, error: String(err.message || err) })
      );
    return true; // Keep message channel open for async response
  }
);

async function handleMessage(message: MessageType): Promise<MessageResponse> {
  switch (message.type) {
    case "LOG": {
      console.log("[Content]", message.payload);
      return { success: true, data: {} as any };
    }
    case "ANALYZE": {
      const settings = await getSettings();
      if (!settings.enabled) {
        return { success: false, error: "Extension is disabled" };
      }
      console.log(`[AI Detector] Analyzing ${message.payload.platform} post from ${message.payload.author}`);
      const data = await callApi(settings, message.payload);
      console.log(`[AI Detector] Result: score=${data.score} label=${data.label}`);
      return { success: true, data };
    }
    case "GET_SETTINGS": {
      const data = await getSettings();
      return { success: true, data };
    }
    case "UPDATE_SETTINGS": {
      const data = await saveSettings(message.payload);
      return { success: true, data };
    }
    case "GET_HISTORY": {
      const settings = await getSettings();
      const data = await fetchHistory(settings);
      return { success: true, data };
    }
  }
}

// Health check on install
chrome.runtime.onInstalled.addListener(async () => {
  const settings = await getSettings();
  try {
    const resp = await fetch(`${settings.apiUrl}/api/health`);
    if (resp.ok) {
      console.log("[AI Detector] Server connected successfully");
    }
  } catch {
    console.warn("[AI Detector] Server not reachable at", settings.apiUrl);
  }
});
