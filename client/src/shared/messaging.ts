import type {
  AnalyzeRequest,
  AnalyzeResponse,
  ExtensionSettings,
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

export async function getHistory(): Promise<HistoryResponse> {
  const response = await sendMessage({ type: "GET_HISTORY" });
  if (!response.success) {
    throw new Error(response.error);
  }
  return response.data as HistoryResponse;
}
