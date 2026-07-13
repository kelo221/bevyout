# OpenMW-derived save format primitives

This isolated module follows OpenMW's versioned, tagged record/subrecord
organization used by `ESM::SavedGame` and `apps/openmw/mwstate/statemanagerimp.cpp`.
It is a small Rust implementation of the framing behavior needed by bevyout;
it is not a binary-compatible OpenMW save reader and does not include OpenMW
runtime code.

The project-specific state model and durability policy are implemented outside
this directory. See `NOTICE.md` for provenance and the intentional boundary.
