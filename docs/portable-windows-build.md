# Portable Windows preview builds

**Status — 2026-10-22:** submitted for draft pull-request review at the user's request;
Windows build and release publication remain unverified. Check the PR checks and Actions run
for the current build status.
There is **no new executable download yet**. Full application checks are blocked locally by
Cargo registry access (TLS EOF retrieving `config.json`). Do not mistake an upstream release
or an older installed version for a build containing these branch-local fixes.

## For someone who only wants to run the app

After the first successful build, open this fork's [Releases](https://github.com/AfzalAshraf/pdfcraft/releases)
and choose a **Windows portable preview**, checking its commit/build date. Download its
`pdfcraft-<version>-windows-x64-portable-<commit>.zip`, not GitHub's source-code ZIP.

1. Close the older running PdfCraft.
2. Right-click the downloaded ZIP and choose **Extract All** into a writable folder.
3. Open the extracted application folder and double-click **pdfcraft.exe**.

No Rust, PowerShell, installation or administrator access is needed to **run** it. This is a
Windows x64 build, not ARM64 or 32-bit. Do not run the executable from inside the ZIP and do
not move it away from `portable.txt`, `models/` and the licence files. It does not replace an
installed copy. Portable settings, logs, recovery files and new digital IDs use the adjacent
`PdfCraftData/` folder when writable; the app reports a fallback to per-user folders if not.

These are **unsigned preview builds**, not official upstream releases. Windows may warn;
do not disable security software. Test with copies of important PDFs. Successful CI is not
proof of smoothness, correct physical-printer output or absence of bugs. In particular,
Kyocera/Canon acceptance is still pending. See [the verification ledger](fix-status.md).

`BUILD-INFO.json` inside the ZIP records the commit, version, build label, target and font
revision. The About version includes `portable-preview-<commit>`. A `.sha256` file beside the
download provides integrity verification; it does not replace Authenticode signing.

## How the flow works

Source: [windows-portable.yml](../.github/workflows/windows-portable.yml).

- Starts on pushes to this session's branch (`arena/7534c8f4-pdfcraft`) or `main`, or manually
  through `workflow_dispatch`. A manual workflow becomes visible in GitHub's Actions menu
  after the file is on the default branch; its initial branch push can run without that.
- Checks out **the triggering SHA**, not whichever revision `main` happens to contain later.
  Local uncommitted edits cannot be included in a GitHub Actions build.
- Uses Windows/MSVC, statically linked CRT, enforced icon/version resources and the existing
  approved `craft-fonts` revision `3d9ab3a132ed0eff457e424226a435a901da30d3`.
- Checks native PowerShell syntax, formatting, workspace Clippy/tests, dependency layering,
  asset policy and WASM compatibility before packaging. Existing CI remains unchanged.
- Calls the existing packaging script with `-PortableOnly`: no WiX, MSI or signing secrets
  are required. It downloads SHA-256-verified OCR models using the existing xtask command,
  carries the licence notices and includes the portable marker.
- Checks executable PE architecture/subsystems, extracts the ZIP, checks required files and
  the embedded commit identity, and runs the extracted CLI's `--version`. This is **not** a
  GUI smoke test or a physical-printer test.
- Saves a 30-day Actions artifact with the ZIP and checksum. A separate, least-privilege job
  publishes a uniquely tagged **prerelease** with direct downloads in this fork only; it does
  not mark it as the latest stable release or overwrite an existing release.
- A release-publication failure leaves the already-uploaded Actions artifact available to a
  signed-in user. Failed build/check/package steps produce no distributable artifact.

## Activation and remaining verification

The full local checks remain blocked by registry dependency access. At the user's explicit
request, this branch is being submitted as a **draft PR**, not as a verified release or a
merge-ready change. The commit and push allow GitHub CI to attempt the application gates;
no local test or build success is implied. Review the resulting runs before using a preview.
Do not build/publish the old `main` as a substitute for the current branch.

At the initial setup review, the GitHub Actions workflows API reported no registered
workflows; that alone did not prove Actions was disabled. Reading the repository's Actions
permission settings returned HTTP 403, so this session could not confirm or change that setting.

If Actions is disabled for the fork, a repository administrator must
first enable it. Release publication needs the workflow token's `contents: write` permission;
the build job itself uses read-only repository access. No private shared workflows or custom
GitHub secrets are needed.

### Checks completed here

- Rust formatting and all 20 standalone production-policy tests passed again.
- Production asset renderer/check functions passed in an isolated harness (263 repository
  assets); this is not the complete Cargo xtask/CI gate. Restricted upstream trademark artwork
  and the home-screen screenshot containing it were removed; the existing open-source app
  icon is used instead. Copyright notices and trademark licence text are preserved.
- Workflow YAML/structure, pinned action revisions and whitespace checked locally.
- Native Windows PowerShell execution, Windows application compilation, GUI startup and
  end-to-end release publication remain unverified until the first real Windows run.

## Revision history

- 2026-10-22: Added standalone portable-preview flow and portable-only packaging path; no
  executable published and no repair milestone promoted to complete.

- 2026-10-22: User requested a pull request; submitting as draft with full application and
  Windows verification explicitly outstanding. Previous no-push status describes the setup review.
