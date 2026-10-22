# Repair Milestone 4 — Scrolling, navigation and link interaction

Status: **execution specification, not implementation or verification evidence**.

## Scope

#3817 touchscreen scrolling, #3795 held-middle release, #3793 scroll speed,
#3774 Windows middle-click autoscroll, #3792 two-page spread and #3783 links.
Preserve and verify the user's Hand-pointer pan requirement throughout.

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

## Stage 1 — Inventory input ownership and reproduce devices

### Execution prompt

Trace raw pointer/wheel/touch events, smoothing, page navigation, ScrollArea offsets, Hand panning,
middle-button state and modal/tab ownership. Reproduce with synthetic multi-page PDFs, short/long
pages, landscape pages, spreads and links. Record device, OS, scaling and continuous/single-page mode.
Separate mouse wheel lines, precise touchpad points, touch drags and pen events. A synthetic event
pass does not replace touchscreen or Windows mouse hardware acceptance.

### Exit gate

Each report has an event trace or a precise device/reproduction blocker, with no double handling of the same movement.

## Stage 2 — Complete the Hand-pan contract

### Execution prompt

Reuse milestone-1 Hand implementation, tests and unchanged acceptance evidence; do not rebuild
its state machine here. Extend regressions only for interaction defects this milestone exposes.
Verify open-hand Grab hover and closed-hand Grabbing primary drag over text, links and empty canvas. Keep same-frame one-to-one horizontal/vertical motion, bounds and no post-release fling.
Test H, V, temporary Space and restoration to the previous tool. Suppress shortcuts while typing.
Cancel gestures on release, pointer/focus loss, modal opening and tab switch; never resume stale drags.
Reject NaN/infinite coordinates and avoid activating links, selecting text or editing during pan.
Exercise the existing branch candidates before replacing any part.

### Exit gate

Real UI/control tests and an actual-window inspection confirm both cursor states and panning without document mutation.

## Stage 3 — Repair held versus latched middle-button policies

### Execution prompt

Validate the current #3795 candidate: Linux click/release latches; a held excursion beyond the
dead zone stops on release, including a return to the anchor or movement delivered in the release
frame. A second click, Escape, wheel, other buttons and lost focus cancel safely. Reproduce #3774
before defining Windows latch semantics; do not assume Linux behaviour is the required Windows
behaviour. Implement an explicit tested per-platform policy where required. Never leak middle-button
events into drawing, crop, selection or object editing, including the Organize grid.

### Exit gate

Expected held/click policy is documented and verified on its target OS; unchanged platforms retain tested behaviour.

## Stage 4 — Fix touch and scrolling performance

### Execution prompt

Use real touch device traces for #3817. Prevent single-touch pan from becoming text/object editing
or double-applying ScrollArea motion. Keep two-finger zoom anchored correctly where supported. For
#3793, measure frame time, input-to-motion latency, render demand and CPU rather than just increasing
wheel multipliers. Bound acceleration, preserve fractional deltas, and use elapsed time consistently
at 30/60/120/144 Hz in deterministic event tests. Measure actual UI latency on the agreed reference
machine rather than requiring every display/device combination. Coalesce render demand, prioritize
visible/current pages and use bounded prefetch with revision-aware cancellation. Reuse a clearly
interim cached preview during motion, then request final fidelity after settling; no persistent blur,
missing pages or stale tiles. Stop unnecessary work/redraw at rest. Do not remove memory limits.

### Exit gate

Correct input scale and responsive measured motion are demonstrated without extra easing, duplicated events or memory growth.

## Stage 5 — Correct spreads and page transitions

### Execution prompt

Reproduce #3792 with cover on/off, odd/even page counts, RTL preference if supported, mixed widths,
continuous and single-row modes. Trace current-page selection, visible row, next/previous, wheel-edge
paging, page labels and destinations. Keep the chosen PDF layout separate from temporary view state.
Test beginning/end boundaries, single remaining pages, rotation, zoom anchors and tab restoration.
Prevent blank intermediate frames and unnecessary horizontal jumps.

### Exit gate

Spread membership, current page and transitions match the accepted reproduction with no off-by-one page loss.

## Stage 6 — Repair links without weakening security

### Execution prompt

For #3783, distinguish hit-testing/transforms, overlays consuming clicks, malformed targets and
permission prompts. Test local destinations, named destinations, URI, remote GoTo and layer actions.
Keep the existing safe URI/remote-file confirmation and automation confinement. Select may show a
pointing finger and follow a permitted link; Hand must keep Grab and never follow the link while
panning. Preserve right-click context menus and reject unsupported actions honestly.

### Exit gate

Valid links activate from the correct tool/geometry; unsafe actions remain blocked or confirmed, and Hand never activates them.

## Stage 7 — Run the complete input interaction matrix

### Execution prompt

Drive ui.set, ui.move, ui.drag, ui.key and registry commands through the opt-in control channel.
Test dialogs, printer preferences, tab switches, text/form inputs, Organize Pages, background windows,
resizes and display scaling. Inspect actual cursor output, viewport offsets and document bytes.
Run real touchscreen/Windows mouse acceptance for their reports and retain explicit blockers when
those devices are unavailable.

### Exit gate

Engine-independent state tests, full UI/control tests and the required device acceptance all pass before closing individual reports.

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
