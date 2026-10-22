# Repair Milestone 2 — Reliable, clear and correctly scaled printing

Copy-ready implementation prompt. This is **repair milestone 2 from the conversation**, not the
repository's original M2 rendering milestone. Writing this specification does not complete the
milestone or imply that repair milestone 1 has passed.

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

Implement a reliable printing workflow from document state to print-ready PDF, printer discovery,
driver settings, queue submission and physical output. Prioritize Windows desktop and the user's
**Kyocera TASKalfa 5054ci KX**, then the Canon PrintTicket and printer-discovery reports.

Scope:

- Kyocera faded/pale or unclear output reported in the conversation; the printer showed
  **Toner Low**. Determine the cause rather than assuming either software or toner is responsible.
- Canon imagePROGRAF PRO-4000S ticket merge failure: **#3823**.
- Printer loading/discovery: **#3810**.
- General printing failure: **#3800**; obtain its actual reproduction before grouping it with Canon.
- Smart-board paper size request: **#3811**; do not invent dimensions from its title.
- Existing image-print geometry and browser-print quality candidates, which require full validation.

Read `AGENTS.md`, `CLAUDE.md`, `docs/fix-status.md`, `docs/print-quality.md`,
`docs/milestone-1-execution-prompt.md`, printing-related crate READMEs and existing tests. Work only
on `arena/7534c8f4-pdfcraft`. Preserve the current working tree. Do not mark milestone 1 complete
because a prompt exists; verify its prerequisite build/test gates first.

The existing engine and print compositor are present. Repair and extend them before considering
replacement. **If a genuinely required component is absent, build the smallest complete component
from scratch in safe Rust**, using public specifications and tests, following Stage 9. Missing
hardware, inaccessible dependencies and failing compilation are not evidence of a missing engine.

**Preserve the user's Hand-pointer pan requirement:** open-hand/Grab when ready, closed-hand/
Grabbing during drag, correct H/V/temporary-Space handling, no document mutation, and no stuck
gesture after opening or closing Print. Printing must not regress that behaviour.

Non-negotiable safety rules:

- Preserve print permissions, unsaved work, undo history, signatures and source document bytes.
- Never bypass driver paper/scale validation, silently change printers, or silently rasterize.
- Keep vector printing the default where supported. Preview textures are never print payloads.
- Do not darken, sharpen or alter document colours globally to hide a printer/driver defect.
- No unsafe application code, unbounded allocations, panic paths, new default listeners or telemetry.
- Use synthetic fixtures and approved assets; never publish private PDFs, printer credentials,
  identifying diagnostics or third-party application screenshots.
- Discover and print only through configured/explicitly selected destinations. Do not scan the
  user's network or contact the printer's address simply because it appeared in the conversation.
- Require explicit operator approval before sending physical test jobs. Bound pages and copies;
  never repeatedly submit while debugging a failure with uncertain delivery status.

For each stage, record findings, necessary changes, checks and pass/fail/blocked. Prove each
bug fix with a failing regression before the fix and success after restoration; an audit that
finds correct behaviour needs no manufactured edit. A mocked driver or queue acceptance does
not prove paper printed correctly. Reuse milestone-1 Hand/build evidence when unchanged.

---

## Stage 1 — Audit the complete printing pipeline and prerequisites

### Prompt

Confirm normal build/test prerequisites from repair milestone 1. Trace the actual paths:

1. UI print draft, form-input commit, page selection and print permissions.
2. `Session::print_pdf` and image-print handling in `crates/engine`.
3. Page geometry, annotations/forms, imposition and sheet PDF generation in `crates/print`.
4. `crates/print/src/spool.rs`, which delegates native discovery/submission to pinned `craft-print`.
5. The shared Windows ticket/native transport and its `winprint` dependency.
6. `crates/automation/src/printing.rs` and the `doc_print` tool/schema.
7. `crates/ui-egui/src/print_ui.rs` and `web_print.rs`.

Record dependency revisions and the methods that own each responsibility. Inspect implementation,
not just README claims: identify and correct stale documentation only after confirming behaviour.
Preserve the current per-page raster-sizing and browser-quality candidates until tested.

Create a responsibility table separating application bugs, shared-library bugs, driver restrictions,
unsupported features, hardware conditions and reproduction gaps. Record baseline UI responsiveness,
PDF size, effective raster DPI and observed errors for the small fixtures.

### Exit gate

Each reported failure has a suspected pipeline stage with evidence, and the normal test environment
is available or explicitly blocks acceptance. No application workaround duplicates the native spooler.

---

## Stage 2 — Build synthetic print fixtures and a controlled reproduction matrix

