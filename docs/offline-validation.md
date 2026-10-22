# Validation when the Cargo registry is inaccessible

Status: 2026-10-21 — a registry-independent policy test entry point is available. **The full
application is not built or certified offline in this sandbox.** No dependency bundle has been
created here. Instructions below distinguish an executable fallback check from a full offline build.

## What failed, and why rewriting the app does not repair it

The normal locked UI test attempt failed while fetching crates.io `config.json`, with a TLS
connection error. Offline Cargo resolution also failed because `thiserror` is not cached. This
happens before the app's Rust code compiles. It is not evidence that every source file is defective
or that the PDF engine is missing. The existing engine, rendering, editing and native UI depend
on packages recorded in `Cargo.lock`.

Do not fix this by deleting the lockfile, removing real features, disabling TLS/checksums,
writing fake third-party packages, editing the Cargo cache, ignoring failing tests or replacing
the full application with a small dependency-free demo. A complete dependency-free engine and
cross-platform UI would be a separate, substantial implementation project with its own acceptance
criteria; it would not establish that all existing behaviour survived.

## 1. Run the registry-independent production-policy tests now

Requires only a compatible installed `rustc` and native linker. It does **not** invoke Cargo,
look up a package registry or download anything. Run from the repository root:

```sh
rustc --edition 2024 --test tools/offline-tests.rs -o /tmp/pdfcraft-offline-tests
/tmp/pdfcraft-offline-tests
```

For Windows PowerShell, use a native temporary executable path:

```powershell
rustc --edition 2024 --test tools/offline-tests.rs -o "$env:TEMP\pdfcraft-offline-tests.exe"
& "$env:TEMP\pdfcraft-offline-tests.exe"
if ($LASTEXITCODE -ne 0) { throw 'Offline policy tests failed' }
```

This sandbox's cached toolchain needs an explicit available linker:

```sh
rustc --edition 2024 --test tools/offline-tests.rs \
  -C linker=cc -C link-arg=-fuse-ld=bfd \
  -o /tmp/pdfcraft-offline-tests
/tmp/pdfcraft-offline-tests
```

The entry point includes the **actual production modules**, not rewritten approximations:

- Hand-pan geometry: finite coordinates, bounds, both axes and no residual movement.
- Print destinations: sticky queue/file choice and missing/pending destination guards.
- Printer-discovery worker: non-blocking start, failure wake-up, empty success and dropped receiver.
- Middle-button gesture: held-versus-latched release and excursion tracking.

The current set contains **20 tests**. Read the test summary and require nonzero tests and exit
status zero. Optional standalone Clippy, when installed, can use the repository's lint config:

```sh
CLIPPY_CONF_DIR="$PWD" clippy-driver --edition 2024 --test tools/offline-tests.rs \
  -D warnings -o /tmp/pdfcraft-offline-clippy-tests
/tmp/pdfcraft-offline-clippy-tests
```

Add the same explicit linker flags if this toolchain needs them. The separate CI policy job runs
these checks without Cargo dependencies; its existence is not evidence a remote run has passed.

**Not covered:** compiling the app, egui widget ownership/cursors, actual windows, browser APIs,
PDF parse/edit/save fidelity, automation/control integration, Windows driver APIs, rendering quality,
printing on paper, full dependency lints, assets or WASM. Retain the normal CI jobs and full tests.

## 2. Obtain complete, verified dependencies for a real offline application build

Use an authorized machine with registry and pinned Git dependency access. Use this exact source
revision and working patchset, not an unrelated checkout. No credentials belong in the repository,
source bundle or logs. Install the repository's supported Rust toolchain and the native prerequisites
for the intended target. Read `.github/workflows/ci.yml` and the release/platform workflows.

A source-vendor bundle is one supported Cargo approach. Choose a fresh external bundle directory,
**not the repository's existing `vendor/` directory**, which contains maintained renderer patches.
The following is a procedure to validate on the connected machine, not a claim it ran here:

```sh
# BUNDLE must be an absolute path to a new external directory you own.
# Run at the repository root. Retain Cargo.lock and the current .cargo/config.toml.
: "${BUNDLE:?Set BUNDLE to an absolute external directory}"
mkdir -p "$BUNDLE"
cargo vendor --locked --versioned-dirs "$BUNDLE/crates" > "$BUNDLE/sources.toml"

# Prove that the generated source replacement works with networking disabled.
cargo test --frozen --config "$BUNDLE/sources.toml" \
  -p pdfcraft-ui-egui -j 2 --test view --test print --test control
```

Use the source-replacement configuration emitted by Cargo; do not invent checksums or replace
only crates.io while overlooking pinned Git dependencies. Preserve existing path patches and aliases.
Review the generated configuration, confirm its source paths match the bundle's installed location,
and retain package licence/checksum files. If Cargo cannot vendor a dependency, resolve that error
on the connected host instead of silently omitting it.

Transfer the bundle through an approved integrity-verified channel, record a manifest of its files
and hashes and validate that manifest after transfer. Source bundles generally can be shared across
targets, but Rust toolchains, linkers, SDKs, system libraries, optional inputs and build-script tools
must be installed for each target. Never assume Linux build artifacts are a Windows application.

Use `--frozen` on the disconnected machine to prevent lockfile changes and network resolution.
Cargo's generated checksum verification must remain enabled. Optional craft-fonts inputs must
follow the repository's pinned attribution/build-input policy; without them, don't claim that
Japanese/Chinese font coverage was validated. Also provision required independent oracle tools;
an absent oracle is skipped/blocked, not an independent-reader pass.

Large source bundles, Cargo caches, target artifacts, font files and executable outputs must stay
outside Git. No new vendored third-party product dependency is introduced by this document.

## 3. Run the real gates after dependencies are available

Use bounded scoped commands for local work and normal full-workspace CI for the sweep:

```sh
cargo fmt --all -- --check
cargo test --frozen --config "$BUNDLE/sources.toml" \
  -p pdfcraft-engine -p pdfcraft-automation -p pdfcraft-ui-egui -j 2
cargo clippy --frozen --config "$BUNDLE/sources.toml" \
  -p pdfcraft-engine -p pdfcraft-automation -p pdfcraft-ui-egui \
  -j 2 --all-targets -- -D warnings
cargo run --frozen --config "$BUNDLE/sources.toml" -p xtask -- layers
cargo run --frozen --config "$BUNDLE/sources.toml" -p xtask -- assets
cargo run --frozen --config "$BUNDLE/sources.toml" -p xtask -- wasm
```

Install the required WASM target and its build prerequisites before disconnected execution.
Check actual command/tool support against the current checkout. Run additional affected crate
and integration targets, inspect real UI/control output and obtain Windows/physical-printer
acceptance independently. A source bundle fixes dependency availability, not application bugs,
OS-driver incompatibility or hardware conditions.

## Reporting contract

Report separately: policy tests; full application compilation; crate/integration tests; platform
runtime tests; browser tests; physical output; and unexecuted gates. Never turn an unavailable
registry into "all tests pass" or a policy-only check into a production-ready application build.
See [fix-status.md](fix-status.md) for the current execution evidence and unresolved reports.
