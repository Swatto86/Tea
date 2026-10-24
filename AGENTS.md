# Tea

Tray-only keep-awake utility. Its tray menu toggles sleep prevention, picks a screen mode and turns
start-at-login on or off. Tauri 2 with no visible window: Rust in `src-tauri/`, a near-empty Vite +
TypeScript page, Vitest. The code builds for Windows, macOS and Linux, but releases ship Windows
downloads only (portable exe and NSIS installer). Swatto runs a source build on macOS. Public
repository `Swatto86/Tea`, MIT, single branch `main`.

## Build, test, verify

- `npm install`, then `npm run tauri dev` and `npm run tauri build`. The README and `BUILD.md` use
  `cargo tauri dev` / `cargo tauri build`, which need the cargo Tauri CLI. Bundles go under
  `src-tauri/target/release/bundle/`; Linux build prerequisites are in `BUILD.md`.
- Tests: `npm run test:run` (Vitest; `npm test` watches) and `cargo test` in `src-tauri`. One
  wake-service test is `#[ignore]` because it simulates input.
- `src-tauri/src/main.rs` has `#![deny(warnings)]`, so any Rust warning fails the build.
- No verify script and no CI test job. The Vitest suites build their own DOM fixtures and document
  autostart; they never touch the Rust side. The Rust tests and a manual tray check are the real
  verification.
- Release: keep the version equal in `package.json`, `src-tauri/Cargo.toml` and
  `src-tauri/tauri.conf.json`. A pushed `v*` tag runs `release.yml` on windows-latest (`npm ci`,
  `npm run tauri build`) and publishes `Tea_<tag version>_x64-portable.exe` and the NSIS
  `*setup.exe`. Nothing is built for macOS or Linux.

## Where things are

- `src-tauri/src/main.rs`: start-up, tray menu and handlers, autostart plugin. `commands.rs`:
  `toggle_sleep`, `change_screen_mode`, `get_state`, `AppStateManager`, `start_wake_service`.
- `wake_service.rs`: the background loop. `platform.rs`: `DisplayControl`; the Windows
  implementation calls `SetThreadExecutionState`, the others do nothing. `persistence.rs`: the
  state file. `core/`: `ScreenMode`, `TooltipText`. `icon.rs`: the allow/block tray icons.

## Behaviour that must hold

- Windows "Keep Screen On": `ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED` plus an
  F15 press every 60 s. Windows "Allow Screen Off" (the default): `ES_CONTINUOUS |
  ES_SYSTEM_REQUIRED`, no F15. Turning it off restores plain `ES_CONTINUOUS`.
- macOS and Linux: an F15 press every 60 s through `enigo`. That keeps the display on as well, so
  the tray shows no screen-mode items there.
- State (`sleep_disabled`, `screen_mode`) is written on every change to
  `%LOCALAPPDATA%\tea\state.json` (Windows), `~/Library/Application Support/tea/state.json`
  (macOS) or `$XDG_CONFIG_HOME/tea/state.json`, else `~/.config/tea/state.json` (Linux). A missing
  or corrupt file falls back to defaults.
- Start at login uses `tauri-plugin-autostart` (HKCU Run key, LaunchAgent, `~/.config/autostart`).
  When it is on, start-up disables and re-enables it so the entry points at the running binary.
- No `unwrap` or `expect` in production paths (design note at the top of `main.rs`).

## Open questions

- How the macOS source build is installed and kept current is not recorded in the repo, nor
  whether it needs the Accessibility permission for `enigo`'s F15 events (an `enigo` start-up
  failure surfaces as `AppError::InputSimulation`).
- The README says "Download the latest release for your platform"; only Windows assets exist.

## Where longer material lives

- `README.md`: features, how it works, state and autostart locations. `BUILD.md`: per-platform
  prerequisites.
- `CHANGELOG.md`: one past change set (state file move, test rewrite); history, not rules.
- `docs/`: the "Building Desktop Apps with Rust and Tauri" book built on this code
  (`docs/SUMMARY.md`); `docs/PROGRESS.md` tracks the book.
- There is no `ARCHITECTURE.md` or `CONTEXT.md` yet.
