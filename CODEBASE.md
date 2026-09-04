# Habitus Codebase Map

## Current layout

```text
habit-tracker/
├── .agent-tasks/                 # Approved plan, decisions, and task ledger
├── src/                          # Phase 4 typed TypeScript dashboard
│   ├── habit-api.ts              # Renderer-only typed Tauri command adapter
│   ├── main.ts                   # Dashboard state loading and event handling
│   └── styles.css                # Responsive dark dashboard presentation
├── src-tauri/
│   ├── src/
│   │   ├── application.rs        # Typed command-facing service
│   │   ├── commands.rs           # Tauri adapters and host app-data setup
│   │   ├── domain/               # Date checks, task policy, daily record behavior
│   │   └── persistence/          # Transfer validation, backups, SQLite repository
│   └── Cargo.toml                # Rust crate and explicit test targets
├── tests/
│   ├── spec/                     # Frozen Phase 1 behavioral contracts
│   └── integration/              # Mutable repository behavior tests
├── README.md
└── CODEBASE.md
```

## Native integration architecture

`src-tauri/src/domain/task_policy.rs` owns stable task IDs, Monday/Wednesday/Friday exercise applicability, and completion colors. `date_validation.rs` rejects future local dates. `history.rs` retains historical task selections independently of the current day.

`src-tauri/src/persistence/transfer.rs` validates import/export documents and applies a typed task change while preserving the record's initial applicable-task snapshot. `repository.rs` is the sole SQLite boundary: it opens an idempotent schema, serializes whole-state documents, and commits imports or task changes transactionally. `backup_recovery.rs` stages and rotates the two JSON recovery files.

`application.rs` owns the typed use cases and a mutex-protected repository. `commands.rs` resolves only Tauri's local app-data directory, supplies current local/UTC time, and registers the seven approved commands. The Phase 0 lifecycle in `src-tauri/src/lib.rs` still owns close-to-tray, tray Open, and tray Exit; its temporary CLI autostart diagnostic has been removed. The sole capability is `core:default`; the renderer has no direct SQLite, filesystem, autostart-plugin, or arbitrary command access.

`src/habit-api.ts` is the renderer's single IPC adapter. `src/main.ts` loads today's record, a selected historical record, and calendar summaries through that adapter; it renders task controls from the host-provided applicable-task snapshot and sends only stable task IDs back to the host. `styles.css` keeps the approved gray, black, and green dashboard responsive without a framework dependency.

## Data flow

```text
Renderer typed command
  -> commands.rs validates host context and time
  -> application.rs typed use case
  -> Repository typed mutation/import
  -> validate full state and stage pre-change JSON
  -> SQLite transaction
  -> promote backup rotation
  -> typed result to renderer
```

## Verification

`tests/spec/` is frozen without owner approval and currently contains 11 assertions for task policy, history, backups, and import/export. `tests/integration/repository.rs` verifies backup-aware import, typed task mutation snapshots, and idempotent reopening of an existing SQLite database. `tests/integration/command_boundary.rs` verifies host-owned persistence, input rejection, and typed day/transfer operations.

The verified command is `cargo test --manifest-path src-tauri/Cargo.toml --tests`; it runs 16 assertions. Rust formatting, Rust compile checks, the Vite production build, and an x64 MSI/NSIS package build are also verified. Manual Windows lifecycle, autostart, and persistence checks remain pending.

`npm run test:ui` builds the renderer and runs the mutable Playwright dashboard integration script with a typed Tauri IPC mock. It verifies historical selection and a today-task mutation at desktop and narrow viewports.
