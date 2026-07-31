# Dialogue Wave 9 prompt — voice, localization, and presentation polish

Add presentation services only after lifecycle, host commands, saves, and
imported semantics are stable.

- Add voice indexes keyed by stable line key, subtitle timing, speaker-name
  localization, language selection, prefetch, skip/fast-forward, and
  accessibility settings.
- Add optional lip/facial metadata and debug hot reload for authored Yarn.
- Keep production source discovery explicit and content-hash validated.
- Add coverage and timing diagnostics for unsupported records and runner
  failures.

Voice and presentation providers report completion; they do not mutate
authoritative dialogue or world state directly.
