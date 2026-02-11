# AI Content Detector

Chrome/Firefox extension + Rust backend that detects AI-generated content on X/Twitter, Instagram, and LinkedIn. Posts are analyzed using OpenRouter LLM API combined with local heuristic analysis, and inline score badges are injected next to each post.

## Table of Contents

- [Screenshots](#screenshots)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Docker](#docker)
- [API](#api)
- [Detection Pipeline](#detection-pipeline)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [Changelog](#changelog)
- [License](#license)

## Screenshots

<table>
  <tr>
    <td><img src="client/public/x.png" alt="X/Twitter detection" width="400"></td>
    <td><img src="client/public/instagram.png" alt="Instagram detection" width="400"></td>
  </tr>
  <tr>
    <td><img src="client/public/linkedin.png" alt="LinkedIn detection" width="400"></td>
    <td><img src="client/public/settings.png" alt="Settings panel" width="400"></td>
  </tr>
</table>

## Architecture

```
┌─────────────────────┐     POST /api/analyze     ┌──────────────────────┐
│  Browser Extension   │ ──────────────────────►   │  Rust/Axum Server    │
│                      │                           │                      │
│  Content Scripts     │     { score, label,       │  OpenRouter LLM API  │
│  (X, IG, LinkedIn)  │ ◄──────────────────────   │  + Heuristic Engine  │
│                      │       breakdown }         │  + SQLite Cache      │
│  Popup UI (React)    │                           │                      │
└─────────────────────┘                           └──────────────────────┘
```

## Prerequisites

- **Rust** (1.75+): https://rustup.rs
- **Node.js** (18+): https://nodejs.org
- **OpenRouter API key**: https://openrouter.ai/keys

## Setup

### 1. Server

```bash
cd server
cp .env.example .env
# Edit .env — set your OPENROUTER_API_KEY and API_KEY
```

`.env` variables:

| Variable | Required | Description |
|---|---|---|
| `PORT` | No (default: `3000`) | Server port |
| `DATABASE_URL` | No (default: `sqlite:data.db`) | SQLite database path |
| `OPENROUTER_API_KEY` | **Yes** | Your OpenRouter API key |
| `OPENROUTER_API_MODEL` | **Yes** | LLM model (e.g. `qwen/qwen3-coder`) |
| `API_KEY` | No | Extension auth key (leave empty to disable auth) |

```bash
cargo build --release
cargo run --release
```

Verify:

```bash
curl http://localhost:3000/api/health
# {"status":"ok","version":"0.1.0"}
```

### 2. Extension

```bash
cd client
npm install
npm run build
```

Produces `client/dist/` — the loadable extension.

### 3a. Chrome

1. Open `chrome://extensions`
2. Enable **Developer mode**
3. Click **Load unpacked** → select `client/dist`

### 3b. Firefox

1. Open `about:debugging#/runtime/this-firefox`
2. Click **Load Temporary Add-on...**
3. Select `client/dist/manifest.json`

> Firefox temporary add-ons are removed on close. For permanent install, sign via [addons.mozilla.org](https://addons.mozilla.org).

### 4. Configure

1. Click the extension icon → **Settings** tab
2. Set **Server URL** to `http://localhost:3000`
3. Set **API Key** if configured in server `.env`
4. Save — green dot in header confirms connection

### 5. Use

Browse X, Instagram, or LinkedIn. Score badges appear inline:

- **Green (0-3)**: Human-written
- **Yellow (4-6)**: Mixed / uncertain
- **Red (7-10)**: AI-generated

Hover for breakdown. Click **x** to dismiss.

## Docker

```bash
cd server

# Set env vars
cp .env.example .env
# Edit .env with your keys

# Run
docker compose -f docker/compose.yml up -d
```

The server runs on port 3000 with SQLite data persisted in a Docker volume.

## API

### `GET /api/health`
Health check. No auth required.

### `POST /api/analyze`
Requires `x-api-key` header if `API_KEY` is set.

```json
// Request
{
  "content": "Text to analyze...",
  "platform": "twitter",
  "post_id": "optional-id",
  "author": "optional-username"
}

// Response
{
  "score": 8,
  "confidence": 0.95,
  "label": "ai",
  "breakdown": {
    "llm_score": 9,
    "heuristic_score": 6,
    "signals": ["low_sentence_variance", "formulaic_phrases"]
  }
}
```

Labels: `human` (0-3), `mixed` (4-5), `likely_ai` (6-7), `ai` (8-10)

### `GET /api/history?limit=20&offset=0`
Paginated analysis history. Requires `x-api-key` header if `API_KEY` is set.

## Detection Pipeline

Two engines run in parallel per analysis:

1. **OpenRouter LLM** (60% weight) — structured AI detection prompt
2. **Heuristic Engine** (40% weight) — pure Rust statistical analysis:
   - Sentence length variance
   - Type-token ratio / vocabulary diversity
   - Burstiness measurement
   - Formulaic phrase detection (35+ patterns)
   - Punctuation pattern analysis

Results cached by content hash in SQLite.

## Project Structure

```
server/                    Rust/Axum backend
├── src/
│   ├── main.rs            Server, routes, middleware
│   ├── config.rs          Env configuration
│   ├── db.rs              SQLite pool + queries
│   ├── auth.rs            API key middleware
│   ├── errors.rs          Error types
│   ├── models.rs          Request/response/DB types
│   ├── routes/
│   │   ├── analyze.rs     POST /api/analyze
│   │   ├── health.rs      GET /api/health
│   │   └── history.rs     GET /api/history
│   └── services/
│       ├── detector.rs    LLM + heuristics orchestration
│       ├── openrouter.rs  OpenRouter API client
│       └── heuristics.rs  Statistical text analysis
├── migrations/
│   └── 001_init.sql
├── docker/
│   ├── Dockerfile
│   └── compose.yml
└── .env.example

client/                    Browser Extension (React/Vite/TypeScript)
├── src/
│   ├── background/        Service worker (API calls, caching)
│   ├── content/           Content scripts
│   │   ├── platforms/     X, Instagram, LinkedIn extractors
│   │   ├── observer.ts    MutationObserver for dynamic feeds
│   │   └── inject.ts      Shadow DOM badge injection
│   ├── popup/             Extension popup UI (React)
│   ├── shared/            Types, constants, messaging
│   └── styles/            Badge + popup CSS
├── public/
│   ├── manifest.json      Manifest V3 (Chrome + Firefox)
│   └── icons/
└── vite.config.ts
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## License

[MIT](LICENSE) - Adolfo Usier