### Prompt

Generate original test PDFs in code rather than committing large binary documents. Cover:

- Small/large text using approved embedded fonts; black, grey and coloured text.
- Thin vector lines, measured rulers, colour/grey patches and continuous gradients.
- Transparent artwork, masks, images with known pixel dimensions and rotation.
- Printable/non-printable annotations, form values and edited text with saved appearances.
- Portrait/landscape, `/Rotate`, crop offsets, mixed sizes and fractional page dimensions.
- Normal pages and narrow/large pages that hit renderer caps without huge test allocations.
- Encrypted documents with allowed and denied printing; no permission bypass in any mode.

Distinguish low source-image resolution, low rendering resolution, faded colour, missing glyphs,
clipped content and incorrect physical scale. Record the same specimen across these paths:

| Path | Purpose |
|---|---|
| Saved source in an independent reader | Establish source content and appearance |
| PdfCraft print-ready vector PDF | Isolate compositor output from the native driver |
| PdfCraft image-print PDF at 150/300/600 DPI | Isolate raster/geometry behaviour |
| Windows Microsoft Print to PDF | Exercise a virtual queue; not proof of physical output |
| Selected physical Kyocera KX queue | Validate the user's actual quality problem |
| Canon standard/XPS queues, when available | Reproduce the separate ticket-merge failure |
| Browser print/PDF output | Validate web-specific planning and lifecycle |

Use matching media, page selection, scale and driver settings. Record unavailable paths as such.
For physical measurements, define an agreed tolerance from the device specification before
comparing results. Do not silently loosen a threshold after observing a failure.

### Exit gate

Reproduction cases distinguish the failure classes and identify the first stage where output
becomes wrong. Physical test jobs are explicitly approved and bounded.

---

## Stage 3 — Add useful, privacy-conscious printing diagnostics

### Prompt

Audit failure propagation from driver discovery, COM/WinRT setup, capabilities, ticket construction,
merge/validation, rendering and submission to the user's notification and local log.

Preserve the failure stage, underlying HRESULT/OS code, bounded error chain and actionable message.
For Windows ticket merge errors, retain the returned driver diagnostic when available rather than
reducing it to only "Failed to merge print tickets". Use safe APIs and explicit error variants;
do not introduce unsafe Win32 calls in this repository to retrieve diagnostics.

Make failures visible without dumping full PDFs, raw tickets, tokens, passwords, private document
names or network identifiers into ordinary logs. Bound vendor messages and diagnostic sizes;
escape control characters. Any detailed support export must be explicit, redacted and reviewed
by the user. No hidden upload or background network collection.

Distinguish: discovery failed, no configured printer, selected queue disappeared, unsupported media,
validation changed requested scale, preparation failed, submission failed and submission accepted.
Never display "printed successfully" when only queue acceptance is known. For an uncertain
post-submission error, explain that retrying may duplicate output; do not auto-resubmit.

### Exit gate

Synthetic failures preserve useful causes through engine/UI/automation boundaries, logs stay
bounded and privacy-conscious, and a diagnostic can identify which native stage failed.

---

## Stage 4 — Fix printer discovery, loading and dialog responsiveness

### Prompt

Reproduce #3810 using configured Windows queues and injected discovery failures. Test no printers,
multiple queues, offline/slow network printers, default changes, removed queues, invalid capability
XML and Unicode queue names. Discover how the existing worker/result ownership model operates.

Keep blocking discovery, capability reads and preference calls off the interactive frame path;
a cheap non-blocking OS-window launch need not acquire a new worker just for abstraction's sake.
Show loading, empty and error states separately. Use the shared contract's result-ownership rules:
a printer list may update an app-wide cache, but an obsolete dialog request must not overwrite its
current destination/options. Keep the status of explicitly launched jobs even after tab/dialog
changes. Bound requests/caches; never spawn replacement workers to simulate cancelling a stuck OS call.

Audit **the entire preparation path**, not just spool submission. Commit pending form typing once,
validate permissions/settings, then capture the intended document revision, page range, destination
and options. Expensive imposition, image rendering/encoding and byte preparation run through a
bounded existing worker or a bounded new snapshot-based engine API, not inside a UI click/frame.
Never move a live UI-owned Session across threads without an ownership design or read a later
active tab by accident. Show Preparing before Submitting; keep Cancel usable before irreversible
submission, reject duplicate activation and preserve source work. Check job byte/memory admission
before allocations and release temporary images promptly. Keep PDF output atomic and confined.

