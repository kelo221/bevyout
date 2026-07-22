# M4 AI-packages wave — kickoff prompt

## Requested
After the actor animation / walking / facing work landed, the human asked to
audit the M4A gate (#86) and then "get P1 done today" — the autonomous-AI arc
of M4.

## Scope
- **#115 → #193–#198**: the AI package runtime — selection, lifecycle,
  location/target resolution, and the seven package families (Travel, Patrol,
  Idle, Eat, Sleep, Follow, Sandbox).
- **#116**: perception, disposition, faction hostility, target awareness.
- **#185**: key-aware locked doors.

Turns `tna`-only debug movement into actors that select and run behavior on
their own. Combat execution (M5), companion dialogue (M7), and the full GECK
condition VM (#15) stay out of scope.
