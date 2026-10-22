# Shared repair contract — responsiveness, scope and acceptance

Status: 2026-10-21 — **execution requirements and proposed reference targets, not measured results**.
Read this with each of the seven [repair prompts](repair-milestones.md). Repository safety, asset,
layering and commit rules still apply. These requirements do not certify current app performance.

## 1. Fix the smallest demonstrated cause

- Reuse working engine/UI infrastructure. Audit first; change only the owning path and necessary
  callers. Add a missing component only when the scoped reproduction proves it is required.
- Do not rewrite a whole engine, change frameworks, add a second scheduler or introduce a generic
  state machine merely to reorganize code. A new abstraction needs a demonstrated ownership,
  duplication or correctness problem and a smaller tested solution.
- Treat audits and already-correct stages as no-change results. For each actual bug fix, prove the
  regression fails without that fix and passes with it. Do not demand a red/green source change
  for a documentation audit or invent a historical implementation for a genuinely new feature.
- Keep one independently reviewable fix at a time. No unrelated formatting, dependency upgrades,
  speculative tuning, new preferences or feature expansion in a responsiveness patch.
- Preserve all agreed reports in the backlog. Sequencing/deferred implementation is not deletion
  of a requested feature, permission to disable it silently or permission to claim completion.

## 2. Keep frame work bounded and results correctly owned

The UI thread handles input, cheap state changes, scheduling and drawing. Measure before moving
trivial work to threads, but never leave lengthy PDF interpretation, font discovery/shaping of an
entire document, image decoding, OCR, print preparation, driver enumeration or blocking file/network
I/O on an interactive frame path.

Reuse bounded existing workers/queues. Coalesce redundant previews and superseded search/render
requests; prioritize visible pages and the current gesture ahead of speculative prefetch. Use stable
job/document/page IDs, revision/generation tokens and immutable snapshots where appropriate. Do not
clone or reparse an entire large document on every pointer move or typed character. Respect existing
render/cache limits and admit work before allocating large buffers.

Distinguish result ownership:

- A document preview/search/edit background result is applied only to the matching document,
  revision and request. Closing/switching views must not apply it to a different tab.
- A printer list may be an application-wide cached result; closing Print need not cancel discovery.
  It must not reopen a dialog, replace an explicit destination or overwrite unrelated draft state.
- An explicitly launched export/print job belongs to its captured snapshot and destination. Switching
  tabs is not permission to retarget it or discard its eventual submission error/status.

A cancelled/superseded task stops at safe checkpoints and releases owned resources. A native call
that cannot be interrupted remains bounded to the existing worker slot; ignore only obsolete UI
results, not an uncertain physical submission. Never leak replacement threads to simulate cancellation.

Progress should identify preparing, submitting, accepted, failed or cancelled accurately. A progress
indicator must not itself cause expensive per-frame rescans, continuous discovery or hidden retries.
At rest, stop unnecessary redraws, timers and render requests; deliberate animation/progress remains
bounded and honours reduced motion. Do not remove keyboard/accessibility feedback to save frames.

## 3. Define and measure a reference workload before optimizing

Record OS, hardware, app revision, release-like build/profile, renderer, display refresh/scaling,
font/model inputs and cold/warm cache state. Choose an agreed reference machine and representative
synthetic documents before testing. Proposed starting workloads:

| Workload | Required observations |
|---|---|
| Small PDF: 1–10 pages, text, link, form and image | Open acknowledgement, input latency, tool switching, typing, save/reopen |
| Medium PDF: about 100 mixed pages | Continuous pan/zoom, thumbnails, search, page jumps, print-dialog lifecycle |
| Long PDF: about 1,000 lightweight pages | Lazy scheduling, cache plateau, tab switches and no full-document frame scans |
| Small page count with one image-/vector-heavy page | First useful preview, full-quality completion, bounded memory and cancellation |
| Hostile/invalid inputs near current documented bounds | Clean errors, no panic, no limit removal or damaged output |

Page count is not a complexity or memory budget. Generate fixtures in code, use approved assets,
and record object/image complexity and relevant byte sizes. Do not commit large binaries. Large
print counts are planner/lifecycle tests, not instructions to send thousands of physical sheets.

### Proposed targets for the agreed 60 Hz reference environment

| Measurement | Initial target and meaning |
|---|---|
| Warm pan/zoom UI-thread frame work | p95 at or below 16.7 ms; record p99 and all stalls over 100 ms |
| Input-to-visible response during a warm gesture | p95 at or below 50 ms; never add easing to hide delayed Hand movement |
| Cached dialog/tool acknowledgement | p95 at or below 100 ms; slow results may arrive asynchronously |
| Long-task acknowledgement | Loading/progress visible within 100 ms; first progress update within 250 ms where observable |
| Cancellation acknowledgement | UI acknowledges within 100 ms; actual stop occurs at a documented safe checkpoint |
| Idle | No unexplained continuous rendering, discovery or queue growth after settling |

