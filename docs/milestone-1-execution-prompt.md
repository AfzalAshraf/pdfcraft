# Repair Milestone 1 — Build, test and Hand-pan acceptance

Copy-ready implementation prompt. This is **repair milestone 1 from the conversation**, not the
repository's original M1 PDF-parser milestone. This document is a work specification, not a
claim that the work has been completed.

For the other six repair specifications, see [repair-milestones.md](repair-milestones.md).
For registry-independent checks and full dependency recovery, see
[offline-validation.md](offline-validation.md). Limited offline policy tests do not satisfy
normal application/UI/control build gates.

## Responsiveness and scope requirements

Read [repair-smoothness-contract.md](repair-smoothness-contract.md) before implementation. It
specifies bounded UI/background work, reference workloads, proposed latency targets, cancellation,
resource/idle checks and milestone ownership. Establish measurements first; these are requirements,
not a claim the app is already smooth. Reuse unchanged verified work and add only the smallest
required component. Record no-change audit stages without manufacturing edits or abstractions.

## Master instruction — apply to every stage

You are implementing PdfCraft's first repair milestone: establish a reproducible, normally
built and tested application, validate the existing fixes, and make the Hand tool reliable for
panning. Execute the stages below in dependency order. Do the engineering work; do not merely
write another plan or change checklist statuses.

Read `AGENTS.md`, `CLAUDE.md`, `docs/fix-status.md`, `docs/print-quality.md`,
`docs/roadmap.md` and the READMEs for each crate you touch. Read local-only `plan/` instructions
if present; record their absence if they are not present. Work only on the current session
branch, `arena/7534c8f4-pdfcraft`. Preserve the existing uncommitted changes. Never reset the
checkout, switch branches, or discard another contributor's work.

**Non-negotiable user requirement: provide a real Hand tool for pan.** When active over the
PDF canvas, show an open-hand/grab cursor. While dragging, show a closed-hand/grabbing cursor.
Dragging moves the document with the pointer horizontally and vertically where the view can
scroll. A pointing-finger/link cursor is not a substitute for the pan cursor.

**If a required engine/component is genuinely absent, implement that component from scratch in
Rust, from public specifications and documented behaviour.** First prove what is missing. A
network failure, build error, unavailable optional font input or untested platform does not
mean the engine is absent. Do not throw away an existing engine or rewrite it wholesale.

Keep user content safe: no silent edit loss, no unsafe code, no new panic paths for untrusted
input, no unbounded allocations/recursion, no disabled security checks, and no hidden network
services. Use only approved assets. Use native cursor shapes or existing approved icons; do not
copy cursor artwork from another PDF application. All new user-facing behaviour must be
reachable through the appropriate engine/automation or opt-in UI control surface and tested.

Every stage must produce:

1. The observed problem and evidence, or a documented finding that no change is necessary.
2. Focused changes, if needed, with file paths and reasons.
3. Tests and exact commands/results, separating pass, fail, skip and blocked.
4. Remaining risks and a clear exit decision. Do not call a blocked stage passed.

Do not claim every GitHub issue is resolved, every platform was tested, physical printing was
verified, or future bugs are impossible. Keep the milestone open until its acceptance gates pass.

---

## Stage 1 — Audit the checkout and establish the baseline

### Prompt

Inspect the repository before editing. Record the branch, base commit, working-tree changes,
current compiler/dependency requirements and existing CI jobs. Inventory the existing modules
for document parsing/model, rendering, editing, printing, engine/session state, automation and
UI input. Identify which of the current changes already have tests and which remain candidates.

Starting points, to be confirmed against current source:

- `crates/engine`: existing L6 `Session`/document façade and edit history.
- `crates/render`: rendering façade; currently backed by hayro/lopdf, not an absent engine.
- `crates/print` and `crates/engine/src/print_raster.rs`: print composition and image printing.
- `crates/automation`: headless tool registry.
- `crates/ui-egui/src/canvas.rs`: document viewport and pan behaviour.
- `crates/ui-egui/src/lib.rs`: `QuickTool::Hand`, H shortcut and temporary Space-hand handling.
- `crates/ui-egui/src/autoscroll.rs` and `autoscroll_gesture.rs`: middle-button gestures.
- `apps/pdfcraft/src/main.rs`: desktop startup and Windows taskbar-icon candidate.
- `.github/workflows/ci.yml`: Linux/Windows tests and WASM checks.

Run `git status --short`, `git branch --show-current`, and `git diff --check`. Inspect the diff
before changing anything. Record the distinction between existing code and code added during
this session. Do not update milestone percentages based on this inventory.

### Exit gate

A component map and baseline issue/test list exist. Existing changes are preserved. Any claim
that a component is missing cites inspected files or failed API discovery, not an assumption.

---

## Stage 2 — Make the normal build reproducible

