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
- **just** (command runner): https://just.systems (`cargo install just`)
- **OpenRouter API key**: https://openrouter.ai/keys

## Quick Start

```bash
git clone https://github.com/adolfousier/aidetector.git
cd aidetector

# Configure server env
cp server/.env.example server/.env
# Edit server/.env — set your OPENROUTER_API_KEY and OPENROUTER_API_MODEL

# Build everything and start the server
just
```

`just` (or `just run`) installs npm dependencies, builds the client extension, compiles the Rust server, and starts it.

| Command | Description |
|---|---|
| `just` / `just run` | Build everything and start the server |
| `just build` | Build client + server without starting |
| `just build-client` | Build only the browser extension |
| `just build-server` | Build only the Rust server |
| `just stop` | Stop the running server |
| `just clean` | Stop server and remove all build artifacts |

### Load the Extension

After running `just`, load `client/dist/` in your browser:

#### Chrome

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

## Manual Setup (without just)

<details>
<summary>Click to expand</summary>

### Environment Variables

Configure in `server/.env`:

| Variable | Required | Description |
|---|---|---|
| `PORT` | No (default: `3000`) | Server port |
| `DATABASE_URL` | No (default: `sqlite:data.db`) | SQLite database path |
| `OPENROUTER_API_KEY` | **Yes** | Your OpenRouter API key |
| `OPENROUTER_API_MODEL` | **Yes** | LLM model (e.g. `qwen/qwen3-coder`) |
| `API_KEY` | No | Extension auth key (leave empty to disable auth) |

### Server

```bash
cd server
cp .env.example .env
# Edit .env with your keys
cargo build
cargo run
```

### Extension

```bash
cd client
npm install
npm run build
```

Produces `client/dist/` — load it in your browser as described in [Quick Start](#quick-start).

### Verify

```bash
curl http://localhost:3000/api/health
```

</details>

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

### `GET /api/history?limit=20&offset=0&author=username`
Paginated analysis history. Requires `x-api-key` header if `API_KEY` is set.

| Parameter | Description |
|---|---|
| `limit` | Max items to return (default: 20, max: 100) |
| `offset` | Skip N items for pagination |
| `author` | Filter by author username |

### `GET /api/authors`
Returns distinct author usernames. Requires `x-api-key` header if `API_KEY` is set.

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
