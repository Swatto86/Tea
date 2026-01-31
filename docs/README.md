# Building Desktop Apps with Rust and Tauri

> *From Zero to System Tray: A Beginner's Journey Through Real-World Code*

---

## Title Options Considered

1. **"Brewing Desktop Apps: A Rust & Tauri Journey"** — Plays on the "Tea" app name
2. **"Building Desktop Apps with Rust and Tauri"** — Clear, professional, searchable
3. **"The Tauri Handbook: Rust Desktop Development from First Principles"** — Comprehensive feel

**Selected: Option 2** — Clarity wins. Beginners searching for help will find this.

---

## What This Book Is

This book teaches you to build cross-platform desktop applications using **Rust** and **Tauri 2.0**. You'll learn by studying and extending a real, working application: **Tea**, a system tray utility that prevents your computer from sleeping.

By the end, you will:

- Understand Rust's ownership model, error handling, and module system
- Build native desktop apps with Tauri's Rust-first architecture
- Create system tray applications with menus and icons
- Implement cross-platform code (Windows, macOS, Linux)
- Handle state persistence, background services, and platform APIs
- Write tests and ship production builds

---

## Who This Book Is For

- **Complete beginners to Rust** who want a practical project, not abstract exercises
- **Web developers** curious about native desktop development
- **Tauri explorers** who've done the tutorials but want to see production patterns
- **Anyone** who learns best by reading real code with explanations

### Prerequisites

- Basic programming experience in any language (variables, functions, conditionals)
- A computer running Windows, macOS, or Linux
- Willingness to read code and experiment

No prior Rust or Tauri experience required.

---

## How This Book Is Organized

### Part 0: The Map
Before coding, understand the terrain. Architecture diagrams, data flows, and the "big picture" of how Tea works.

### Part 1: Foundations (Chapters 1–4)
Rust fundamentals through the lens of Tea's actual code. Ownership, structs, enums, error handling.

### Part 2: Tauri Core (Chapters 5–8)
How Tauri bridges Rust and the frontend. Commands, state management, the plugin system.

### Part 3: Building Tea (Chapters 9–14)
Feature by feature, we rebuild Tea. System tray, icons, persistence, background services, platform-specific code.

### Part 4: Production (Chapters 15–17)
Testing, building installers, and shipping your app.

---

## The Running Example: Following Data Through the App

Throughout this book, we trace one action end-to-end:

> **"The user clicks 'Disable Sleep' in the system tray menu"**

This single action touches:
- Menu event handling (`main.rs`)
- Business logic (`commands.rs`)
- State management (`AppStateManager`)
- Background service control (`wake_service.rs`)
- Platform APIs (`platform.rs`)
- File persistence (`persistence.rs`)
- Icon and tooltip updates (`icon.rs`, `tooltip.rs`)

Every chapter connects back to this flow.

---

## How to Read This Book

### Option A: Linear Path (Recommended for Beginners)
Read chapters in order. Each builds on the last.

### Option B: Reference Mode
Jump to specific chapters when you need them. The [Summary](SUMMARY.md) provides quick navigation.

### Option C: Code-First
Open the Tea codebase alongside the book. Read code, then find explanations here.

---

## Conventions Used

| Convention | Meaning |
|------------|---------|
| `monospace` | Code, file paths, commands |
| **Bold** | Key terms on first use |
| *Italics* | Emphasis or analogies |
| `// File: path/to/file.rs` | Code excerpts from the repo |
| 📍 | "You are here" in a data flow |
| ⚠️ | Common pitfall or gotcha |
| 💡 | Tip or insight |

### Diagrams

All diagrams use Mermaid syntax and render in most Markdown viewers (GitHub, VS Code, etc.).

---

## The Tea Application

**Tea** is a cross-platform system tray application that prevents your computer from sleeping. It's small enough to understand completely, but complex enough to teach real patterns:

- ~800 lines of Rust
- System tray with dynamic menus
- Platform-specific implementations (Windows API, F15 key simulation)
- JSON state persistence
- Background async service
- Structured error handling

Perfect for learning.

---

## Getting Started

1. Clone or open the Tea repository
2. Install prerequisites (see [Chapter 3: Setting Up Your Environment](chapters/03-setting-up-your-environment.md))
3. Start reading [Part 0: The Map](chapters/00-part-0-the-map.md)

---

## Repository Structure

```
Tea/
├── docs/                    # This book
│   ├── README.md            # You are here
│   ├── SUMMARY.md           # Table of contents
│   ├── PROGRESS.md          # Writing progress tracker
│   └── chapters/            # Book chapters
├── src/                     # Frontend (minimal for tray apps)
├── src-tauri/               # Rust backend (where the action is)
│   ├── src/
│   │   ├── main.rs          # Entry point, tray setup
│   │   ├── commands.rs      # Tauri commands
│   │   ├── persistence.rs   # State file I/O
│   │   ├── platform.rs      # OS-specific code
│   │   ├── wake_service.rs  # Background service
│   │   └── core/            # Shared types
│   └── Cargo.toml           # Rust dependencies
└── tests/                   # Test files
```

---

## Let's Begin

Turn to [Part 0: The Map](chapters/00-part-0-the-map.md) to see the full picture before we dive in.

---

*This book is grounded in the actual Tea codebase. Every claim is verifiable. Every path is real.*
