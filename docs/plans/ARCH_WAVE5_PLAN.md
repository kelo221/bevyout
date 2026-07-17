# Architecture wave 5 — production-backed extension traits (#147)

Wave under epic #142 on branch `Refactor`; it builds on waves 1–4.

## Fixed feature list

1. Add an object-safe `ConsoleCommandProvider` contract over the existing
   console registry, with deterministic duplicate/alias handling unchanged.
2. Register built-in viewer commands through provider implementations rather
   than a monolithic installation function.
3. Add a `ContentRecordResolver` contract using core `FormId` and a stable,
   parser-independent record view.
4. Implement the content trait for the canonical indexed content source and
   route at least one real lookup path through the trait.
5. Document determinism, lifetime/ownership, error, and thread-safety
   expectations; do not add an unused `ItemHolderAdapter` abstraction.

## Tests before implementation

- Synthetic console provider registers a command and executes through the same
  registry/executor path as built-ins.
- Synthetic content resolver supports FormID and EditorID lookup without ESM
  data.
- Built-in help, aliases, duplicate handling, selector ambiguity, and content
  override provenance remain deterministic.

## Gate and acceptance

Full Rust gate plus the architecture-wave manual acceptance script against a
representative prepared cell. The final PR targets `master` and closes
#143–#147.

## Shipped amendments

None yet.
