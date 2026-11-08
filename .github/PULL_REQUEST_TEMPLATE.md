<!--
Thanks for the PR! Fill out the checklist so reviews go fast.
-->
## What does this PR change?

<!-- bugfix / feature / docs / tooling … one or two sentences -->

## How did you test it?

- [ ] `cargo build --release` passes
- [ ] `cargo clippy` reports no new warnings
- [ ] `cargo fmt` clean (run it!)
- [ ] On a real system: Mono ↔ Stereo switching works repeatedly,
      with a browser/video stream playing, and back to stereo removes the
      `mono_output` virtual sink.

**Environment I tested on:** (distro, audio server (PipeWire or PulseAudio), DE)

## Checklist

- [ ] This does not add heavyweight dependencies without discussing it first
- [ ] CHANGELOG.md updated (for user-visible changes)
- [ ] Docs updated if behavior changed
