[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait Specializer 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#176)

```rust
pub trait Specializer<T>:
    Send
    + Sync
    + 'staticwhere
    T: Specializable,{
    type Key: SpecializerKey;

    // Required method
    fn specialize(
        &self,
        key: Self::Key,
        descriptor: &mut <T as Specializable>::Descriptor,
    ) -> Result<<Self::Key as SpecializerKey>::Canonical, BevyError>;
}
```

Defines a type capable of “specializing” values of a type T.

Specialization is the process of generating variants of a type T from small hashable keys, and specializers themselves can be thought of as [pure functions](https://en.wikipedia.org/wiki/Pure_function) from the key type to `T`, that [memoize](https://en.wikipedia.org/wiki/Memoization) their results based on the key.

Because specialization is designed for use with render and compute pipelines, specializers act on _descriptors_ of `T` rather than produce `T` itself, but the above comparison is still valid.

Since compiling render and compute pipelines can be so slow, specialization allows a Bevy app to detect when it would compile a duplicate pipeline and reuse what’s already in the cache. While pipelines could all be memoized hashing each whole descriptor, this would be much slower and could still create duplicates. In contrast, memoizing groups of _related_ pipelines based on a small hashable key is much faster. See the docs on [`SpecializerKey`](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey") for more info.

### Composing Specializers

This trait can be derived with `#[derive(Specializer)]` for structs whose fields all implement [`Specializer`](trait.Specializer.html "trait bevy::render::render_resource::Specializer"). This allows for composing multiple specializers together, and makes encapsulation and separating concerns between specializers much nicer. One could make individual specializers for common operations and place them in entirely separate modules, then compose them together with a single `#[derive]`

```rust
struct A;
struct B;
#[derive(Copy, Clone, PartialEq, Eq, Hash, SpecializerKey)]
struct BKey { contrived_number: u32 };

impl Specializer<RenderPipeline> for A {
    type Key = ();

    fn specialize(
        &self,
        key: (),
        descriptor: &mut RenderPipelineDescriptor
    ) -> Result<(), BevyError>  {
        // mutate the descriptor here
        Ok(key)
    }
}

impl Specializer<RenderPipeline> for B {
    type Key = BKey;

    fn specialize(
        &self,
        key: BKey,
        descriptor: &mut RenderPipelineDescriptor
    ) -> Result<BKey, BevyError> {
        // mutate the descriptor here
        Ok(key)
    }
}

#[derive(Specializer)]
#[specialize(RenderPipeline)]
struct C {
    #[key(default)]
    a: A,
    b: B,
}

/*
The generated implementation:
impl Specializer<RenderPipeline> for C {
    type Key = BKey;
    fn specialize(
        &self,
        key: Self::Key,
        descriptor: &mut RenderPipelineDescriptor
    ) -> Result<Canonical<Self::Key>, BevyError> {
        let _ = self.a.specialize((), descriptor);
        let key = self.b.specialize(key, descriptor);
        Ok(key)
    }
}
*/
```

The key type for a composed specializer will be a tuple of the keys of each field, and their specialization logic will be applied in field order. Since derive macros can’t have generic parameters, the derive macro requires an additional `#[specialize(..targets)]` attribute to specify a list of types to target for the implementation. `#[specialize(all)]` is also allowed, and will generate a fully generic implementation at the cost of slightly worse error messages.

Additionally, each field can optionally take a `#[key]` attribute to specify a “key override”. This will hide that field’s key from being exposed by the wrapper, and always use the value given by the attribute. Values for this attribute may either be `default` which will use the key’s [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") implementation, or a valid rust expression of the key type.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#177)

#### type [Key](#associatedtype.Key): [SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey")

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#178-182)

#### fn [specialize](#tymethod.specialize)( &self, key: Self::[Key](trait.Specializer.html#associatedtype.Key "type bevy::render::render_resource::Specializer::Key"), descriptor: &mut <T as [Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable")\>::[Descriptor](trait.Specializable.html#associatedtype.Descriptor "type bevy::render::render_resource::Specializable::Descriptor"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<Self::[Key](trait.Specializer.html#associatedtype.Key "type bevy::render::render_resource::Specializer::Key") as [SpecializerKey](trait.SpecializerKey.html "trait bevy::render::render_resource::SpecializerKey")\>::[Canonical](trait.SpecializerKey.html#associatedtype.Canonical "type bevy::render::render_resource::SpecializerKey::Canonical"), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#237)

### impl<T, V> [Specializer](trait.Specializer.html "trait bevy::render::render_resource::Specializer")<T> for [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<V>

where T: [Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable"), V: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#238)

#### type [Key](#associatedtype.Key) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#240-244)

#### fn [specialize](#tymethod.specialize)( &self, \_key: <[PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<V> as [Specializer](trait.Specializer.html "trait bevy::render::render_resource::Specializer")<T>>::[Key](trait.Specializer.html#associatedtype.Key "type bevy::render::render_resource::Specializer::Key"), \_descriptor: &mut <T as [Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable")\>::[Descriptor](trait.Specializable.html#associatedtype.Descriptor "type bevy::render::render_resource::Specializable::Descriptor"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#225)

### impl<T> [Specializer](trait.Specializer.html "trait bevy::render::render_resource::Specializer")<T> for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

where T: [Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#226)

#### type [Key](#associatedtype.Key) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/specializer.rs.html#228-232)

#### fn [specialize](#tymethod.specialize)( &self, \_key: <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [Specializer](trait.Specializer.html "trait bevy::render::render_resource::Specializer")<T>>::[Key](trait.Specializer.html#associatedtype.Key "type bevy::render::render_resource::Specializer::Key"), \_descriptor: &mut <T as [Specializable](trait.Specializable.html "trait bevy::render::render_resource::Specializable")\>::[Descriptor](trait.Specializable.html#associatedtype.Descriptor "type bevy::render::render_resource::Specializable::Descriptor"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

## Implementors

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#197)

### impl [Specializer](trait.Specializer.html "trait bevy::render::render_resource::Specializer")<[RenderPipeline](struct.RenderPipeline.html "struct bevy::render::render_resource::RenderPipeline")\> for [CasPipelineSpecializer](../../anti_alias/contrast_adaptive_sharpening/struct.CasPipelineSpecializer.html "struct bevy::anti_alias::contrast_adaptive_sharpening::CasPipelineSpecializer")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#198)

#### type [Key](#associatedtype.Key) = [CasPipelineKey](../../anti_alias/contrast_adaptive_sharpening/struct.CasPipelineKey.html "struct bevy::anti_alias::contrast_adaptive_sharpening::CasPipelineKey")