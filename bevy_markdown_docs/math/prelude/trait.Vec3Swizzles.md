[bevy](../../index.html)::[math](../index.html)::[prelude](index.html)

# Trait Vec3Swizzles 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#96)

```rust
pub trait Vec3Swizzles:
    Sized
    + Copy
    + Clone {
    type Vec2;
    type Vec4;

    // Required methods
    fn xx(self) -> Self::Vec2;
    fn xy(self) -> Self::Vec2;
    fn with_xy(self, rhs: Self::Vec2) -> Self;
    fn xz(self) -> Self::Vec2;
    fn with_xz(self, rhs: Self::Vec2) -> Self;
    fn yx(self) -> Self::Vec2;
    fn with_yx(self, rhs: Self::Vec2) -> Self;
    fn yy(self) -> Self::Vec2;
    fn yz(self) -> Self::Vec2;
    fn with_yz(self, rhs: Self::Vec2) -> Self;
    fn zx(self) -> Self::Vec2;
    fn with_zx(self, rhs: Self::Vec2) -> Self;
    fn zy(self) -> Self::Vec2;
    fn with_zy(self, rhs: Self::Vec2) -> Self;
    fn zz(self) -> Self::Vec2;
    fn xxx(self) -> Self;
    fn xxy(self) -> Self;
    fn xxz(self) -> Self;
    fn xyx(self) -> Self;
    fn xyy(self) -> Self;
    fn xzx(self) -> Self;
    fn xzy(self) -> Self;
    fn xzz(self) -> Self;
    fn yxx(self) -> Self;
    fn yxy(self) -> Self;
    fn yxz(self) -> Self;
    fn yyx(self) -> Self;
    fn yyy(self) -> Self;
    fn yyz(self) -> Self;
    fn yzx(self) -> Self;
    fn yzy(self) -> Self;
    fn yzz(self) -> Self;
    fn zxx(self) -> Self;
    fn zxy(self) -> Self;
    fn zxz(self) -> Self;
    fn zyx(self) -> Self;
    fn zyy(self) -> Self;
    fn zyz(self) -> Self;
    fn zzx(self) -> Self;
    fn zzy(self) -> Self;
    fn zzz(self) -> Self;
    fn xxxx(self) -> Self::Vec4;
    fn xxxy(self) -> Self::Vec4;
    fn xxxz(self) -> Self::Vec4;
    fn xxyx(self) -> Self::Vec4;
    fn xxyy(self) -> Self::Vec4;
    fn xxyz(self) -> Self::Vec4;
    fn xxzx(self) -> Self::Vec4;
    fn xxzy(self) -> Self::Vec4;
    fn xxzz(self) -> Self::Vec4;
    fn xyxx(self) -> Self::Vec4;
    fn xyxy(self) -> Self::Vec4;
    fn xyxz(self) -> Self::Vec4;
    fn xyyx(self) -> Self::Vec4;
    fn xyyy(self) -> Self::Vec4;
    fn xyyz(self) -> Self::Vec4;
    fn xyzx(self) -> Self::Vec4;
    fn xyzy(self) -> Self::Vec4;
    fn xyzz(self) -> Self::Vec4;
    fn xzxx(self) -> Self::Vec4;
    fn xzxy(self) -> Self::Vec4;
    fn xzxz(self) -> Self::Vec4;
    fn xzyx(self) -> Self::Vec4;
    fn xzyy(self) -> Self::Vec4;
    fn xzyz(self) -> Self::Vec4;
    fn xzzx(self) -> Self::Vec4;
    fn xzzy(self) -> Self::Vec4;
    fn xzzz(self) -> Self::Vec4;
    fn yxxx(self) -> Self::Vec4;
    fn yxxy(self) -> Self::Vec4;
    fn yxxz(self) -> Self::Vec4;
    fn yxyx(self) -> Self::Vec4;
    fn yxyy(self) -> Self::Vec4;
    fn yxyz(self) -> Self::Vec4;
    fn yxzx(self) -> Self::Vec4;
    fn yxzy(self) -> Self::Vec4;
    fn yxzz(self) -> Self::Vec4;
    fn yyxx(self) -> Self::Vec4;
    fn yyxy(self) -> Self::Vec4;
    fn yyxz(self) -> Self::Vec4;
    fn yyyx(self) -> Self::Vec4;
    fn yyyy(self) -> Self::Vec4;
    fn yyyz(self) -> Self::Vec4;
    fn yyzx(self) -> Self::Vec4;
    fn yyzy(self) -> Self::Vec4;
    fn yyzz(self) -> Self::Vec4;
    fn yzxx(self) -> Self::Vec4;
    fn yzxy(self) -> Self::Vec4;
    fn yzxz(self) -> Self::Vec4;
    fn yzyx(self) -> Self::Vec4;
    fn yzyy(self) -> Self::Vec4;
    fn yzyz(self) -> Self::Vec4;
    fn yzzx(self) -> Self::Vec4;
    fn yzzy(self) -> Self::Vec4;
    fn yzzz(self) -> Self::Vec4;
    fn zxxx(self) -> Self::Vec4;
    fn zxxy(self) -> Self::Vec4;
    fn zxxz(self) -> Self::Vec4;
    fn zxyx(self) -> Self::Vec4;
    fn zxyy(self) -> Self::Vec4;
    fn zxyz(self) -> Self::Vec4;
    fn zxzx(self) -> Self::Vec4;
    fn zxzy(self) -> Self::Vec4;
    fn zxzz(self) -> Self::Vec4;
    fn zyxx(self) -> Self::Vec4;
    fn zyxy(self) -> Self::Vec4;
    fn zyxz(self) -> Self::Vec4;
    fn zyyx(self) -> Self::Vec4;
    fn zyyy(self) -> Self::Vec4;
    fn zyyz(self) -> Self::Vec4;
    fn zyzx(self) -> Self::Vec4;
    fn zyzy(self) -> Self::Vec4;
    fn zyzz(self) -> Self::Vec4;
    fn zzxx(self) -> Self::Vec4;
    fn zzxy(self) -> Self::Vec4;
    fn zzxz(self) -> Self::Vec4;
    fn zzyx(self) -> Self::Vec4;
    fn zzyy(self) -> Self::Vec4;
    fn zzyz(self) -> Self::Vec4;
    fn zzzx(self) -> Self::Vec4;
    fn zzzy(self) -> Self::Vec4;
    fn zzzz(self) -> Self::Vec4;

    // Provided method
    fn xyz(self) -> Self { ... }
}
```

