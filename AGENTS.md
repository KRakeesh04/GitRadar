# Agent Instructions for LLM Models

This file sets the operating rules for any LLM agent (AI coding assistant) working on this repository.

## Project Overview

GitRadar is a **Tauri v2 desktop application** that tracks and visualizes local Git repositories. It uses:
- **Frontend**: React 19 + TypeScript, TanStack Router, TanStack Query, Zustand, TailwindCSS v4, shadcn/ui
- **Backend**: Rust (Tauri commands/services), SQLite database
- **Architecture**: Frontend calls Rust functions directly via Tauri IPC (`invoke`). No HTTP API layer.

## IMPORTANT: Read CONTEXT.md First

**Before making any changes to this repository, you MUST read `CONTEXT.md` in the project root.**

`CONTEXT.md` is a living document that tracks:
- The current state and architecture of the project
- Recent changes made and where they were implemented
- Feature-specific implementation details
- Gotchas and conventions established during development

It represents the system's "memory" so that any LLM agent can understand the project's current status without scanning the entire codebase. Always treat it as a source of truth and update it after completing any changes.

## Operating Rules

1. **Read `CONTEXT.md` before starting** any task to understand the project's current state.
2. **Update `CONTEXT.md` after completing changes** — append a summary of what you changed, where, and why. This keeps the document current for future agents.
3. **When updating CONTEXT.md**, follow these guidelines:
   - Append new changes to the appropriate section (or create a new section if it's a new area).
   - Keep entries concise but specific: file paths, function names, and the reasoning behind changes.
   - If a change makes an old entry obsolete, update or remove the old entry.
   - Note any new conventions, patterns, or gotchas you discovered.
   - Record API endpoints/commands you added or modified.
   - Record database schema changes.
4. **Never guess** about project state from stale memory — always verify against `CONTEXT.md` and the codebase.
5. **Preserve context** — if you learn something important while working (a quirk, a design decision, a non-obvious pattern), record it in `CONTEXT.md` so it's not lost.

## Verification Commands

After making changes, run the appropriate checks:
- `bun run type-check` — TypeScript type checking
- `bun run lint` — ESLint
- `bun run format:check` — Prettier
- `bun run rust:check` — Rust `cargo check`
- `bun run rust:lint` — Rust `cargo clippy`

Note: pre-existing lint errors exist in:
- `src/components/commit-previewer/commit-details.tsx`
- `src/components/commit-previewer/commit-graph-hooks.ts`
- `src/components/ui/chart.tsx`
- `src/components/ui/sidebar.tsx`
- `src/routes/root-paths.tsx`

Do not "fix" these unless explicitly asked — they are unrelated to most work.
