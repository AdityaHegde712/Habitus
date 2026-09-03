# Habit Tracker

Windows-local habit tracker built with Tauri 2, a TypeScript renderer, and a typed Rust persistence boundary.

## Current status

Phase 2 is complete. The Rust domain and repository layers implement deterministic daily task applicability, historical task snapshots, validated JSON import and export, SQLite storage, and two rotating JSON backups. Native Tauri commands, the final app-data location, and UI are deferred to later approved phases.

## Verified developer commands

Run these from the repository root with the stable Windows MSVC Rust toolchain:

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --tests
cargo check --manifest-path src-tauri/Cargo.toml
npm run build
```

Phase 0 also verified the following Windows commands and packaged tray lifecycle:

```powershell
npm run tauri dev
npm run tauri build
```

## Data and recovery behavior

The Rust host will provide the per-user application-data directory in Phase 3. Given that host-owned directory, `Repository` creates `habit-tracker.sqlite3` and a `backups/` directory. Validated imports and typed task changes follow this order:

1. Read and stage the complete pre-change state as JSON.
2. Commit the replacement state in one SQLite transaction.
3. Promote the staged JSON to `previous-1.json`, rotating the former latest backup to `previous-2.json`.

Startup recovery promotes a valid staged backup left by an interrupted rotation. Invalid imports are rejected before any state write. Export serializes the current complete state.

## Scope

The approved MVP is local-first and Windows-only. Cloud sync, accounts, reminders, mobile clients, habit customization, multi-user sharing, encryption at rest, and automatic cloud backup are not in scope.
