# Local Data Manager

Local Data Manager is a desktop-first portfolio application built with Tauri v2, Svelte 5, TailwindCSS, and Rust. It started as a UI-forward prototype for handling sensitive local operational data and has evolved into a local-first desktop workspace with secure auth, encrypted local persistence, customer sync flows, and a loopback integration API.

This repository is part of a broader ecosystem that also includes:

- a consulting-facing Avtaja web platform
- a mobile-connected local agent layer
- a larger compliance-oriented backend agent

Within that ecosystem, this repository is the desktop operator surface.

## What It Does

The current app includes:

- operator setup and unlock flow
- Argon2-hashed master password storage
- one-time recovery key generation and password reset flow
- encrypted external API token storage
- SQLCipher-backed local SQLite persistence
- local customer cache and cloud sync command flow
- polished multi-view desktop dashboard UI
- loopback-only local API for other apps on the same machine
- Windows and macOS packaging workflow

## Tech Stack

- Tauri v2
- Svelte 5 + TypeScript
- Vite
- TailwindCSS
- Rust
- `rusqlite` with bundled SQLCipher
- `reqwest`
- `axum`

## Local Development

Install dependencies:

```bash
npm install
```

Start the app:

```bash
npm run tauri dev
```

On Windows PowerShell, use `npm.cmd` if execution policy blocks `npm`:

```powershell
npm.cmd install
npm.cmd run tauri dev
```

### Windows Tooling Note

Because this project uses bundled SQLCipher on Windows, the Rust build may require:

- `clang`
- `perl`
- Developer PowerShell for Visual Studio

If standard PowerShell cannot build the Rust side, use `Developer PowerShell for Visual Studio`.

## Runtime Environment

The app expects these runtime values during development:

- `LDM_TOKEN_CIPHER_KEY`
  - base64 text that decodes to exactly 32 bytes
  - used to encrypt and decrypt the stored external API token
- `LDM_SQLCIPHER_KEY`
  - passphrase used by SQLCipher for the local database
- `LDM_LOCAL_API_PORT`
  - optional
  - defaults to `18432`

The local integration API key is no longer provided by environment variable. It is generated and revoked from inside the app and stored as a hash in the local database.

### PowerShell Example

A starter file is included at [run-local.example.ps1](C:\Users\subsu\Documents\Mushaffar-Portfolio\run-local.example.ps1).

Copy the values you want to keep locally, then run:

```powershell
.\run-local.example.ps1
```

## Secure Workspace Features

### Operator Auth

- operator account stored locally
- master password hashed with Argon2
- unlock flow handled through Tauri IPC

### Recovery Flow

- one-time recovery key generated during first setup
- recovery key shown once
- password reset available from `Forgot password?`
- if both password and recovery key are lost, the current fallback is a destructive local reset

### Local Storage

- local SQLite database at runtime
- SQLCipher enabled through `rusqlite`
- external CRM/API token encrypted before persistence

## Local API

The app exposes a loopback-only HTTP API on:

- `127.0.0.1`
- default port: `18432`

Routes:

- `GET /health`
- `GET /v1/operator`
- `GET /v1/customers`
- `POST /v1/customers/sync`

### Local API Security Model

- loopback-only binding
- bearer-token protected
- integration key generated from the Settings screen
- only the hashed form of the key is stored locally
- full key is shown once after generation

### Local API Usage

Generate or rotate the key from the Settings screen, then use the built-in curl examples or commands like:

```powershell
curl.exe -H "Authorization: Bearer YOUR_GENERATED_KEY" http://127.0.0.1:18432/v1/customers
```

```powershell
curl.exe -X POST -H "Authorization: Bearer YOUR_GENERATED_KEY" http://127.0.0.1:18432/v1/customers/sync
```

## Packaging

### Generate App Icons

Use the source icon at:

- `src-tauri/icons/app-icon.png`

Generate platform icons:

```bash
npm run tauri icon ./src-tauri/icons/app-icon.png
```

On Windows PowerShell:

```powershell
npm.cmd run tauri icon .\src-tauri\icons\app-icon.png
```

### Windows Installers

Targets:

- `aarch64-pc-windows-msvc`
- `x86_64-pc-windows-msvc`
- `i686-pc-windows-msvc`

Install targets:

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

Build x86:

```powershell
npm.cmd run tauri build -- --target i686-pc-windows-msvc --bundles nsis
```

Output:

- `src-tauri/target/<target-triple>/release/bundle/nsis/`

### macOS DMG

Build on a Mac:

```bash
npm install
npm run tauri icon ./src-tauri/icons/app-icon.png
npm run tauri build -- --bundles dmg
```

Output:

- `src-tauri/target/release/bundle/dmg/`

## GitHub Releases

This repo includes:

- [.github/workflows/release.yml](C:\Users\subsu\Documents\Mushaffar-Portfolio\.github\workflows\release.yml)

It builds:

- Windows x64 NSIS installers
- macOS DMG bundles

Trigger it by pushing a version tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

If GitHub Actions cannot create the release, enable:

- `Repository Settings -> Actions -> General -> Workflow permissions -> Read and write permissions`

## Release Checklist

Before publishing this repo or shipping a demo build:

1. Verify `npm.cmd run tauri dev` starts cleanly from your local setup script.
2. Test operator setup, unlock, recovery, and sync.
3. Generate a local API key and confirm:
   - `/health` works
   - `/v1/operator` works
   - `/v1/customers` works
   - revoking the key invalidates old requests
4. Regenerate icons if branding changed.
5. Build the installer or DMG you want to distribute.
6. Test the packaged app, not just `tauri dev`.

## Current Status

This repo is no longer just a UI prototype. It is now a local-first desktop portfolio piece with real secure application flows and a small integration surface. It is still demo-scoped in a few important ways:

- the external CRM service is still mock-friendly
- there is no multi-operator role system yet
- local API clients do not yet have scoped permissions
- public distribution still needs signing and notarization work

## Natural Next Steps

- scoped integration clients for the local API
- create/edit customer forms
- stronger audit logging
- signed installers
- tighter ecosystem integration with the other Avtaja/compliance repos
