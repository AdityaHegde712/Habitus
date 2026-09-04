# Habit Tracker

Windows-local habit tracker built with Tauri 2, a TypeScript renderer, and a typed Rust persistence boundary.

## Current status

Phase 4 provides the approved single-page dashboard: today's editable checklist is fixed at the top, and a Git-style history tracker with an inspectable saved checklist fills the lower page. The Rust host exposes only typed commands for daily records, JSON transfer, and opt-in autostart.

## Verified developer commands

Run these from the repository root with the stable Windows MSVC Rust toolchain:

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --manifest-path src-tauri/Cargo.toml --tests
cargo check --manifest-path src-tauri/Cargo.toml
npm run build
npm run test:ui
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

## Dashboard behavior

The app opens to today's local checklist. Checkbox changes invoke the typed `set_task_checked` command and refresh the dashboard from persisted state. The completion tracker represents the preceding 18 weeks. Selecting a tracker cell invokes `get_day` for that local date and displays its persisted task snapshot; historical applicability is never recalculated in the renderer. The lower detail panel sits beside the tracker on desktop and below it on narrow viewports.

## Scope

The approved MVP is local-first and Windows-only. Cloud sync, accounts, reminders, mobile clients, habit customization, multi-user sharing, encryption at rest, and automatic cloud backup are not in scope.
