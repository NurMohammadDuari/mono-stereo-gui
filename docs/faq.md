# FAQ — frequently asked questions

## General

**Is it safe? What is it doing to my sound system?**

It uses the *official* PulseAudio/PipeWire API via `pactl` — the same
commands `pavucontrol` exposes. No root, no config edits, no daemons, no
network. See [how it works](how-it-works.md) for the exact four commands,
and see that everything used is either create-a-sink, set-default,
move-stream or unload-module.

**Do I need to restart PulseAudio/PipeWire when something breaks?**

No. Switching back to stereo in the app is a complete cleanup. If the app
crashed while in mono:

```bash
pactl unload-module module-remap-sink
```

…the virtual device is gone, done.

**Does it work on my distro?**

Any Linux with PipeWire or PulseAudio + `pactl`:

| Distro | Status |
| --- | --- |
| Ubuntu 20.04+ | ✅ |
| Linux Mint 20+ | ✅ |
| Debian 11+ | ✅ |
| Fedora 35+ | ✅ PipeWire-native |
| Arch / Manjaro / EndeavourOS | ✅ |
| Pop!_OS 22.04 | ✅ |
| Steam Deck (SteamOS 3) | ✅ — use `./install.sh` user-mode, not sudo |

For the build-only requirements see [building from source](building.md).

## Installation & versions

**`.deb` vs `tar.gz` — which one should I get?**

- Debian-family distro → **`.deb`** (double-click or `sudo apt install ./...`)
- Anything else → **`.tar.gz`** + run `./install.sh` from it

**How do I uninstall?**

- `.deb`: `sudo apt remove mono-stereo-gui`
- `tar.gz` install mode: `rm ~/.local/bin/mono_stereo_gui
  ~/.local/share/applications/mono_stereo_gui.desktop
  -rf ~/.local/share/icons/hicolor/{16x16,32x32,48x48,64x64,128x128,256x256,512x512,1024x1024}/apps/mono_stereo_gui.png`
  (system-wide: same paths under `/usr/local/bin`, `/usr/share/…`)

The uninstall ever cleanly removes everything the app created — including
the virtual mono device, if it was still on.

## Usage

**Does my music start over when switching?**

No. Running streams are *moved* between sinks — no stop, no restart, and
both browser and media players continue without a hiccup (you might hear a
below-50 ms buffer blip on rare players).

**Closing the window while in mono — am I stuck?**

No, but you remain in mono. Reopen → click Switch to Stereo. The app
doesn't daemonize; every click is separate.

**Can I open two copies at the same time?**

You can, but don't. They'd fight over which mode you're in. If you truly
need concurrent use the sticky point is `is_mono()` semantics with the
modules list, not two GUIs.

**One earbud only — should I use this?**

Yes: this downmix creates true mono (left+right are combined into one
mixed channel, then played on every speaker). Compare with `pactl load-module
module-loopback [channels=2]`, which still can output one ear-muted.

## Contributing / asking

**Found a bug, where do I report?**

The [Bug report template](https://github.com/NurMohammadDuari/mono-stereo-gui/issues/new?template=bug_report.md)
—it asks for `pactl info`, `pactl list short sinks`, distro and DE which
makes bug bisection 5× faster.

**How can I support the project?**

⭐ Star the repo; share with your Linux friends; open issues; or sponsor
via the "Sponsor" button in the sidebar (`.github/FUNDING.yml`).
