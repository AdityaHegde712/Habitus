# Codebase Map

## Current layout

```text
habit-tracker/
├── .agent-tasks/
│   ├── DECISIONS.md
│   ├── PLAN.md
│   └── TASKS.md
├── CODEBASE.md
└── README.md
```

## Current implementation state

The repository contains no application scaffold, source modules, tests, package manifest, or Cargo manifest yet. The approved implementation plan is the source of truth in `.agent-tasks/PLAN.md`; accepted architecture decisions are in `.agent-tasks/DECISIONS.md`; execution status is in `.agent-tasks/TASKS.md`.

## Planned architecture

Tauri 2 will host a bundled TypeScript renderer. A small Rust layer will own typed Tauri commands, tray lifecycle, autostart integration, and a `rusqlite` repository boundary. The renderer will have no direct SQLite or arbitrary filesystem access.

`src-tauri/src/domain/` contains pure local-date validation, deterministic task applicability, color calculation, and historical record snapshots. `src-tauri/src/persistence/` contains JSON transfer types, rotating-backup recovery, and the SQLite repository boundary. Root-level `tests/spec/` is the immutable contract suite for those modules.

The Phase 0 Rust lifecycle module also contains a temporary, command-line-only autostart diagnostic. It accepts only `--phase0-autostart=enable`, `disable`, or `status`, does nothing by default, and must be removed before the Phase 3 command boundary is implemented.

## Verification status

On 2026-09-02, Node.js `v24.14.0`, npm `11.9.0`, Cargo `1.98.0`, and Rustc `1.98.0` for `x86_64-pc-windows-msvc` were verified. The frontend production build, Rust formatting check, Cargo compile check, development launch, x64 MSI/NSIS packaging, autostart enable/disable diagnostic, and owner-run MSI tray lifecycle check pass. Per-user data-location verification is deferred to Phase 2 because persistence does not exist yet.
