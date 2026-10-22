# Repair Milestone 6 — Rendering, colour and conversion fidelity

Status: **execution specification, not implementation or verification evidence**.

## Scope

#3776 CMYK/transparency blending and the branch's discovered Word table-width defect.
Broader renderer replacement or Office import is not automatically part of this repair milestone;
include new findings only through explicit scope and acceptance criteria.

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

## Stage 1 — Establish numerical and visual reproductions

### Execution prompt

Create original PDFs covering DeviceCMYK/RGB/Gray, approved ICC data where available, soft masks,
alpha, blend modes, isolated/knockout groups, overprint and image colour spaces relevant to #3776.
Record the expected result from public PDF semantics and independent external oracles; never read
or copy proprietary/GPL implementation code. Distinguish reading-mode colour transformations from
actual document/print colour. For Word, create unequal, merged and row-reset table grids in code.

### Exit gate

The failing colour and conversion cases have explicit numerical/structural expectations and approved fixtures.

## Stage 2 — Locate the first incorrect colour stage

### Execution prompt

Trace parsing → colour-space conversion → group/backdrop blending → premultiplication → surface
composition → export/print. Check component ranges, black generation, alpha-zero handling, transfer
functions and mask/group nesting. Verify current over-white compositing before changing it; do not
add contrast boosts or double-unpremultiply pixels. Compare both pixels and preserved vector content.
Keep every traversal/allocation bounded for hostile or deeply nested group input.

### Exit gate

The owning stage is identified with a small regression that fails for the actual defect, not merely a screenshot difference.

## Stage 3 — Implement the required rendering fix

### Execution prompt

Repair existing renderer adapters or the owning vendored component under its documented patch and
licence policy. If a needed device/colour primitive is absent, implement a narrow specification-based
Rust component with correct numerical domains and independently checked tests. Do not claim a new
full PDF renderer after implementing a single blend mode, replace every crate, or discard unknown
operators. Unsupported cases must emit honest diagnostics rather than silently dropping content.
Avoid a document-wide workaround for one colour space or driver.

### Exit gate

Required blending/conversion cases pass numerical and visual regressions without altering already-correct RGB/alpha output.

## Stage 4 — Validate Word table geometry and package structure

### Execution prompt

Reuse milestone-1 source-regression evidence for the unchanged width candidate; this stage owns
full package/independent-reader fidelity and any additional fixes. Each cell follows its grid columns,
including spans and row resets, without inventing equal columns or mixing independent tables.
Test empty/invalid grids, bounded column/cell counts, nested content, overlaps and font/image limits.
Inspect DOCX XML, relationships and package consistency. Maintain existing approved fonts and content
ordering. Use actual-source tests and a real Word/LibreOffice reader; an optional oracle silently
skipped because it is absent is not cross-reader verification.

### Exit gate

Both new width regressions fail without the fix and pass with it, and generated packages render acceptably in the required independent reader.

## Stage 5 — Propagate fidelity through export and printing

### Execution prompt

Exercise on-screen rendering, page_render, image export, print-ready vector PDF, explicit image
printing and relevant Office exports. Respect PRINT_TAG and original colours; reading themes and
validation badges must not enter printed/exported content. Compare DPI/geometry independently from
colour correctness. Keep raw source bytes, permissions, edit history and saved signatures intact.
Do not route every print through a raster to conceal a vector colour defect.

### Exit gate

The same corrected content survives applicable output paths with correct geometry, permissions and no preview-only substitution.

## Stage 6 — Measure fidelity, safety and performance

### Execution prompt

Set tolerances before comparison and separate antialiasing differences from real colour defects.
Use the shared reference workloads and separate interpretation, raster, colour conversion, encoding
and UI upload costs. Measure time/peak memory for representative and adversarial groups/tables.
Reuse current caches and bounded tile/work admission; do not introduce a GPU backend, new engine or
extra full-document copy solely to chase a benchmark. Run focused
mutation/fuzz cases with bounded duration; turn findings into minimal regressions. Use authorized CI
for all changed target backends and test CPU/GPU or browser paths actually affected. Report absent
oracles and hardware explicitly rather than converting them into successful test counts.

### Exit gate

Numerical/visual/structural tests and affected platform gates pass with no unexplained fidelity or resource regressions.

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
