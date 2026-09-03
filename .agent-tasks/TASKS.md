# Task Ledger

## Current phase: Phase 0 toolchain and lifecycle spike in progress

1. [x] Capture the stated task checklist and tray/startup requirements.
   - Acceptance: Plan identifies close-to-tray and explicit-exit semantics.
2. [x] Compare established Windows desktop/tray foundations at a preliminary level.
   - Acceptance: a proposed stack and trade-off is recorded as unapproved.
3. [x] Resolve product choices with the owner.
   - Evidence: Tauri approved; setup asks about startup; current Windows local time defines a new day; past days are editable; two prior JSON backups plus JSON import/export are in scope.
4. [x] Obtain architectural approval and mark ADRs accepted.
   - Evidence: ADR-001 through ADR-005 accepted on 2026-09-02.
5. [/] Execute the Phase 0 Windows toolchain/tray/autostart spike.
   - Acceptance: five manual lifecycle and startup checks in `PLAN.md` pass without elevation.
   - Documentation: initialize and maintain `README.md` and `CODEBASE.md` from verified repository facts; do not invent paths or commands.
   - Git: inspect `https://github.com/AdityaHegde712/Ephemera.git`, confirm remote branches, then use a dedicated `feature/*` branch from `dev`; the owner performs merges.
   - Evidence: canonical remote was queried on 2026-09-02; it returned no published `main` or `dev` refs. A local `dev` repository and its `origin` were initialized without a push. Node.js `v24.14.0` and npm `11.9.0` launch successfully.
   - Blockers: (1) the discovered `C:\Users\hifia\.cargo\bin\cargo.exe` and `rustc.exe` cannot launch because Windows reports that no application is associated with the executables; (2) Git can read `.git` but cannot create `.git/index.lock` or `.git/HEAD.lock`, so the required baseline commit and `feature/phase-0-tauri-spike` branch cannot be created. Tauri scaffolding, tray validation, and Rust contract tests cannot begin until the Rust toolchain and Git metadata write permission are repaired.
   - Next action: restore `.git` write permission and repair or reinstall the Windows Rust toolchain, then verify Git can commit plus `cargo --version` and `rustc --version` before creating the Tauri scaffold.
6. [/] Define the streak qualifying-day rule.
   - Blocker: streak calculations cannot be implemented without knowing which completion fraction qualifies.
   - Acceptance: ADR-006 is accepted and `tests/spec/streaks.rs` receives frozen assertions.
