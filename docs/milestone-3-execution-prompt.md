# Repair Milestone 3 — Text editing, fonts and saved appearances

Status: **execution specification, not implementation or verification evidence**.

## Scope

#3815 added Chinese text, #3804 rotated Romanian text, #3803 edited text becoming blocks,
#3799 text-editing failures and #3797 spaces disappearing. These reports need separate
reproductions; increasing print DPI cannot repair missing glyphs.

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

## Stage 1 — Reproduce and map the text-edit pipeline

### Execution prompt

Obtain each report's actual reproduction through authorized sources. Generate original fixtures
for supported embedded fonts: simple fonts, subset/composite fonts, ligatures, spaces, mixed scripts,
portrait/rotated pages, crop offsets and text matrices. Include Romanian U+0218/U+0219/U+021A/U+021B
and Chinese characters only when a policy-approved face covers them. Keep private documents local.
Trace block detection → editor draft → shaping/encoding → content replacement → appearances → save.
Separate font-coverage failures, unsupported turned-text editing, lost whitespace and renderer bugs.

### Exit gate

Each report has a minimal reproduction or a named missing input; no font/encoding defect is grouped under printing quality.

## Stage 2 — Implement honest font coverage and embedding

### Execution prompt

Audit installed versus embedded versus permitted PDF-output fonts. A system UI fallback must never
become an unapproved font embedded in a PDF. Probe coverage for every input character before mutation,
including punctuation and combining marks. Distinguish a missing face from missing glyphs or an
unsupported encoding. Use approved craft-fonts inputs with pinned provenance when available; do not
add forbidden Noto CJK/Source Han or proprietary fonts. Report unsupported script/face combinations
before discarding the draft. Validate shaped clusters/combining sequences as well as code-point
coverage; a supported character is not proof of correct shaping. If encoding/embedding is absent,
implement only the required bounded slice and validate saved output.

### Exit gate

Supported text round-trips with the correct approved font; unsupported input leaves the draft and document intact with an actionable error.

## Stage 3 — Preserve spaces and text boundaries

### Execution prompt

Reproduce #3797 on insert, replace, line joins, paragraph edges and save/reopen. Keep consecutive,
leading, trailing and non-breaking spaces distinct from inferred visual gaps. Do not indiscriminately
trim strings or normalize user text. Map editor offsets to glyph/content boundaries without breaking
UTF-8, PDF UTF-16 surrogate pairs, ligatures or adjacent text objects. Bound replacement work and
preserve neighbouring blocks, hidden text, links and unknown operators. Add a regression that checks
both extracted Unicode and rendered placement, not only an in-memory draft string.

### Exit gate

Whitespace survives commit, undo/redo and save/reopen without deleting neighbouring content.

## Stage 4 — Repair rotation and coordinate transforms

### Execution prompt

Trace page /Rotate, /UserUnit, crop origin, text matrix and current transformation matrix through
selection, editing and replacement. Test 0/90/180/270 degrees and nontrivial text transforms. Diagnose
Romanian coverage separately from coordinate placement. Never fix rotation by flattening the page
or applying an unrelated global rotation. If a transform class remains unsupported, reject it before
changing content and preserve the editable draft. Implement missing transforms at the engine layer,
then expose them through the existing editing operation/tool.

### Exit gate

Supported turned text has correct positions and glyphs after save; rejected transforms do not corrupt the PDF.

## Stage 5 — Regenerate correct appearances and prevent block output

### Execution prompt

For #3803/#3799, inspect selected font resources, character codes, widths, ToUnicode, text state and
saved resources using public PDF semantics. Ensure newly encoded glyphs actually exist in the embedded
face and required resource references survive save. Validate all changed text, form and annotation
appearance paths relevant to the reproduction. A preview-only substitution must not conceal an invalid
saved PDF. Keep validation/signature badges out of actual document content. Preserve permissions and
make failed edits atomic.

### Exit gate

Edited text renders and extracts correctly from saved bytes in PdfCraft and an independent reader; no missing-glyph blocks are hidden by preview fallback.

## Stage 6 — Integrate draft recovery, UI and headless tools

### Execution prompt

Use the existing edit/Session/automation architecture. Extend schemas only where needed and validate
all arguments before mutation. Test multiline edits, form typing, keyboard shortcuts, tool changes,
Print, closing a dirty tab and failed save. Undo/redo must restore text and resources together. Errors
must keep user typing recoverable. Verify Hand/Space do not steal input from text fields, and cancel
page gestures when dialogs take input. Test IME preedit/commit/cancel, rapid typing, focus changes
and paste without committing incomplete composition. Preserve the last valid draft on shape/save
failure. Cache font discovery/coverage safely, coalesce superseded layout previews and never scan
all fonts/pages or save the full document on every key. Use explicit revision ownership; background
completion cannot replace newer typing. Measure draft-to-preview latency and undo grouping without
changing text or degrading final fidelity. Add actual UI/control regressions alongside engine tests.

### Exit gate

The same supported edit succeeds through engine, automation and real UI; every failure preserves original data and the draft.

## Stage 7 — Run cross-reader and print acceptance

### Execution prompt

Compare synthetic source and edited output at multiple zoom levels, after save/reopen and after
undo/redo. Use independent text extraction and rendered output, with required oracle tools present.
Print a minimal approved edited-text specimen through vector and explicit image paths when hardware
acceptance is needed; never send unattended jobs. Test both permitted and denied encrypted documents.
Record font build inputs and platform coverage explicitly; missing approved Chinese coverage is not
a passed Chinese test.

### Exit gate

Each scoped report has evidence from saved bytes and its required reader/platform path; remaining font/hardware limitations stay open.

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
