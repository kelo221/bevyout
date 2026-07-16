# M3 wave 5 — canonical item instances and atomic holder transactions (#95)

Implement the full #95 gate after M3 waves 1–3, with wave 4's console/pickup
surface treated as a staging dependency. The runtime must preserve item
identity, condition, count, ownership provenance, quest restrictions, and
opaque mutable state through player, container, world-drop, save/load, pickup,
equipment/use, and a minimal static merchant.

Decisions fixed for this wave:

- every canonical stack has a stable instance ID;
- partial splits allocate a destination ID, while compatible merges keep the
  deterministic lowest ID and return an old-to-survivor remap;
- extra state is sorted, namespaced, tagged opaque binary data;
- the acceptance surface is console/API driven, with no full barter/equipment
  UI redesign;
- merchant buy and sell both use the non-negative catalog base value per unit.

Combat, services, restocking, speech, crime effects, and full gameplay effect
systems remain outside #95.
