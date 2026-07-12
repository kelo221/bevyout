# OpenMW-derived player locomotion

This isolated module adapts the movement rules needed for the Fallout viewer's
FPS capsule from the supplied `openmw-master` snapshot (identified as OpenMW
0.52.0). It keeps the OpenMW-style stationary/directional jump split, reduced
air control, and explicit airborne-to-landing transition while leaving Bevy's
Tnua/Avian collision and walking solver in charge of scene movement.

The viewer has no actor skill, fatigue, damage, or animation systems yet, so
those OpenMW systems are intentionally not included. Fallout-native landing
audio is staged by the Fallout preparation slice, not copied from OpenMW.

See `NOTICE.md` for source paths, hashes, and license attribution.
