# Task Ledger

## Current phase: Phase 1 frozen domain contracts - Red

1. [x] Capture the stated task checklist and tray/startup requirements.
   - Acceptance: Plan identifies close-to-tray and explicit-exit semantics.
2. [x] Compare established Windows desktop/tray foundations at a preliminary level.
   - Acceptance: a proposed stack and trade-off is recorded as unapproved.
3. [x] Resolve product choices with the owner.
   - Evidence: Tauri approved; setup asks about startup; current Windows local time defines a new day; past days are editable; two prior JSON backups plus JSON import/export are in scope.
4. [x] Obtain architectural approval and mark ADRs accepted.
   - Evidence: ADR-001 through ADR-005 accepted on 2026-09-02.
5. [x] Execute the Phase 0 Windows toolchain/tray/autostart spike.
   - Acceptance: five manual lifecycle and startup checks in `PLAN.md` pass without elevation.
   - Documentation: initialize and maintain `README.md` and `CODEBASE.md` from verified repository facts; do not invent paths or commands.
   - Git: inspect `https://github.com/AdityaHegde712/Ephemera.git`, confirm remote branches, then use a dedicated `feature/*` branch from `dev`; the owner performs merges.
   - Evidence: canonical remote was queried on 2026-09-02; it returned no published `main` or `dev` refs. Local `dev` baseline commit `37fc2ff` and `feature/phase-0-tauri-spike` were created without a push. Node.js `v24.14.0`, npm `11.9.0`, Cargo `1.98.0`, and Rustc `1.98.0` launch successfully after the current-user Rustup installation. The official Tauri 2 vanilla TypeScript scaffold, autostart plugin, narrow default capability, and native tray lifecycle spike are present.
   - Automated verification: the frontend production build, Rust formatting check, and Cargo compile check pass. The documented `tray-icon` feature is enabled. A clean `npm run tauri dev` launch built and ran `target\\debug\\habit-tracker.exe` on the stable MSVC target.
   - Packaging evidence: `npm run tauri build` completed on the stable MSVC target. `src-tauri/target/release/bundle/msi/Habit Tracker_0.1.0_x64_en-US.msi` is 2,965,504 bytes with SHA-256 `0B37287EB17A3BE2002C6739985B45DE000BC40254C79E32C72CBB7A56EC821A`. `src-tauri/target/release/bundle/nsis/Habit Tracker_0.1.0_x64-setup.exe` is 1,907,213 bytes with SHA-256 `AD521E6944A158309DA96D4F2DCD8A41C3F21D01F9FF52513B3C7EF570CD5CC2`.
   - Manual evidence: the owner installed the generated artifact and confirmed close hides the window, tray Open restores it, and tray Exit removes both the window and tray icon. Per-user data-location verification is deferred because SQLite persistence begins in Phase 2.
   - Autostart evidence: the approved temporary `--phase0-autostart=enable|disable|status` Rust diagnostic compiled. It performed `enable -> true`, independent `status -> true`, and `disable -> false` as the current user without elevation. The debug WebView emitted a non-fatal class-unregistration error after each intentionally immediate diagnostic exit; Cargo returned success. The diagnostic performs no operation without one exact argument, is not renderer-accessible, and must be removed before Phase 3's final typed command boundary.
   - Next action: commit Phase 0, then create and run the frozen Phase 1 contract tests.
6. [/] Define the streak qualifying-day rule.
   - Blocker: streak calculations cannot be implemented without knowing which completion fraction qualifies.
   - Acceptance: ADR-006 is accepted and `tests/spec/streaks.rs` receives frozen assertions.
