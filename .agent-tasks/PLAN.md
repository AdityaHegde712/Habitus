# Habit Tracker - Proposed Backend-First Plan

## Status

Architecture approved on 2026-09-02. Phases 0 through 2 are complete; Phase 3 native integration is active. UI work remains deferred until the dedicated Phase 4 decision pass.

## Proposed interaction flow

1. A Windows logon trigger starts the packaged app for the current user.
2. The app opens its dashboard once, then remains resident in the notification area after the window close button is used.
3. Selecting a task writes that calendar day's state locally and recalculates its completion fraction from that date's applicable task set.
4. Calendar hover shows the completed-task count; selection shows that day's completed and incomplete task labels.
5. Only the tray context-menu Exit command performs a deliberate application shutdown.

## Approved implementation path

- Tauri 2 desktop shell with a dark HTML/CSS/TypeScript renderer and a deliberately small Rust host layer.
- Tauri's native tray API, with a tray menu containing Open and Exit. Closing the window hides it; only Exit terminates the process.
- SQLite database in the operating system's per-user application-data location, accessed only through a typed Rust repository boundary implemented with `rusqlite`.
- Tauri's opt-in autostart integration, surfaced as a visible toggle rather than a silently-created scheduled task.
- A small, well-tested domain module shared by persistence and calendar UI: it determines applicability from the record's local date and calculates the RGB green from completed divided by applicable tasks.

## Rejected foundation

Electron remains a viable 4/5 fallback but is rejected for v1. Its broader JavaScript/Node runtime would require a hardened preload/IPC boundary and produces a heavier package. Tauri is the approved 4.5/5 foundation for this Windows-only, local-first scope.

## Approved data and command contract

### Stable task identifiers

`meals`, `sleep_7h`, `exercise`, `job_application`, `vitamins`, `leetcode_or_dsa`, and `surfaces_clean` are stable IDs. Their displayed text may change later without changing stored history.

### Daily record

Each record contains `local_date` (`YYYY-MM-DD`), `applicable_task_ids`, `checked_task_ids`, `applicable_count`, `completed_count`, `policy_version`, and `updated_at_utc`. The date is derived from the current Windows local civil date for new activity. A record is a historical snapshot: later policy changes must not alter its denominator or color.

### Mutations

1. The renderer invokes only typed Tauri commands: `get_day`, `set_task_checked`, `list_calendar_days`, `export_state`, `import_state`, `get_autostart_status`, and `set_autostart_enabled`.
2. The Rust command layer rejects unknown task IDs, malformed dates, duplicate IDs, inconsistent totals, and dates after the current Windows local date.
3. `set_task_checked` permits past and current dates, recomputes totals from the stored applicable set, serializes the complete pre-change state to a staging JSON file, then commits the record in one SQLite transaction.
4. After that commit, it atomically promotes the staged pre-change state to `previous-1.json` and rotates the prior `previous-1.json` to `previous-2.json`. A mutation reports success only after this promotion. Startup detects and repairs an interrupted rotation from the staging file; a partial file is never treated as valid.
5. `import_state` validates the entire JSON schema and every record before it uses the same pre-change staging and recovery protocol to replace the live state in one transaction. An invalid import leaves the live state unchanged. `export_state` serializes the current validated state.

### Security and lifecycle contract

- The bundled renderer is the only content loaded. No remote WebView content, arbitrary shell commands, generic command invocation, or direct database access is allowed.
- Tauri capabilities use the narrowest command and plugin permissions. The autostart toggle is off until the installer/setup flow explicitly asks the user; it is user-scoped, non-elevated, status-visible, and removed on uninstall or disable.
- Normal window close hides the main window after state writes have already completed. The tray Exit action sets the quitting state and then terminates; it is the only user-facing quit route.

## Session continuity, documentation, and Git contract

1. UI/UX is not designed or implemented during backend phases. When Phase 4 is reached, the owner returns to this planning session for a dedicated UI/UX decision pass before UI implementation begins.
2. `README.md` and `CODEBASE.md` are feature deliverables. Every feature reviews both documents, updates affected public usage, setup, architecture, data-flow, test, and known-constraint sections, and verifies referenced paths and commands before its commit.
3. The canonical remote is `https://github.com/AdityaHegde712/Ephemera.git`. At repository initialization, confirm this URL and the actual remote branch layout before changing Git state; do not assume that the remote or `dev` branch already exists.
4. For the remaining single-developer build, work directly on `dev`: fetch and prune after owner-performed remote merges, make concise conventional commits after verified milestones, and push `dev`. The owner remains responsible for merges from `dev` to `main`. Resume dedicated feature branches only if the owner explicitly requests them.
5. Feature completion requires separate automated-test, applicable manual Windows/UI, and documentation-plus-Git-state evidence. Passing tests alone does not make a feature release-ready.

## vNext proposed addition: streak tracking

Expose a current streak and all-time longest streak derived from historical daily records; do not persist duplicate streak counters. The qualifying-day rule is pending owner approval. Once defined, the Rust backend computes contiguous local dates, with absent dates and non-qualifying records breaking a streak. The UI presentation remains deferred to the dedicated Phase 4 planning pass.

## Test-first implementation plan

### Phase 0: Toolchain and risk spike (~30 minutes)

Create the empty Tauri/TypeScript project, lock package versions, and verify the Windows build prerequisites. Build a throwaway tray/autostart proof that is discarded or folded into production only after it proves: close hides, tray Open restores, tray Exit exits, startup can be enabled/disabled without elevation, and app data resolves under the per-user path.

**Exit criteria:** the five behaviors are manually demonstrated on Windows and the documented commands remain usable after a packaged install.