## Required Associated Types

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#97)

#### type [Vec2](#associatedtype.Vec2)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#99)

#### type [Vec4](#associatedtype.Vec4)

## Required Methods

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#108)

#### fn [xx](#tymethod.xx)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#111)

#### fn [xy](#tymethod.xy)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#114)

#### fn [with\_xy](#tymethod.with_xy)(self, rhs: Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#117)

#### fn [xz](#tymethod.xz)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#120)

#### fn [with\_xz](#tymethod.with_xz)(self, rhs: Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#123)

#### fn [yx](#tymethod.yx)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#126)

#### fn [with\_yx](#tymethod.with_yx)(self, rhs: Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#129)

#### fn [yy](#tymethod.yy)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#132)

#### fn [yz](#tymethod.yz)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#135)

#### fn [with\_yz](#tymethod.with_yz)(self, rhs: Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#138)

#### fn [zx](#tymethod.zx)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#141)

#### fn [with\_zx](#tymethod.with_zx)(self, rhs: Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#144)

#### fn [zy](#tymethod.zy)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#147)

#### fn [with\_zy](#tymethod.with_zy)(self, rhs: Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#150)

#### fn [zz](#tymethod.zz)(self) -> Self::[Vec2](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec3Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#153)

#### fn [xxx](#tymethod.xxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#156)

#### fn [xxy](#tymethod.xxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#159)

