# User Guide — Mono / Stereo Switcher

A tiny Apple-style desktop app for Linux that switches your audio output
between **Mono** (everything you hear is played through one ear) and
**Stereo** (normal left/right separation) — with one click.

## When is Mono useful?

- 🎧 **Single-ear headphones / one earbud** — hear both channels.
- 🔊 **Speakers not centered** — perfect when a laptop sits to your side.
- 🦻 **Hearing difference between ears** — balance the sound.
- 🎥 **Old videos with broken channel mixing** — fix "audio only right side".

## Using the app

1. Start it from your app menu (**"Mono / Stereo"**) or run:

   ```bash
   mono_stereo_gui
   ```

2. A small window appears showing the current mode:

   - `Current mode: Mono` or `Current mode: Stereo`
3. Click the big button once:

   | Current | Click does this |
   | --- | --- |
   | Stereo | Creates a virtual mono sink, sets it as default, moves every running audio stream to it |
   | Mono | Removes the virtual sink, restores your real stereo sink and moves streams back |

4. Close the window any time — switching happens on click only; closing does
   **not** revert your audio.

> ⚠️ If the app is closed while in **Mono** mode, your sound stays in mono.
> Reopen it and click **Switch to Stereo** to go back, or run
> `pactl unload-module module-remap-sink` manually.

## What it changes in your audio system?

While running in mono mode you will see an extra audio device called
`mono_output` in settings (the app creates it with PulseAudio's
`module-remap-sink`). It is removed again when you switch back to stereo —
the device list stays clean.

## Nice to know

- 🔁 Show any running **browser / player** volume applet — its streams move
  seamlessly; you don't need to pause, restart or reconnect anything.
- 🖥️ The app sets its own window/taskbar icon so it looks native in most
  desktop environments (GNOME, KDE, XFCE, Cinnamon…).
- 🍃 No background daemon, no autostart, no network access — just a small GUI.
