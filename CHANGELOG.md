# Changelog

## [0.1.0] - 2026-02-11

### Added
- Rust/Axum backend server with SQLite storage
- OpenRouter LLM integration for AI content scoring
- Heuristic analysis engine (sentence variance, TTR, burstiness, formulaic phrases, punctuation)
- Weighted scoring: 60% LLM + 40% heuristic
- Content hash caching for deduplication
- API key authentication middleware
- REST API: `/api/analyze`, `/api/health`, `/api/history`
- Chrome/Firefox browser extension (Manifest V3)
- Content scripts for X/Twitter, Instagram, LinkedIn
- Shadow DOM inline score badges (green/yellow/red)
- MutationObserver for dynamic feed detection
- React popup UI with history, status, and settings tabs
- Expandable score cards with full breakdown (LLM score, heuristic score, confidence, signals)
- Fullscreen mode (open popup in new tab)
- Auto-refresh history every 5 seconds
- Manual refresh button
- Toast notifications on settings save
- IIFE builds for Firefox compatibility
- Docker support (Dockerfile + compose.yml)
