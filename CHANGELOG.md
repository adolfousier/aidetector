# Changelog

## [0.1.12] - 2026-02-11

### Added
- History pagination with "Load more" button (20 items per page)
- Author filter dropdown to filter history by username
- `/api/authors` endpoint returning distinct authors
- `author` query parameter on `/api/history` for server-side filtering
- New rounded extension icon

### Changed
- Refresh button now also reloads the author list
- Auto-refresh interval increased from 5s to 10s

## [0.1.11] - 2026-02-11

### Added
- Dark mode support (follows system appearance via prefers-color-scheme)
- `justfile` for one-command build and run (`just` or `just run`)

### Changed
- README rewritten with Quick Start using `just`, manual setup in collapsible section

## [0.1.1] - 2026-02-11

### Changed
- Updated all dependencies to latest versions (React 19, Vite 7, TypeScript 5.9, Axum 0.8, SQLx 0.8)
- Platform tag displays "X" instead of "twitter" in history cards
- Expanded score cards show full post content instead of truncated 150-char preview
- Score card hover lifts card with visible shadow effect
- Fullscreen mode uses full viewport width
- History panel fills full available height in popup and fullscreen
- Clickable author names linking to platform profiles
- "View on X/Instagram/LinkedIn" links in expanded card details

### Fixed
- Popup height only filling ~30% of space (missing #root height in flex chain)
- Expanded cards still truncating text in popup (CSS fix only applied in fullscreen media query)

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
