import React, { useState } from "react";
import type { HistoryItem } from "../../shared/types";
import { getScoreStyle } from "../../shared/constants";

interface Props {
  item: HistoryItem;
}

export function ScoreCard({ item }: Props) {
  const [expanded, setExpanded] = useState(false);
  const style = getScoreStyle(item.score);
  const signals: string[] = (() => {
    try { return JSON.parse(item.signals); } catch { return []; }
  })();

  return (
    <div
      className="score-card"
      style={{ borderLeftColor: style.color }}
      onClick={() => setExpanded(!expanded)}
    >
      <div className="score-card-header">
        <span
          className="score-pill"
          style={{ backgroundColor: style.bg, color: style.color }}
        >
          {item.score}/10
        </span>
        <span className="score-label-text">{style.label}</span>
        <span className="platform-tag">{item.platform}</span>
        <span className="expand-icon">{expanded ? "\u25B2" : "\u25BC"}</span>
      </div>

      <div className="score-card-author">
        <span className="author-name">@{item.author || "unknown"}</span>
        <span className="score-time">
          {new Date(item.created_at + "Z").toLocaleString()}
        </span>
      </div>

      <div className="score-card-preview">
        {item.content_preview}
        {item.content_preview.length >= 148 && "..."}
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
          {item.post_id && (
            <div className="detail-row">
              <span className="detail-label">Post</span>
              <span className="detail-value detail-postid">{item.post_id}</span>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
