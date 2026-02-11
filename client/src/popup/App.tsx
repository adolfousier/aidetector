import React, { useEffect, useState, useRef } from "react";
import type { ExtensionSettings, HistoryItem } from "../shared/types";
import { getSettings, updateSettings, getHistory } from "../shared/messaging";
import { ScoreCard } from "./components/ScoreCard";
import { Settings } from "./components/Settings";

type Tab = "history" | "status" | "settings";

export function App() {
  const [tab, setTab] = useState<Tab>("history");
  const [settings, setSettings] = useState<ExtensionSettings | null>(null);
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [historyTotal, setHistoryTotal] = useState(0);
  const [serverOnline, setServerOnline] = useState<boolean | null>(null);
  const [error, setError] = useState("");
  const [toast, setToast] = useState("");
  const [refreshing, setRefreshing] = useState(false);
  const refreshRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const toastRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    loadSettings();
    loadHistory();
    refreshRef.current = setInterval(loadHistory, 5000);
    return () => {
      if (refreshRef.current) clearInterval(refreshRef.current);
    };
  }, []);

  function showToast(msg: string) {
    setToast(msg);
    if (toastRef.current) clearTimeout(toastRef.current);
    toastRef.current = setTimeout(() => setToast(""), 2500);
  }

  async function loadSettings() {
    try {
      const s = await getSettings();
      setSettings(s);
      checkServer(s.apiUrl);
    } catch {
      setError("Failed to load settings");
    }
  }

  async function checkServer(apiUrl: string) {
    try {
      const resp = await fetch(`${apiUrl}/api/health`);
      setServerOnline(resp.ok);
    } catch {
      setServerOnline(false);
    }
  }

  async function handleSettingsUpdate(updates: Partial<ExtensionSettings>) {
    try {
      const s = await updateSettings(updates);
      setSettings(s);
      if (updates.apiUrl) checkServer(s.apiUrl);
      if (updates.apiUrl || updates.apiKey) {
        showToast("Settings saved");
      }
    } catch {
      setError("Failed to save settings");
    }
  }

  async function loadHistory() {
    try {
      const h = await getHistory();
      setHistory(h.items);
      setHistoryTotal(h.total);
    } catch {
      // silent
    }
  }

  async function handleRefresh() {
    setRefreshing(true);
    await loadHistory();
    setTimeout(() => setRefreshing(false), 400);
  }

  function openFullscreen() {
    const url = chrome.runtime.getURL("popup.html");
    chrome.tabs.create({ url });
  }

  return (
    <div className="popup">
      <header className="popup-header">
        <div className="header-left">
          <h1>AI Detector</h1>
          <span className="header-count">{historyTotal} scanned</span>
        </div>
        <div className="header-right">
          <button
            className="expand-btn"
            onClick={openFullscreen}
            title="Open in full tab"
          >
            &#x26F6;
          </button>
          <div className="status-dot-container">
            <span
              className={`status-dot ${
                serverOnline === null
                  ? "checking"
                  : serverOnline
                  ? "online"
                  : "offline"
              }`}
            />
            <span className="status-label">
              {serverOnline === null
                ? "..."
                : serverOnline
                ? "Connected"
                : "Offline"}
            </span>
          </div>
        </div>
      </header>

      <nav className="tabs">
        {(["history", "status", "settings"] as Tab[]).map((t) => (
          <button
            key={t}
            className={`tab ${tab === t ? "active" : ""}`}
            onClick={() => { setTab(t); setError(""); }}
          >
            {t.charAt(0).toUpperCase() + t.slice(1)}
          </button>
        ))}
      </nav>

      {error && <div className="error">{error}</div>}
      {toast && <div className="toast">{toast}</div>}

      <main className="popup-content">
        {tab === "history" && (
          <div className="history-panel">
            <div className="history-toolbar">
              <span className="history-toolbar-label">{historyTotal} results</span>
              <button
                className={`refresh-btn${refreshing ? " spinning" : ""}`}
                onClick={handleRefresh}
                title="Refresh history"
              >
                &#x21BB;
              </button>
            </div>
            {history.length === 0 ? (
              <p className="empty">No analyses yet — browse X, Instagram, or LinkedIn</p>
            ) : (
              <div className="history-list">
                {history.map((item) => (
                  <ScoreCard key={item.id} item={item} />
                ))}
              </div>
            )}
          </div>
        )}

        {tab === "status" && settings && (
          <div className="status-panel">
            <div className="toggle-row">
              <span>Extension Enabled</span>
              <label className="toggle">
                <input
                  type="checkbox"
                  checked={settings.enabled}
                  onChange={(e) =>
                    handleSettingsUpdate({ enabled: e.target.checked })
                  }
                />
                <span className="slider" />
              </label>
            </div>
            <div className="platform-toggles">
              <h3>Platforms</h3>
              {(["twitter", "instagram", "linkedin"] as const).map((p) => (
                <div key={p} className="toggle-row">
                  <span>{p === "twitter" ? "X / Twitter" : p.charAt(0).toUpperCase() + p.slice(1)}</span>
                  <label className="toggle">
                    <input
                      type="checkbox"
                      checked={settings.enabledPlatforms[p]}
                      onChange={(e) =>
                        handleSettingsUpdate({
                          enabledPlatforms: {
                            ...settings.enabledPlatforms,
                            [p]: e.target.checked,
                          },
                        })
                      }
                    />
                    <span className="slider" />
                  </label>
                </div>
              ))}
            </div>
          </div>
        )}

        {tab === "settings" && settings && (
          <Settings settings={settings} onUpdate={handleSettingsUpdate} />
        )}
      </main>
    </div>
  );
}