#### fn [xxz](#tymethod.xxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#162)

#### fn [xyx](#tymethod.xyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#165)

#### fn [xyy](#tymethod.xyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#168)

#### fn [xzx](#tymethod.xzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#171)

#### fn [xzy](#tymethod.xzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#174)

#### fn [xzz](#tymethod.xzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#177)

#### fn [yxx](#tymethod.yxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#180)

#### fn [yxy](#tymethod.yxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#183)

#### fn [yxz](#tymethod.yxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#186)

#### fn [yyx](#tymethod.yyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#189)

#### fn [yyy](#tymethod.yyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#192)

#### fn [yyz](#tymethod.yyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#195)

#### fn [yzx](#tymethod.yzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#198)

#### fn [yzy](#tymethod.yzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#201)

#### fn [yzz](#tymethod.yzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#204)

#### fn [zxx](#tymethod.zxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#207)

#### fn [zxy](#tymethod.zxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#210)

#### fn [zxz](#tymethod.zxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#213)

#### fn [zyx](#tymethod.zyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#216)

#### fn [zyy](#tymethod.zyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#219)

#### fn [zyz](#tymethod.zyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#222)

#### fn [zzx](#tymethod.zzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#225)

#### fn [zzy](#tymethod.zzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#228)

#### fn [zzz](#tymethod.zzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#231)

#### fn [xxxx](#tymethod.xxxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#234)

#### fn [xxxy](#tymethod.xxxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#237)

#### fn [xxxz](#tymethod.xxxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#240)

#### fn [xxyx](#tymethod.xxyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#243)

#### fn [xxyy](#tymethod.xxyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#246)

#### fn [xxyz](#tymethod.xxyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#249)

#### fn [xxzx](#tymethod.xxzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#252)

#### fn [xxzy](#tymethod.xxzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#255)

#### fn [xxzz](#tymethod.xxzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#258)

#### fn [xyxx](#tymethod.xyxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#261)

#### fn [xyxy](#tymethod.xyxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#264)

#### fn [xyxz](#tymethod.xyxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#267)

#### fn [xyyx](#tymethod.xyyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#270)

#### fn [xyyy](#tymethod.xyyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#273)

#### fn [xyyz](#tymethod.xyyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#276)

#### fn [xyzx](#tymethod.xyzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#279)

#### fn [xyzy](#tymethod.xyzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#282)

#### fn [xyzz](#tymethod.xyzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#285)

#### fn [xzxx](#tymethod.xzxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#288)

#### fn [xzxy](#tymethod.xzxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#291)

#### fn [xzxz](#tymethod.xzxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#294)

#### fn [xzyx](#tymethod.xzyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#297)

#### fn [xzyy](#tymethod.xzyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#300)

#### fn [xzyz](#tymethod.xzyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#303)

#### fn [xzzx](#tymethod.xzzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#306)

#### fn [xzzy](#tymethod.xzzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#309)

#### fn [xzzz](#tymethod.xzzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#312)

#### fn [yxxx](#tymethod.yxxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#315)

#### fn [yxxy](#tymethod.yxxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#318)

#### fn [yxxz](#tymethod.yxxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#321)

#### fn [yxyx](#tymethod.yxyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#324)

#### fn [yxyy](#tymethod.yxyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#327)

#### fn [yxyz](#tymethod.yxyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#330)

#### fn [yxzx](#tymethod.yxzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#333)

#### fn [yxzy](#tymethod.yxzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#336)

#### fn [yxzz](#tymethod.yxzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#339)

#### fn [yyxx](#tymethod.yyxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#342)

#### fn [yyxy](#tymethod.yyxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#345)

#### fn [yyxz](#tymethod.yyxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#348)

#### fn [yyyx](#tymethod.yyyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#351)

#### fn [yyyy](#tymethod.yyyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#354)

#### fn [yyyz](#tymethod.yyyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#357)

#### fn [yyzx](#tymethod.yyzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#360)

#### fn [yyzy](#tymethod.yyzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#363)

