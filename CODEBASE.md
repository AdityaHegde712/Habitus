# Codebase Map

## Current layout

```text
habit-tracker/
├── .agent-tasks/                 # Approved plan, decisions, and task ledger
├── src/                          # Phase 0 TypeScript renderer scaffold
├── src-tauri/
│   ├── src/
│   │   ├── domain/               # Date checks, task policy, daily record behavior
│   │   └── persistence/          # Transfer validation, backups, SQLite repository
│   └── Cargo.toml                # Rust crate and explicit test targets
├── tests/
│   ├── spec/                     # Frozen Phase 1 behavioral contracts
│   └── integration/              # Mutable repository behavior tests
├── README.md
└── CODEBASE.md
```

## Phase 2 architecture

`src-tauri/src/domain/task_policy.rs` owns stable task IDs, Monday/Wednesday/Friday exercise applicability, and completion colors. `date_validation.rs` rejects future local dates. `history.rs` retains historical task selections independently of the current day.

`src-tauri/src/persistence/transfer.rs` validates import/export documents and applies a typed task change while preserving the record's initial applicable-task snapshot. `repository.rs` is the sole SQLite boundary: it opens an idempotent schema, serializes whole-state documents, and commits imports or task changes transactionally. `backup_recovery.rs` stages and rotates the two JSON recovery files.

The Phase 0 lifecycle in `src-tauri/src/lib.rs` owns close-to-tray, tray Open, and tray Exit. It contains only a temporary command-line autostart diagnostic; Phase 3 will replace it with the approved typed command boundary. No renderer code has direct SQLite, filesystem, or arbitrary command access.

## Data flow

```text
Phase 3 typed command (deferred)
  -> Repository typed mutation/import
  -> validate full state and stage pre-change JSON
  -> SQLite transaction
  -> promote backup rotation
  -> typed result to renderer
```

## Verification

`tests/spec/` is frozen without owner approval and currently contains 11 assertions for task policy, history, backups, and import/export. `tests/integration/repository.rs` verifies backup-aware import, typed task mutation snapshots, and idempotent reopening of an existing SQLite database.

The verified Phase 2 command is `cargo test --manifest-path src-tauri/Cargo.toml --tests`; it runs 14 assertions. Rust formatting, Rust compile checks, and the Vite production build are also verified.
