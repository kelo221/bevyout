# M3 wave 1 kickoff — inventory and persistent dropping

Requested on 2026-07-15 against epic #7 and gate #8.

- Use OpenMW's item record and inventory concepts as the basis, preserving the
  repository's isolated VSA provenance rule.
- `E` picks up supported world items.
- `Tab` opens a paused, pointer-driven Pip-Boy-style Items screen with item
  icons, carried weight, and category-aware stats.
- Left-click selects only. Right-click drops one from stacks of at most three;
  larger stacks open a quantity picker.
- A multi-item quantity becomes one labeled world stack. Dropped objects have
  physics and survive save/reload without loss or duplication.
- The first UI is a flat full-screen green approximation of the supplied
  reference, not a 3D wrist device.

Child issues: #70 prepared catalog, #71 inventory/UI, #72 dropped persistence.
