# Local Data Manager

Local Data Manager is a desktop-first portfolio application built with Tauri v2, Svelte 5, TailwindCSS, and Rust. It was created as part of a broader product ecosystem and grew out of a very practical need: managing sensitive local operational data, including credentials and environment-level assets, in a cleaner and more intentional way.

This repository currently focuses on the desktop experience, interaction design, and local-first workflow. It is intentionally scoped as a polished UI prototype with mock data and basic local state management. It does not yet implement real encryption, remote sync, or production-grade secrets handling.

## Why This Exists

This app sits alongside a wider ecosystem of related projects:

- a consulting-facing web platform for the Avtaja brand
- a mobile-connected agent layer used to bridge into local machine workflows
- a larger compliance-oriented backend agent responsible for heavier automation and policy logic

Within that ecosystem, this repository represents the desktop surface: a local operator dashboard for reviewing records, navigating activity, and shaping the UX of a future local data and credential management workflow.

In short, this is the desktop-facing interface for a bigger vision.

## Current Scope

This project is currently optimized as a portfolio-quality desktop prototype with:

- a mock login flow
- a local dashboard with multiple views
- analytics and audit-style presentation panels
- record inspection and pagination
- packaging for Windows and macOS distribution

It intentionally does not include:

- real encryption
- cloud key management
- production authentication
- external API integrations

## Product Positioning

The easiest way to think about this repository is:

- web app: public and consulting-facing surface
- desktop app: local operator workspace
- mobile agent: bridge into local device workflows
- compliance agent: backend automation and policy engine

That makes this repo a useful standalone portfolio piece, but it is even stronger when presented as one component in a connected multi-surface system.

## Tech Stack

- Tauri v2
- Svelte 5 + TypeScript
- Vite
- TailwindCSS
- Rust + rusqlite

## Local Development

Install dependencies:

```bash
npm install
```

Run the desktop app in development:

```bash
npm run tauri dev
```

On Windows PowerShell, use `npm.cmd` if script execution blocks `npm`:

```powershell
npm.cmd install
npm.cmd run tauri dev
```

## Project Highlights

- Dark-mode desktop UI with a polished dashboard layout
- Sidebar-driven multi-view navigation
- Searchable records grid with pagination
- Detail drawer and mock audit timeline
- Analytics section designed for presentation and product storytelling
- Local SQLite bootstrap on the Rust side
- Tauri IPC used for mock record generation

## Packaging

### Generate app icons

Use a square PNG or SVG source icon. The current project icon source is:

- `src-tauri/icons/app-icon.png`

Generate platform icons with:

```bash
npm run tauri icon ./src-tauri/icons/app-icon.png
```

On Windows PowerShell, use:

```powershell
npm.cmd run tauri icon .\src-tauri\icons\app-icon.png
```

### Windows installers

Tauri can build different Windows targets depending on the Rust target triple:

- `aarch64-pc-windows-msvc`: Windows ARM64
- `x86_64-pc-windows-msvc`: Windows x64
- `i686-pc-windows-msvc`: Windows x86 (32-bit)

Install the target you want first:

```powershell
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
rustup target add i686-pc-windows-msvc
```

Build ARM64:

```powershell
npm.cmd run tauri build -- --target aarch64-pc-windows-msvc --bundles nsis
```

Build x64:

```powershell
npm.cmd run tauri build -- --target x86_64-pc-windows-msvc --bundles nsis
```

Build x86 (32-bit):

```powershell
npm.cmd run tauri build -- --target i686-pc-windows-msvc --bundles nsis
```

Generated installers are placed under:

- `src-tauri/target/<target-triple>/release/bundle/nsis/`

Examples:

- `src-tauri/target/aarch64-pc-windows-msvc/release/bundle/nsis/`
- `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/`
- `src-tauri/target/i686-pc-windows-msvc/release/bundle/nsis/`

### macOS DMG

Build the DMG on a Mac. Tauri's DMG bundling is macOS-based.

1. Install Node.js, Rust, and Xcode Command Line Tools on the Mac.
2. Copy this project to the Mac.
3. Install dependencies:

```bash
npm install
```

4. Generate icons:

```bash
npm run tauri icon ./src-tauri/icons/app-icon.png
```

5. Build the DMG:

```bash
npm run tauri build -- --bundles dmg
```

The DMG output will be under:

- `src-tauri/target/release/bundle/dmg/`

### Notes

- Unsigned Windows installers may show SmartScreen warnings.
- Unsigned macOS apps may show Gatekeeper warnings.
- For public distribution, add code signing for Windows and code signing plus notarization for macOS.

## GitHub Releases

This repository now includes a GitHub Actions workflow at:

- `.github/workflows/release.yml`

It creates:

- Windows x64 NSIS installers
- macOS DMG bundles

### How to trigger it

You can trigger the workflow in either of these ways:

1. Run it manually from the GitHub Actions tab with `workflow_dispatch`.
2. Push a version tag such as:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow creates a draft GitHub Release and uploads the generated installers as release assets.

### Important GitHub setting

If GitHub Actions cannot create releases, go to:

- `Repository Settings -> Actions -> General -> Workflow permissions`

and enable:

- `Read and write permissions`

### Recommended release flow

1. Test locally with `npm run tauri dev`.
2. Generate icons with `tauri icon`.
3. Bump the app version in:
   - `src-tauri/tauri.conf.json`
   - `package.json` if you want the frontend package version to match
4. Commit your changes.
5. Push a tag like `v0.1.0`.
6. Let GitHub Actions build the installers.
7. Review the draft release and publish it.

## Roadmap Direction

Natural next steps for this repository would be:

- replacing mock data with real local records
- introducing secure credential storage flows
- connecting with the broader agent/compliance ecosystem
- adding signed production installers
- refining the desktop information architecture around real operator tasks

## Status

This repository is production-polished from a UI and packaging perspective, but still prototype-scoped from a security and systems-integration perspective.