**Git/documentation gate:** Initialize the local repository only after inspecting the canonical remote. Before the Phase 0 commit, create or update `README.md` and `CODEBASE.md` from verified scaffold facts.

### Phase 1: Frozen domain contracts - Red (~45 minutes)

Create only immutable `tests/spec/` assertions before implementation:

1. `tests/spec/day_policy.rs`: seven applicable IDs on Monday, Wednesday, Friday; six on all other weekdays; exercise absent on non-exercise days; colors for zero, partial, and complete days use the approved rounding formula.
2. `tests/spec/history.rs`: a Tuesday record retains six tasks when viewed on a later Friday; a past edit changes only that record; future dates are rejected.
3. `tests/spec/backup_contract.rs`: each successful state mutation retains the immediately preceding two full JSON states in order; a third mutation evicts only the oldest backup; interrupted rotation recovers from its staged file.
4. `tests/spec/import_export_contract.rs`: export/import round trips a valid multi-day state; invalid schema, unknown IDs, future records, and inconsistent counts fail without altering live state.
5. `tests/spec/streaks.rs` (after the qualifying-day rule is approved): current streak ends on the current local date only when it qualifies; longest streak is correct across a gap, a partial day, a Monday/Wednesday/Friday denominator, and an imported historic record.

Run them and record their expected initial failure. These assertions become frozen; changing them later requires explicit owner approval.

**Exit criteria:** all contract tests fail solely because the production modules do not yet exist or are unimplemented.

### Phase 2: Domain and persistence - Green (~1 afternoon)

Implement the smallest Rust domain/repository modules needed to pass the frozen contract tests: `task_policy`, `date_validation`, `repository`, `backup_recovery`, and `transfer`. Configure Cargo test targets to point at the root-level `tests/spec/` directory, then test repository behavior with isolated temporary per-test database directories.

**Exit criteria:** contract suite passes; schema migration from an empty database is idempotent; no renderer-facing command accepts arbitrary SQL or filesystem paths.

**Git/documentation gate:** Update `README.md` with backend developer commands and backup/import behavior; update `CODEBASE.md` with verified module and data-flow paths.

### Phase 3: Native integration - Green (~1 afternoon)

Add the narrow Tauri commands, capability configuration, tray lifecycle, and autostart toggle. Root-level `tests/integration/` targets cover command validation and persistence through the command boundary; a manual Windows check proves close-to-tray, explicit Exit, startup opt-in, installer enable/disable, and persistence across relaunch.

**Exit criteria:** automated integration tests pass and a manual evidence checklist is complete. UI layout is deliberately deferred.

**Git/documentation gate:** Document the tray/autostart lifecycle and capability boundary from implemented code, then verify documentation links before the feature commit.

### Phase 4: Frontend and calendar - design approval pending

The Phase 4 design pass is complete enough for owner approval. The proposed one-page dashboard keeps today's editable checklist fixed at the top. Below it, a Git-style completion tracker displays historical days, and the selected historical checklist is beside the tracker on desktop and stacks below it on narrow viewports. The UI may consume only the approved typed command contract; it cannot recalculate historical denominators itself. The isolated visual prototype and its reproducible Playwright render script live in `.agent-tasks/phase4-ui/`; they are design evidence, not production renderer code.

**Design evidence:** Chromium screenshots at 1160px and 390px were regenerated on 2026-09-03 after a Playwright interaction assertion selected Sep 14 and verified its `6 / 7` historical detail state. Both widths have no horizontal overflow. Owner approval remains required before production UI implementation.

**Exit criteria:** screenshot-led visual evidence at desktop and narrow widths, including hover task count and selected-day detail.

**Git/documentation gate:** Update user-facing usage and calendar/streak behavior in `README.md`, and UI-to-command/data-flow mappings in `CODEBASE.md`.

### Phase 5: Packaging and recovery - deferred

Package the Windows installer, test uninstall cleanup and backup/restore on a clean user profile, and record version, checksum, and installer size.

**Exit criteria:** install, opt-in startup, relaunch, export, invalid-import rollback, valid-import restore, disable startup, and uninstall all have separate evidence.

**Git/documentation gate:** Document verified installer, recovery, and removal behavior; verify the release feature branch is current with the owner-selected `dev` base before handoff. The owner performs any merge.

## Phases and exit criteria

1. Toolchain risk spike: prove packaged tray persistence, explicit Exit, and opt-in user-scoped startup. Exit: manual Windows evidence recorded.
2. TDD domain and storage: write immutable behavioral tests before domain/persistence implementation. Exit: tests cover all weekday, backup, and saved-state cases.
3. Native integration: expose only typed commands and verified lifecycle behavior. Exit: automated integration plus manual Windows evidence.
4. UI and visualization: implement the dashboard and calendar against the approved domain contract. Exit: automated plus screenshot-led visual evidence.
5. Packaging: install/uninstall, backup/recovery, and startup verification. Exit: clean-machine/user-profile evidence and release metadata.

## Strict non-goals until separately approved

- Cloud sync, accounts, notifications/reminders, mobile clients, habit customization, multi-user sharing, encryption at rest, and automatic cloud backup.

## Sources consulted

- Electron Tray documentation: https://www.electronjs.org/docs/latest/tutorial/tray
- Electron application lifecycle documentation: https://www.electronjs.org/docs/latest/api/app
- Microsoft schtasks documentation: https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/schtasks-create
- Tauri system tray documentation: https://v2.tauri.app/learn/system-tray/
- Tauri autostart documentation: https://v2.tauri.app/plugin/autostart/
- Tauri SQL documentation: https://tauri.app/reference/javascript/sql/
