#!/usr/bin/env bash
# Mono/Stereo Audio Switcher — one-shot installer
# Usage:   ./install.sh        (inside an extracted release tarball)
#          sudo ./install.sh   (same, but installs system-wide with menu entry)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ ! -x "$SCRIPT_DIR/mono_stereo_gui" ]; then
    echo "ERROR: $SCRIPT_DIR/mono_stereo_gui not found."
    echo "Run this script from the folder you extracted from the release tarball."
    exit 1
fi

if [ "$(id -u)" -eq 0 ]; then
    echo "==> Installing system-wide (with application-menu entry)…"
    install -Dm755 "$SCRIPT_DIR/mono_stereo_gui" /usr/local/bin/mono_stereo_gui
    install -Dm644 "$SCRIPT_DIR/mono_stereo_gui.desktop" /usr/share/applications/mono_stereo_gui.desktop
    install -Dm644 "$SCRIPT_DIR/mono_stereo_gui.desktop" /home/"${SUDO_USER:-$USER}"/.local/share/applications/mono_stereo_gui.desktop 2>/dev/null || true
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -f /usr/share/icons/hicolor 2>/dev/null || true
    fi
    echo "✅ Installed! Search for \"Mono / Stereo\" in your app menu, or run: mono_stereo_gui"
else
    echo "==> Installing for current user only…"
    LOCAL_BIN="$HOME/.local/bin"
    APPS="$HOME/.local/share/applications"
    ICONS_LOCAL="$HOME/.local/share/icons/hicolor"
    mkdir -p "$LOCAL_BIN" "$APPS"
    install -Dm755 "$SCRIPT_DIR/mono_stereo_gui" "$LOCAL_BIN/mono_stereo_gui"
    install -Dm644 "$SCRIPT_DIR/mono_stereo_gui.desktop" "$APPS/mono_stereo_gui.desktop"
    for size in 16 32 48 64 128 256 512 1024; do
        icon="$SCRIPT_DIR/icon-${size}.png"
        if [ -f "$icon" ]; then
            install -Dm644 "$icon" "$ICONS_LOCAL/${size}x${size}/apps/mono_stereo_gui.png"
        fi
    done
    if ! command -v mono_stereo_gui >/dev/null 2>&1 \
       && ! echo "$PATH" | grep -q "$LOCAL_BIN"; then
        echo "⚠ Warning: $LOCAL_BIN is not in your PATH."
        echo "  Add it with:  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    fi
    echo "✅ Installed! Search for \"Mono / Stereo\" in your app menu, or run: $LOCAL_BIN/mono_stereo_gui"
    echo "   (Log out & back in once if the menu entry does not appear right away.)"
fi
