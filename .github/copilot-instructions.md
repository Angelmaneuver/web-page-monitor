# Copilot instructions for this workspace

This repository is a Tauri v2 desktop application.

## Stack

- Frontend: React 19 + TypeScript + Vite
- UI: shadcn/ui + Tailwind CSS 4
- Backend: Rust with Tauri 2
- Package manager: Bun

## Project structure

- Frontend code lives in src/
  - src/components/ui: shadcn/ui-style primitives
  - src/components/features: feature-level React components
  - src/lib/utils: shared utility helpers
  - src/lib/tauri: frontend wrappers for Tauri APIs
- Backend code lives in src-tauri/
  - src/lib.rs and src/window.rs: Tauri commands and window behavior

## Development rules

- Assume the target is a desktop app, not a web-only app.
- Prefer Tauri commands for native interactions and OS integration.
- Keep frontend and backend responsibilities separated: UI in React, native behavior in Rust.
- When adding a feature, implement it end-to-end when possible: UI, state, and Tauri bridge.
- Follow existing shadcn/ui patterns and keep components composable and small.
- Reuse existing helpers such as the shared utility functions and Tauri window helpers instead of introducing duplicate logic.
- Avoid adding new frameworks unless explicitly requested.

## Common commands

- bun run dev
- bun run build
- cargo check
- cargo test
- cargo tauri dev

## Verification expectations

- For frontend changes, run bun run build.
- For Rust or Tauri integration changes, run cargo check.
- If the change affects runtime behavior, verify it locally with the app.
