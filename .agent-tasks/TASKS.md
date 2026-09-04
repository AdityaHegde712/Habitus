# Task Ledger

## Current phase: Phase 4 frontend and calendar - design approval pending

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
   - Git evidence: Phase 0 was merged to `origin/dev` by the owner as `f65cd60`. After the owner merged Phase 1, `git fetch --prune origin` confirmed both feature remotes deleted and `origin/dev` at `97ddd2f`.
6. [x] Create and run the frozen Phase 1 domain and persistence contracts.
   - Red evidence: `cargo test --manifest-path src-tauri/Cargo.toml --tests` failed as expected on 2026-09-02. Each frozen suite failed only because `tauri_app_lib::domain` or `tauri_app_lib::persistence` was not implemented.
   - Contract: `tests/spec/day_policy.rs`, `history.rs`, `backup_contract.rs`, and `import_export_contract.rs` are immutable without owner approval.
   - Green evidence: all 11 frozen assertions pass on the code retained by `origin/dev` at `97ddd2f`. The merged source tree matches the former Phase 1 working tree.
7. [x] Complete Phase 2 repository behavior and its exit criteria.
   - Evidence: `tests/integration/repository.rs` adds mutable assertions for backup-aware validated import, typed task changes that retain date-specific snapshots and pre-change backup state, and idempotent reopening of an existing SQLite database. The frozen `tests/spec/` assertions remain unchanged.
   - Verification: `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml --tests` (14 assertions), `cargo check --manifest-path src-tauri/Cargo.toml`, and `npm run build` passed on 2026-09-02. The frontend build required the normal workspace permission to let Vite resolve its configuration; it then completed normally.
   - Documentation: `README.md` and `CODEBASE.md` now describe only verified Phase 2 data flow and clearly defer the host app-data location and typed Tauri commands to Phase 3.
8. [/] Add the Phase 3 native command boundary and remove the temporary Phase 0 autostart diagnostic.
   - Acceptance: commands expose only `get_day`, `set_task_checked`, `list_calendar_days`, `export_state`, `import_state`, `get_autostart_status`, and `set_autostart_enabled`; caller-controlled SQL and filesystem paths remain impossible; integration tests and manual Windows lifecycle evidence pass.
   - Automated evidence: `application.rs` owns a mutex-protected typed repository service; `commands.rs` resolves only Tauri's local app-data directory, provides live local/UTC timestamps, and registers exactly the approved commands. `tests/integration/command_boundary.rs` covers host-owned storage, unknown-ID/future-date rejection, and typed daily/import/export operations. The frozen contracts remain unchanged; all 16 assertions pass.
   - Capability evidence: `src-tauri/capabilities/desktop.json` was removed. The sole `default.json` capability contains only `core:default`; renderer-side autostart plugin access is unavailable. The temporary `--phase0-autostart=*` diagnostic was removed from `lib.rs`.
   - Build evidence: `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml --tests`, `cargo check --manifest-path src-tauri/Cargo.toml`, `npm run build`, and `npm run tauri build` passed on 2026-09-03. The x64 MSI is 3,915,776 bytes, SHA-256 `9A15F86D5EC8D0A0DF86DED998C96F09AA660E9C7362A4F7A7DC55571002A0BF`; the NSIS installer is 2,671,038 bytes, SHA-256 `2CD63B96400457EB6CCB6D59EC26FF3F63E623CE649C7BEA5712614E2925924C`.
   - Smoke evidence: the release executable launched hidden, remained running, and created `habit-tracker.sqlite3` under Tauri's local app-data identifier directory. Only the exact smoke-test process was then stopped; autostart state was not changed.
   - Blocker: manual Windows evidence is owner-facing because the Phase 4 UI is deliberately deferred. It must demonstrate tray close/Open/Exit, command-backed autostart enable/disable, and persistence across relaunch before Phase 3 is marked complete.
   - Owner decision: interactive autostart verification is deferred to the approved Phase 4 settings UI; package, automated, tray, and local app-data smoke evidence are retained.
9. [/] Define the streak qualifying-day rule.
   - Blocker: streak calculations cannot be implemented without knowing which completion fraction qualifies.
   - Acceptance: ADR-006 is accepted and `tests/spec/streaks.rs` receives frozen assertions.
10. [/] Approve the Phase 4 one-page dashboard design before production implementation.
   - Decision: Today remains fixed at the top. The historical tracker is below it; its selected day appears beside the tracker at desktop width and below it at narrow width.
   - Evidence: `.agent-tasks/phase4-ui/render_prototype.mjs` regenerated Chromium desktop (1160px) and narrow (390px) screenshots on 2026-09-03. Its interaction assertion selects Sep 14 and verifies the historical panel reports `6 / 7`.
   - Acceptance: the owner explicitly approves this screenshot-led layout and interaction model. Then replace the prototype with production code that invokes only the existing typed command contract.
   - Implementation evidence: owner approved the rendered design on 2026-09-03. `src/habit-api.ts` contains the renderer's narrow typed adapter; `src/main.ts` renders and refreshes the fixed Today panel, 18-week tracker, and selected historical snapshot. `tests/integration/dashboard.mjs` asserts history selection plus a checkbox mutation through a mocked typed IPC boundary. `npm run build`, `npm run test:ui`, `cargo fmt --manifest-path src-tauri/Cargo.toml --check`, `cargo test --manifest-path src-tauri/Cargo.toml --tests` (16 assertions), and `cargo check --manifest-path src-tauri/Cargo.toml` passed on 2026-09-03.
   - Regression fix: the first renderer implementation sent camelCase IPC arguments despite Rust's explicit `snake_case` command contract. This rejected the initial `get_day` call and the empty-root error path hid the diagnostic. The adapter now sends `local_date` and `task_id`; startup failures render a visible error card.
