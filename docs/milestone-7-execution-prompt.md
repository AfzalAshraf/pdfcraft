# Repair Milestone 7 — Desktop integration, themes, detached tabs and Greek OCR

Status: **execution specification, not implementation or verification evidence**.

## Scope

#3818 Windows taskbar/Alt+Tab icon, #3807 detached tabs/windows, #3802 theme customization
and #3805 Greek OCR. These are separate subfeatures with separate platform and asset requirements;
one successful icon test does not complete the milestone.

Deliver independent slices: **7A icon**, **7B detached windows**, **7C themes**, **7D Greek OCR**.
Validate the existing icon candidate first. Build the larger feature slices after core build,
printing/data safety and input responsiveness are stable; no single giant desktop rewrite. Keep
all requested slices tracked, but a missing OCR model must not block an unrelated verified icon
fix. Milestone 7 as a whole remains incomplete until its scoped slices pass or exclusions are agreed.

## Responsiveness and scope requirements

Read [repair-smoothness-contract.md](repair-smoothness-contract.md) before implementation. It
specifies bounded UI/background work, reference workloads, proposed latency targets, cancellation,
resource/idle checks and milestone ownership. Establish measurements first; these are requirements,
not a claim the app is already smooth. Reuse unchanged verified work and add only the smallest
required component. Record no-change audit stages without manufacturing edits or abstractions.

## Master execution instruction

Implement this repair milestone; do not substitute another plan for engineering. Read `AGENTS.md`,
`CLAUDE.md`, `docs/fix-status.md`, the relevant crate READMEs, `docs/gaps.md` and
`docs/target-app-parity.md`. Read local `plan/` instructions if present and record absence otherwise.
Work only on `arena/7534c8f4-pdfcraft`. Preserve the existing uncommitted work and source documents.
These are the conversation's seven **repair milestones**, not the original M0–M14 roadmap.

An existing engine is present. Audit it before replacing anything. If a required capability is
truly absent, implement only that bounded component in safe Rust from public specifications.
Document operations use engine → automation → UI; view/window operations use the opt-in UI control
surface, not fake PDF edits or unnecessary new engine APIs. No placeholders. Network failure,
absent hardware or a failing test does not mean the engine is missing. Never rewrite every file just to hide a build
failure, remove dependencies without equivalent functionality, disable tests, fake package checksums,
disable TLS/permissions or substitute a reduced demo for the existing application.

Preserve unknown PDF data, incremental-save semantics, encryption permissions, undo/redo, atomic
writes and automation root confinement. No new unsafe code, unbounded input-dependent allocations,
unchecked indexing, panic paths, hidden listeners or default-on MCP/control services. Use only
approved assets/fonts under the repository's attribution and clean-room policies. No prohibited
fonts, copied proprietary graphics, GPL/AGPL implementation code or committed private fixtures.

Preserve the user's **Hand pointer for pan**: Grab while ready, Grabbing during primary-button drag,
horizontal/vertical panning, H/V and temporary Space, no link activation or PDF mutation, safe
cancellation on modal/focus/tab boundaries. New tools must not steal form/editor typing.

For each stage, record findings, necessary changes, exact checks and pass/fail/skip/blocked.
For each bug fix, demonstrate red/green regression evidence; no-change audits need no invented fix.
For a new capability, test its explicit contract, failure modes and real entry point. Source-level
policy tests never replace required UI, native-platform, independent-reader or printer acceptance.

## Stage 1 — Reproduce desktop and OCR requirements

### Execution prompt

Read each issue and confirm expected behaviour, affected OS, app version and exact reproduction.
Inventory existing Windows icon code, multi-window/session ownership, theme persistence and OCR
interfaces/models. Distinguish detaching a document into another window from a second independent
app instance. Determine Greek language/model coverage and whether recognition output is extraction,
search or editable text; do not infer feature scope from an issue title alone.

### Exit gate

Each report has a written acceptance contract, existing-component inventory and required platform/model inputs.

## Stage 2 — Finish Windows executable/window/taskbar identity

### Execution prompt

Validate the existing ICON_BIG candidate rather than duplicating it. Check executable resources,
launcher/portable startup, regular title bar, taskbar, Alt+Tab and window recreation with wgpu and
OpenGL where supported. Use the same approved bundled icon via maintained safe APIs. Preserve Linux,
macOS and web builds. A PNG decode unit test or Linux compile is not Windows shell verification.
Add target-specific regressions and record Windows scale/backend/version evidence.

### Exit gate

The intended approved icon appears in affected Windows shell surfaces in an actual running build; other targets do not regress.

## Stage 3 — Implement safe detached document windows

### Execution prompt

Design ownership using existing engine/session and frontend patterns. A document must not acquire
two divergent edit histories when detached, lose dirty drafts, duplicate autosave or silently change
its file-watch identity. Specify per-window active document, modal focus, selection, render demand,
control routing and close/quit semantics. Transfer/move views atomically or reject the operation
without loss. Keep one opt-in authenticated control model; do not open unauthenticated listeners or
spawn hidden services. Reuse the engine and render pool, prioritize the active visible window and
avoid duplicating entire document bytes/caches for every view. Bound combined multi-window resources
and test closing a window during queued work. Implement missing ownership only as a bounded slice.

