# M3 wave 4 — kickoff prompt

Verbatim request (2026-07-16), following the #81 acceptance discussion about
item behavior not being stageable from the console:

> "player.additem <FormID> <count> command (and optionally letting activate
> [work on pickups])" yes, good idea

Interpretation: single-issue wave (#84) adding console `additem` and pickup
`activate` support so item-flag acceptance (#81 and future inventory waves)
can be staged manually and over the agent bridge. First wave run under the
strict model routing rule added to AGENTS.md this wave cycle: a Sonnet
executor writes all code and tests; the orchestrating session plans,
reviews, and evaluates.
