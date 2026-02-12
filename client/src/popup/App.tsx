import React, { useEffect, useState, useRef } from "react";
import type { ExtensionSettings, HistoryItem } from "../shared/types";
import { getSettings, updateSettings, getHistory, getAuthors, rescan } from "../shared/messaging";
import { ScoreCard } from "./components/ScoreCard";
import { Settings } from "./components/Settings";

type Tab = "history" | "status" | "settings";
const PAGE_SIZE = 20;

export function App() {
  const [tab, setTab] = useState<Tab>("history");
  const [settings, setSettings] = useState<ExtensionSettings | null>(null);
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [historyTotal, setHistoryTotal] = useState(0);
  const [serverOnline, setServerOnline] = useState<boolean | null>(null);
  const [serverInfo, setServerInfo] = useState<{ provider: string; model: string } | null>(null);
  const [error, setError] = useState("");
  const [toast, setToast] = useState("");
  const [refreshing, setRefreshing] = useState(false);
  const [loading, setLoading] = useState(false);
  const [authors, setAuthors] = useState<string[]>([]);
  const [selectedAuthor, setSelectedAuthor] = useState("");
  const refreshRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const toastRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    loadSettings();
    loadHistory(true);
    loadAuthors();
    refreshRef.current = setInterval(() => loadHistory(true), 10000);
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
      if (resp.ok) {
        const data = await resp.json();
        setServerOnline(true);
        setServerInfo({ provider: data.provider, model: data.model });
      } else {
        setServerOnline(false);
        setServerInfo(null);
      }
    } catch {
      setServerOnline(false);
      setServerInfo(null);
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

  async function loadAuthors() {
    try {
      const a = await getAuthors();
      setAuthors(a);
    } catch {
      // silent
    }
  }

  async function loadHistory(reset = false, authorFilter?: string) {
    const author = authorFilter !== undefined ? authorFilter : selectedAuthor;
    const offset = reset ? 0 : history.length;
    try {
      const h = await getHistory({
        limit: PAGE_SIZE,
        offset,
        author: author || undefined,
      });
      if (reset) {
        setHistory(h.items);
      } else {
        setHistory((prev) => [...prev, ...h.items]);
      }
      setHistoryTotal(h.total);
    } catch {
      // silent
    }
  }

  async function handleRefresh() {
    setRefreshing(true);
    // Trigger content script to scan new posts on the active tab
    try { await rescan(); } catch { /* tab may not have content script */ }
    // Wait a moment for analyses to complete, then reload history
    setTimeout(async () => {
      await loadHistory(true);
      await loadAuthors();
      setRefreshing(false);
    }, 2000);
  }

  async function handleLoadMore() {
    setLoading(true);
    await loadHistory(false);
    setLoading(false);
  }

  async function handleAuthorChange(author: string) {
    setSelectedAuthor(author);
    await loadHistory(true, author);
  }

  function openFullscreen() {
    const url = chrome.runtime.getURL("popup.html");
    chrome.tabs.create({ url });
  }

  const hasMore = history.length < historyTotal;

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
              <select
                className="author-filter"
                value={selectedAuthor}
                onChange={(e) => handleAuthorChange(e.target.value)}
              >
                <option value="">All authors ({historyTotal})</option>
                {authors.map((a) => (
                  <option key={a} value={a}>{a}</option>
                ))}
              </select>
              <button
                className={`refresh-btn${refreshing ? " spinning" : ""}`}
                onClick={handleRefresh}
                title="Refresh history"
              >
                &#x21BB;
              </button>
            </div>
            {history.length === 0 ? (
              <p className="empty">
                {selectedAuthor
                  ? `No results for ${selectedAuthor}`
                  : "No analyses yet — browse X, Instagram, or LinkedIn"}
              </p>
            ) : (
              <>
                <div className="history-list">
                  {history.map((item) => (
                    <ScoreCard key={item.id} item={item} />
                  ))}
                </div>
                {hasMore && (
                  <button
                    className="load-more-btn"
                    onClick={handleLoadMore}
                    disabled={loading}
                  >
                    {loading ? "Loading..." : `Load more (${history.length} of ${historyTotal})`}
                  </button>
                )}
              </>
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
          <Settings settings={settings} onUpdate={handleSettingsUpdate} serverInfo={serverInfo} />
        )}
      </main>
    </div>
  );
}