### Exit gate

Detach/reattach, edits, undo, save and close behave consistently across windows without lost work or duplicated ownership.

## Stage 4 — Expose detachment through UI and opt-in control

### Execution prompt

Add accessible tab/context/window controls, precise drop targets if required and a discoverable
control command. Test initial/last/dirty tabs, window focus, OS close, cancelled save, display scale,
monitor removal and restored layouts. Never route a destructive action through stale tab indices.
Keep engine document identity stable and cancel Hand/object gestures across window transfers.
Do not claim physical drag support solely from a programmatic detach command.

### Exit gate

Real-window and control-channel tests demonstrate safe transfers and focus/modal handling on affected desktop platforms.

## Stage 5 — Implement theme customization without changing PDF content

### Execution prompt

Define supported theme tokens, validated colour input, persistence/version migration and reset to
defaults. Maintain readable contrast, keyboard focus, disabled/error states and light/dark/system
behaviour. Bound imported settings and handle corrupt files atomically. Apply only interface tokens:
PDF content, image exports and printing remain unchanged by theme selection. A separate documented
reading mode may transform on-screen page colours only; it must not leak into saved/exported/printed
output. Add headless settings/control parity and ensure
Hand/link cursors remain visible and semantically correct across themes. Apply token changes only
when settings/system theme changes, not by rebuilding fonts, thumbnails or the document each frame.
Do not add a bespoke theme-file format/importer unless the accepted report needs it; use existing
validated settings first and test long localized labels, reduced motion and keyboard-only use.

### Exit gate

Themes survive restart, invalid settings fail safely, interface contrast is acceptable and document/export/print colours are unchanged.

## Stage 6 — Implement approved Greek OCR end to end

### Execution prompt

Inspect the actual Rust OCR engine and supported languages. Add missing Greek recognition through
an approved, licence-proven model/data path with immutable version/hash and required attribution.
No hidden downloads or prohibited font/model assets. If the engine capability is absent, implement
or integrate a safe Rust specification-compliant slice with bounded image sizes, cancellation and
clear unsupported-language errors; never relabel Latin OCR as Greek support. Test Greek accents,
final sigma, mixed Greek/Latin and rotated/scanned pages using original approved fixtures. Keep
recognition confidence and unknown characters honest; preserve the source image and permissions.
Run recognition on bounded background work, page by page, with progress/cancellation and explicit
result ownership. Keep model initialization cached within declared memory limits instead of loading
it per frame/page. Define an agreed character/word-error metric including Greek diacritics before
evaluation; retain actual recognized text and report failures, not a post-normalized illusion of
accuracy. No model-training project or new automatic downloader is implied by this repair slice.

### Exit gate

Greek text is recognized with measured agreed accuracy and exposed through engine/automation/UI, or the missing approved model remains a blocker.

## Stage 7 — Validate persistence, packaging and regressions

### Execution prompt

Test profile migration, clean installs, portable settings, window restore, optional model absence,
failed OCR, cancellation and interrupted saves. Native window/icon changes need native Windows
coverage; detachment needs actual multiple windows, not only mocked IDs. OCR needs the declared
model/version and accuracy corpus. Run Hand-pan, Print lifecycle, opt-in control security and atomic
settings regressions along with normal build/lint/assets/WASM gates.

### Exit gate

Each independent desktop/theme/OCR report passes its own required runtime and safety checks; unsupported platform/model cases stay explicitly open.

## Quality and delivery gates

Run normal locked scoped tests for all touched crates with `-j 2` and bounded execution times.
Test touched crates and their affected callers/integration/control targets; do not rebuild every
unrelated crate during each edit. List actual targets and nonzero counts rather than guessing filters.
Run formatting, scoped all-target Clippy with `-D warnings`, layers, assets, parity and WASM gates.
Use repository CI for the full workspace and appropriate native platform coverage. On a restricted
machine, follow `docs/offline-validation.md`; report limited offline checks honestly and obtain a
checksum-verified source bundle or use a registry-enabled runner for the full application.

Exercise real windows through the opt-in control channel; inspect and capture only synthetic,
approved content. Compare save/reopen output with independent readers where relevant. Verify undo,
redo, encrypted permissions, failed operations, interrupted work and confinement escapes. Missing
fonts, oracle tools, target devices or OS runners are blocked/skipped checks, not passes.

Review each update at clause/function/module/project scope. Preserve unrelated edits and inspect
the complete diff. Do not commit binaries, downloaded packages, large fixtures, private captures or
font assets. Update the issue-by-issue ledger, affected READMEs/parity notes, the relevant original
roadmap row and progress log. Change no readiness percentage without measurement. Follow the
repository's green-commit and review requirements; do not close issues or merge without verified
acceptance and authorization.

## Final report and definition of done

Provide an issue-by-issue table with: reproduced behaviour; cause; changed paths; engine/tool/UI
tests; saved-output/oracle evidence; platform/runtime coverage; and unresolved limitations. State
**complete**, **partially verified** or **blocked** for the milestone. A feature's presence is not
evidence that its reported defect is fixed. Any exclusion needs an explicit reason and agreement,
not a silent scope cut. Required gates must pass before declaring completion. Do not claim every
backlog issue is fixed, or promise that no future bugs can occur.
