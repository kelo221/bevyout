[bevy](../../../index.html)::[prelude](../../index.html)::[light\_consts](../index.html)

# Module lumens 

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#107)

Approximations for converting the wattage of lamps to lumens.

The **lumen** (symbol: **lm**) is the unit of [luminous flux](https://en.wikipedia.org/wiki/Luminous_flux), a measure of the total quantity of [visible light](https://en.wikipedia.org/wiki/Visible_light) emitted by a source per unit of time, in the [International System of Units](https://en.wikipedia.org/wiki/International_System_of_Units) (SI).

For more information, see [wikipedia](https://en.wikipedia.org/wiki/Lumen_\(unit\))

## Constants

[LUMENS\_PER\_HALOGEN\_WATTS](constant.LUMENS_PER_HALOGEN_WATTS.html "constant bevy::prelude::light_consts::lumens::LUMENS_PER_HALOGEN_WATTS")

The conversion factor used to determine how many lumens a typical halogen light of a given wattage produces.

[LUMENS\_PER\_INCANDESCENT\_WATTS](constant.LUMENS_PER_INCANDESCENT_WATTS.html "constant bevy::prelude::light_consts::lumens::LUMENS_PER_INCANDESCENT_WATTS")

The conversion factor used to determine how many lumens a typical incandescent light of a given wattage produces.

[LUMENS\_PER\_LED\_WATTS](constant.LUMENS_PER_LED_WATTS.html "constant bevy::prelude::light_consts::lumens::LUMENS_PER_LED_WATTS")

The conversion factor used to determine how many lumens a typical LED light of a given wattage produces.

[VERY\_LARGE\_CINEMA\_LIGHT](constant.VERY_LARGE_CINEMA_LIGHT.html "constant bevy::prelude::light_consts::lumens::VERY_LARGE_CINEMA_LIGHT")

1,000,000 lumens is a very large “cinema light” capable of registering brightly at Bevy’s default [`bevy_camera::Exposure::BLENDER`](../../../camera/struct.Exposure.html#associatedconstant.BLENDER "associated constant bevy::camera::Exposure::BLENDER") exposure level. For “indoor lighting” with a lower exposure, this would be way too bright.