#### fn [yyzz](#tymethod.yyzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#366)

#### fn [yzxx](#tymethod.yzxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#369)

#### fn [yzxy](#tymethod.yzxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#372)

#### fn [yzxz](#tymethod.yzxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#375)

#### fn [yzyx](#tymethod.yzyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#378)

#### fn [yzyy](#tymethod.yzyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#381)

#### fn [yzyz](#tymethod.yzyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#384)

#### fn [yzzx](#tymethod.yzzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#387)

#### fn [yzzy](#tymethod.yzzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#390)

#### fn [yzzz](#tymethod.yzzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#393)

#### fn [zxxx](#tymethod.zxxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#396)

#### fn [zxxy](#tymethod.zxxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#399)

#### fn [zxxz](#tymethod.zxxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#402)

#### fn [zxyx](#tymethod.zxyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#405)

#### fn [zxyy](#tymethod.zxyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#408)

#### fn [zxyz](#tymethod.zxyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#411)

#### fn [zxzx](#tymethod.zxzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#414)

#### fn [zxzy](#tymethod.zxzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#417)

#### fn [zxzz](#tymethod.zxzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#420)

#### fn [zyxx](#tymethod.zyxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#423)

#### fn [zyxy](#tymethod.zyxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#426)

#### fn [zyxz](#tymethod.zyxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#429)

#### fn [zyyx](#tymethod.zyyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#432)

#### fn [zyyy](#tymethod.zyyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#435)

#### fn [zyyz](#tymethod.zyyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#438)

#### fn [zyzx](#tymethod.zyzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#441)

#### fn [zyzy](#tymethod.zyzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#444)

#### fn [zyzz](#tymethod.zyzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#447)

#### fn [zzxx](#tymethod.zzxx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#450)

#### fn [zzxy](#tymethod.zzxy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#453)

#### fn [zzxz](#tymethod.zzxz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#456)

#### fn [zzyx](#tymethod.zzyx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#459)

#### fn [zzyy](#tymethod.zzyy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#462)

#### fn [zzyz](#tymethod.zzyz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#465)

#### fn [zzzx](#tymethod.zzzx)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#468)

#### fn [zzzy](#tymethod.zzzy)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#471)

#### fn [zzzz](#tymethod.zzzz)(self) -> Self::[Vec4](../../prelude/trait.Vec3Swizzles.html#associatedtype.Vec4 "type bevy::prelude::Vec3Swizzles::Vec4")

## Provided Methods

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#103)

#### fn [xyz](#method.xyz)(self) -> Self

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [DVec3](../struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [DVec2](../struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [DVec4](../struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [I8Vec3](../struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [I8Vec2](../struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [I8Vec4](../struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [I16Vec3](../struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [I16Vec2](../struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [I16Vec4](../struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [I64Vec3](../struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [I64Vec2](../struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [I64Vec4](../struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [ISizeVec3](../struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [ISizeVec2](../struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [ISizeVec4](../struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [IVec3](../../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [IVec2](../../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [IVec4](../../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [U8Vec3](../struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [U8Vec2](../struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [U8Vec4](../struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [U16Vec3](../struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [U16Vec2](../struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [U16Vec4](../struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [U64Vec3](../struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [U64Vec2](../struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [U64Vec4](../struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [USizeVec3](../struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [USizeVec2](../struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [USizeVec4](../struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [UVec3](../../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [UVec2](../../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [UVec4](../../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#5)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [Vec3](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec3_impl.rs.html#8)

#### type [Vec4](#associatedtype.Vec4) = [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec3a_impl.rs.html#12)

### impl [Vec3Swizzles](../../prelude/trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles") for [Vec3A](../../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec3a_impl.rs.html#13)

#### type [Vec2](#associatedtype.Vec2) = [Vec2](../../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec3a_impl.rs.html#15)

#### type [Vec4](#associatedtype.Vec4) = [Vec4](../../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")