Native spooling may be non-cancellable after handoff. Say so accurately, never retry uncertain jobs
automatically and do not label mere UI cancellation as spooler cancellation. Test close/reopen,
edit/tab switch during preparation, stale preview completion and preparation failure. Measure Print
acknowledgement, frame stalls, memory and cancellation using the shared reference workload.

Keep selection stable by real queue identity. Refresh explicitly and preserve valid user settings.
A missing selected queue must never redirect a confidential job to another/default printer.
Opening Print must not enumerate indefinitely every frame or freeze Hand-pan in the main window.

### Exit gate

The dialog and document input remain within the agreed responsiveness targets during both
preparation and discovery. Stale results cannot overwrite selection; errors and non-cancellable
states are honest; queue changes cannot reroute jobs. Required Windows runtime tests pass.

---

## Stage 5 — Resolve Windows PrintTicket and native transport failures

### Prompt

Reproduce #3823 with approved, redacted capabilities/default-ticket captures from both Canon
queues and a Windows runtime test. Do not assume Kyocera fading has the same cause. Inspect the
actual shared `craft-print`/`winprint` code and pinned revisions before changing dependencies.

Validate namespace handling, feature scope, option names, dependent parameters, media dimensions,
orientation, output colour, collation, duplex and copies against the driver's capabilities.
Preserve vendor-defined media options and required parameter bindings. Do not invent unsupported
features, force two-sided settings onto a non-duplex queue or equate UI labels with schema names.

When the defect belongs to the shared dependency, keep the fix there through the owner's approved
contribution process and integrate a reviewed immutable revision here. Do not edit only a Cargo
cache, silently track a moving branch, vendor an unaudited native library, or work on a different
PdfCraft session branch. If shared-library contribution/integration is unavailable, record that
blocker instead of pretending an app-side retry fixed the dependency.

Retain validation of final media, orientation and intended scale. An alternate native transport
may be added only as an explicit, tested capability with clear limits and permission to change
transport. Never quietly discard settings, flatten everything, bypass ticket validation, or send
an unscaled architectural drawing at an arbitrary size just to get paper out.

### Exit gate

The reported Canon driver failure is explained and corrected with shared-library regression
coverage and real Windows validation, or it remains a named blocker. Unsupported options fail
before submission without modifying requested scale or rerouting the queue.

---

## Stage 6 — Diagnose and fix Kyocera faded or unclear output

### Prompt

Use the selected Kyocera TASKalfa 5054ci KX queue and the same synthetic specimen throughout.
Record Windows/app/KX driver versions and the actual configuration. Check EcoPrint/Toner Save,
Draft/quality mode, colour/monochrome, media type, resolution and any application/driver scaling.
Do not change persistent printer defaults without explicit approval.

Compare the printer's internal test/status page with source-PDF, vector-PDF and image-print jobs.
The reported Toner Low status is a confounder, not proof of the cause. If supplies/maintenance are
changed by the operator, repeat the controlled comparison and record the change.

Investigate the first differing software stage: preview raster used accidentally as payload,
extra resampling, requested/effective DPI mismatch, font embedding, alpha handling, transparency,
colour conversion, driver economy settings or a second fit-to-page operation. Missing-glyph
blocks are a font/appearance defect; increasing DPI cannot repair them.

Keep vector text/lines as vectors in the normal print-ready document. Preserve approved embedded
fonts and source colours. If image mode is explicitly selected, render from the current document
state at the requested supported resolution; never reuse a thumbnail or screen-zoom texture.
Use opaque paper compositing correctly, with no double premultiplication/unpremultiplication and
no automatic contrast boost. Reading-mode theme colours must not leak into print output.

Fix confirmed software causes at their owning layer. If only the physical device output is wrong,
present the measured driver/hardware finding and next action without claiming a software fix.
If a required hardware action cannot be performed, keep physical acceptance blocked.

### Exit gate

The Kyocera symptom is reproduced and attributable, and the corrected path is confirmed on paper
with controlled settings. Any residual supply/driver restriction is documented, not concealed.

---

## Stage 7 — Guarantee paper dimensions, scale and raster geometry

### Prompt

Validate page selection, labels, order, duplicates, orientation, clipping, margins, Actual size,
Fit, Shrink, custom scale, multiple-per-sheet, booklet and poster layouts through the compositor
and native driver. Ensure the driver's second scaling step cannot undo the application layout.
Test mixed source sizes, crop offsets, rotation and physical unit conversions.

Verify the current image-print fix with save/reopen and final sheet placement: each image occupies
its own source displayed rectangle even when another page was rendered at a different effective
DPI. Pixel rounding must not become a change in physical paper dimensions. Record requested and
effective resolution; do not claim 600 DPI when a safety cap produced less. Make material quality
reductions visible to the user, and provide a safe vector/PDF alternative where appropriate.

