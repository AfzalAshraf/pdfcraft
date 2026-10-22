# Repair milestone execution prompts — separate files

Status: 2026-10-21 — all seven execution specifications are available. **No milestone is
certified complete by these documents.** Execution evidence belongs in [fix-status.md](fix-status.md).

These are the seven repair milestones agreed in the conversation, not the original repository
M0–M14 roadmap. Each linked file is a separately usable, stage-by-stage implementation prompt
with scope, safety rules, tasks, exit gates, testing and a completion contract.

| Repair milestone | Separate full prompt | Scope |
|---|---|---|
| 1 | [Build, tests and Hand-pan acceptance](milestone-1-execution-prompt.md) | Reproducible build; existing candidates; conditional missing-engine work; real Hand pointer and panning |
| 2 | [Reliable, clear printing](milestone-2-execution-prompt.md) | Kyocera fading; Canon #3823; #3810 discovery; #3800 printing; #3811 media; geometry, quality and lifecycle |
| 3 | [Text editing and fonts](milestone-3-execution-prompt.md) | #3815 Chinese, #3804 Romanian/rotation, #3803 blocks, #3799 editing, #3797 spaces |
| 4 | [Scrolling and navigation](milestone-4-execution-prompt.md) | #3817 touch, #3795 release, #3793 speed, #3774 Windows middle-click, #3792 spreads, #3783 links |
| 5 | [Page, image and object editing](milestone-5-execution-prompt.md) | #3813 image drops, #3809 context menu/lag, #3808 thumbnails, #3798 z-order, #3780 deletion |
| 6 | [Rendering and conversion fidelity](milestone-6-execution-prompt.md) | #3776 CMYK/transparency; discovered Word table widths |
| 7 | [Desktop integration and features](milestone-7-execution-prompt.md) | #3818 Windows icon, #3807 detached tabs, #3802 themes, #3805 Greek OCR |

## Required shared responsiveness contract

Each prompt now requires [repair-smoothness-contract.md](repair-smoothness-contract.md). It adds:

- Measured reference workloads and **proposed** 60 Hz reference targets: p95 UI frame work ≤16.7 ms,
  p95 input response ≤50 ms and cached dialog/task acknowledgement ≤100 ms. These are not current
  measurements or guarantees for every device; confirm hardware/targets before implementation.
- Bounded workers/caches/queues, visible-page priority, coalesced previews, revision-aware results,
  cancellation and no unexplained continuous work at rest.
- Repeated-operation resource checks, actionable errors and preservation of drafts/IME, undo,
  save fidelity, print scale/colours and the Hand tool.
- A single acceptance owner per feature, so unchanged evidence can be reused without repeating
  expensive or physical tests. No normal safety/quality gate is removed.

## Execution order and constraints

1. Establish milestone 1's normal build, smoke and Hand baseline. Dependency-free policy checks
   are supplementary, not a completed app build. Preserve the working tree and session branch.
2. Prioritize milestone 2's printing/data-safety defects. Diagnose and test software while waiting
   for approved physical-printer access; do not perform duplicate test jobs in milestone 1.
3. Repair text integrity (3), navigation/input (4), object/thumbnail behaviour (5) and rendering/
   conversion (6). Reproduction and independent fixes can proceed while another track is blocked;
   do not hold a safe scroll fix hostage to a printer test. Required build/release gates still apply.
4. Deliver milestone 7 as separate icon, detached-window, theme and Greek-OCR slices after core
   stability. None of these requested features is deleted; avoid one large feature rewrite and do
   not require a missing OCR model to block unrelated verified fixes.
5. Keep **Hand pointer for pan** throughout: Grab/Grabbing, one-to-one primary drag, H/V/temporary
   Space, no PDF edits/link activation and safe modal/focus/tab cancellation. Milestone 1 owns the
   initial contract; milestone 4 extends its interaction coverage instead of rebuilding it.
6. Build a genuinely missing scoped component only after the reproduction proves it is needed.
   Do not add a new engine/framework/scheduler/state machine merely to reorganize working code.
7. Keep pass/fail/skip/blocked explicit. No mandatory platform can be excluded just to declare a
   milestone complete, and performance requires measurements rather than a prompt or helper test.

## Review changes in this revision

- Removed duplicate physical-printer acceptance from milestone 1; milestone 2 owns it.
- Removed an invitation to expand build repairs into a complete parser/renderer rewrite.
- Removed a default requirement to invent a new Hand state machine or repeat its implementation.
- Narrowed per-edit test invocations to actual affected library/integration targets while retaining
  full CI, permissions, asset, layering and WASM gates.
- Replaced mandatory per-stage code changes/red-green claims with no-change audit results and
  meaningful per-bug red/green evidence. New features test their explicit contracts.
- Added print-preparation threading/snapshot ownership, typing/IME responsiveness, thumbnail
  backpressure, renderer cost attribution, multi-window memory ownership and asynchronous OCR.

These edits change **execution specifications only**. They do not alter runtime behaviour, prove
that targets have been met or mark the seven repair milestones complete.

## Dependency access versus a source rewrite

The full application requires its pinned third-party packages, system libraries and toolchain.
The current crates.io access restriction is not removed by rewriting application files.
[Offline validation and full-build recovery](offline-validation.md) provides:

- A new `rustc`-only entry point for **20 actual production-policy tests**, without Cargo resolution.
- A separate CI check that does not replace normal workspace/UI/platform gates.
- A connected-machine procedure to create a real, checksum-preserving dependency source bundle,
  followed by `--frozen` full application testing on a properly provisioned disconnected machine.

No full dependency bundle or new release executable has been produced here. The current prompts,
code changes and limited tests must not be represented as a from-scratch, feature-complete,
bug-free replacement application.
