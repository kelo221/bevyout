[bevy](../../index.html)::[ui\_render](../index.html)

# Module ui\_material 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#15)

## Structs

[MaterialNode](struct.MaterialNode.html "struct bevy::ui_render::ui_material::MaterialNode")

[MaterialNodeTemplate](struct.MaterialNodeTemplate.html "struct bevy::ui_render::ui_material::MaterialNodeTemplate")

[UiMaterialKey](struct.UiMaterialKey.html "struct bevy::ui_render::ui_material::UiMaterialKey")

## Traits

[UiMaterial](trait.UiMaterial.html "trait bevy::ui_render::ui_material::UiMaterial")

Materials are used alongside [`UiMaterialPlugin`](../../prelude/struct.UiMaterialPlugin.html "struct bevy::prelude::UiMaterialPlugin") and [`MaterialNode`](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode") to spawn entities that are rendered with a specific [`UiMaterial`](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") type. They serve as an easy to use high level way to render `Node` entities with custom shader logic.