Respect the current explicit image-page bounds unless a separately tested `/UserUnit` or tiled
implementation replaces them. Do not remove memory limits to make huge pages appear supported.
If implementing tiled high-resolution printing, bound retained tiles/encoded data, avoid seams,
and test coordinate transforms and transparency at tile edges.

For #3811, obtain exact width/height, units, orientation, intended physical scale and supported
printer media. Implement a validated custom-size path or a confirmed preset, not a guessed
"Smart Board" size. Keep UI, engine, automation and native capability validation consistent.
Unknown dimensions or unavailable driver support remain explicit blockers for that report.

### Exit gate

PDF geometry tests and physical calibration agree within the predeclared tolerance. Unsupported
paper, precision, size or resolution is reported before submission instead of silently changed.

---

## Stage 8 — Validate browser printing without reducing quality silently

### Prompt

Run the real web build in supported browsers. Validate the candidate stable 300-DPI planning,
renderer side/pixel caps, exact-colour CSS request, named/rotated page handling and final-page
break behaviour. Explain that browser/user print preferences can override colour requests.

Test image decoding before print, repeated jobs, cancellation, dialog lifecycle, iframe/blob URL
cleanup, page-load failures, encoded-job byte limits and downloadable print-ready PDF fallback.
Test both sides of 100/101 and 300/301 sheets without allocating unbounded raster memory. Use
bounded lifecycle tests for large jobs and a smaller real print comparison where appropriate.

Never replace a high-resolution print payload with the preview to satisfy a limit. Do not invoke
print before images are ready, revoke URLs while a dialog still uses them, or duplicate a job
because multiple callbacks fire. Physical output and generated PDF checks are distinct results.

### Exit gate

The full browser flow works, limit/failure paths recover safely, no job-length quality drop is
hidden, and browser-specific runtime evidence exists beyond extracted-function tests.

---

## Stage 9 — Implement missing printing capabilities, UI and automation together

### Prompt

Only after the pipeline audit, identify genuinely missing capabilities required by the reports.
For each one, implement a complete safe Rust vertical slice rather than a mock, button stub,
manual-only workaround or second print engine in the UI:

1. Define data types, points/mm/inch conversions, page indexing, bounds and actionable errors.
2. Implement headless document/geometry logic in the correct existing lower-level crate.
3. Keep native driver transport in the shared backend behind its appropriate platform boundary;
   keep browser APIs in the web frontend. Follow existing layering and no-unsafe rules.
4. Expose document operations through `Session` and the existing automation table. Discover
   `doc_print`/printer schemas before extending them; reject unknown or invalid arguments.
5. Add the actual UI control, accessible labels, validation, progress and recoverable errors.
6. Test engine → automation → generated PDF/native adapter, then exercise the real frontend.

All file outputs must retain atomic writing and automation root confinement, including temp files,
new custom-media/export options and fallback paths. Resolve permissions before expensive work or
submission. Source edits must remain intact after failed or cancelled jobs. Clean temporary job
files on success/failure and protect them appropriately while they exist.

Build genuinely absent components from original Rust code based on public PDF/OS specifications.
Do not copy GPL/AGPL implementation code, read proprietary application internals or add prohibited
fonts/assets. A clean failure is preferable to a success response that lost content or settings.

### Exit gate

Each required new capability is usable through its real engine/tool/UI path, bounded and tested.
No hidden UI-only operation or unverified replacement backend is called complete.

---

## Stage 10 — Preserve Hand-pan and document interaction around Print

### Prompt

Run milestone-1 Hand-pan tests before and after printing changes. Open/close Print during Hand
mode, temporary Space-hand and an active middle-button gesture. Modal takeover terminates active
canvas gestures safely, without leaked release/stale Space state or automatic drag resumption.

After cancellation, successful preparation, submission failure and printer-preferences dismissal:

- Hand shows Grab when available and Grabbing during a real canvas drag.
- H/V and temporary Space behave correctly without stealing form/editor typing.
- Panning moves only the document viewport, does not activate links or change PDF content.
- Completion cannot reopen a closed dialog or restore its input capture. App-wide discovery may
  update its cache safely; explicitly launched jobs retain correctly owned status/error reporting.
- Preview gestures are scoped to the preview; they must not move the underlying document.
- Current page, zoom and layout are preserved unless the user explicitly changes them.

Do not reset the active tool to Select merely as a workaround for a stuck cursor. Fix ownership
and focus restoration, and cover the failure with a real control-channel/UI regression.

