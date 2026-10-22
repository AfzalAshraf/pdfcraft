# Print quality and remaining printer failures

Status: 2026-10-21 — pure browser planner/CSS tests passed; full engine/UI and physical output remain unverified. See the [verification ledger](fix-status.md).

## Changes in this branch

- **Browser clarity:** ordinary pages request 300 DPI regardless of job length. Previously,
  101 sheets lowered the entire job to 200 DPI and 301 sheets lowered it to 150 DPI.
- **Honest browser limits:** DPI planning accounts for both the renderer's pixel budget and
  its maximum side. A sheet that cannot fit even at 72 DPI is refused with instructions to
  save the print-ready PDF, rather than silently rendering below that floor. The existing
  sheet-count and encoded-job-byte limits remain; high-detail long jobs may reach the byte
  limit earlier and need the PDF fallback.
- **Browser colours:** the print iframe requests `print-color-adjust: exact`, including the
  WebKit-prefixed property. This asks the browser not to economize colours. User preferences,
  printer-driver settings and physical colour management can still override the request.
- **Print as image geometry:** every raster is placed on its original displayed page size,
  independent of renderer caps and pixel rounding. Previously the smallest effective DPI of
  any page was used to size all images. That enlarged smaller pages and could cause another
  fit-to-paper reduction. The change does not sharpen a low-resolution source image or
  remove the renderer's safety caps. Displayed sides over 200 inches are refused in image mode
  with a vector-printing recommendation, rather than silently resized or emitted outside the
  default-unit PDF page limits.
- **Invalid image-print DPI:** the engine rejects non-finite or out-of-range values instead
  of clamping them silently. Supported input remains 72–600 DPI.

Normal desktop printing still uses the existing vector-PDF path by default. These changes do
not apply contrast enhancement, replace black with a darker colour, or change the source PDF.

## Destination and discovery safety follow-up

The Print dialog now keeps the selected queue or explicit Save as PDF choice across refreshes
and reopening. If that queue disappears, it remains named and submission is refused rather than
redirected. Both the button and `print_now` check the destination. Worker failures request a
repaint and are shown separately from an empty successful discovery; thread creation is fallible
and repeated opens reuse the one pending query. This does not interrupt a hung OS driver call or
recover driver diagnostics swallowed inside the shared discovery API.

The actual destination/discovery policies pass 11 standalone tests; five Hand-pan geometry tests
pass too. Removed-safeguard mutations are detected. Full UI/control tests were added but cannot
run until normal registry dependencies are available. These safety fixes do **not** prove that
#3810, Canon merging, Kyocera fading or any physical print-quality defect is resolved. Hand-pan
cursor/focus/tab/Print lifecycle fixes are likewise awaiting real-window acceptance. See the
[ledger](fix-status.md#repair-milestones-1-and-2--implementation-follow-up) for exact boundaries.

## Verification still required

A Rust compiler and Cargo are now available. Three tests of the actual pure browser planning
and HTML functions passed in a standalone extracted-source harness; the same tests fail against
the old planner/CSS. Formatting passes. Normal workspace tests cannot fetch crates.io in this
sandbox, so the full engine rasterization, UI, browser lifecycle and automation tests remain
unexecuted. Independent arithmetic checks reproduce the old mixed-page sizing defect; they do
not replace the engine save/reopen regression or physical printer checks.

Run with a compatible Rust toolchain and the repository's native build prerequisites:

```sh
cargo fmt --check
cargo test -p pdfcraft-engine -j 2 print_raster
cargo test -p pdfcraft-ui-egui -j 2 web_print
cargo clippy -p pdfcraft-engine -p pdfcraft-ui-egui -j 2 --all-targets -- -D warnings
```

Also run the existing `doc_print_as_image` automation regression and repository quality gates
before merging. Manually compare vector and image printing at 150/300/600 DPI, including fine
black lines, small text, colour swatches, transparency, rotated pages and mixed page sizes.
Check browser jobs around 100/101 and 300/301 sheets; compare identical sheet content and verify
that the requested resolution no longer changes at those boundaries. Test the browser's PDF
fallback and byte-limit refusal as well as successful jobs.

## Not fixed: Canon PrintTicket merge failure

[Upstream issue #3823](https://github.com/storytold/pdfcraft/issues/3823) describes a Windows
Canon imagePROGRAF PRO-4000S failure before a job reaches the printer queue. Native ticket
construction and merging live in the pinned `craft-print` dependency in `storytold/craft-libs`,
not this application's raster composer. No dependency update or unverified driver fallback has
been applied in this branch.

Next steps are to capture the driver's PrintCapabilities/default PrintTicket and the underlying
merge error, add a synthetic shared-library regression, then validate on both Canon queues.
Do not bypass ticket validation: that can silently change paper size or drawing scale. A fix
needs Windows runtime coverage and physical calibration before claiming support for this driver.

The other reports remain separate work items. The [fix ledger](fix-status.md) records the
Windows icon and Linux held-scroll candidates and every issue from the conversation; neither
they nor the broader open-issue backlog are marked fully resolved by the printing changes.