### Prompt

Read the current `rust-version`, Cargo.lock and platform prerequisites. Install/use a compatible
Rust toolchain with Cargo, rustfmt and Clippy. Keep tool downloads, caches and binaries outside
tracked product files. Record versions and provenance. Do not lower the MSRV, remove dependencies,
disable features or weaken checks merely to make a build succeed.

Check access to crates.io and the pinned Git dependencies. Use the lockfile. Distinguish network,
authentication, missing native library, compiler-version and actual source errors. Retry only
with a reason; do not run indefinite fetch loops. Do not route traffic around the environment's
network restrictions. If normal registry access is unavailable, record the exact failure and
use an authorized CI runner with access. Isolated-source harnesses can provide supplementary
evidence, but cannot satisfy the normal-build gate.

Install the native Linux/Windows build prerequisites documented by the repository/CI. For font
coverage, test both the no-craft-fonts configuration and the pinned optional craft-fonts input
where CI requires it; do not commit font files or substitute prohibited fonts.

Use scoped builds, normally `-j 2`, with a suitable timeout. Do not run a local full-workspace
sweep unless the machine has been assessed as able to handle it; the existing CI matrix owns
that sweep. Check the desktop, CLI and web/WASM build paths rather than assuming one implies
that the others compile.

### Exit gate

Normal locked builds succeed on the required runners, or the stage remains explicitly blocked.
No local dependency-only harness is presented as the release build.

---

## Stage 3 — Reproduce and validate the current candidate fixes

### Prompt

For each candidate, reproduce the failure using a small synthetic fixture and test the behaviour
through its real entry point. Avoid testing only a helper when integration is the reported failure.

Validate at least:

1. **Word table widths:** unequal columns, merged cells at different positions and row resets;
   inspect the generated DOCX package, not just an arithmetic model.
2. **Image printing:** pages rendered at different effective resolutions retain their exact
   displayed sizes after save/reopen and imposition; invalid DPI/dimensions fail clearly.
3. **Browser printing:** run the current planner/CSS regressions: stable ordinary-page DPI at
   101/301 sheets, bounded pixel/side planning and honest oversized-page refusal. Milestone 2
   owns full browser preparation, invocation, cleanup, byte-limit and PDF-fallback acceptance.
4. **Linux middle-button scrolling:** held movement stops on release; a simple click latches;
   return-to-anchor, same-frame input, focus loss and cancellation do not leak input to tools.
5. **Windows icon:** compile the existing safe API candidate on Windows and run its available
   regression. Record shell-runtime status for milestone 7; Linux syntax is not Windows evidence.

Actually remove/revert each relevant fix temporarily, run its regression to see it fail for the
expected reason, restore the fix and see it pass. Use safe temporary copies/backups and restore
in a guaranteed cleanup path; never use a destructive checkout reset. If a test still passes
without the fix, repair the test. Mark optional external-reader checks as skipped when their
oracle is unavailable, not as cross-reader verification.

### Exit gate

Relevant regressions pass in normal scoped crate/integration runs. Record browser lifecycle,
format-reader and Windows shell-runtime handoffs for milestones 2, 6 and 7 rather than making
all future milestones prerequisites for this baseline. Missing required baseline builds/tests
still block this milestone; missing handoff evidence is not a pass for the owning milestone.

---

## Stage 4 — Repair or build genuinely missing engine capabilities

### Prompt

For each failing workflow, trace UI → control/automation → engine → lower-level implementation.
Name the missing function or broken contract. If an implementation exists, fix it in place with
a regression. Never duplicate the same operation in a UI-only shortcut to hide an engine bug.

If a necessary subsystem truly does not exist, implement the smallest complete vertical slice
from scratch:

1. Define inputs, units, page indexing, limits, output types and actionable errors.
2. Add a public-spec-based design note and identify its architectural layer.
3. Implement safe Rust core logic with finite-number checks, checked/saturating arithmetic,
   bounded work, cycle/depth protection and deterministic errors where appropriate.
4. Preserve the existing object graph and unknown PDF data; retain permission checks and
   transactional edit/undo/save behaviour. Validate saved and reopened output.
5. Connect the capability to `Session` or the appropriate façade, without UI dependencies below L7.
6. Add/extend the automation tool and its schema for document operations. For viewport pan,
   use the opt-in `ui.*` channel: pan is view state, not a PDF edit.
7. Add property/unit, crate integration and end-to-end tests with synthetic inputs.
8. Connect the real UI to that same capability and verify errors are visible and recoverable.

Do not replace the parser/renderer or introduce a new framework to make a build green. If a
scoped repair truly needs a missing low-level capability, document its contract and implement
only that slice. A finding that an entire engine is missing is an architectural scope decision:
record the blocker and obtain the required owner decision rather than expanding this build
milestone into an unplanned engine project. No blank-page placeholder or mock success is acceptable.

