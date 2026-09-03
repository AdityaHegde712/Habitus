# Habit Tracker

Windows-local habit tracker in its approved Phase 0 toolchain-validation stage.

## Current status

No application scaffold or executable code has been created. The approved implementation target is a Tauri 2 desktop application with a TypeScript renderer, a minimal Rust host, a native tray lifecycle, opt-in autostart, and local SQLite persistence.

## Repository workflow

The canonical remote is `https://github.com/AdityaHegde712/Ephemera.git`. It was verified on 2026-09-02 and currently has no published `main` or `dev` branch. Local work begins from `dev` and proceeds on dedicated `feature/*` branches; the owner performs merges.

## Planned developer commands

The following commands will be verified after the Windows Rust toolchain is repaired and the Tauri scaffold exists:

```powershell
npm install
npm run tauri dev
npm run tauri build
cargo test
```

## Data and recovery behavior

Daily history is planned for the current user's application-data directory. Each successful state mutation will retain exactly two preceding full-state JSON backups. JSON import will validate all records before it replaces live state; export will serialize the current validated state.

## Scope

The approved MVP is local-first and Windows-only. Cloud sync, accounts, reminders, mobile clients, habit customization, multi-user sharing, encryption at rest, and automatic cloud backup are not in scope.
