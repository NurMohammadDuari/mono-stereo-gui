# Mono / Stereo Audio Switcher

A lightweight desktop app to easily switch between **Mono** and **Stereo** audio output on Linux (PipeWire / PulseAudio). Built with **Rust** + **egui** for a clean, Apple-style interface.

## 🚀 Quick Start (for noobs — no coding needed!)

**1. Download the app** — go to the
[**Releases page**](https://github.com/NurMohammadDuari/mono-stereo-gui/releases) and
download the newest file ending in `.tar.gz`
(for example `mono_stereo_gui-linux-x86_64.tar.gz`).

**2. Extract it** — either right-click the file → *Extract here*, or run:

```bash
tar -xzf ~/Downloads/mono_stereo_gui-linux-x86_64.tar.gz
```

**3. Install it** — you have two very easy options:

```bash
# EASIEST: user install (no password asked)
./install.sh

# OR system-wide install with a menu entry for all users
sudo ./install.sh
```

Done ✅ — search **"Mono / Stereo"** in your app menu, or just double-click the
extracted `mono_stereo_gui` file.

> 🐧 Requirements: any modern Linux with PipeWire or PulseAudio and `pactl`
> (Ubuntu, Mint, Fedora, Pop!_OS, Arch etc. already have it).

## Features

- 🎚️ One-click toggle between Mono and Stereo audio output
- 🔄 Automatically moves all active audio streams (browser, music player, video) to the new sink
- 🍎 Apple-style UI with toggle switch and status feedback
- 🧹 Auto-cleanup: removes the temporary mono remap module when switching back to stereo
- 🖥️ Linux native (PipeWire / PulseAudio via `pactl`)
- 📦 Installs as a system app via `.desktop` file (appears in your app menu)

## Screenshot

*(Coming soon — add a screenshot here)*

## Installation

### Option 1 — Download ready-made binary (no compiler needed)

See **[Quick Start](#-quick-start-for-noobs--no-coding-needed)** above — grab the
latest binary from the
[Releases page](https://github.com/NurMohammadDuari/mono-stereo-gui/releases).

### Option 2 — Build from source

```bash
# Clone the repository
git clone https://github.com/NurMohammadDuari/mono-stereo-gui.git
cd mono-stereo-gui

# Build in release mode
cargo build --release

# Run
./target/release/mono_stereo_gui
```

### System-wide install

```bash
cargo build --release
sudo cp target/release/mono_stereo_gui /usr/local/bin/mono_stereo_gui
sudo cp assets/icon-1024.png /usr/share/icons/hicolor/1024x1024/apps/mono_stereo_gui.png
sudo cp mono_stereo_gui.desktop /usr/share/applications/mono_stereo_gui.desktop
sudo gtk-update-icon-cache /usr/share/icons/hicolor
```

Then search for **"Mono / Stereo"** in your application menu.

## How it works

The app uses PipeWire/PulseAudio's `module-remap-sink` to create a dedicated mono downmix sink:

1. **Mono mode**: Loads a `module-remap-sink` with `channels=1 channel_map=mono`, sets it as the default sink, and moves all active audio streams to it.
2. **Stereo mode**: Switches back to your original stereo sink, moves streams back, and unloads the mono module to keep your device list clean.

## Requirements

- Linux with PipeWire or PulseAudio
- `pactl` command-line tool
- Rust 1.70+ (for building from source)

## Project Structure

```
mono-stereo-gui/
├── Cargo.toml          # Rust package manifest
├── Cargo.lock          # Locked dependencies
├── src/
│   └── main.rs         # Application source code
├── assets/             # App icons (16px – 1024px)
│   └── icon-*.png
├── install.sh          # One-shot installer used by the releases
├── mono_stereo_gui.desktop   # Linux desktop launcher
├── LICENSE             # MIT License
└── README.md           # This file
```

## Building

```bash
cargo build --release
```

The optimized binary will be at `target/release/mono_stereo_gui`.

## Author

**Nur Mohammad Duari** — https://github.com/NurMohammadDuari

## 📚 More documentation

| Document | What's inside |
| --- | --- |
| [User Guide](docs/user-guide.md) | When to use mono, how switching works exactly |
| [Troubleshooting](docs/troubleshooting.md) | "`pactl` missing", no sound after swap, icon problems, immutable distros… |
| [Contributing](CONTRIBUTING.md) | How to build, test and submit pull requests |
| [Changelog](CHANGELOG.md) | Every release and its changes |
| [Security Policy](SECURITY.md) | How to report vulnerabilities privately |

## License

MIT — see [LICENSE](LICENSE) for details.