These are **proposed acceptance targets**, not universal FPS guarantees or current measurements.
Confirm them against intended minimum hardware before implementation. Record any justified alternate
baseline in advance; do not silently relax targets after observing failures. Distinguish UI-thread
CPU time from GPU presentation, OS scheduling and full-document readiness. Showing a loading label
within 100 ms does not mean the document has opened/rendered in 100 ms.

Use at least three comparable runs with a declared warm-up and enough samples for meaningful
percentiles (at least 200 warm interaction frames where applicable). Keep cold-start/first-page
numbers separate. Report sample count, p50/p95/p99, maximum stall, memory high-water mark, queued
work/cache counters and errors. Repeat without profiling to identify instrumentation overhead.

Check boundedness with repeated open/pan/edit/close cycles (for example 20 cycles) and a five-minute
mixed interaction session. Live resource/job counts must return to their intended baseline, caches
must remain within declared limits and memory must reach a documented plateau rather than grow
per cycle. RSS need not immediately fall after freeing allocations; explain allocator/cache retention
and corroborate it with live counters. Define limits from the existing architecture/reference device,
not an invented global megabyte threshold or a benchmark that silently skips complex pages.

Keep deterministic correctness/queue-limit tests in ordinary CI. Use controlled runners for noisy
wall-clock measurements; a busy shared CI machine alone must not decide a microbenchmark regression.
If a target cannot be measured, report unverified performance, not a smoothness pass.

## 4. Preserve correctness while improving responsiveness

- Hand remains Grab/Grabbing with primary-button one-to-one movement, correct H/V/Space, no
  link activation/document edits and safe focus/modal/tab cancellation. Touch and latched
  middle-button autoscroll remain distinct policies; no global speed hack.
- Draft typing, IME composition, undo/redo and atomic saves must not race a worker. Preview quality
  may be temporarily lower only when identifiable as a preview; final saved/printed/exported output
  must use the documented fidelity path. Do not call a thumbnail a completed render.
- Show useful, actionable failures and keep user work recoverable. A timeout is not an empty success.
  No duplicate submissions, automatic printer changes, hidden OCR/model downloads or silent fallbacks.
- Do not weaken source permissions, print scale/colour validation, control authentication or root
  confinement. Every new document operation needs engine/automation parity. View/window-only actions
  use the opt-in UI control channel, not fake PDF edits or an unnecessary new engine API.
- Check keyboard focus, IME, long localized labels, display scaling, reduced motion and accessible
  labels whenever the corresponding control changes. Keep menus/dialogs usable on small windows.

## 5. Own acceptance once; reuse unchanged evidence

| Milestone | Owns acceptance for | Other milestones do |
|---|---|---|
| 1 | Reproducible build, baseline smoke flow, initial Hand correctness and performance harness | Reuse the baseline, rerun affected paths |
| 2 | Printer/driver diagnostics, paper/DPI/quality, browser print lifecycle, approved physical output | Do not repeat physical jobs without a relevant output change |
| 3 | Text/font/whitespace/rotation correctness and draft responsiveness | Refer text defects here rather than changing print contrast |
| 4 | Touch/wheel/middle-button policies, navigation/links and input latency | Preserve milestone-1 Hand behaviour; extend rather than rebuild it |
| 5 | Image/object/page operations, thumbnails and edit hit-test latency | Reuse stable identities and cache admission |
| 6 | CMYK/blend fidelity and Word width/package/oracle acceptance | Reuse verified geometry/format tests |
| 7 | Windows shell icon, detached-window lifecycle, themes and Greek OCR | Deliver separate slices after core stability, not one large rewrite |

Milestone 1 compiles/tests existing candidates and records handoffs; it does not require every
milestone-2 physical printer test or milestone-7 feature before a build baseline can pass. Audits,
reproduction and isolated safe fixes may proceed while another milestone awaits hardware/dependency
access; integration/release claims remain blocked by their required unpassed gates.

Reuse evidence only when the code revision, configuration, inputs and affected behaviour still
match. Rerun relevant tests when a dependency or shared path changes. Use risk-based representative
platform/scale/layout coverage with explicit incident cases and edge cases; do not run the entire
Cartesian product for every cosmetic change. Keep normal repository quality gates and full CI.
No excluded mandatory target may be labelled passed.

## 6. Delivery evidence

For each fix, record the reproduction, owning path, test commands/counts, before/after measurements,
resource/cancellation behaviour, save/permission safety and remaining platform blockers. Do not run
physical printers or publish private captures without approval. Prompt review alone changes no app
behaviour and satisfies no performance or runtime acceptance gate.
