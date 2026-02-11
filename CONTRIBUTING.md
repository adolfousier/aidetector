# Contributing

Contributions are welcome. Here's how to get started.

## Development Setup

### Server

```bash
cd server
cp .env.example .env
# Edit .env with your OpenRouter API key
cargo run
```

### Client

```bash
cd client
npm install
npm run build
```

Load `client/dist/` as an unpacked extension in Chrome or Firefox.

## Making Changes

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Test manually on X, Instagram, and LinkedIn
5. Commit: `git commit -m "Add your feature"`
6. Push: `git push origin feature/your-feature`
7. Open a pull request

## Guidelines

- Keep changes focused and minimal
- Test on both Chrome and Firefox
- Don't commit `.env` files or API keys
- Run `cargo build` (server) and `npm run build` (client) before committing to verify no errors

## Reporting Issues

Open an issue with:
- Steps to reproduce
- Expected vs actual behavior
- Browser and platform (X/IG/LinkedIn)
- Console logs if applicable
