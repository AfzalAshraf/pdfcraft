# Repair Milestone 5 — Page, image and object editing

Status: **execution specification, not implementation or verification evidence**.

## Scope

#3813 dropping an image into the current PDF, #3809 edit-mode context menu/lag,
#3808 page thumbnail, #3798 object z-order and #3780 deleting objects.
Do not equate an existing selection or delete button with a verified fix.

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

## Stage 1 — Reproduce and inventory object identity

### Execution prompt

Generate original PDFs with text, images, vector shapes, annotations, repeated XObjects, clipped
content, rotated/cropped pages and overlapping objects. Reproduce each report and identify whether
its object is page content, a form widget, annotation or derived thumbnail. Trace stable identity,
hit-testing, edit selection, engine mutation and render invalidation. Keep shared resources distinct
from individual drawing instances.

### Exit gate

Each report targets the correct PDF entity and has a bounded synthetic reproduction or an explicit missing-input blocker.

## Stage 2 — Implement safe image drop into the active document

### Execution prompt

For #3813 distinguish dropping onto a tab/window (open file) from dropping onto a page (insert image).
Capture the intended document/page and transformed drop position, not a later active tab. Decode
supported images with pixel/byte limits, validate dimensions and provenance of committed fixtures,
respect rotation/crop and preserve aspect ratio. Unsupported files or permissions must fail before
mutation. Use the existing undoable engine image operation; implement a missing slice from scratch
only if absent. Decode/downsample previews off the frame path with byte/pixel admission before
allocation; retain full-fidelity content for insertion. Bind completion to the original document/
page/revision and reject or explicitly reconfirm a changed target. A closed target must not receive
a late insertion. Add a headless tool and atomic batch semantics if multiple drops are accepted.

### Exit gate

An approved image lands on the intended page at the intended position, survives save/reopen and undoes as one intended operation.

## Stage 3 — Repair context menus and edit-mode latency

### Execution prompt

Reproduce #3809 while selection, drag, text editing and a context menu compete. Define which entity
owns the right click and ensure the menu reflects that entity rather than stale selection. Instrument
hit-testing, preview regeneration and resource scans. Cache only immutable/revision-keyed results,
cap work and invalidate after real edits; do not run expensive scans every UI frame. Keep the UI
responsive without dropping edits or recycling stale object IDs.

### Exit gate

Correct menus and measured latency improvements are demonstrated on the reproduction, with no unsafe worker/UI ownership changes.

## Stage 4 — Fix thumbnails and revision invalidation

### Execution prompt

For #3808 identify whether the symptom is missing, stale, incorrectly rotated or misordered output.
Key render demand to the document/page revision and required texture extent. Cancel/discard stale
results after page edits, deletion, undo, tab close or reordering. Test shared page resources, mixed
sizes and rapid selection changes. A thumbnail is a preview, never the saved or printed page payload.
Keep texture/memory admission bounded. Schedule visible thumbnails first and defer far-offscreen
work; do not regenerate every thumbnail after a local edit. Rapid scrolling must coalesce requests
rather than build a backlog. Measure upload/frame cost and live texture counts after repeated
edit/undo/reorder/close cycles. No stale result may reattach resources to a closed/replaced document.

### Exit gate

Thumbnails track the correct revised page after edits/undo/save without stale uploads, blank persistent tiles or excess memory.

## Stage 5 — Implement correct instance-local z-order

### Execution prompt

For #3798 determine accepted bring-forward/back/front/back semantics for page content versus
annotations. Preserve graphics-state boundaries, clips, transparency groups and marked content.
Reordering one XObject invocation must not reorder all instances or corrupt q/Q balancing. Reject
unsupported combinations before mutation rather than rewriting the whole page from visual guesses.
Expose supported operations through Edit/Session/automation and real selection/context controls.

### Exit gate

Only selected intended instances change paint order; saved output and unrelated content remain correct, including undo/redo.

## Stage 6 — Repair deletion without collateral data loss

### Execution prompt

For #3780 identify what is actually selected and which permission controls deletion. Remove only
the intended object/instance; preserve other users of shared fonts/images/XObjects. Update annotation,
form and resource references using existing safe APIs. Bound graph traversal and handle malformed or
cyclic resources. Keep at least the repository-required valid page structure. Test empty/no selection,
protected objects, repeated Delete, multi-selection and failures between validation and application.

### Exit gate

Delete changes exactly the allowed selection, fails atomically otherwise and restores all content/resources on undo.

## Stage 7 — Verify save, automation and viewport safety

### Execution prompt

Exercise each operation via engine and automation with valid/invalid page/object IDs, confined file
paths and malicious sizes. Save/reopen with an independent reader, compare rendered output and
inspect remaining resources. Test rapid edits, tab changes and failure/cancellation with dirty work.
Use actual UI/control events for image drop where the control surface supports file injection;
otherwise add a bounded opt-in control operation rather than silently claiming drag-and-drop parity.
Hand must pan instead of moving/deleting objects and must retain its cursor over editable content.

### Exit gate

All scoped operations have saved-output and real interaction evidence; no tool-only or preview-only implementation is marked complete.

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
