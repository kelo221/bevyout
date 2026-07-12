[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)

# Module lifetimeless 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2107)

Contains type aliases for built-in [`SystemParam`](../trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s with `'static` lifetimes. This makes it more convenient to refer to these types in contexts where explicit lifetime annotations are required.

Note that this is entirely safe and tracks lifetimes correctly. This purely exists for convenience.

You can’t instantiate a static `SystemParam`, you’ll always end up with `Res<'w, T>`, `ResMut<'w, T>` or `&'w T` bound to the lifetime of the provided `&'w World`.

## Type Aliases

[Read](type.Read.html "type bevy::ecs::system::lifetimeless::Read")

A shorthand for writing `&'static T`.

[SCommands](type.SCommands.html "type bevy::ecs::system::lifetimeless::SCommands")

[`Commands`](../../../prelude/struct.Commands.html "struct bevy::prelude::Commands") with `'static` lifetimes.

[SQuery](type.SQuery.html "type bevy::ecs::system::lifetimeless::SQuery")

A [`Query`](../../../prelude/struct.Query.html "struct bevy::prelude::Query") with `'static` lifetimes.

[SRes](type.SRes.html "type bevy::ecs::system::lifetimeless::SRes")

A [`Res`](../../../prelude/struct.Res.html "struct bevy::prelude::Res") with `'static` lifetimes.

[SResMut](type.SResMut.html "type bevy::ecs::system::lifetimeless::SResMut")

A [`ResMut`](../../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut") with `'static` lifetimes.

[Write](type.Write.html "type bevy::ecs::system::lifetimeless::Write")

A shorthand for writing `&'static mut T`.