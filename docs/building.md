# Building from source — full guide

Everything you need to compile `mono-stereo-gui` yourself, including the
`deb` package and the release tarball.

## 1. Toolchain

You need Rust 1.70+ (the newer the better — the `2021` edition is used):

```bash
# via rustup (recommended): https://rustup.rs
curl -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
```

Debian/Ubuntu users who prefer the distro toolchain:

```bash
sudo apt install rustc cargo
```

## 2. System build dependencies

 compiling pulls them from crates.io; runtime system libs are dlopen'ed,
 so you typically need at least a desktop graphical session with:

| Distro | One-liner |
| --- | --- |
| Ubuntu / Mint / Debian / Pop!_OS | `sudo apt install libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libx11-dev zlib1g-dev` (vast majority will already have all libs, since the app dlopens them) |
| Fedora | `sudo dnf install libxkbcommon-devel libxkbcommon-x11-devel wayland-devel libx11-devel` |
| Arch | `pacman -S libxkbcommon wayland libx11` |

## 3. Build

```bash
git clone https://github.com/NurMohammadDuari/mono-stereo-gui.git
cd mono-stereo-gui
cargo build --release        # ~3-4 min on a modern laptop, once
```

The optimized binary goes to `target/release/mono_stereo_gui` (~16 MB).

Quality gates used in CI — run them too when contributing:

```bash
cargo fmt -- --check
cargo clippy -- -D warnings
```

## 4. Pack it like we do in releases

### The release tarball

```bash
B=target/release
PKG=/tmp/mono_stereo_gui-linux-x86_64
rm -rf $PKG && mkdir -p $PKG/assets
cp $B/mono_stereo_gui install.sh mono_stereo_gui.desktop LICENSE README.md $PKG/
for s in 16 32 48 64 128 256 512 1024; do cp assets/icon-$s.png $PKG/assets/; done
(cd /tmp && tar -czf mono_stereo_gui-linux-x86_64.tar.gz mono_stereo_gui-linux-x86_64)
```

### The .deb package

```bash
P=/tmp/mono-stereo-gui
mkdir -p $P/DEBIAN $P/usr/bin $P/usr/share/applications \
  $P/usr/share/doc/mono-stereo-gui
for s in 16x16 32x32 48x48 64x64 128x128 256x256 512x512 1024x1024; do \
  mkdir -p $P/usr/share/icons/hicolor/$s/apps; done

install -m755 target/release/mono_stereo_gui $P/usr/bin/
install -m644 mono_stereo_gui.desktop $P/usr/share/applications/
for s in 16x16:16 32x32:32 48x48:48 64x64:64 128x128:128 256x256:256 512x512:512 1024x1024:1024; do \
  install -m644 assets/icon-${s#*:}.png \
    $P/usr/share/icons/hicolor/${s%%:*}/apps/mono_stereo_gui.png; done

cp docs/*.md README.md $P/usr/share/doc/mono-stereo-gui/

cat > $P/DEBIAN/control <<EOF
Package: mono-stereo-gui
Source: mono-stereo-gui
Version: $(sed -n 's/^version *= *"\(.*\)".*/\1/p' Cargo.toml | head -1)
Section: sound
Priority: optional
Architecture: amd64
Maintainer: Nur Mohammad Duari <NurMohammadDuari@users.noreply.github.com>
Depends: libc6 (>= 2.31), libx11-6, libx11-xcb1, libxcursor1, libxrandr2, libxi6, libxkbcommon0, libxkbcommon-x11-0, libwayland-client0, libegl1, libgbm1, libgl1, pulseaudio-utils
Description: One-click Mono <-> Stereo audio output switcher for Linux
 A tiny Apple-style desktop app that switches your audio output between
 mono and stereo on Linux (PipeWire / PulseAudio) and automatically moves
 every running audio stream (browser, player, video) to the new sink.
Homepage: https://github.com/NurMohammadDuari/mono-stereo-gui
EOF

dpkg-deb --root-owner-group --build $P mono-stereo-gui_$(VERSION)_amd64.deb
```

## 5. CI-defined packaging (nice: nothing above to type)

As soon as you push a tag like `v0.3.0`, GitHub Actions (`.github/workflows/`)
builds, packages and publishes the same `tar.gz` + `.deb` artifacts — same
layout as in this guide. If you see `releases <tags> vanish`, look at the
Actions tab for any red ✗ to debug.

## 6. Smoke-testing your build

In your actual session:

```bash
pactl info | head -3                       # audio server reachable
target/release/mono_stereo_gui             # window appears w/ icon
# click the big button twice, correctly back to whatever it was
pactl list short modules | grep remap      # should be EMPTY after stereo
```

Have ideas to improve the build, [contributing](../CONTRIBUTING.md) is open.
