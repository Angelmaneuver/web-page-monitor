---
name: tauri-v2-development
description: Use this skill when implementing Tauri v2 backend features, Rust commands, window behavior, or native integrations for this desktop app.
---

# Tauri v2 development

## When to use

- Adding or changing Tauri commands in src-tauri/src
- Implementing window behavior or desktop-specific features
- Connecting React frontend code to Rust backend through invoke
- Working with Tauri plugins such as dialog, store, opener, or window state

## Repository-specific guidance

- Treat the app as a desktop application first, not a web app.
- Keep native behavior in Rust and UI logic in React.
- Prefer small, focused commands and keep the frontend/backend boundary explicit.
- Reuse existing helpers in src/lib/tauri and src-tauri/src instead of creating duplicate logic.
- Follow the existing Tauri v2 structure in src-tauri/src/lib.rs and src-tauri/src/window.rs.

## Recommended workflow

1. Understand the existing command or window flow before editing.
2. Implement the Rust-side behavior in src-tauri/src.
3. Expose it through a clear Tauri command or window helper.
4. Wire the frontend through src/lib/tauri.
5. Verify with cargo check and, when possible, run the app locally.

## Verification

- Run cargo check after Rust or Tauri changes.
- Run bun run build when frontend wiring changes.
- If the change affects runtime behavior, verify it in the desktop app.
