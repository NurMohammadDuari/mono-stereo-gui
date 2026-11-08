# Troubleshooting

Common problems and fixes, ordered the way people usually hit them.

## 1. "Button does nothing" / status shows error

The switcher needs **`pactl`**. Check it is installed and working:

```bash
which pactl        # should print e.g. /usr/bin/pactl
pactl info         # should print "Server Name: …"
```

- If `which` says nothing, install it:
  - Ubuntu / Mint: `sudo apt install pulseaudio-utils`
  - Fedora: `sudo dnf install pulseaudio-utils`
  - Arch: `sudo pacman -S pulseaudio-utils`
- If you are on a **flatpak-only** setup where the host has no `pactl`,
  install `pulseaudio-utils` on the host system too.

## 2. Switching to Mono fails every time

Test the module load manually to see the real error:

```bash
pactl load-module module-remap-sink \
  sink_name=mono_test master=$(pactl get-default-sink) \
  channels=1 channel_map=mono
pactl list-short modules | grep remap
pactl unload-module mono_test
```

If `load-module` errors here, the problem is your audio server (e.g.
access-controlled PipeWire user config), not this app.

## 3. No sound after switching back to Stereo

Some stream ended up pointing at a removed sink. The app restores your
previous master sink and moves all streams, but if something still went
wrong:

```bash
# see where streams are
pactl list short sink-inputs

# snap everything back to your physical sink
pactl set-default-sink <name-from: pactl list short sinks | grep -v mono>

# nuke leftover remap modules
pactl unload-module module-remap-sink
```

## 4. Audio stayed mono after I closed the app

Switching happens only when you click. If the app is closed while in mono
you remain in mono. Reopen the app → **Switch to Stereo**; or now:

```bash
pactl unload-module module-remap-sink
```

## 5. The app-menu icon is generic / missing

- User install: icons are placed in `~/.local/share/icons/hicolor/…`.
  Refresh the icon cache / log out and back in.
- Wayland users: some environments only pick up new `.desktop` files after
  a re-login; that is expected behavior of your session, not a bug.

## 6. Wayland: title bar shows the icon but taskbar doesn't (or vice versa)

Window/runtime icon support differs per stack. The binary embeds the icon
for X11; on some Wayland compositors only the `.desktop`/menu icon shows.
Nothing is broken — cosmetic only.

## 7. Nothing happens on Fedora Silverblue / Steam Deck

Read-only/immutable systems: run the **user** install

```bash
./install.sh
```

(`sudo ./install.sh` needs a writable `/usr`, which plain Silverblue
containers often have via overlay, but if it fails, use the user mode.)

## Still stuck?

Open an issue with the **Bug report** template and include the output of:

```bash
pactl info | head -20
pactl list short sinks
```
