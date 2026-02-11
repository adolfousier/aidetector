import React, { useState } from "react";
import type { HistoryItem } from "../../shared/types";
import { getScoreStyle } from "../../shared/constants";

interface Props {
  item: HistoryItem;
}

function getPostUrl(item: HistoryItem): string | null {
  const pid = item.post_id;
  if (!pid) return null;

  if (item.platform === "twitter" && pid.startsWith("/")) {
    return `https://x.com${pid}`;
  }
  if (item.platform === "linkedin" && pid.startsWith("urn:li:activity:")) {
    const activityId = pid.replace("urn:li:activity:", "");
    return `https://www.linkedin.com/feed/update/urn:li:activity:${activityId}`;
  }
  if (item.platform === "linkedin" && item.author && item.author !== "unknown") {
    const slug = item.author.toLowerCase().replace(/\s+/g, "");
    return `https://www.linkedin.com/in/${slug}`;
  }
  if (item.platform === "instagram" && item.author && item.author !== "unknown") {
    return `https://www.instagram.com/${item.author.replace("@", "")}/`;
  }

  return null;
}

export function ScoreCard({ item }: Props) {
  const [expanded, setExpanded] = useState(false);
  const style = getScoreStyle(item.score);
  const signals: string[] = (() => {
    try { return JSON.parse(item.signals); } catch { return []; }
  })();
  const postUrl = getPostUrl(item);

  function handleClick(e: React.MouseEvent) {
    // If clicking the link, don't toggle expand
    if ((e.target as HTMLElement).closest("a")) return;
    setExpanded(!expanded);
  }

  return (
    <div
      className={`score-card${expanded ? " score-card--expanded" : ""}`}
      style={{ borderLeftColor: style.color }}
      onClick={handleClick}
    >
      <div className="score-card-header">
        <span
          className="score-pill"
          style={{ backgroundColor: style.bg, color: style.color }}
        >
          {item.score}/10
        </span>
        <span className="score-label-text">{style.label}</span>
        <span className="platform-tag">{item.platform === "twitter" ? "X" : item.platform}</span>
        <span className="expand-icon">{expanded ? "\u25B2" : "\u25BC"}</span>
      </div>

      <div className="score-card-author">
        <span className="author-name">
          {postUrl ? (
            <a
              href={postUrl}
              target="_blank"
              rel="noopener noreferrer"
              className="author-link"
              title={`Open on ${item.platform}`}
            >
              @{item.author || "unknown"}
            </a>
          ) : (
            <>@{item.author || "unknown"}</>
          )}
        </span>
        <span className="score-time">
          {new Date(item.created_at + "Z").toLocaleString()}
        </span>
      </div>

      <div className="score-card-preview">
        {expanded ? item.content : (
          <>
            {item.content_preview}
            {item.content_preview.length >= 148 && "..."}
          </>
        )}
      </div>

      {expanded && (
        <div className="score-card-details">
          <div className="detail-row">
            <span className="detail-label">LLM Score</span>
            <span className="detail-value">
              {item.llm_score !== null ? `${item.llm_score}/10` : "N/A"}
            </span>
          </div>
          <div className="detail-row">
            <span className="detail-label">Heuristic Score</span>
            <span className="detail-value">{item.heuristic_score}/10</span>
          </div>
          <div className="detail-row">
            <span className="detail-label">Confidence</span>
            <span className="detail-value">
              {Math.round(item.confidence * 100)}%
            </span>
          </div>
          {signals.length > 0 && (
            <div className="detail-signals">
              <span className="detail-label">Signals</span>
              <div className="signal-tags">
                {signals.map((s) => (
                  <span key={s} className="signal-tag">{s.replace(/_/g, " ")}</span>
                ))}
              </div>
            </div>
          )}
          {postUrl && (
            <div className="detail-row">
              <a
                href={postUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="detail-link"
              >
                View on {item.platform === "twitter" ? "X" : item.platform.charAt(0).toUpperCase() + item.platform.slice(1)}
              </a>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
