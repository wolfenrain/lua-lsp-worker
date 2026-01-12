# Lua LSP Worker

[![Worker CI](https://github.com/wolfenrain/lua-lsp-worker/actions/workflows/worker.yml/badge.svg)](https://github.com/wolfenrain/lua-lsp-worker/actions/workflows/worker.yml)
[![Container CI](https://github.com/wolfenrain/lua-lsp-worker/actions/workflows/container.yml/badge.svg)](https://github.com/wolfenrain/lua-lsp-worker/actions/workflows/container.yml)

A Cloudflare Worker that provides [Lua Language Server](https://github.com/LuaLS/lua-language-server) functionality over WebSocket. Includes a web-based Monaco editor playground.

## Live Demo

Try it out at [lua.wolfenrain.dev](https://lua.wolfenrain.dev)

## Features

- Full LSP support for Lua via WebSocket
- Web-based Monaco editor with LSP integration
- Autocompletion, hover information, diagnostics
- Support for LuaLS annotations (`---@class`, `---@param`, etc.)
- Each connection gets an isolated LSP instance

The worker routes WebSocket connections to Cloudflare containers. Each container runs a Rust bridge that translates between WebSocket JSON messages and the LSP's stdio protocol.

## Local Development

### Prerequisites

- Node.js 22+
- Docker (for container development)
- Rust (for container development)

### Running the Worker

```bash
npm install
npm run cf-typegen
npm run dev
```

Open http://localhost:8787 in your browser.

### Container Development

```bash
cd container
cargo test
cargo build --release
```

## Deployment

Requires Cloudflare account on the paid plan.

```bash
npm run deploy
```
