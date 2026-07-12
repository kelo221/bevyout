[bevy](../../index.html)::[render](../index.html)::[diagnostic](index.html)

# Trait RecordDiagnostics 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#133)

```rust
pub trait RecordDiagnostics: Send + Sync {
    // Required methods
    fn record_f32<N>(
        &self,
        command_encoder: &mut CommandEncoder,
        buffer: &BufferSlice<'_>,
        name: N,
    )
       where N: Into<Cow<'static, str>>;
    fn record_u32<N>(
        &self,
        command_encoder: &mut CommandEncoder,
        buffer: &BufferSlice<'_>,
        name: N,
    )
       where N: Into<Cow<'static, str>>;

    // Provided methods
    fn time_span<E, N>(
        &self,
        encoder: &mut E,
        name: N,
    ) -> TimeSpanGuard<'_, Self, E>
       where E: WriteTimestamp,
             N: Into<Cow<'static, str>> { ... }
    fn pass_span<P, N>(
        &self,
        pass: &mut P,
        name: N,
    ) -> PassSpanGuard<'_, Self, P>
       where P: Pass,
             N: Into<Cow<'static, str>> { ... }
}
```

Allows recording diagnostic spans.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#170-172)

#### fn [record\_f32](#tymethod.record_f32)<N>( &self, command\_encoder: &mut [CommandEncoder](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder"), buffer: &[BufferSlice](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html "struct wgpu::api::buffer::BufferSlice")<'\_>, name: N, )

where N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

Reads a f32 from the specified buffer and uploads it as a diagnostic.

The provided buffer slice must be 4 bytes long, and the buffer must have [`wgpu::BufferUsages::COPY_SRC`](../render_resource/struct.BufferUsages.html#associatedconstant.COPY_SRC "associated constant bevy::render::render_resource::BufferUsages::COPY_SRC");

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#177-179)

#### fn [record\_u32](#tymethod.record_u32)<N>( &self, command\_encoder: &mut [CommandEncoder](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder"), buffer: &[BufferSlice](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html "struct wgpu::api::buffer::BufferSlice")<'\_>, name: N, )

where N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

Reads a u32 from the specified buffer and uploads it as a diagnostic.

The provided buffer slice must be 4 bytes long, and the buffer must have [`wgpu::BufferUsages::COPY_SRC`](../render_resource/struct.BufferUsages.html#associatedconstant.COPY_SRC "associated constant bevy::render::render_resource::BufferUsages::COPY_SRC");

## Provided Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#137-140)

#### fn [time\_span](#method.time_span)<E, N>( &self, encoder: [&mut E](https://doc.rust-lang.org/nightly/std/primitive.reference.html), name: N, ) -> [TimeSpanGuard](struct.TimeSpanGuard.html "struct bevy::render::diagnostic::TimeSpanGuard")<'\_, Self, E>

where E: WriteTimestamp, N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

Begin a time span, which will record elapsed CPU and GPU time.

Returns a guard, which will panic on drop unless you end the span.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#153-156)

#### fn [pass\_span](#method.pass_span)<P, N>(&self, pass: [&mut P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), name: N) -> [PassSpanGuard](struct.PassSpanGuard.html "struct bevy::render::diagnostic::PassSpanGuard")<'\_, Self, P>

where P: Pass, N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

Begin a pass span, which will record elapsed CPU and GPU time, as well as pipeline statistics on supported platforms.

Returns a guard, which will panic on drop unless you end the span.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#283)

### impl<'a, T> [RecordDiagnostics](trait.RecordDiagnostics.html "trait bevy::render::diagnostic::RecordDiagnostics") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [RecordDiagnostics](trait.RecordDiagnostics.html "trait bevy::render::diagnostic::RecordDiagnostics"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#284-286)

#### fn [record\_f32](#tymethod.record_f32)<N>( &self, command\_encoder: &mut [CommandEncoder](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder"), buffer: &[BufferSlice](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html "struct wgpu::api::buffer::BufferSlice")<'\_>, name: N, )

where N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#293-295)

#### fn [record\_u32](#tymethod.record_u32)<N>( &self, command\_encoder: &mut [CommandEncoder](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder"), buffer: &[BufferSlice](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html "struct wgpu::api::buffer::BufferSlice")<'\_>, name: N, )

where N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#239)

### impl<T> [RecordDiagnostics](trait.RecordDiagnostics.html "trait bevy::render::diagnostic::RecordDiagnostics") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>>

where T: [RecordDiagnostics](trait.RecordDiagnostics.html "trait bevy::render::diagnostic::RecordDiagnostics"),

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#240-242)

#### fn [record\_f32](#tymethod.record_f32)<N>( &self, command\_encoder: &mut [CommandEncoder](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder"), buffer: &[BufferSlice](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html "struct wgpu::api::buffer::BufferSlice")<'\_>, name: N, )

where N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/mod.rs.html#249-251)

#### fn [record\_u32](#tymethod.record_u32)<N>( &self, command\_encoder: &mut [CommandEncoder](../render_resource/struct.CommandEncoder.html "struct bevy::render::render_resource::CommandEncoder"), buffer: &[BufferSlice](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/buffer/struct.BufferSlice.html "struct wgpu::api::buffer::BufferSlice")<'\_>, name: N, )

where N: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>,

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/diagnostic/internal.rs.html#147)

### impl [RecordDiagnostics](trait.RecordDiagnostics.html "trait bevy::render::diagnostic::RecordDiagnostics") for [DiagnosticsRecorder](struct.DiagnosticsRecorder.html "struct bevy::render::diagnostic::DiagnosticsRecorder")