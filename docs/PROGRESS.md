# Book Progress Tracker

> **Purpose**: This file tracks writing progress and can be pasted into a new chat to continue writing.

---

## Book Metadata

- **Title**: Building Desktop Apps with Rust and Tauri
- **Subtitle**: From Zero to System Tray
- **Focus**: Rust + Tauri 2.0 desktop application development
- **Example App**: Tea (system tray sleep prevention utility)

---

## Current Status

**Last Updated**: 2026-01-31

**Overall Progress**: 22 of 22 chapters complete (100%) ✅

### Chapters Completed

| Chapter | File | Summary |
|---------|------|---------|
| README | `docs/README.md` | Book introduction, title selection, learning paths, conventions |
| Part 0 | `chapters/00-part-0-the-map.md` | Architecture overview, module map, data flow diagram, platform differences |
| Ch 1 | `chapters/01-how-to-use-this-book.md` | Learning paths, conventions, file reading order, prerequisites |
| Ch 2 | `chapters/02-rust-fundamentals.md` | Variables, types, functions, structs, enums, pattern matching, ownership basics |
| Ch 3 | `chapters/03-setting-up-your-environment.md` | Rust/Node.js/platform tools installation, running Tea, troubleshooting |
| Ch 4 | `chapters/04-ownership-and-borrowing.md` | Move/Copy/Clone, borrowing rules, Arc, AtomicBool, Mutex, thread-safe state |
| Ch 5 | `chapters/05-structs-enums-patterns.md` | Structs, tuple structs, newtype, enums with data, match, if let, derive macros |
| Ch 6 | `chapters/06-error-handling.md` | Result, ?, map_err, custom errors, Display/Error traits, defensive defaults, logging |
| Ch 7 | `chapters/07-tauri-architecture.md` | Tauri vs Electron, tray-first architecture, builder pattern, lifecycle, IPC, capabilities |
| Ch 8 | `chapters/08-commands-and-state.md` | #[tauri::command], State<T> injection, _impl pattern, business logic separation |
| Ch 9 | `chapters/09-plugins-and-permissions.md` | Tauri plugins, autostart plugin, capabilities, ManagerExt trait, permissions |
| Ch 10 | `chapters/10-configuration-capabilities.md` | tauri.conf.json, bundle, CSP, capabilities deep dive, schemas |
| Ch 11 | `chapters/11-system-tray.md` | TrayIconBuilder, MenuBuilder, menu events, dynamic updates, Arc for shared items |
| Ch 12 | `chapters/12-menus-and-icons.md` | include_bytes!, image crate, RGBA conversion, bundled vs embedded resources |
| Ch 13 | `chapters/13-wake-service.md` | Background tasks, tokio::spawn, F15 simulation, enigo crate, DisplayControl trait |
| Ch 14 | `chapters/14-platform-specific.md` | cfg attributes, conditional compilation, traits for abstraction, windows crate |
| Ch 15 | `chapters/15-persistence.md` | Serde JSON, platform paths, defensive defaults, write-through pattern |
| Ch 16 | `chapters/16-autostart.md` | tauri-plugin-autostart, ManagerExt, platform mechanisms, path refresh |
| Ch 17 | `chapters/17-testing.md` | Vitest, jsdom, mocking Tauri, test patterns, documentation tests |
| Ch 18 | `chapters/18-building.md` | Vite/Cargo build, release profile, bundle config, platform outputs |
| Ch 19 | `chapters/19-distribution.md` | GitHub Releases, versioning, code signing, auto-updates, package managers |
| App A | `appendices/a-command-reference.md` | npm, cargo, tauri CLI, git, platform-specific commands |
| App B | `appendices/b-data-models.md` | All Rust types, file formats, configuration structures |
| App C | `appendices/c-glossary.md` | Term definitions A-Z |

### Book Complete! 🎉

All 22 chapters plus 3 appendices have been written.

---

## Repository Facts (Grounded)

### Tauri Commands

Located in `src-tauri/src/commands.rs`:

| Command | Parameters | Returns | Purpose |
|---------|-----------|---------|---------|
| `toggle_sleep` | none | `(bool, String)` | Toggle awake state |
| `change_screen_mode` | `new_mode: String` | `String` | Set screen mode |
| `get_state` | none | `(bool, String)` | Get current state |

### Key Data Models

