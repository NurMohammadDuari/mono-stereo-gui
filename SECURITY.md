# Security Policy

## Supported versions

| Version | Supported |
| ------- | --------- |
| 0.2.x   | ✅        |
| < 0.2   | ❌ (please upgrade) |

## Reporting a vulnerability

If you discover a security issue, **please do NOT open a public issue.**

Instead, contact the maintainer directly:

- 🐙 GitHub: open a private message via
  [NurMohammadDuari](https://github.com/NurMohammadDuari)

You will normally get a response within a few days. Please include:

1. A description of the issue and how it might be exploited.
2. Step-by-step instructions to reproduce it.
3. Your OS / audio stack (PipeWire or PulseAudio) and app version.

## Scope notes

- This is a small desktop utility that shells out to `pactl`. It does **not**
  open network sockets, and it does **not** store credentials.
- Reports about argument injection via crafted sink names are still
  relevant (the sink name comes from your local audio server, so we treat
  it as semi-trusted input) — please report anything suspicious.
