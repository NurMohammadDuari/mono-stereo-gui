# Contributing

Thanks for considering a contribution to **mono-stereo-gui**! 🎉

There are many ways to help — code, bug reports, ideas, docs and testing
all count.

## Quick links

- 🐛 Report a bug → [Issue templates](https://github.com/NurMohammadDuari/mono-stereo-gui/issues)
- 💡 Suggest a feature → open an issue with the "Feature request" template
- 📖 Improve docs → `README.md`, `docs/*` — even typo fixes are welcome!

## Before you open a Pull Request

1. **Discuss first for big changes** — if you plan something major (e.g. a
   rewrite of the audio backend, adding Wayland-specific volume control),
   please open an issue first so we can agree on the approach.
2. **Check that you can build it**:

   ```bash
   git clone https://github.com/NurMohammadDuari/mono-stereo-gui.git
   cd mono-stereo-gui
   cargo build --release
   cargo clippy            # no new warnings, please
   cargo fmt               # Rust standard formatting
   ```

3. **Keep commits small and focused** — one logical change per commit.
4. **Test on a real system** — this app directly manipulates PipeWire /
   PulseAudio sinks and moves running audio streams. Please verify:
   - Switching Mono → Stereo → Mono several times in a row
   - Switching while music/video is playing in the browser
   - That the "mono_output" virtual sink disappears after switching back

## Code style

- This project uses `rustfmt` defaults — run `cargo fmt` before committing.
- Comments are lowercase-light and only where they add value.
- Prefer `pactl` once-per-action calls; avoid busy loops that respawn
  `pactl` constantly (the UI already repaints every 500 ms).

## License

By contributing you agree that your contributions will be licensed under
the MIT License that covers this project.
