# Agent guide for this repository

## Overview

This project is a Tauri v2 desktop app with a React frontend and a Rust backend.

## Tech stack

- React 19 + TypeScript + Vite
- shadcn/ui + Tailwind CSS 4
- Rust with Tauri 2
- Bun for package management

## Repository layout

- src/: frontend application
  - src/components/ui: UI primitives and shadcn-style components
  - src/components/features: feature modules
  - src/lib/utils: shared utilities
  - src/lib/tauri: Tauri API wrappers
- src-tauri/: Rust backend and Tauri setup
  - src/lib.rs: command definitions and app setup
  - src/window.rs: window-related logic

## Working conventions

- Treat this as a desktop application first.
- Use Rust for native behavior and React for UI.
- Keep changes scoped and minimal.
- Prefer composable React components over deeply nested logic.
- Avoid introducing new state libraries unless necessary.
- When a task needs OS access or window behavior, implement it in the Rust side and expose it cleanly to the frontend.

## Verification

- Run bun run build for frontend-facing changes.
- Run cargo check for Rust-related changes.
- Run the app locally when a change affects behavior or user experience.
