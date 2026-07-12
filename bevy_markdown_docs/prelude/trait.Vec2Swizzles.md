[bevy](../index.html)::[prelude](index.html)

# Trait Vec2Swizzles 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#3)

```rust
pub trait Vec2Swizzles:
    Sized
    + Copy
    + Clone {
    type Vec3;
    type Vec4;

    // Required methods
    fn xx(self) -> Self;
    fn yx(self) -> Self;
    fn yy(self) -> Self;
    fn xxx(self) -> Self::Vec3;
    fn xxy(self) -> Self::Vec3;
    fn xyx(self) -> Self::Vec3;
    fn xyy(self) -> Self::Vec3;
    fn yxx(self) -> Self::Vec3;
    fn yxy(self) -> Self::Vec3;
    fn yyx(self) -> Self::Vec3;
    fn yyy(self) -> Self::Vec3;
    fn xxxx(self) -> Self::Vec4;
    fn xxxy(self) -> Self::Vec4;
    fn xxyx(self) -> Self::Vec4;
    fn xxyy(self) -> Self::Vec4;
    fn xyxx(self) -> Self::Vec4;
    fn xyxy(self) -> Self::Vec4;
    fn xyyx(self) -> Self::Vec4;
    fn xyyy(self) -> Self::Vec4;
    fn yxxx(self) -> Self::Vec4;
    fn yxxy(self) -> Self::Vec4;
    fn yxyx(self) -> Self::Vec4;
    fn yxyy(self) -> Self::Vec4;
    fn yyxx(self) -> Self::Vec4;
    fn yyxy(self) -> Self::Vec4;
    fn yyyx(self) -> Self::Vec4;
    fn yyyy(self) -> Self::Vec4;

    // Provided method
    fn xy(self) -> Self { ... }
}
```

## Required Associated Types

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#4)

#### type [Vec3](#associatedtype.Vec3)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#6)

#### type [Vec4](#associatedtype.Vec4)

## Required Methods

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#15)

#### fn [xx](#tymethod.xx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#18)

#### fn [yx](#tymethod.yx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#21)

#### fn [yy](#tymethod.yy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#24)

#### fn [xxx](#tymethod.xxx)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#27)

#### fn [xxy](#tymethod.xxy)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#30)

#### fn [xyx](#tymethod.xyx)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#33)

#### fn [xyy](#tymethod.xyy)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#36)

#### fn [yxx](#tymethod.yxx)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#39)

#### fn [yxy](#tymethod.yxy)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#42)

#### fn [yyx](#tymethod.yyx)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#45)

#### fn [yyy](#tymethod.yyy)(self) -> Self::[Vec3](trait.Vec2Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec2Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#48)

#### fn [xxxx](#tymethod.xxxx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#51)

#### fn [xxxy](#tymethod.xxxy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#54)

#### fn [xxyx](#tymethod.xxyx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#57)

#### fn [xxyy](#tymethod.xxyy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#60)

#### fn [xyxx](#tymethod.xyxx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#63)

#### fn [xyxy](#tymethod.xyxy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#66)

#### fn [xyyx](#tymethod.xyyx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#69)

#### fn [xyyy](#tymethod.xyyy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#72)

#### fn [yxxx](#tymethod.yxxx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#75)

#### fn [yxxy](#tymethod.yxxy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#78)

#### fn [yxyx](#tymethod.yxyx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#81)

#### fn [yxyy](#tymethod.yxyy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#84)

#### fn [yyxx](#tymethod.yyxx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#87)

#### fn [yyxy](#tymethod.yyxy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#90)

#### fn [yyyx](#tymethod.yyyx)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#93)

#### fn [yyyy](#tymethod.yyyy)(self) -> Self::[Vec4](trait.Vec2Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec2Swizzles::Vec4")

## Provided Methods

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#10)

#### fn [xy](#method.xy)(self) -> Self

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/2d/2d\_viewport\_to\_world.rs ([line 34](../../src/2d_viewport_to_world/2d_viewport_to_world.rs.html#34))

```rust
22fn draw_cursor(
23    camera_query: Single<(&Camera, &GlobalTransform)>,
24    window: Single<&Window>,
25    mut gizmos: Gizmos,
26) {
27    let (camera, camera_transform) = *camera_query;
28
29    if let Some(cursor_position) = window.cursor_position()
30        // Calculate a world position based on the cursor's position.
31        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
32        // To test Camera::world_to_viewport, convert result back to viewport space and then back to world space.
33        && let Ok(viewport_check) = camera.world_to_viewport(camera_transform, world_pos.extend(0.0))
34        && let Ok(world_check) = camera.viewport_to_world_2d(camera_transform, viewport_check.xy())
35    {
36        gizmos.circle_2d(world_pos, 10., WHITE);
37        // Should be the same as world_pos
38        gizmos.circle_2d(world_check, 8., RED);
39    }
40}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [DVec2](../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [DVec3](../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [I8Vec2](../math/struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [I8Vec3](../math/struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [I16Vec2](../math/struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [I16Vec3](../math/struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [I64Vec2](../math/struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [I64Vec3](../math/struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [ISizeVec2](../math/struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [ISizeVec3](../math/struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [IVec2](struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [IVec3](struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [U8Vec2](../math/struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [U8Vec3](../math/struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [U16Vec2](../math/struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [U16Vec3](../math/struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [U64Vec2](../math/struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [U64Vec3](../math/struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [USizeVec2](../math/struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [USizeVec3](../math/struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#5)

### impl [Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles") for [Vec2](struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#6)

#### type [Vec3](#associatedtype.Vec3) = [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec2_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [Vec4](struct.Vec4.html "struct bevy::prelude::Vec4")