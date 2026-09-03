# Architecture Decisions

## Accepted

### ADR-001: Tauri 2 as the desktop shell

**Status:** Accepted, 2026-09-02

**Context:** The MVP needs a Windows notification-area icon, hide-on-close behavior, an explicit tray Exit command, a familiar UI implementation path, and a local persistence boundary.

**Decision:** Use Tauri 2 with a TypeScript renderer and minimal Rust host integration.

**Consequences:** Its documented tray and autostart integrations reduce custom Windows startup work. SQLite access remains behind a `rusqlite` Rust repository rather than renderer-exposed SQL commands. It introduces Rust tooling and a smaller ecosystem than Electron. A technical spike must validate packaging, tray lifecycle, and opt-in autostart before execution proceeds.

### ADR-002: User-scoped local persistence

**Status:** Accepted, 2026-09-02

**Decision:** Persist daily completion state in a SQLite database under the current user's application-data directory; do not transmit data. Each toggle writes atomically through one repository boundary.

**Consequences:** Past and current days are editable; future dates are rejected. The live database is accompanied by exactly two rotating previous-state JSON backups. JSON import/export is in scope. Data is local to the Windows user and plaintext at rest.

### ADR-003: Explicit startup consent

**Status:** Accepted, 2026-09-02

**Decision:** The installer/setup flow asks whether to enable startup by default. Startup is controlled thereafter by a visible Settings toggle via Tauri's autostart integration. It is removed on uninstall/when toggled off.

**Reason:** Background startup should be visible and reversible, rather than silently enabled. Scheduled Task registration remains a fallback only if the integration spike fails.

### ADR-004: Deterministic daily applicability

**Status:** Accepted, 2026-09-02

**Decision:** For a given record's local calendar date, exercise is applicable only on Monday, Wednesday, and Friday. The completion denominator is seven on those days and six otherwise. The calendar color is `rgb(0, round(255 * completed / applicable), 0)`. Persist the record's local ISO date and applicable-task snapshot so historic visualization never depends on the current date. New-day determination uses the current Windows local time zone.

### ADR-005: JSON recovery and transfer

**Status:** Accepted, 2026-09-02

**Decision:** Retain exactly two rotating previous full-state JSON backups. Offer user-initiated JSON export and validated JSON import. A failed import changes nothing; a successful import first creates a recoverable pre-import state.

**Consequences:** This gives simple local recovery and portability without cloud infrastructure. It does not protect against disk loss beyond the current device or provide encryption at rest.

### ADR-006: Derived streak metrics

**Status:** Proposed

**Context:** The product needs current and longest streak metrics, but the qualifying completion rule is not yet specified.

**Decision:** Derive streaks from daily historical records at query time; never persist mutable streak counters. A qualifying-day threshold must be accepted before implementation.

**Consequences:** Past edits and imports automatically recalculate both metrics correctly. Queries are tiny at the expected personal scale; no cache is needed.
