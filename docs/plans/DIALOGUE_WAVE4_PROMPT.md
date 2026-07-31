# Dialogue Wave 4 prompt — narrative variables and boundary saves

Add Yarn-local variable storage and safe first-generation save semantics.

- Implement custom variable storage with persistent, session, and temporary
  namespaces.
- Keep inventory, quests, actor state, and globals authoritative in Bevyout;
  Yarn queries them through host functions.
- Add `DialogueSnapshot` and the versioned `DLOG` record under save format v6.
- Save only at dialogue boundaries; reject or defer manual saves during active
  execution.

The snapshot includes an optional checkpoint field for Wave 8, but Wave 4
never writes an active checkpoint.
