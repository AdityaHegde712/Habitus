# Habit Tracker

Windows-local habit tracker built with Tauri 2, a TypeScript renderer, and a typed Rust persistence boundary.

## Current status

Phase 3 native integration is ready for manual Windows verification. The Rust host exposes only typed commands for daily records, JSON transfer, and opt-in autostart. UI layout and settings presentation remain deferred to the approved Phase 4 design pass.

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

The Rust host resolves Tauri's per-user local app-data directory; the renderer never supplies a filesystem path. `Repository` creates `habit-tracker.sqlite3` and a `backups/` directory there. Validated imports and typed task changes follow this order:

1. Read and stage the complete pre-change state as JSON.
2. Commit the replacement state in one SQLite transaction.
3. Promote the staged JSON to `previous-1.json`, rotating the former latest backup to `previous-2.json`.

Startup recovery promotes a valid staged backup left by an interrupted rotation. Invalid imports are rejected before any state write. Export serializes the current complete state.

## Native command boundary

The bundled renderer can invoke only `get_day`, `set_task_checked`, `list_calendar_days`, `export_state`, `import_state`, `get_autostart_status`, and `set_autostart_enabled`. Dates, stable task IDs, totals, and import documents are validated in Rust. The autostart plugin is managed only by these Rust commands and defaults to disabled; there is no renderer-facing plugin permission or arbitrary SQL, filesystem, or shell command path.

## Scope

The approved MVP is local-first and Windows-only. Cloud sync, accounts, reminders, mobile clients, habit customization, multi-user sharing, encryption at rest, and automatic cloud backup are not in scope.
