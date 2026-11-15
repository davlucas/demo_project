# Cross-Platform Digital Clock

A graphical digital clock built in Rust, featuring real-time local time, UTC time, dynamic multi-timezone management, and multi-language support.

## Features

- Multi-language support (5 languages):
  - French
  - English
  - Japanese
  - Chinese
  - Spanish
  - Real-time language switching via dropdown menu.
  - Selected language is automatically saved and restored on startup.
  - CJK font fallback support (Japanese and Chinese) using system fonts across Linux, Windows, and macOS.

- Cross-platform compatibility:
  - Linux (native Wayland and X11 support)
  - Windows
  - macOS (Universal binary supporting Apple Silicon and Intel x86_64)

- Reference clocks:
  - Local Time (including UTC offset and formatted date).
  - Coordinated Universal Time (UTC reference).

- Dynamic timezone management:
  - Hierarchical grouping by continent.
  - Alphabetical sorting of countries and cities within each continent.
  - Real-time text search filter for quick city lookup.
  - Add and remove clocks dynamically.

- Configuration persistence:
  - Saved timezones and selected language are persisted to:
    - Linux / macOS: $HOME/.digital_clock
    - Windows: %USERPROFILE%\.digital_clock
  - Restored automatically upon application launch.

- Responsive screen constraint handling:
  - Automatically queries the display resolution.
  - Each column height is strictly constrained to not exceed the user's screen height.
  - Multi-column layout with horizontal scrolling when displaying many clocks.

- Display preferences:
  - 24-hour and 12-hour (AM/PM) toggle.
  - Toggle seconds display.
  - Toggle date display.

## Prerequisites

### Linux (Debian / Ubuntu)

```bash
sudo apt update
sudo apt install build-essential libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

### Linux (Arch Linux / Manjaro)

```bash
sudo pacman -S base-devel libxkbcommon
```

### Windows / macOS

Standard Rust installation (rustc and cargo).

## Build and Execution

### Run directly

```bash
cargo run --release
```

### Build production binary

```bash
cargo build --release
```

The compiled standalone executable will be located in:
- Linux / macOS: target/release/digital_clock
- Windows: target/release/digital_clock.exe
