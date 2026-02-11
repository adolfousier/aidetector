import React, { useState } from "react";
import type { ExtensionSettings } from "../../shared/types";

interface Props {
  settings: ExtensionSettings;
  onUpdate: (updates: Partial<ExtensionSettings>) => void;
}

export function Settings({ settings, onUpdate }: Props) {
  const [apiUrl, setApiUrl] = useState(settings.apiUrl);
  const [apiKey, setApiKey] = useState(settings.apiKey);

  function handleSave() {
    onUpdate({ apiUrl, apiKey });
  }

  return (
    <div className="settings-panel">
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