### Exit gate

Every capability required by this repair milestone exists and works end to end, or its missing
subtasks remain named blockers. No unrelated whole-engine rewrite has been introduced.

---

## Stage 5 — Make Hand-pointer panning correct and discoverable

### Prompt

Inspect and repair the existing Hand tool first. Keep it available through the existing visible
Hand control and **H** shortcut. **V** returns to Select. Holding **Space** temporarily activates
Hand and releasing Space restores the previous tool. These shortcuts must not steal text input
from editors, PDF form fields, dialogs, the command palette or IME composition.

Implement or repair this interaction contract:

- Hover the usable PDF canvas with Hand active: native open-hand/`Grab` cursor.
- Press the primary button and drag: native closed-hand/`Grabbing` cursor, with movement following
  the pointer 1:1 in logical coordinates on both axes where scrollable.
- Begin only on an unobstructed viewport, not a toolbar, popup, panel, scrollbar or dialog.
- Keep ownership through the release so another tool cannot finish a drawing or object edit.
- Continue tracking a legitimate captured drag outside the canvas where the window system
  supports it; cancel safely on lost capture/focus, PointerGone, modal takeover or tab closure.
- Release stops panning with no unintended continuing auto-scroll. Return to the correct cursor.
- Never select text, move an image, draw ink, crop, reorder a page, submit a form or activate a
  link as a side effect of a Hand drag. Clicking/dragging with Hand must not dirty the document.
- Preserve zoom and the chosen layout. Clamp scroll bounds; handle pages smaller than the
  viewport without jitter. Do not move application panels instead of the document.
- Use one stable logical-coordinate conversion; test 100%, 125%, 150% and 200% display scaling,
  deep zoom, rotated pages, continuous and two-up layouts, and both light/dark themes.
- Keep dedicated Hand-pan, temporary Space-pan, middle-button held panning and Linux latched
  auto-scroll as distinct gestures. Preserve the established Windows/macOS middle-button policy
  unless a separate, explicitly tested feature changes it. Do not silently implement #3774 by
  changing Hand-pan semantics.
- Respect reduced-motion settings and focus. Hand drags should not acquire unnecessary velocity
  smoothing or a speed curve; those belong to separate scrolling behaviours.
- Give the Hand control an accessible label, tooltip, focus indication and keyboard activation.
  Make view/tool state inspectable through the opt-in control channel.

Reuse the existing egui drag ownership and Hand implementation. Introduce a shared state helper
only when a reproduced ownership/duplication defect requires one; do not mandate a new state
machine for code that already meets the contract. Any helper stays separate from document
mutations and receives real UI/control events, not a second pan implementation used only by tests.

### Exit gate

The user can visibly select Hand, see the hand cursor, drag the PDF naturally and stop on release.
Neither PDF content nor edit history changes. Tests cover restoration of the prior tool after
Space, including lost-focus and modal cases.

---

## Stage 6 — Add regression and automation coverage for Hand-pan

### Prompt

Write tests for the actual viewport and control channel, not only the new state helper. Discover
the current `ui.commands`, `ui.set`, `ui.key`, `ui.move` and `ui.drag` schemas before scripting;
do not invent command IDs or payloads. Launch the UI control channel only with explicit
`--control`, keeping loopback binding, fresh token and owner-only endpoint file protections.

Required cases:

| Case | Required assertion |
|---|---|
| Hand hover | Grab/open-hand cursor over the usable canvas |
| Drag left/right/up/down | Rendered document geometry follows input; cursor is Grabbing |
| Primary release | Motion stops; next interaction works normally |
| H / V | Correct tool and cursor; no document edit |
| Hold/release Space | Temporary Hand restores previous tool |
| Space in text/form input | Space is typed normally; no pan activation |
| Link/image/annotation under drag | No navigation, selection, drawing or object movement |
| Focus/capture loss, Escape, dialog/tab change | Gesture ends safely; no stuck hand or leaked release |
| Zoom/rotation/layout/DPI variations | Direction/scale correct; no coordinate multiplication error |
| Edge or non-scrollable page | Bounds respected; no jitter, NaN or overscroll |
| Safety | Saved bytes/dirty flag/edit history unchanged by panning |
| Middle-button coexistence | Existing held-pan and click-latch tests remain valid |

Check cursor output and visible geometry in addition to internal flags. Add a visual check using
a synthetic approved PDF. Test both the native app and the web build where their interaction
paths differ. Prove regressions fail without the pan fix before accepting them.

### Exit gate

Hand-pan is reachable by a user and by the opt-in UI control channel, with meaningful integration
coverage and no document mutations. Unexecuted tests remain listed separately.

---

## Stage 7 — Run quality gates and the platform matrix

### Prompt

