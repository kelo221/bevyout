[bevy](../index.html)::[reflect](index.html)

# Trait FromType 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#824)

```rust
pub trait FromType<T> {
    // Required method
    fn from_type() -> Self;

    // Provided method
    fn insert_dependencies(_type_registration: &mut TypeRegistration) { ... }
}
```

Trait used to generate [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData") for trait reflection.

This is used by the `#[derive(Reflect)]` macro to generate an implementation of [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData") to pass to [`TypeRegistration::insert`](struct.TypeRegistration.html#method.insert "method bevy::reflect::TypeRegistration::insert").

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#826)

#### fn [from\_type](#tymethod.from_type)() -> Self

Creates an instance of `Self` for type `T`.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#830)

#### fn [insert\_dependencies](#method.insert_dependencies)(\_type\_registration: &mut [TypeRegistration](struct.TypeRegistration.html "struct bevy::reflect::TypeRegistration"))

Inserts [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData") dependencies of this [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData"). This is especially useful for trait [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData") that has a supertrait (ex: `A: B`). When the [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData") for `A` is inserted, the `B` [`TypeData`](trait.TypeData.html "trait bevy::reflect::TypeData") will also be inserted.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/event.rs.html#125-127)

### impl<'a, E> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<E> for [ReflectEvent](../prelude/struct.ReflectEvent.html "struct bevy::prelude::ReflectEvent")

where E: [Event](../prelude/trait.Event.html "trait bevy::prelude::Event") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), <E as [Event](../prelude/trait.Event.html "trait bevy::prelude::Event")\>::[Trigger](../prelude/trait.Event.html#associatedtype.Trigger "type bevy::prelude::Event::Trigger")<'a>: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#159)

### impl<A> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<A> for [ReflectAsset](../asset/struct.ReflectAsset.html "struct bevy::asset::ReflectAsset")

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/reflect.rs.html#256)

### impl<A> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<[Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>> for [ReflectHandle](../asset/struct.ReflectHandle.html "struct bevy::asset::ReflectHandle")

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/bundle.rs.html#151)

### impl<B> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<B> for [ReflectBundle](../ecs/reflect/struct.ReflectBundle.html "struct bevy::ecs::reflect::ReflectBundle")

where B: [Bundle](../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BundleFromComponents](../ecs/bundle/trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/from_world.rs.html#81)

### impl<B> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<B> for [ReflectFromWorld](../prelude/struct.ReflectFromWorld.html "struct bevy::prelude::ReflectFromWorld")

where B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/component.rs.html#309)

### impl<C> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<C> for [ReflectComponent](../prelude/struct.ReflectComponent.html "struct bevy::prelude::ReflectComponent")

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/map_entities.rs.html#28)

### impl<C> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<C> for [ReflectMapEntities](../ecs/reflect/struct.ReflectMapEntities.html "struct bevy::ecs::reflect::ReflectMapEntities")

where C: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [MapEntities](../ecs/entity/trait.MapEntities.html "trait bevy::ecs::entity::MapEntities"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/message.rs.html#100)

### impl<M> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<M> for [ReflectMessage](../prelude/struct.ReflectMessage.html "struct bevy::prelude::ReflectMessage")

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/resource.rs.html#33)

### impl<R> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<R> for [ReflectResource](../prelude/struct.ReflectResource.html "struct bevy::prelude::ReflectResource")

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/reflect.rs.html#93)

### impl<S> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<S> for [ReflectFreelyMutableState](../prelude/struct.ReflectFreelyMutableState.html "struct bevy::prelude::ReflectFreelyMutableState")

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/reflect.rs.html#38)

### impl<S> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<S> for [ReflectState](../prelude/struct.ReflectState.html "struct bevy::prelude::ReflectState")

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#59)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectAdd](../prelude/struct.ReflectAdd.html "struct bevy::prelude::ReflectAdd")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add"), <T as [Add](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html "trait core::ops::arith::Add")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html#associatedtype.Output "type core::ops::arith::Add::Output"): [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#310)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectAddAssign](../prelude/struct.ReflectAddAssign.html "struct bevy::prelude::ReflectAddAssign")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [AddAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.AddAssign.html "trait core::ops::arith::AddAssign"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#23)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectDefault](../prelude/struct.ReflectDefault.html "struct bevy::prelude::ReflectDefault")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#905)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectDeserialize](../prelude/struct.ReflectDeserialize.html "struct bevy::prelude::ReflectDeserialize")

where T: for<'a> [Deserialize](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize")<'a> + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/de/deserialize_with_registry.rs.html#77-78)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectDeserializeWithRegistry](serde/struct.ReflectDeserializeWithRegistry.html "struct bevy::reflect::serde::ReflectDeserializeWithRegistry")

where T: [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + for<'de> [DeserializeWithRegistry](serde/trait.DeserializeWithRegistry.html "trait bevy::reflect::serde::DeserializeWithRegistry")<'de>,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#209)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectDiv](../prelude/struct.ReflectDiv.html "struct bevy::prelude::ReflectDiv")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div"), <T as [Div](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html "trait core::ops::arith::Div")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Div.html#associatedtype.Output "type core::ops::arith::Div::Output"): [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#448)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectDivAssign](../prelude/struct.ReflectDivAssign.html "struct bevy::prelude::ReflectDivAssign")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [DivAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.DivAssign.html "trait core::ops::arith::DivAssign"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#1009)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectFromPtr](struct.ReflectFromPtr.html "struct bevy::reflect::ReflectFromPtr")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/from_reflect.rs.html#120)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectFromReflect](../prelude/struct.ReflectFromReflect.html "struct bevy::prelude::ReflectFromReflect")

where T: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#159)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectMul](../prelude/struct.ReflectMul.html "struct bevy::prelude::ReflectMul")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul"), <T as [Mul](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html "trait core::ops::arith::Mul")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Mul.html#associatedtype.Output "type core::ops::arith::Mul::Output"): [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#402)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectMulAssign](../prelude/struct.ReflectMulAssign.html "struct bevy::prelude::ReflectMulAssign")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [MulAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.MulAssign.html "trait core::ops::arith::MulAssign"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#260)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectRem](../prelude/struct.ReflectRem.html "struct bevy::prelude::ReflectRem")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem"), <T as [Rem](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html "trait core::ops::arith::Rem")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Rem.html#associatedtype.Output "type core::ops::arith::Rem::Output"): [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#494)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectRemAssign](../prelude/struct.ReflectRemAssign.html "struct bevy::prelude::ReflectRemAssign")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [RemAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.RemAssign.html "trait core::ops::arith::RemAssign"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_registry.rs.html#842)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectSerialize](../prelude/struct.ReflectSerialize.html "struct bevy::prelude::ReflectSerialize")

where T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [Serialize](erased_serde/trait.Serialize.html "trait bevy::reflect::erased_serde::Serialize"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/serde/ser/serialize_with_registry.rs.html#75)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectSerializeWithRegistry](serde/struct.ReflectSerializeWithRegistry.html "struct bevy::reflect::serde::ReflectSerializeWithRegistry")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [SerializeWithRegistry](serde/trait.SerializeWithRegistry.html "trait bevy::reflect::serde::SerializeWithRegistry"),

[Source](https://docs.rs/bevy-settings/0.19.0/x86_64-unknown-linux-gnu/src/bevy_settings/lib.rs.html#165)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectSettingsGroup](../settings/struct.ReflectSettingsGroup.html "struct bevy::settings::ReflectSettingsGroup")

where T: [SettingsGroup](../settings/trait.SettingsGroup.html "trait bevy::settings::SettingsGroup") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#109)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectSub](../prelude/struct.ReflectSub.html "struct bevy::prelude::ReflectSub")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub"), <T as [Sub](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html "trait core::ops::arith::Sub")\>::[Output](https://doc.rust-lang.org/nightly/core/ops/arith/trait.Sub.html#associatedtype.Output "type core::ops::arith::Sub::Output"): [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/std_traits.rs.html#356)

### impl<T> [FromType](trait.FromType.html "trait bevy::reflect::FromType")<T> for [ReflectSubAssign](../prelude/struct.ReflectSubAssign.html "struct bevy::prelude::ReflectSubAssign")

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [SubAssign](https://doc.rust-lang.org/nightly/core/ops/arith/trait.SubAssign.html "trait core::ops::arith::SubAssign"),