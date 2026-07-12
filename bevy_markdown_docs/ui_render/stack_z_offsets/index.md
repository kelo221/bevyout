[bevy](../../index.html)::[ui\_render](../index.html)

# Module stack\_z\_offsets 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#108)

Local Z offsets of “extracted nodes” for a given entity. These exist to allow rendering multiple “extracted nodes” for a given source entity (ex: render both a background color _and_ a custom material for a given node).

When possible these offsets should be defined in _this_ module to ensure z-index coordination across contexts. When this is _not_ possible, pick a suitably unique index unlikely to clash with other things (ex: `0.1826823` not `0.1`).

Offsets should be unique for a given node entity to avoid z fighting. These should pretty much _always_ be larger than -0.5 and smaller than 0.5 to avoid clipping into nodes above / below the current node in the stack.

A z-index of 0.0 is the baseline, which is used as the primary “background color” of the node.

Note that nodes “stack” on each other, so a negative offset on the node above could clip _into_ a positive offset on a node below.

## Constants

[BACKGROUND\_COLOR](constant.BACKGROUND_COLOR.html "constant bevy::ui_render::stack_z_offsets::BACKGROUND_COLOR")

[BORDER](constant.BORDER.html "constant bevy::ui_render::stack_z_offsets::BORDER")

[BORDER\_GRADIENT](constant.BORDER_GRADIENT.html "constant bevy::ui_render::stack_z_offsets::BORDER_GRADIENT")

[BOX\_SHADOW](constant.BOX_SHADOW.html "constant bevy::ui_render::stack_z_offsets::BOX_SHADOW")

[GRADIENT](constant.GRADIENT.html "constant bevy::ui_render::stack_z_offsets::GRADIENT")

[IMAGE](constant.IMAGE.html "constant bevy::ui_render::stack_z_offsets::IMAGE")

[MATERIAL](constant.MATERIAL.html "constant bevy::ui_render::stack_z_offsets::MATERIAL")

[TEXT](constant.TEXT.html "constant bevy::ui_render::stack_z_offsets::TEXT")

[TEXT\_CURSOR](constant.TEXT_CURSOR.html "constant bevy::ui_render::stack_z_offsets::TEXT_CURSOR")

[TEXT\_SELECTION](constant.TEXT_SELECTION.html "constant bevy::ui_render::stack_z_offsets::TEXT_SELECTION")

[TEXT\_STRIKETHROUGH](constant.TEXT_STRIKETHROUGH.html "constant bevy::ui_render::stack_z_offsets::TEXT_STRIKETHROUGH")