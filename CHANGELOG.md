# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-11-08

### Added
- 🖼️ **Window & taskbar icon** — the app icon (`assets/icon-256.png`) is now
  embedded in the binary and shown in the title bar and taskbar, not only in
  the app-menu launcher.
- 📦 **Ready-made releases** — download `mono_stereo_gui-linux-x86_64.tar.gz`
  from the [Releases page](https://github.com/NurMohammadDuari/mono-stereo-gui/releases)
  and install with the bundled `install.sh` (no compiler needed).
- 🧑‍💻 **`install.sh`** — one-shot installer with both user-only
  (`./install.sh`) and system-wide (`sudo ./install.sh`) modes, including
  icons, `.desktop` entry and PATH sanity-checking.
- 📚 **Documentation** — this changelog, `CONTRIBUTING.md`, `SECURITY.md`,
  a user guide (`docs/user-guide.md`), a troubleshooting guide
  (`docs/troubleshooting.md`), and issue/PR templates.

### Changed
- The mono sink name is now defined once (`MONO_NAME`) instead of being
  hard-coded in three separate places — removes a compiler warning.

## [0.1.0] - 2026-11-08

### Added
- Initial release.
- One-click toggle between Mono and Stereo audio output.
- Automatic migration of all active audio streams to the new sink.
- Apple-style UI built with Rust + egui (eframe 0.29).
- Auto-cleanup of the temporary `module-remap-sink` mono module.
- App icons (16 px – 1024 px) and a Linux `.desktop` launcher.
