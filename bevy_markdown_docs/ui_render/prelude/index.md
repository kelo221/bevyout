[bevy](../../index.html)::[ui\_render](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#85)

## Structs

[BoxShadowSamples](struct.BoxShadowSamples.html "struct bevy::ui_render::prelude::BoxShadowSamples")

Number of shadow samples. A larger value will result in higher quality shadows. Default is 4, values higher than ~10 offer diminishing returns.

[GlobalUiDebugOptions](struct.GlobalUiDebugOptions.html "struct bevy::ui_render::prelude::GlobalUiDebugOptions")

Configuration for the UI debug overlay

[MaterialNode](struct.MaterialNode.html "struct bevy::ui_render::prelude::MaterialNode")

[MaterialNodeTemplate](struct.MaterialNodeTemplate.html "struct bevy::ui_render::prelude::MaterialNodeTemplate")

[UiDebugOptions](struct.UiDebugOptions.html "struct bevy::ui_render::prelude::UiDebugOptions")

Configuration for the UI debug overlay

[UiMaterialKey](struct.UiMaterialKey.html "struct bevy::ui_render::prelude::UiMaterialKey")

[UiMaterialPlugin](struct.UiMaterialPlugin.html "struct bevy::ui_render::prelude::UiMaterialPlugin")

Adds the necessary ECS resources and render logic to enable rendering entities using the given [`UiMaterial`](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") asset type (which includes [`UiMaterial`](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") types).

## Enums

[UiAntiAlias](enum.UiAntiAlias.html "enum bevy::ui_render::prelude::UiAntiAlias")

Marker for controlling whether UI is rendered with or without anti-aliasing in a camera. By default, UI is always anti-aliased.

## Traits

[UiMaterial](trait.UiMaterial.html "trait bevy::ui_render::prelude::UiMaterial")

Materials are used alongside [`UiMaterialPlugin`](../../prelude/struct.UiMaterialPlugin.html "struct bevy::prelude::UiMaterialPlugin") and [`MaterialNode`](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode") to spawn entities that are rendered with a specific [`UiMaterial`](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") type. They serve as an easy to use high level way to render `Node` entities with custom shader logic.