### Exit gate

Print lifecycle does not regress the user's Hand-pointer experience or mutate document content.

---

## Stage 11 — Run regressions, quality gates and real printer acceptance

### Prompt

Use normal locked dependencies on authorized runners. Run scoped tests with limited concurrency
and suitable platform-specific timeouts. Confirm test filters match actual tests. Starting points:

```sh
cargo fmt --all -- --check
cargo test -p pdfcraft-print -j 2 --locked
cargo test -p pdfcraft-engine -j 2 --locked --lib print_raster
cargo test -p pdfcraft-automation -j 2 --locked doc_print
cargo test -p pdfcraft-ui-egui -j 2 --locked --test print --test view --test control
cargo test -p pdfcraft-ui-egui -j 2 --locked --lib web_print
cargo test -p pdfcraft-ui-egui -j 2 --locked --lib autoscroll
cargo clippy -p pdfcraft-print -p pdfcraft-engine -p pdfcraft-automation -p pdfcraft-ui-egui -j 2 --all-targets --locked -- -D warnings
cargo xtask layers
cargo xtask assets
cargo xtask wasm
```

Run complete affected UI/control targets, new diagnostics/discovery tests and shared-library
native-ticket regressions as well; the example filters are not exhaustive. Use existing CI for
full Linux/Windows workspace checks and WASM. Obtain native macOS coverage under repository
runner policy if its print behaviour changed. Record missing independent PDF-reader/oracle tools
as skipped, not passed.

For real acceptance, include configured Windows virtual and physical queues, Kyocera colour/
monochrome and approved quality modes, Canon standard/XPS queues where required, ordinary/large
media and supported duplex/copies. Verify paper came out before claiming physical success.
Use minimal approved jobs and operator checkpoints; no unattended stress printing.

Test failure recovery: unsupported paper, disappearing/offline queue, malformed capabilities,
invalid settings, render failure, temp-file errors, user cancellation and uncertain submission.
Source documents must stay open and intact. Never send a second physical job automatically just
to find out whether the first succeeded.

### Exit gate

Required scoped and CI gates pass, real target printer comparisons are recorded, and failure
paths are safe. Missing target-driver hardware remains a hardware-validation blocker even if
mocked/native-unit tests pass.

---

## Stage 12 — Final review and issue-by-issue completion decision

### Prompt

Review all patches for data safety, geometry, colour/alpha correctness, concurrency, cancellation,
resource bounds, layering, permissions, atomic writes, automation confinement and opt-in security.
Verify any changed native dependency at its immutable revision. Do not commit caches, binaries,
printer captures containing identifying data, font files or large fixture artifacts.

Update `docs/fix-status.md`, `docs/print-quality.md`, affected READMEs, the relevant original
roadmap rows and progress log. Record each report independently rather than closing the entire
printing family after one fix. Separate pre-existing failures from regressions with evidence.
Follow the current branch and repository commit/PR rules. Do not close issues or merge simply
because code was written; obtain the required tests/review and explicit authorization.

Deliver this result table with concrete evidence:

| Acceptance item | Allowed final states |
|---|---|
| Kyocera faded/unclear output | Physically verified fix / identified non-software cause / unresolved |
| Canon #3823 | Verified with affected driver / software candidate only / blocked |
| Printer loading #3810 | Native verified fix / candidate only / unresolved |
| General print report #3800 | Reproduced and verified fix / separate unresolved reproduction |
| Paper request #3811 | Exact dimensions and driver accepted / missing specification / unsupported |
| Image-print dimensions and effective DPI | Normal engine/automation tests pass / blocked or failed |
| Browser quality and lifecycle | Real-browser verified / function tests only / unresolved |
| Hand-pan preserved | Real-window/control tests pass / blocked or failed |
| Build/lint/layer/asset/WASM/CI | Individually pass / fail / skip / blocked |

Provide exact commands, test counts, skipped checks, platform/driver versions, physical comparison
results, changed files and remaining risks. Choose **complete**, **partially verified**, or
**blocked** for this milestone. Do not label software-only evidence as hardware verification.

### Final definition of done

The scoped printing reports have verified resolutions or explicitly agreed, evidenced exclusions;
required build/quality gates pass; geometry, colours, source content and permissions are preserved;
discovery/submission fail safely; and the Hand-pan experience remains correct. Required Kyocera/
Canon output claims need affected-device evidence. Without it, keep the milestone partially
verified or blocked rather than reporting "all printing fixed".

This milestone does not complete the text-editing, OCR, rendering or UI feature backlog, and does
not guarantee that every printer or future document will work without defects.
