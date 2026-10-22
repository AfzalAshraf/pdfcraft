# Fix and verification ledger

Last reviewed: **2026-10-22**. This is a branch-local work record, not a release certification.
**The M0–M14 roadmap and the open-issue backlog are not complete.** No upstream issue has been
closed and no milestone percentage has been increased by this work.

## Implemented changes and evidence

| Change | Evidence obtained | Still required |
|---|---|---|
| Word table widths follow their own grid columns, including merged cells and row resets | 31 export test functions passed in a dependency-only local harness using the actual repository source. Both new regressions failed with the production fix temporarily removed, then passed with it restored. | Normal workspace/CI build; cross-reader checks. The optional LibreOffice test did not exercise LibreOffice because it is not installed. |
| Browser print resolution no longer drops at 101/301 sheets; pixel and side limits are planned honestly; exact colours requested | Three tests compiled from the actual pure production functions passed. All three failed against the old planner/CSS. | Full UI build, browser lifecycle/printing tests, physical output. |
| Image-print pages retain exact displayed dimensions despite raster caps and pixel rounding | Engine save/reopen regression and input/compositing tests added. An independent numeric reproduction shows the old path enlarging a 72.25-point page to about 184.326 points when another page is capped. | Engine/automation tests have **not** run; no physical printer confirmation. |
| Linux held auto-scroll stops on release, while a click still latches (#3795) | Four standalone gesture-state tests passed; three fail if release cancellation is disabled. UI unit/control/comment tests and parity notes updated for the intended behaviour. | Full egui/control tests, real mouse input, visual review. Windows/macOS retain their existing held-pan behaviour. |
| Windows taskbar/Alt+Tab icon (#3818) | Startup now sets winit's taskbar icon (ICON_BIG) from the same bundled PNG used for the small window icon. Safe API signatures checked against the vendored winit and eframe sources. Windows conversion regression added. | Windows compilation and runtime checks with both wgpu and OpenGL. No claim of runtime verification. |

## Repair milestones 1 and 2 — implementation follow-up

The execution prompts are specifications, **not certificates of completion**. The user has now
requested implementation. Both repair milestones remain **partially implemented / validation
blocked**; neither complies with all of its prompt's acceptance gates yet.

New implementation in this follow-up:

- `canvas.rs` keeps Hand's Grab/Grabbing feedback clear of link hover, uses primary-button-only
  manual pan, rejects non-finite offsets, and blocks canvas interaction under modal/focus loss.
  `lib.rs` terminates the old canvas drag on tab changes as well, including when it is not drawn.
- `print_ui.rs` preserves a resolved queue or explicit Save as PDF across refresh/reopen. A
  removed queue remains named and is refused, never redirected to another/default destination.
  The button and `print_now` share validation, so a direct caller cannot bypass pending discovery.
- Discovery uses a fallible named-thread spawn, a one-result bounded channel and last-resort
  worker-panic containment. It wakes on success or failure and presents failure separately from
  a successfully empty list. Only one query is retained; reopening does not leak replacement
  workers for a stuck native call. A truly hung native call still cannot be interrupted here.
  The UI never displays a panic payload; the normal Rust panic hook may still write to stderr.
  Underlying backend errors that `craft-print::printers()` already collapses into an empty list
  remain a diagnostic limitation. This does **not** establish the cause of Windows #3810.

Evidence obtained:

| Scope | Result |
|---|---|
| Actual `hand_pan.rs`, `print_destination.rs`, `print_discovery.rs` through `tests/interaction_policies.rs` | **16 tests pass** (5 geometry, 7 destination, 4 discovery). Standalone Clippy with `-D warnings` passes. |
| Mutation: restore default rerouting policy | **2 tests fail**, 14 pass. |
| Mutation: remove destination guard | **3 tests fail**, 13 pass. |
| Mutation: remove non-finite pan guard | **1 test fails**, 15 pass. |
| Mutation: remove discovery panic containment | **1 test fails**, 15 pass. |
| New real-UI regressions in `tests/view.rs`, `tests/print.rs`, `tests/control.rs` | Added, **not executed**. Normal Cargo resolution still fails on inaccessible crates.io; offline attempts lack `thiserror`. No UI screenshot/desktop verification obtained. |
| Existing scoped export / gesture / extracted browser checks, rerun | **31 / 4 / 3 pass**, with the same narrower-scope caveats documented below. |
| Normal layer gate attempt | **Blocked before execution** by missing registry dependencies. Assets/WASM/full platform gates remain unverified. |

Mutation comparisons used scratch copies of the actual modules with one safeguard removed at a
time. Production sources were not replaced. They demonstrate policy regression sensitivity,
**not** real-window red/green evidence. A discovery test initially assumed the result sender was
already dropped immediately after wake-up; its race was corrected to use a bounded receive.

Reproducible policy command (no third-party downloads):

```sh
rustc --edition 2024 --test crates/ui-egui/tests/interaction_policies.rs -o /tmp/pdfcraft-policy-tests
/tmp/pdfcraft-policy-tests
```

This sandbox needs `-C linker=cc -C link-arg=-fuse-ld=bfd` added to the rustc command. The tests
never submit a physical print job. Required next gates are normal UI/control compilation and
execution, actual-window review, Windows native/driver tests and approved physical output
comparisons. Kyocera fading, Canon #3823 and the unspecified #3811 dimensions remain unresolved.
There is no verified release build and no zero-bug or full-milestone-compliance claim.

## Separate prompt files and registry-independent checks

All seven repair specifications are now available separately through
[repair-milestones.md](repair-milestones.md). Milestones 3–7 are specifications only; writing
those files did not implement their features or resolve their reports.

`tools/offline-tests.rs` is a new Rust-only entry point compiling the existing production Hand,
destination, discovery and middle-button state modules directly. **20 tests passed**, and the
same entry point passed standalone Clippy with `-D warnings` using the repository's configuration.
These are the previously separate 16+4 policy tests, not 20 additional feature/integration tests.
The entry point uses no Cargo resolution or registry access and adds no product dependencies.

A separate `offline-policy` CI job has been added without weakening/removing any normal workspace
or platform jobs. The job has **not** been run remotely in this session. The complete application
has not been recoded or replaced; existing working components and the source/lockfile are preserved.

[Offline validation](offline-validation.md) documents a checksum-preserving source-vendor bundle
procedure for a properly connected machine and full `--frozen` validation after transfer. No full
bundle was produced or tested here, so full application/UI/control builds remain blocked in this
sandbox. A dependency availability failure must not be disguised as an application rewrite or a
zero-bug/full-compliance claim. Formatting and diff checks pass for this follow-up.

## Prompt review — smoothness and scope

All seven specifications were reviewed and linked to the new
[shared responsiveness contract](repair-smoothness-contract.md). The review removes duplicated
printer/Hand implementation work and unnecessary engine/state-machine expansion; assigns acceptance
owners; and adds bounded async preparation, stale-result ownership, IME/draft responsiveness,
thumbnail backpressure, idle/resource checks and measured reference workloads. Desktop extras stay
tracked as independent slices rather than a single rewrite. See the
[review summary](repair-milestones.md#review-changes-in-this-revision).

The numerical targets are **proposals for an agreed reference machine**, not measurements of this
branch. This follow-up edits documentation only, not application behaviour. No performance result,
new issue resolution, native acceptance or milestone completion is claimed. Required full build,
platform and physical-printer gates remain as previously recorded.

## Environment and gates

A Rust 1.99 compiler, Cargo and rustfmt were installed in the sandbox's tool cache from
npm-distributed toolchain components (Clippy was added too); downloaded package integrity was checked against registry
SHA-512 metadata. They are not product dependencies and are not committed.

- `cargo fmt --all -- --check`: **passed**.
- `git diff --check`: **passed**.
- Isolated-source export Clippy with `--all-targets --no-deps -- -D warnings` and the
  repository's `clippy.toml`: **passed**. This excludes dependency linting and does not certify
  the workspace. The local dependency harness emitted an unused-import warning in simd-adler32.
- Standalone gesture-state Clippy with `-D warnings`: **passed**.
- Normal `cargo test -p pdfcraft-export -j 2 --locked`: **blocked**, not passed. The sandbox
  cannot fetch the crates.io registry configuration (TLS/network restriction).
- To get meaningful narrower coverage, the export source was compiled using local dependency
  manifests and GitHub-fetched source for flate2/miniz_oxide/adler2/crc32fast/cfg-if/simd-adler32,
  with versions matching those selected for the pure-Rust backend in Cargo.lock. This is **not**
  a full locked-workspace build or verification of the crates.io package archive checksums.
- Browser planning tests used extracted production functions, existing test bodies and the
  renderer's actual constants. They do **not** validate Exporter, WASM, browser APIs or the UI.
- Gesture tests compile directly with `rustc --test`; full UI integration remains unexecuted.
- Full workspace compilation/Clippy, layer/asset gates, WASM, fuzzing, Windows runtime tests and
  physical printers: **not verified**. No production-ready Windows build was produced.

The dependency-free gesture suite can also be run in a normal checkout without Cargo downloads:

```sh
rustc --edition 2024 --test crates/ui-egui/src/autoscroll_gesture.rs -o /tmp/pdfcraft-gesture-tests
/tmp/pdfcraft-gesture-tests
```

## Requested issue list

The numbers below identify the upstream reports; this table does not assert that every report
has been reproduced. An existing feature is not evidence that its reported bug is fixed.

| Report | State in this branch / next verification |
|---|---|
| [#3823 Canon ticket merge failure](https://github.com/storytold/pdfcraft/issues/3823) | Unresolved; shared craft-print/winprint code and Windows driver diagnostics needed. Do not bypass paper/scale validation. |
| [#3818 Windows generic taskbar icon](https://github.com/storytold/pdfcraft/issues/3818) | Candidate implemented; Windows verification pending. |
| [#3817 Touchscreen scrolling](https://github.com/storytold/pdfcraft/issues/3817) | Unresolved; reproduce touch events on Windows hardware. |
| [#3815 Adding Chinese text](https://github.com/storytold/pdfcraft/issues/3815) | Unresolved; approved font coverage/embedding and saved-output tests needed. No proprietary or prohibited fallback font added. |
| [#3813 Drop image into current document](https://github.com/storytold/pdfcraft/issues/3813) | Not implemented. |
| [#3811 Smart-board paper size](https://github.com/storytold/pdfcraft/issues/3811) | Not implemented; exact dimensions and driver support need confirmation. |
| [#3810 Loading printers](https://github.com/storytold/pdfcraft/issues/3810) | Windows report unresolved. Added tested worker-failure/repaint and queue-selection policies; full UI and native diagnosis pending, not proof this report is fixed. |
| [#3809 Edit-mode context menu and lag](https://github.com/storytold/pdfcraft/issues/3809) | Unresolved; reproduce/profile before changing UI behaviour. |
| [#3808 Page thumbnail](https://github.com/storytold/pdfcraft/issues/3808) | Not reproduced or fixed. |
| [#3807 Detach tab](https://github.com/storytold/pdfcraft/issues/3807) | Not implemented. |
| [#3805 Greek OCR](https://github.com/storytold/pdfcraft/issues/3805) | Not implemented. |
| [#3804 Rotated Romanian text](https://github.com/storytold/pdfcraft/issues/3804) | Unresolved; turned-text font selection and Romanian glyph coverage are separate problems. |
| [#3803 Edited text prints as blocks](https://github.com/storytold/pdfcraft/issues/3803) | Unresolved; reproduce font encoding and saved appearances with a synthetic fixture. Print DPI changes do not repair missing glyphs. |
| [#3802 Theme customization](https://github.com/storytold/pdfcraft/issues/3802) | Not implemented. |
| [#3800 Printing issue](https://github.com/storytold/pdfcraft/issues/3800) | Not reproduced or fixed. |
| [#3799 Edit text](https://github.com/storytold/pdfcraft/issues/3799) | Not reproduced or fixed. |
| [#3798 Image layer order](https://github.com/storytold/pdfcraft/issues/3798) | Not implemented. |
| [#3797 Spaces lost during editing](https://github.com/storytold/pdfcraft/issues/3797) | Unresolved; source font/encoding regression needed. Word-export fixes are unrelated. |
| [#3795 Held auto-scroll release](https://github.com/storytold/pdfcraft/issues/3795) | Candidate implemented; standalone state tests pass, UI verification pending. |
| [#3793 Slow scrolling](https://github.com/storytold/pdfcraft/issues/3793) | Not reproduced or fixed. |
| [#3792 Two-page spread](https://github.com/storytold/pdfcraft/issues/3792) | Not reproduced or fixed; do not equate existing two-up layout with this report's acceptance criteria. |
| [#3783 Unclickable PDF links](https://github.com/storytold/pdfcraft/issues/3783) | Not reproduced or fixed. |
| [#3780 Cannot delete objects](https://github.com/storytold/pdfcraft/issues/3780) | Not reproduced or fixed. |
| [#3776 CMYK transparency blending](https://github.com/storytold/pdfcraft/issues/3776) | Unresolved; renderer colour-space regression/oracle needed. |
| [#3774 Windows middle-click auto-scroll](https://github.com/storytold/pdfcraft/issues/3774) | Not implemented; current Windows held panning is unchanged. |
| Kyocera TASKalfa 5054ci KX faded printing (conversation report) | Not reproduced or confirmed fixed. Need same-document comparison of vector/image output, KX settings and physical output. Printer reported low toner; software and hardware causes must be distinguished. |

## Next gates, in order

1. Run normal scoped crate tests/Clippy on a runner with registry access; resolve any failures. Include the new `view`, `print` and `control` regressions, not just standalone policies.
2. Exercise the Linux held-vs-latched control tests and Windows taskbar icon in real app windows.
3. Run `doc_print_as_image` and the mixed-size save/reopen regression; compare physical vector
   and 150/300/600 DPI image output on the Kyocera, without altering document contrast.
4. Reproduce the high-priority font/spacing/rotation reports with small synthetic fixtures,
   preserving undo, save/reopen, printing and automation coverage for each fix.
5. Diagnose native driver failures in the shared printing library; retain explicit paper-size
   and scaling checks. Continue the remaining roadmap only after these regressions are green.

No amount of testing establishes that future issues cannot occur. A release needs stated
acceptance criteria and measured platform coverage, not a blanket "all fixed" claim.

## Portable distribution follow-up — 2026-10-22

Added a Windows x64 portable-preview workflow using the exact triggering SHA, approved fonts,
existing verified OCR model fetching, static CRT, portable marker, build identity, checksum and
prerelease downloads after successful gates. The existing packaging script now supports
`-PortableOnly` without WiX/MSI. Upstream trademark artwork and the screenshot containing it
were removed for this modified fork; open-source app icon and required notices retained.

Formatting and the 20 production-policy tests pass again; the production asset checker/renderer
also pass an isolated harness, not the full Cargo gate. Full app tests still stop before
compilation on registry `config.json` TLS EOF. No commit, push, Windows build, workflow run or
executable publication has occurred. [Current build status](portable-windows-build.md).

### Draft review submission

At the user's explicit request on 2026-10-22, the accumulated changes are being committed
and pushed for a draft PR. Earlier no-commit/no-push statements describe the prior setup
review. Full application checks and Windows runtime verification remain outstanding; consult
the PR checks/Actions for the current build status. No new executable is certified by this submission.
