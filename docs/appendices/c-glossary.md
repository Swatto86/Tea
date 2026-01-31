# Appendix C: Glossary

> *Definitions of terms used throughout this book.*

---

## A

### AppImage
A portable application format for Linux that packages an application and its dependencies into a single executable file. No installation required—just make it executable and run.

### Arc (Atomic Reference Counting)
A Rust smart pointer that enables multiple ownership of data across threads. The "atomic" part means the reference count is updated safely in concurrent scenarios.

### Async/Await
A programming pattern for writing asynchronous code that looks synchronous. `async` marks a function as asynchronous; `await` pauses execution until a future completes.

### AtomicBool
A boolean value that can be safely shared and modified across threads without locks. Used for simple flags like "is running".

---

## B

### Bundle
The process of packaging an application with all its resources (icons, configuration, dependencies) into a distributable format.

### Borrowing
Rust's mechanism for temporarily accessing data without taking ownership. References (`&T` and `&mut T`) borrow data.

---

## C

### Capability
In Tauri 2.0, a security mechanism that grants specific permissions to application components. Defined in `capabilities/default.json`.

### Cargo
Rust's package manager and build system. Manages dependencies, compiles code, runs tests, and builds documentation.

### cfg Attribute
Rust's conditional compilation mechanism. `#[cfg(windows)]` compiles code only on Windows.

### Checksum
A hash value (like SHA256) used to verify file integrity. If the checksum matches, the file hasn't been corrupted or tampered with.

### CLI (Command Line Interface)
A text-based interface for interacting with software. Tauri CLI (`cargo tauri`) provides commands for development and building.

### Code Signing
Cryptographically signing executables to prove their origin and integrity. Required for trusted distribution on Windows and macOS.

### Crate
Rust's term for a package or library. Similar to npm packages in JavaScript.

---

## D

### deb (Debian Package)
A package format used by Debian-based Linux distributions (Ubuntu, Linux Mint). Installed with `dpkg` or `apt`.

### Derive Macro
A Rust feature that automatically implements traits for types. `#[derive(Debug, Clone)]` generates `Debug` and `Clone` implementations.

### Display (Trait)
A Rust trait for formatting values as user-facing strings. Implemented to enable `println!("{}", value)`.

### DMG (Disk Image)
A macOS disk image format used to distribute applications. Users open the DMG and drag the app to Applications.

### DOM (Document Object Model)
A programming interface for HTML documents. Represents the page structure as a tree of objects.

---

## E

### Enum (Enumeration)
A Rust type that can be one of several variants. `ScreenMode::KeepDisplay` is a variant of the `ScreenMode` enum.

### Extension Trait
A Rust pattern for adding methods to existing types. `ManagerExt` adds `autolaunch()` to Tauri's `AppHandle`.

---

## F

### F15 Key
A function key present on extended keyboards. Rarely used in modern applications, making it safe for simulated keystrokes.

### Frontend
The user-facing part of an application. In Tauri, this is web technology (HTML/CSS/JavaScript).

### Future
A Rust type representing a value that will be available later. The foundation of async programming.

---

## G

### Gatekeeper
macOS security feature that blocks unverified applications. Notarization satisfies Gatekeeper.

### Global State
Application data accessible from anywhere in the code. Must be handled carefully in multi-threaded contexts.

---

## H

### HMR (Hot Module Replacement)
A development feature that updates code in a running application without a full reload. Vite provides HMR.

---

## I

### ICO
Windows icon format that can contain multiple sizes in one file.

### ICNS
macOS icon format (Apple Icon Image) that contains multiple resolutions.

### include_bytes!
A Rust macro that embeds file contents into the binary at compile time. Used for icons.

### IPC (Inter-Process Communication)
Communication between separate processes. Tauri uses IPC between the Rust backend and web frontend.

### Invoke
In Tauri, calling a Rust function from the frontend. `invoke('command_name')` triggers the Rust handler.

---

## J

### jsdom
A JavaScript implementation of web standards for Node.js. Used in tests to simulate a browser environment.

---

## L

### LaunchAgent
macOS mechanism for starting applications at user login. Configured via plist files in `~/Library/LaunchAgents`.

### Lifetime
Rust's way of tracking how long references are valid. Prevents use-after-free bugs.

