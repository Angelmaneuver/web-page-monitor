---
name: desktop-app-operations
description: Use this skill for build, run, debug, and verification tasks specific to this Tauri desktop application.
---

# Desktop app operations

## When to use

- Running the app locally
- Building or checking the frontend and backend
- Troubleshooting startup or runtime issues
- Preparing changes for validation in the desktop environment

## Repository-specific guidance

- This project is a desktop app, so verify behavior in the actual app when changes affect UX or native features.
- Use Bun for frontend tasks and Cargo for Rust/Tauri checks.
- Prefer the existing project commands:
  - bun run dev
  - bun run build
  - cargo check
  - cargo tauri dev

## Recommended workflow

1. Identify whether the change is frontend-only, Rust-only, or cross-stack.
2. Run the relevant verification command before and after the change.
3. If the issue is runtime-related, validate it with the desktop app directly.
4. Keep the change scoped and document any environment-specific caveats.

## Verification

- Frontend changes: bun run build
- Rust or Tauri changes: cargo check
- UI/behavior changes: run the app locally if possible
