# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ACC Tools is a Rust workspace for parsing and displaying Assetto Corsa Competizione (ACC) race session data. It supports both JSON session files (exported by ACC) and PDF qualifying results.

## Build Commands

```bash
# Build entire workspace
cargo build

# Run Tauri desktop app (dev mode)
cd acc-session-reader && cargo tauri dev

# Build WASM frontend only (without Tauri)
trunk serve --config acc-session-reader/Trunk.toml

# Run tests
cargo test

# Run a single test
cargo test test_parse_fp_session_from_bytes

# Format code (uses 2-space tabs per .rustfmt.toml)
cargo fmt

# Lint
cargo clippy --all-targets --all-features

# Build Tailwind CSS
npm run build:css

# Watch Tailwind CSS changes
npm run watch:css
```

## Architecture

```
acc-tools/
├── acc-session-reader/          # Main application
│   ├── app/                     # Leptos frontend (CSR + hydrate)
│   ├── server/                  # Axum SSR backend (not yet implemented)
│   ├── lib/                     # Core parsing library
│   ├── src-tauri/               # Tauri desktop wrapper
│   ├── index.html               # WASM entry point
│   └── Trunk.toml               # Trunk build config
├── acc-ui-components/           # Reusable Leptos UI components
│   └── style/tailwind.css       # Tailwind CSS configuration
├── package.json                 # npm dependencies for Tailwind CSS
└── Cargo.toml                   # Workspace root
```

### Crates

- **acc-session-reader/lib**: Core parsing library (no UI dependencies)
  - JSON parser for ACC session files (UTF-16 encoded)
  - PDF parser for qualifying results (using `nom` + `pdf-extract`)
  - Domain models: `Session`, `QualifyingSession`, `LoadedSession`, `DriverStats`, `Track`, `CarModel`, etc.
  - `SessionParser` trait with `JsonParser` and `PdfParser` implementations

- **acc-ui-components**: Reusable Leptos UI components
  - Uses Tailwind CSS 4 for styling (configured in `style/tailwind.css`)
  - Uses rust-ui component library (`leptos_ui`, `icons`, `tw_merge`)
  - Feature flags: `csr`, `hydrate`, `ssr` for different rendering modes
  - Components: `FileUpload`, `CarSelector`, `DriverStatsTable`, `SessionInfo`, `CopyToClipboardButton`
  - UI primitives in `components/ui/`: button, checkbox, data_table, dialog, dropdown_menu, etc.

- **acc-session-reader/app**: Main Leptos application
  - Feature flags: `csr` (client-side), `ssr` (server-side), `hydrate`
  - `SessionState` for reactive state management via Leptos signals
  - Pages: `UploadPage`, `SessionPage`
  - WASM entry point included (enabled with `csr` feature)

- **acc-session-reader/server**: Axum SSR server (placeholder, not yet implemented)

- **acc-session-reader/src-tauri**: Tauri desktop app configuration

### Key Patterns

- **LoadedSession**: Unified abstraction over JSON `Session` and PDF `QualifyingSession` for UI consumption
- **Feature flags**: Crates use `csr`/`ssr`/`hydrate` features to conditionally compile browser-only code (wasm-bindgen, web-sys)
- **Tailwind CSS**: Built via npm scripts or Trunk pre-build hook; output goes to `acc-session-reader/tailwind-output.css`
- **rust-ui components**: shadcn-style components using `leptos_ui` and `tw_merge` for class merging

## Development Environment

Uses `devenv.nix` with:
- Rust 1.94.0 with `wasm32-unknown-unknown` target
- Node.js 20 for Tailwind CSS builds
- `cargo-tauri`, `trunk` for building
- Pre-commit hook runs `rustfmt`

### devenv Scripts

- `acc-session-reader:desktop` - Run Tauri desktop app in dev mode
- `acc-session-reader:web` - Run trunk web server
- `tailwind:build` - Build Tailwind CSS
- `tailwind:watch` - Watch and rebuild Tailwind CSS

## Test Fixtures

Test session files are in `acc-session-reader/lib/tests/fixtures/`. Tests use `include_bytes!` for compile-time embedding.
