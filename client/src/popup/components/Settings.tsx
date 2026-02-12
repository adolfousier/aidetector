import React, { useState } from "react";
import type { ExtensionSettings } from "../../shared/types";

interface Props {
  settings: ExtensionSettings;
  onUpdate: (updates: Partial<ExtensionSettings>) => void;
  serverInfo: { provider: string; model: string } | null;
}

export function Settings({ settings, onUpdate, serverInfo }: Props) {
  const [apiUrl, setApiUrl] = useState(settings.apiUrl);
  const [apiKey, setApiKey] = useState(settings.apiKey);

  function handleSave() {
    onUpdate({ apiUrl, apiKey });
  }

  return (
    <div className="settings-panel">
      {serverInfo && (
        <div className="server-info">
          {serverInfo.provider === "none" ? (
            <div className="info-row">
              <span className="info-value">Heuristics only (no LLM configured)</span>
            </div>
          ) : (
            <>
              <div className="info-row">
                <span className="info-label">Provider</span>
                <span className="info-value">{serverInfo.provider}</span>
              </div>
              <div className="info-row">
                <span className="info-label">Model</span>
                <span className="info-value">{serverInfo.model}</span>
              </div>
            </>
          )}
        </div>
      )}
      <div className="field">
        <label>Server URL</label>
        <input
          type="text"
          value={apiUrl}
          onChange={(e) => setApiUrl(e.target.value)}
          placeholder="http://localhost:3000"
        />
      </div>
      <div className="field">
        <label>API Key</label>
        <input
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder="Enter API key"
        />
      </div>
      <button className="save-btn" onClick={handleSave}>
        Save Settings
      </button>
    </div>
  );
}
