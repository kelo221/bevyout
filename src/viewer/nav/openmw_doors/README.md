# OpenMW-derived AI door-access rule

This isolated module adapts the decision rule
`MWMechanics::AiPackage::openDoors()` uses to decide whether an AI-controlled
actor may open a door it is routing through: untrapped and unlocked doors are
always passable; a locked door is passable only if the actor's own inventory
holds the door's key; a trapped door is never opened. It is a small, pure
Rust re-statement of that one decision table -- no OpenMW C++ is compiled,
and this module owns no navmesh, inventory, or Bevy state of its own.

`nav/agent.rs` supplies the observation (the door's prepared lock/key/trap
data, and whether the specific routing actor's own canonical inventory holds
the key) and applies the verdict to both route-cost eligibility
(`AgentTypeIndexCostOverrides`) and the door-link open request -- see that
file's module doc comment ("Doors as conditional route topology") for how
the two meet.

See `NOTICE.md` for source provenance.
