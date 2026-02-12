import type { AnalyzeResponse } from "../shared/types";

const BADGE_CSS = `
.aid-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  border-radius: 12px;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  font-size: 11px;
  font-weight: 600;
  line-height: 1.4;
  cursor: default;
  user-select: none;
  transition: opacity 0.2s;
  vertical-align: middle;
  margin-left: 6px;
}
.aid-badge:hover { opacity: 0.8; }
.aid-badge--human { background: #f0fdf4; color: #22c55e; border: 1px solid #bbf7d0; }
.aid-badge--mixed { background: #fefce8; color: #ca8a04; border: 1px solid #fde68a; }
.aid-badge--ai { background: #fef2f2; color: #ef4444; border: 1px solid #fecaca; }
.aid-badge--loading { background: #f5f5f5; color: #999; border: 1px solid #e5e5e5; }
.aid-badge__dismiss { margin-left: 2px; cursor: pointer; opacity: 0.5; font-size: 10px; }
.aid-badge__dismiss:hover { opacity: 1; }
`;

function getVariant(score: number): string {
  if (score <= 3) return "human";
  if (score <= 6) return "mixed";
  return "ai";
}

function getLabel(score: number, heuristicsOnly: boolean): string {
  if (score <= 3) return "Human";
  if (score <= 6) return heuristicsOnly ? "Uncertain" : "Mixed";
  return "AI";
}

export function injectLoadingBadge(target: HTMLElement): HTMLElement {
  const host = document.createElement("span");
  host.className = "aid-badge-host";
  const shadow = host.attachShadow({ mode: "open" });

  const style = document.createElement("style");
  style.textContent = BADGE_CSS;
  shadow.appendChild(style);

  const badge = document.createElement("span");
  badge.className = "aid-badge aid-badge--loading";
  badge.innerHTML = `<span>Analyzing...</span>`;
  shadow.appendChild(badge);

  target.appendChild(host);
  return host;
}

export function updateBadge(host: HTMLElement, result: AnalyzeResponse): void {
  const shadow = host.shadowRoot;
  if (!shadow) {
    const newShadow = host.attachShadow({ mode: "open" });
    renderBadge(newShadow, host, result);
    return;
  }
  renderBadge(shadow, host, result);
}

function renderBadge(shadow: ShadowRoot, host: HTMLElement, result: AnalyzeResponse): void {
  shadow.innerHTML = "";

  const style = document.createElement("style");
  style.textContent = BADGE_CSS;
  shadow.appendChild(style);

  const heuristicsOnly = result.breakdown.llm_score === null;
  const variant = getVariant(result.score);
  const label = getLabel(result.score, heuristicsOnly);

  const badge = document.createElement("span");
  badge.className = `aid-badge aid-badge--${variant}`;
  badge.title = `AI Score: ${result.score}/10 (${Math.round(result.confidence * 100)}% confidence)\nSignals: ${result.breakdown.signals.join(", ") || "none"}`;

  badge.innerHTML = `
    <span class="aid-badge__score">${result.score}</span>
    <span class="aid-badge__label">${label}</span>
    <span class="aid-badge__dismiss" title="Dismiss">×</span>
  `;

  const dismiss = badge.querySelector(".aid-badge__dismiss");
  dismiss?.addEventListener("click", (e) => {
    e.stopPropagation();
    host.remove();
  });

  shadow.appendChild(badge);
}

export function injectErrorBadge(host: HTMLElement): void {
  const shadow = host.shadowRoot;
  if (!shadow) return;

  shadow.innerHTML = "";

  const style = document.createElement("style");
  style.textContent = BADGE_CSS;
  shadow.appendChild(style);

  const badge = document.createElement("span");
  badge.className = "aid-badge aid-badge--loading";
  badge.innerHTML = `<span>Error</span>`;
  shadow.appendChild(badge);
}
