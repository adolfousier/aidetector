import type {
  AnalyzeRequest,
  AnalyzeResponse,
  ExtensionSettings,
  HistoryQuery,
  HistoryResponse,
  MessageType,
  MessageResponse,
} from "./types";

function sendMessage(message: MessageType): Promise<MessageResponse> {
  return new Promise((resolve) => {
    chrome.runtime.sendMessage(message, (response: MessageResponse) => {
      resolve(response);
    });
  });
}

export async function analyzeContent(
  request: AnalyzeRequest
): Promise<AnalyzeResponse> {
  const response = await sendMessage({ type: "ANALYZE", payload: request });
  if (!response.success) {
    throw new Error(response.error);
  }
  return response.data as AnalyzeResponse;
}

export async function getSettings(): Promise<ExtensionSettings> {
  const response = await sendMessage({ type: "GET_SETTINGS" });
  if (!response.success) {
    throw new Error(response.error);
  }
  return response.data as ExtensionSettings;
}

export async function updateSettings(
  settings: Partial<ExtensionSettings>
): Promise<ExtensionSettings> {
  const response = await sendMessage({
    type: "UPDATE_SETTINGS",
    payload: settings,
  });
  if (!response.success) {
    throw new Error(response.error);
  }
  return response.data as ExtensionSettings;
}

export async function getHistory(query?: HistoryQuery): Promise<HistoryResponse> {
  const response = await sendMessage({ type: "GET_HISTORY", payload: query });
  if (!response.success) {
    throw new Error(response.error);
  }
  return response.data as HistoryResponse;
}

export async function getAuthors(): Promise<string[]> {
  const response = await sendMessage({ type: "GET_AUTHORS" });
  if (!response.success) {
    throw new Error(response.error);
  }
  return response.data as string[];
}

export async function rescan(): Promise<void> {
  await sendMessage({ type: "RESCAN" });
}
