# Roadmap

Nothing here is a promise — it's the "would love this" list, roughly in
priority order. Want something moved up? Open/+, it helps steer.
See [CONTRIBUTING](../CONTRIBUTING.md) to start hacking.

## 🚧 Near-term (v0.3)

- [ ] **App screenshot** in the README (works starts on a real laptop build).
- [ ] **Restore volume/balance snapshot** after switching mono→stereo, so
      nothing surges after circular toggling.
- [ ] **Crate the app window with proper Wayland support**: compositor
      window-icon where supported, harmless no-op where not.
- [ ] One-click **update check** (via GitHub Releases API, explicit button,
      no silent telemetry).

## 🎯 Medium-term (0.4–1.0)

- [ ] **Per-app mono**: right-click → "mono only this app", leaving other apps at stereo.
- [ ] Light/dark/dark-mode system-following UI themes.
- [ ] `--switch mono|stereo` CLI flags + a global keybind (no GUI needed).
- [ ] Flatpak and RPM builds (CI-generated, on tags).

## 🚀 Long-term / larger ideas (1.x+)

- [ ] Handbook-quality docs site (mdBook or GitHub Pages re-jigged docs).
- [ ] On-device audio device picker (choose which master the mono sink
      remaps from).
- [ ] ARM64 builds (Raspberry Pi aarch64 `.deb`).
- [ ] Optional KDE Plasma applet beauty — if someone *really* covets it.

## 🧹 Ongoing housekeeping (every release)

- [ ] Bump `CHANGELOG.md`, tag, and let Actions do the publishing.
- [ ] Keep docs in sync with reality — anything broken, open a
      "doc bug" with the misleading text pasted.
