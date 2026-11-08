# Mono / Stereo Audio Switcher

A lightweight desktop app to easily switch between **Mono** and **Stereo** audio output on Linux (PipeWire / PulseAudio). Built with **Rust** + **egui** for a clean, Apple-style interface.

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

### From source

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
├── mono_stereo_gui.desktop   # Linux desktop launcher
├── LICENSE             # MIT License
└── README.md           # This file
```

## Building

```bash
cargo build --release
```

The optimized binary will be at `target/release/mono_stereo_gui`.

## License

MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Author

**NurMohammadDuari** — https://github.com/NurMohammadDuari