| Model | Location | Purpose |
|-------|----------|---------|
| `AppState` | `persistence.rs` | Persisted preferences (JSON) |
| `ScreenMode` | `core/screen_mode.rs` | Display behavior enum |
| `AppStateManager` | `commands.rs` | Runtime state (Arc/Mutex) |
| `WakeService` | `wake_service.rs` | Background F15 simulation |
| `AppError` | `error.rs` | Structured errors with hints |
| `TooltipText` | `core/tooltip.rs` | Type-safe tooltip strings |

### Persistence

- **Format**: JSON
- **Fields**: `sleep_disabled: bool`, `screen_mode: ScreenMode`
- **Locations**:
  - Windows: `%APPDATA%/com.tea.app/state.json`
  - macOS: `~/Library/Application Support/com.tea.app/state.json`
  - Linux: `~/.config/com.tea.app/state.json`

### Platform Code

- **Trait**: `DisplayControl` in `platform.rs`
- **Windows**: Uses `SetThreadExecutionState` API
- **Other**: F15 simulation only (no display control)

### Menu Structure

| ID | Text | Handler |
|----|------|---------|
| `toggle` | Enable/Disable Sleep | `handle_toggle_sleep()` |
| `screen_on` | Keep Screen On | `handle_screen_mode_change()` |
| `screen_off` | Allow Screen Off | `handle_screen_mode_change()` |
| `toggle_autostart` | Start at Login | `handle_toggle_autostart()` |
| `quit` | Quit | `app.exit(0)` |

### Dependencies

**Rust** (Cargo.toml):
- tauri 2, tauri-plugin-autostart 2
- tokio 1, enigo 0.3.0, image 0.24
- serde/serde_json, log/env_logger
- windows 0.58 (Windows only)

**npm** (package.json):
- @tauri-apps/api ^2, @tauri-apps/plugin-shell ^2
- vite ^7.2.6, vitest ^4.0.15, typescript ^5.2.2

---

## Glossary

| Term | Definition |
|------|------------|
| Arc | Atomic Reference Counted pointer for thread-safe sharing |
| Mutex | Mutual exclusion lock for thread-safe mutable access |
| AtomicBool | Thread-safe boolean that can be read/written atomically |
| Tauri Command | Rust function callable from frontend via IPC |
| System Tray | OS notification area icon with menu |
| F15 | Keyboard key that prevents system idle detection |

---

## Style Rules

### Voice
- Second person ("you will learn")
- Present tense for explanations
- Direct and practical

### Analogies
- Ownership = "key to a room"
- Borrowing = "visitor pass"
- Mutex = "bathroom key at a gas station"
- Arc = "shared access card"

### Diagrams
- Use Mermaid exclusively
- flowchart for architecture/data flow
- sequenceDiagram for interactions
- classDiagram for data models
- stateDiagram for state machines

### Code Excerpts
- Always include file path as comment
- Max 25 lines per excerpt
- Annotate complex lines with `// <- explanation`

---

## Open Questions

*None currently. All code paths verified in repository.*

---

## Next Chapter Request

To continue writing this book, paste this file into a new chat and request:

> "Continue writing the book. The next chapter is **Chapter 12: Icons and Resources**. It should cover:
> - How Tea loads and manages icons
> - The icon.rs module in detail
> - Bundled resources vs embedded resources
> - Image crate usage for icon manipulation
> - Platform icon requirements and formats
> - Dynamic icon switching based on state
>
> Use real-life analogies and ground all explanations in Tea's actual code."

---

## Change Log

| Date | Chapters | Notes |
|------|----------|-------|
| 2026-01-31 | README, SUMMARY, 00, 01, 02, PROGRESS | Initial creation |
| 2026-01-31 | 03 | Environment setup chapter complete |
| 2026-01-31 | 04 | Ownership and borrowing with analogies (library book, bathroom key, key cards) |
| 2026-01-31 | 05 | Structs, enums, pattern matching, derive macros (shipping box, mail sorting machine) |
| 2026-01-31 | 06 | Error handling: Result, ?, custom errors (package delivery, doctor's report, hotel concierge) |
| 2026-01-31 | 07 | Tauri architecture: tray-first design, builder pattern, lifecycle (car assembly, receptionist) |
| 2026-01-31 | 08 | Commands and state: #[tauri::command], State<T>, _impl pattern (restaurant waiter, hotel front desk) |
| 2026-01-31 | 09 | Plugins and permissions: tauri-plugin-autostart, capabilities, ManagerExt (appliances, security badges) |
| 2026-01-31 | 10 | Configuration: tauri.conf.json, bundle, CSP, capabilities (blueprints, office key cards) |
| 2026-01-31 | 11 | System tray: TrayIconBuilder, MenuBuilder, events, Arc (doorbell camera, switchboard) |
