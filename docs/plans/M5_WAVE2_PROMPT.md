# M5 wave 2 prompt — canonical ammunition

Implement the second M5 combat wave after the functional 10mm pistol:

- make magazines and loaded rounds canonical per-weapon-instance state;
- move reload and ammo switching through atomic item-ledger transactions;
- generalize the single equipped binding into a loadout that can later carry armor;
- persist the implemented Wave 2 state as save v5;
- route player and console firing through the same ammunition authority;
- expose deterministic typed inspection through console and BRP;
- retain the Wave 1 viewmodel, audio, recoil, hitscan, and actor persistence.

Do not add empty implementations for later condition, armor, limb, V.A.T.S., or
AI waves. Their inspection commands may report a typed unavailable result.

