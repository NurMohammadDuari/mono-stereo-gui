# How It Works — the pipe-friendly deep dive

This page explains exactly what the app does to your audio stack, so you
can trust it (and fix it if you're the type who wants to know).

## Big picture

```
┌────────────────────────────────────────────────────────────┐
│                 stereo physical device                     │
│                  (e.g. analog-stereo)                      │
└───────────────────────▲────────────────────────────────────┘
                        │  master of the remap sink
              ┌─────────┴─────────┐
              │   Virtual mono    │   ← module-remap-sink,
              │      SINK         │     channels=1, map=mono
              │  name: mono_output│
              └─────────▲─────────┘
                        │  set as default sink
          ┌─────────────┴─────────────┐
          │  all running stream-inputs │ ← browser, music, video…
          └───────────────────────────┘
```

**Stereo (normal):** every stream goes straight into your real device —
nothing special.

**Mono:** the app creates a *virtual* one-channel sink that all audio is
routed through; it downmixes left+right into a single channel. Your real
device keeps both channels but they receive identical samples.

## The four `pactl` operations

1. **Discover your master sink**
   ```bash
   pactl list short sinks      # find the non-mono, non-monitor default
   ```

2. **Create the virtual mono sink**
   ```bash
   pactl load-module module-remap-sink \
        sink_name=mono_output master=<your real sink> \
        channels=1 channel_map=mono
   ```

3. **Make it the default & move running streams**
   ```bash
   pactl set-default-sink mono_output
   pactl list short sink-inputs            # every playing stream…
   pactl move-sink-input <id> mono_output  # …moves seamlessly
   ```

4. **Switching back = total cleanup**
   ```bash
   pactl set-default-sink <your real sink>
   pactl move-sink-input <id> <your real sink>
   pactl unload-module module-remap-sink   # device disappears: clean!
   ```

## Design decisions

| Decision | Why |
| --- | --- |
| Shell out to `pactl` instead of linking `libpulse` | Zero extra native deps; `pactl` exists everywhere PA/PW is installed; simpler code |
| 5-second hidden poll (UI repaints every 500 ms) | `is_mono()` just greps `pactl list short modules`, cheap on modern CPUs |
| Hard `unload_remaps()` before each mode change | Safety net for crammed sessions, never leaves stuck modules |
| Window icon embedded via `include_bytes!` | No runtime asset lookup, works regardless of where binary runs from |
| No daemon/network | Does one thing at a time, closes when you close it |

## Known limitations (roadmap candidates!)

- Stereo→mono→stereo: original **channel balance/volume** re-assignments
  aren't healed; volume snapshots are not restored (streams inherit new
  sink's balance).
- One `module-remap-sink` with side-channel apps like Pipewire's
  `pw-metadata` duplicating data isn't handled explicitly.
- No per-app mono-only switch (the app switches the whole device).
- No Wayland fractional-scaling-aware UI tweaks (egui handles most).

See [roadmap](roadmap.md) for planned changes, and
[CONTRIBUTING](../CONTRIBUTING.md) for how to help.