### LTO (Link Time Optimization)
Compiler optimization performed during linking. Produces smaller, faster binaries but increases build time.

---

## M

### Mock
A test double that simulates real functionality. Mocking `invoke()` lets tests run without Tauri runtime.

### MSI (Microsoft Installer)
Windows installer format with enterprise deployment support.

### Mutex (Mutual Exclusion)
A synchronization primitive that ensures only one thread accesses protected data at a time.

---

## N

### Notarization
Apple's process of verifying macOS applications for security. Required for distribution outside the App Store.

### NSIS (Nullsoft Scriptable Install System)
A Windows installer creator. Produces the familiar "wizard" style installers.

---

## O

### opt-level
Cargo setting controlling optimization level. `"z"` optimizes for binary size.

### Ownership
Rust's system of memory management. Each value has one owner; when the owner goes out of scope, the value is dropped.

---

## P

### Panic
Rust's response to unrecoverable errors. By default, unwinds the stack; `panic = "abort"` terminates immediately.

### Permission
In Tauri, authorization to perform specific actions. Granted through capabilities.

### plist (Property List)
macOS/iOS configuration file format (XML or binary). Used for LaunchAgents and app configuration.

### Plugin
Tauri extension that adds functionality. `tauri-plugin-autostart` adds auto-launch capabilities.

---

## R

### RGBA
Color format with Red, Green, Blue, and Alpha (transparency) channels. Used for tray icons.

### Registry
Windows system database for configuration. Autostart entries go in `HKCU\...\Run`.

### Release Profile
Cargo configuration for production builds. Enables optimizations and disables debug features.

### Result Type
Rust's way of representing operations that can fail. `Result<T, E>` is either `Ok(T)` or `Err(E)`.

### RPM (Red Hat Package Manager)
Package format for Red Hat-based Linux distributions (Fedora, CentOS).

---

## S

### Semantic Versioning
Version numbering scheme: MAJOR.MINOR.PATCH. Breaking changes increment MAJOR.

### Serde
Rust framework for **ser**ializing and **de**serializing data. Powers JSON, TOML, and other formats.

### SmartScreen
Windows security feature that warns about unrecognized downloads. Code signing helps avoid warnings.

### Snap
Universal Linux package format. Includes dependencies and runs in a sandbox.

### Static Dispatch
Rust's default method call resolution. The compiler determines which function to call at compile time. Zero runtime overhead.

### System Tray
OS feature showing application icons near the clock. Also called notification area (Windows) or menu bar (macOS).

---

## T

### Tauri
Framework for building desktop applications with web frontends and Rust backends. Alternative to Electron.

### Thread
Independent unit of execution within a process. Rust ensures thread safety at compile time.

### Tokio
Rust async runtime providing timers, I/O, and task scheduling. Tea uses it for the wake service.

### Trait
Rust's way of defining shared behavior. Similar to interfaces in other languages.

### Tray Icon
Small icon displayed in the system tray. Primary UI element for Tea.

---

## U

### Unwrap
Rust method that extracts the value from `Result`/`Option` or panics. Use cautiously in production code.

### URI (Uniform Resource Identifier)
String identifying a resource. File paths, web URLs, and custom schemes are all URIs.

---

## V

### V8
Google's JavaScript engine (used in Chrome and Node.js). Vitest uses V8 for code coverage.

### Vite
Fast frontend build tool. Provides development server and production bundling.

### Vitest
Test framework compatible with Vite. Used for JavaScript/TypeScript tests.

---

## W

### WebView2
Microsoft's browser component (Chromium-based). Tauri uses it on Windows.

### WebView
Browser component embedded in native applications. Renders the web-based UI.

### Winget
Windows Package Manager. Command-line tool for installing applications.

---

## Symbols

### `#[...]`
Rust attribute syntax. Attaches metadata to items (functions, structs, modules).

### `?` Operator
Rust's error propagation operator. Returns early with error if `Result` is `Err`.

### `&`
Rust reference operator. `&T` is an immutable reference; `&mut T` is mutable.

### `*`
Rust dereference operator. Accesses the value behind a reference or pointer.

### `::` 
Rust path separator. `std::io::Error` means `Error` in the `io` module of `std`.

### `_`
Underscore pattern. Ignores a value or marks unused variables.