Run normal scoped commands first, recording exact versions, flags, exit codes and results. For
example, use the current package/test names to run:

```sh
cargo fmt --all -- --check
cargo test -p pdfcraft-export -j 2 --locked
cargo test -p pdfcraft-engine -j 2 --locked --lib print_raster
cargo test -p pdfcraft-automation -j 2 --locked doc_print_as_image
cargo test -p pdfcraft-ui-egui -j 2 --locked --lib autoscroll
cargo test -p pdfcraft-ui-egui -j 2 --locked --test view --test print --test control --test comments
cargo clippy -p pdfcraft-export -p pdfcraft-engine -p pdfcraft-ui-egui -j 2 --all-targets --locked -- -D warnings
cargo xtask layers
cargo xtask assets
cargo xtask wasm
```

A test-name filter that matches zero tests does not establish coverage. Add the actual discovered
Hand-specific test filters or run their complete scoped targets. Add any other touched crates,
including the desktop app's Windows tests. Put sensible timeouts around commands; use native
PowerShell equivalents on Windows rather than assuming Unix tools are installed.

Use existing CI for the full workspace Linux/Windows and WASM matrix. For macOS behaviour, obtain
an authorized macOS runner/manual session under the repository's existing runner policy; do not
quietly add expensive jobs. Do not treat a Linux build, cross-compile or mocked window as a
Windows/macOS runtime test. Record compiler/driver/renderer versions and test Windows startup
with both wgpu and the OpenGL fallback.

Investigate failures rather than deleting assertions, widening tolerances blindly, disabling
features or hiding warnings. Distinguish pre-existing failures from regressions with evidence.
Keep the renderer, permissions, recovery and opt-in automation security guarantees intact.

### Exit gate

Required build/test/lint/architecture/asset/WASM checks are green on their real runners. Every
required platform cell is executed or explicitly blocks milestone completion.

---

## Stage 8 — Perform user-visible acceptance and printer handoff

### Prompt

Launch the actual application and inspect it. Use synthetic documents containing text, images,
links, annotations, forms, rotated pages and multiple page sizes. Verify the full Hand-pan
contract visually, including cursor changes and tool restoration. Confirm panning is smooth
and bounded with the repository's existing large-document/performance tests; record reference
hardware and measurements instead of making an unsupported universal FPS claim.

Exercise open, search, pan, basic edit, undo/redo, save/reopen, export and print-preview smoke
tests without losing state. Establish the shared contract's reference measurements and performance
harness here; later milestones rerun affected workloads rather than inventing different budgets.

Record whether the Windows icon candidate compiles and whether existing runtime evidence is
available; final shell-surface acceptance belongs to milestone 7. Hand off Kyocera/Canon symptoms,
known settings and unverified paths to milestone 2. Do not send physical printer jobs in this
build-baseline stage or make physical output a prerequisite for establishing the normal build.
A successful preview/build does not verify either printer.

### Exit gate

Real-window Hand-pan and milestone-1 smoke acceptance pass on the required platforms. Printer
results are recorded honestly, with unresolved milestone-2 work separated from build readiness.

---

## Stage 9 — Review, document and make the completion decision

### Prompt

Review every diff for input bounds, panic paths, thread behaviour, architectural layering,
permissions, save fidelity, undo semantics, WASM compatibility and automation coverage. Confirm
that the newly added tests exercise production code. Check `git diff --check` again and ensure
no caches, binaries, large fixtures, fonts, credentials or prohibited assets entered the patch.

Update `docs/fix-status.md`, the affected rows in `docs/roadmap.md`, and the root ROADMAP progress
log. Update parity notes/evidence without inflating statuses. Keep historical evidence distinct
from the current verification result. Follow the repository's commit/PR gates, and do not push,
merge, close issues or mark unrelated reports resolved merely to make the backlog look smaller.

Deliver a final report with:

- Implemented changes and files.
- Existing engine capabilities repaired versus genuinely missing capabilities built from scratch.
- Hand-pan acceptance results, cursor behaviour and headless/control coverage.
- Test commands and counts, with skipped oracle checks and zero-match filters called out.
- Platform matrix: Linux, Windows, macOS and web/WASM; build versus runtime distinguished.
- Printer evidence and the remaining milestone-2 defects.
- Every remaining blocker and the exact condition needed to remove it.
- A single truthful decision: **complete**, **partially verified**, or **blocked**.

### Final definition of done

Repair milestone 1 is complete only when the normal locked build and required quality gates
pass, current changes have meaningful integration coverage, and the real Hand-pan experience
passes its required native/web acceptance checks. Windows hardware-dependent claims require
Windows execution. No missing engine component required by this milestone is a placeholder.

Passing this milestone does **not** complete all M0–M14 roadmap work, fix every GitHub report,
certify every printer, or guarantee that future defects cannot occur.
