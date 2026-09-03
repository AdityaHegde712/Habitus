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

## Verification status

On 2026-09-02, Node.js `v24.14.0` and npm `11.9.0` were verified. Rust tooling is currently unavailable because Windows cannot launch the discovered `cargo.exe` or `rustc.exe`; no Tauri scaffold can be created until that toolchain is repaired.
