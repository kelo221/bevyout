[bevy](../index.html)::[prelude](index.html)

# Trait Vec4Swizzles 

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#474)

```rust
pub trait Vec4Swizzles:
    Sized
    + Copy
    + Clone {
    type Vec2;
    type Vec3;

    // Required methods
    fn xx(self) -> Self::Vec2;
    fn xy(self) -> Self::Vec2;
    fn with_xy(self, rhs: Self::Vec2) -> Self;
    fn xz(self) -> Self::Vec2;
    fn with_xz(self, rhs: Self::Vec2) -> Self;
    fn xw(self) -> Self::Vec2;
    fn with_xw(self, rhs: Self::Vec2) -> Self;
    fn yx(self) -> Self::Vec2;
    fn with_yx(self, rhs: Self::Vec2) -> Self;
    fn yy(self) -> Self::Vec2;
    fn yz(self) -> Self::Vec2;
    fn with_yz(self, rhs: Self::Vec2) -> Self;
    fn yw(self) -> Self::Vec2;
    fn with_yw(self, rhs: Self::Vec2) -> Self;
    fn zx(self) -> Self::Vec2;
    fn with_zx(self, rhs: Self::Vec2) -> Self;
    fn zy(self) -> Self::Vec2;
    fn with_zy(self, rhs: Self::Vec2) -> Self;
    fn zz(self) -> Self::Vec2;
    fn zw(self) -> Self::Vec2;
    fn with_zw(self, rhs: Self::Vec2) -> Self;
    fn wx(self) -> Self::Vec2;
    fn with_wx(self, rhs: Self::Vec2) -> Self;
    fn wy(self) -> Self::Vec2;
    fn with_wy(self, rhs: Self::Vec2) -> Self;
    fn wz(self) -> Self::Vec2;
    fn with_wz(self, rhs: Self::Vec2) -> Self;
    fn ww(self) -> Self::Vec2;
    fn xxx(self) -> Self::Vec3;
    fn xxy(self) -> Self::Vec3;
    fn xxz(self) -> Self::Vec3;
    fn xxw(self) -> Self::Vec3;
    fn xyx(self) -> Self::Vec3;
    fn xyy(self) -> Self::Vec3;
    fn xyz(self) -> Self::Vec3;
    fn with_xyz(self, rhs: Self::Vec3) -> Self;
    fn xyw(self) -> Self::Vec3;
    fn with_xyw(self, rhs: Self::Vec3) -> Self;
    fn xzx(self) -> Self::Vec3;
    fn xzy(self) -> Self::Vec3;
    fn with_xzy(self, rhs: Self::Vec3) -> Self;
    fn xzz(self) -> Self::Vec3;
    fn xzw(self) -> Self::Vec3;
    fn with_xzw(self, rhs: Self::Vec3) -> Self;
    fn xwx(self) -> Self::Vec3;
    fn xwy(self) -> Self::Vec3;
    fn with_xwy(self, rhs: Self::Vec3) -> Self;
    fn xwz(self) -> Self::Vec3;
    fn with_xwz(self, rhs: Self::Vec3) -> Self;
    fn xww(self) -> Self::Vec3;
    fn yxx(self) -> Self::Vec3;
    fn yxy(self) -> Self::Vec3;
    fn yxz(self) -> Self::Vec3;
    fn with_yxz(self, rhs: Self::Vec3) -> Self;
    fn yxw(self) -> Self::Vec3;
    fn with_yxw(self, rhs: Self::Vec3) -> Self;
    fn yyx(self) -> Self::Vec3;
    fn yyy(self) -> Self::Vec3;
    fn yyz(self) -> Self::Vec3;
    fn yyw(self) -> Self::Vec3;
    fn yzx(self) -> Self::Vec3;
    fn with_yzx(self, rhs: Self::Vec3) -> Self;
    fn yzy(self) -> Self::Vec3;
    fn yzz(self) -> Self::Vec3;
    fn yzw(self) -> Self::Vec3;
    fn with_yzw(self, rhs: Self::Vec3) -> Self;
    fn ywx(self) -> Self::Vec3;
    fn with_ywx(self, rhs: Self::Vec3) -> Self;
    fn ywy(self) -> Self::Vec3;
    fn ywz(self) -> Self::Vec3;
    fn with_ywz(self, rhs: Self::Vec3) -> Self;
    fn yww(self) -> Self::Vec3;
    fn zxx(self) -> Self::Vec3;
    fn zxy(self) -> Self::Vec3;
    fn with_zxy(self, rhs: Self::Vec3) -> Self;
    fn zxz(self) -> Self::Vec3;
    fn zxw(self) -> Self::Vec3;
    fn with_zxw(self, rhs: Self::Vec3) -> Self;
    fn zyx(self) -> Self::Vec3;
    fn with_zyx(self, rhs: Self::Vec3) -> Self;
    fn zyy(self) -> Self::Vec3;
    fn zyz(self) -> Self::Vec3;
    fn zyw(self) -> Self::Vec3;
    fn with_zyw(self, rhs: Self::Vec3) -> Self;
    fn zzx(self) -> Self::Vec3;
    fn zzy(self) -> Self::Vec3;
    fn zzz(self) -> Self::Vec3;
    fn zzw(self) -> Self::Vec3;
    fn zwx(self) -> Self::Vec3;
    fn with_zwx(self, rhs: Self::Vec3) -> Self;
    fn zwy(self) -> Self::Vec3;
    fn with_zwy(self, rhs: Self::Vec3) -> Self;
    fn zwz(self) -> Self::Vec3;
    fn zww(self) -> Self::Vec3;
    fn wxx(self) -> Self::Vec3;
    fn wxy(self) -> Self::Vec3;
    fn with_wxy(self, rhs: Self::Vec3) -> Self;
    fn wxz(self) -> Self::Vec3;
    fn with_wxz(self, rhs: Self::Vec3) -> Self;
    fn wxw(self) -> Self::Vec3;
    fn wyx(self) -> Self::Vec3;
    fn with_wyx(self, rhs: Self::Vec3) -> Self;
    fn wyy(self) -> Self::Vec3;
    fn wyz(self) -> Self::Vec3;
    fn with_wyz(self, rhs: Self::Vec3) -> Self;
    fn wyw(self) -> Self::Vec3;
    fn wzx(self) -> Self::Vec3;
    fn with_wzx(self, rhs: Self::Vec3) -> Self;
    fn wzy(self) -> Self::Vec3;
    fn with_wzy(self, rhs: Self::Vec3) -> Self;
    fn wzz(self) -> Self::Vec3;
    fn wzw(self) -> Self::Vec3;
    fn wwx(self) -> Self::Vec3;
    fn wwy(self) -> Self::Vec3;
    fn wwz(self) -> Self::Vec3;
    fn www(self) -> Self::Vec3;
    fn xxxx(self) -> Self;
    fn xxxy(self) -> Self;
    fn xxxz(self) -> Self;
    fn xxxw(self) -> Self;
    fn xxyx(self) -> Self;
    fn xxyy(self) -> Self;
    fn xxyz(self) -> Self;
    fn xxyw(self) -> Self;
    fn xxzx(self) -> Self;
    fn xxzy(self) -> Self;
    fn xxzz(self) -> Self;
    fn xxzw(self) -> Self;
    fn xxwx(self) -> Self;
    fn xxwy(self) -> Self;
    fn xxwz(self) -> Self;
    fn xxww(self) -> Self;
    fn xyxx(self) -> Self;
    fn xyxy(self) -> Self;
    fn xyxz(self) -> Self;
    fn xyxw(self) -> Self;
    fn xyyx(self) -> Self;
    fn xyyy(self) -> Self;
    fn xyyz(self) -> Self;
    fn xyyw(self) -> Self;
    fn xyzx(self) -> Self;
    fn xyzy(self) -> Self;
    fn xyzz(self) -> Self;
    fn xywx(self) -> Self;
    fn xywy(self) -> Self;
    fn xywz(self) -> Self;
    fn xyww(self) -> Self;
    fn xzxx(self) -> Self;
    fn xzxy(self) -> Self;
    fn xzxz(self) -> Self;
    fn xzxw(self) -> Self;
    fn xzyx(self) -> Self;
    fn xzyy(self) -> Self;
    fn xzyz(self) -> Self;
    fn xzyw(self) -> Self;
    fn xzzx(self) -> Self;
    fn xzzy(self) -> Self;
    fn xzzz(self) -> Self;
    fn xzzw(self) -> Self;
    fn xzwx(self) -> Self;
    fn xzwy(self) -> Self;
    fn xzwz(self) -> Self;
    fn xzww(self) -> Self;
    fn xwxx(self) -> Self;
    fn xwxy(self) -> Self;
    fn xwxz(self) -> Self;
    fn xwxw(self) -> Self;
    fn xwyx(self) -> Self;
    fn xwyy(self) -> Self;
    fn xwyz(self) -> Self;
    fn xwyw(self) -> Self;
    fn xwzx(self) -> Self;
    fn xwzy(self) -> Self;
    fn xwzz(self) -> Self;
    fn xwzw(self) -> Self;
    fn xwwx(self) -> Self;
    fn xwwy(self) -> Self;
    fn xwwz(self) -> Self;
    fn xwww(self) -> Self;
    fn yxxx(self) -> Self;
    fn yxxy(self) -> Self;
    fn yxxz(self) -> Self;
    fn yxxw(self) -> Self;
    fn yxyx(self) -> Self;
    fn yxyy(self) -> Self;
    fn yxyz(self) -> Self;
    fn yxyw(self) -> Self;
    fn yxzx(self) -> Self;
    fn yxzy(self) -> Self;
    fn yxzz(self) -> Self;
    fn yxzw(self) -> Self;
    fn yxwx(self) -> Self;
    fn yxwy(self) -> Self;
    fn yxwz(self) -> Self;
    fn yxww(self) -> Self;
    fn yyxx(self) -> Self;
    fn yyxy(self) -> Self;
    fn yyxz(self) -> Self;
    fn yyxw(self) -> Self;
    fn yyyx(self) -> Self;
    fn yyyy(self) -> Self;
    fn yyyz(self) -> Self;
    fn yyyw(self) -> Self;
    fn yyzx(self) -> Self;
    fn yyzy(self) -> Self;
    fn yyzz(self) -> Self;
    fn yyzw(self) -> Self;
    fn yywx(self) -> Self;
    fn yywy(self) -> Self;
    fn yywz(self) -> Self;
    fn yyww(self) -> Self;
    fn yzxx(self) -> Self;
    fn yzxy(self) -> Self;
    fn yzxz(self) -> Self;
    fn yzxw(self) -> Self;
    fn yzyx(self) -> Self;
    fn yzyy(self) -> Self;
    fn yzyz(self) -> Self;
    fn yzyw(self) -> Self;
    fn yzzx(self) -> Self;
    fn yzzy(self) -> Self;
    fn yzzz(self) -> Self;
    fn yzzw(self) -> Self;
    fn yzwx(self) -> Self;
    fn yzwy(self) -> Self;
    fn yzwz(self) -> Self;
    fn yzww(self) -> Self;
    fn ywxx(self) -> Self;
    fn ywxy(self) -> Self;
    fn ywxz(self) -> Self;
    fn ywxw(self) -> Self;
    fn ywyx(self) -> Self;
    fn ywyy(self) -> Self;
    fn ywyz(self) -> Self;
    fn ywyw(self) -> Self;
    fn ywzx(self) -> Self;
    fn ywzy(self) -> Self;
    fn ywzz(self) -> Self;
    fn ywzw(self) -> Self;
    fn ywwx(self) -> Self;
    fn ywwy(self) -> Self;
    fn ywwz(self) -> Self;
    fn ywww(self) -> Self;
    fn zxxx(self) -> Self;
    fn zxxy(self) -> Self;
    fn zxxz(self) -> Self;
    fn zxxw(self) -> Self;
    fn zxyx(self) -> Self;
    fn zxyy(self) -> Self;
    fn zxyz(self) -> Self;
    fn zxyw(self) -> Self;
    fn zxzx(self) -> Self;
    fn zxzy(self) -> Self;
    fn zxzz(self) -> Self;
    fn zxzw(self) -> Self;
    fn zxwx(self) -> Self;
    fn zxwy(self) -> Self;
    fn zxwz(self) -> Self;
    fn zxww(self) -> Self;
    fn zyxx(self) -> Self;
    fn zyxy(self) -> Self;
    fn zyxz(self) -> Self;
    fn zyxw(self) -> Self;
    fn zyyx(self) -> Self;
    fn zyyy(self) -> Self;
    fn zyyz(self) -> Self;
    fn zyyw(self) -> Self;
    fn zyzx(self) -> Self;
    fn zyzy(self) -> Self;
    fn zyzz(self) -> Self;
    fn zyzw(self) -> Self;
    fn zywx(self) -> Self;
    fn zywy(self) -> Self;
    fn zywz(self) -> Self;
    fn zyww(self) -> Self;
    fn zzxx(self) -> Self;
    fn zzxy(self) -> Self;
    fn zzxz(self) -> Self;
    fn zzxw(self) -> Self;
    fn zzyx(self) -> Self;
    fn zzyy(self) -> Self;
    fn zzyz(self) -> Self;
    fn zzyw(self) -> Self;
    fn zzzx(self) -> Self;
    fn zzzy(self) -> Self;
    fn zzzz(self) -> Self;
    fn zzzw(self) -> Self;
    fn zzwx(self) -> Self;
    fn zzwy(self) -> Self;
    fn zzwz(self) -> Self;
    fn zzww(self) -> Self;
    fn zwxx(self) -> Self;
    fn zwxy(self) -> Self;
    fn zwxz(self) -> Self;
    fn zwxw(self) -> Self;
    fn zwyx(self) -> Self;
    fn zwyy(self) -> Self;
    fn zwyz(self) -> Self;
    fn zwyw(self) -> Self;
    fn zwzx(self) -> Self;
    fn zwzy(self) -> Self;
    fn zwzz(self) -> Self;
    fn zwzw(self) -> Self;
    fn zwwx(self) -> Self;
    fn zwwy(self) -> Self;
    fn zwwz(self) -> Self;
    fn zwww(self) -> Self;
    fn wxxx(self) -> Self;
    fn wxxy(self) -> Self;
    fn wxxz(self) -> Self;
    fn wxxw(self) -> Self;
    fn wxyx(self) -> Self;
    fn wxyy(self) -> Self;
    fn wxyz(self) -> Self;
    fn wxyw(self) -> Self;
    fn wxzx(self) -> Self;
    fn wxzy(self) -> Self;
    fn wxzz(self) -> Self;
    fn wxzw(self) -> Self;
    fn wxwx(self) -> Self;
    fn wxwy(self) -> Self;
    fn wxwz(self) -> Self;
    fn wxww(self) -> Self;
    fn wyxx(self) -> Self;
    fn wyxy(self) -> Self;
    fn wyxz(self) -> Self;
    fn wyxw(self) -> Self;
    fn wyyx(self) -> Self;
    fn wyyy(self) -> Self;
    fn wyyz(self) -> Self;
    fn wyyw(self) -> Self;
    fn wyzx(self) -> Self;
    fn wyzy(self) -> Self;
    fn wyzz(self) -> Self;
    fn wyzw(self) -> Self;
    fn wywx(self) -> Self;
    fn wywy(self) -> Self;
    fn wywz(self) -> Self;
    fn wyww(self) -> Self;
    fn wzxx(self) -> Self;
    fn wzxy(self) -> Self;
    fn wzxz(self) -> Self;
    fn wzxw(self) -> Self;
    fn wzyx(self) -> Self;
    fn wzyy(self) -> Self;
    fn wzyz(self) -> Self;
    fn wzyw(self) -> Self;
    fn wzzx(self) -> Self;
    fn wzzy(self) -> Self;
    fn wzzz(self) -> Self;
    fn wzzw(self) -> Self;
    fn wzwx(self) -> Self;
    fn wzwy(self) -> Self;
    fn wzwz(self) -> Self;
    fn wzww(self) -> Self;
    fn wwxx(self) -> Self;
    fn wwxy(self) -> Self;
    fn wwxz(self) -> Self;
    fn wwxw(self) -> Self;
    fn wwyx(self) -> Self;
    fn wwyy(self) -> Self;
    fn wwyz(self) -> Self;
    fn wwyw(self) -> Self;
    fn wwzx(self) -> Self;
    fn wwzy(self) -> Self;
    fn wwzz(self) -> Self;
    fn wwzw(self) -> Self;
    fn wwwx(self) -> Self;
    fn wwwy(self) -> Self;
    fn wwwz(self) -> Self;
    fn wwww(self) -> Self;

    // Provided method
    fn xyzw(self) -> Self { ... }
}
```

## Required Associated Types

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#475)

#### type [Vec2](#associatedtype.Vec2)

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#477)

#### type [Vec3](#associatedtype.Vec3)

## Required Methods

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#486)

#### fn [xx](#tymethod.xx)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#489)

#### fn [xy](#tymethod.xy)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#492)

#### fn [with\_xy](#tymethod.with_xy)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#495)

#### fn [xz](#tymethod.xz)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#498)

#### fn [with\_xz](#tymethod.with_xz)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#501)

#### fn [xw](#tymethod.xw)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#504)

#### fn [with\_xw](#tymethod.with_xw)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#507)

#### fn [yx](#tymethod.yx)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#510)

#### fn [with\_yx](#tymethod.with_yx)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#513)

#### fn [yy](#tymethod.yy)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#516)

#### fn [yz](#tymethod.yz)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#519)

#### fn [with\_yz](#tymethod.with_yz)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#522)

#### fn [yw](#tymethod.yw)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#525)

#### fn [with\_yw](#tymethod.with_yw)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#528)

#### fn [zx](#tymethod.zx)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#531)

#### fn [with\_zx](#tymethod.with_zx)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#534)

#### fn [zy](#tymethod.zy)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#537)

#### fn [with\_zy](#tymethod.with_zy)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#540)

#### fn [zz](#tymethod.zz)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#543)

#### fn [zw](#tymethod.zw)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#546)

#### fn [with\_zw](#tymethod.with_zw)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#549)

#### fn [wx](#tymethod.wx)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#552)

#### fn [with\_wx](#tymethod.with_wx)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#555)

#### fn [wy](#tymethod.wy)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#558)

#### fn [with\_wy](#tymethod.with_wy)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#561)

#### fn [wz](#tymethod.wz)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#564)

#### fn [with\_wz](#tymethod.with_wz)(self, rhs: Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#567)

#### fn [ww](#tymethod.ww)(self) -> Self::[Vec2](trait.Vec4Swizzles.html#associatedtype.Vec2 "type bevy::prelude::Vec4Swizzles::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#570)

#### fn [xxx](#tymethod.xxx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#573)

#### fn [xxy](#tymethod.xxy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#576)

#### fn [xxz](#tymethod.xxz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#579)

#### fn [xxw](#tymethod.xxw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#582)

#### fn [xyx](#tymethod.xyx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#585)

#### fn [xyy](#tymethod.xyy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#588)

#### fn [xyz](#tymethod.xyz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#591)

#### fn [with\_xyz](#tymethod.with_xyz)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#594)

#### fn [xyw](#tymethod.xyw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#597)

#### fn [with\_xyw](#tymethod.with_xyw)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#600)

#### fn [xzx](#tymethod.xzx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#603)

#### fn [xzy](#tymethod.xzy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#606)

#### fn [with\_xzy](#tymethod.with_xzy)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#609)

#### fn [xzz](#tymethod.xzz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#612)

#### fn [xzw](#tymethod.xzw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#615)

#### fn [with\_xzw](#tymethod.with_xzw)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#618)

#### fn [xwx](#tymethod.xwx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#621)

#### fn [xwy](#tymethod.xwy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#624)

#### fn [with\_xwy](#tymethod.with_xwy)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#627)

#### fn [xwz](#tymethod.xwz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#630)

#### fn [with\_xwz](#tymethod.with_xwz)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#633)

#### fn [xww](#tymethod.xww)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#636)

#### fn [yxx](#tymethod.yxx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#639)

#### fn [yxy](#tymethod.yxy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#642)

#### fn [yxz](#tymethod.yxz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#645)

#### fn [with\_yxz](#tymethod.with_yxz)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#648)

#### fn [yxw](#tymethod.yxw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#651)

#### fn [with\_yxw](#tymethod.with_yxw)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#654)

#### fn [yyx](#tymethod.yyx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#657)

#### fn [yyy](#tymethod.yyy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#660)

#### fn [yyz](#tymethod.yyz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#663)

#### fn [yyw](#tymethod.yyw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#666)

#### fn [yzx](#tymethod.yzx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#669)

#### fn [with\_yzx](#tymethod.with_yzx)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#672)

#### fn [yzy](#tymethod.yzy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#675)

#### fn [yzz](#tymethod.yzz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#678)

#### fn [yzw](#tymethod.yzw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#681)

#### fn [with\_yzw](#tymethod.with_yzw)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#684)

#### fn [ywx](#tymethod.ywx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#687)

#### fn [with\_ywx](#tymethod.with_ywx)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#690)

#### fn [ywy](#tymethod.ywy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#693)

#### fn [ywz](#tymethod.ywz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#696)

#### fn [with\_ywz](#tymethod.with_ywz)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#699)

#### fn [yww](#tymethod.yww)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#702)

#### fn [zxx](#tymethod.zxx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#705)

#### fn [zxy](#tymethod.zxy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#708)

#### fn [with\_zxy](#tymethod.with_zxy)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#711)

#### fn [zxz](#tymethod.zxz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#714)

#### fn [zxw](#tymethod.zxw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#717)

#### fn [with\_zxw](#tymethod.with_zxw)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#720)

#### fn [zyx](#tymethod.zyx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#723)

#### fn [with\_zyx](#tymethod.with_zyx)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#726)

#### fn [zyy](#tymethod.zyy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#729)

#### fn [zyz](#tymethod.zyz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#732)

#### fn [zyw](#tymethod.zyw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#735)

#### fn [with\_zyw](#tymethod.with_zyw)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#738)

#### fn [zzx](#tymethod.zzx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#741)

#### fn [zzy](#tymethod.zzy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#744)

#### fn [zzz](#tymethod.zzz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#747)

#### fn [zzw](#tymethod.zzw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#750)

#### fn [zwx](#tymethod.zwx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#753)

#### fn [with\_zwx](#tymethod.with_zwx)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#756)

#### fn [zwy](#tymethod.zwy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#759)

#### fn [with\_zwy](#tymethod.with_zwy)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#762)

#### fn [zwz](#tymethod.zwz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#765)

#### fn [zww](#tymethod.zww)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#768)

#### fn [wxx](#tymethod.wxx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#771)

#### fn [wxy](#tymethod.wxy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#774)

#### fn [with\_wxy](#tymethod.with_wxy)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#777)

#### fn [wxz](#tymethod.wxz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#780)

#### fn [with\_wxz](#tymethod.with_wxz)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#783)

#### fn [wxw](#tymethod.wxw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#786)

#### fn [wyx](#tymethod.wyx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#789)

#### fn [with\_wyx](#tymethod.with_wyx)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#792)

#### fn [wyy](#tymethod.wyy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#795)

#### fn [wyz](#tymethod.wyz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#798)

#### fn [with\_wyz](#tymethod.with_wyz)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#801)

#### fn [wyw](#tymethod.wyw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#804)

#### fn [wzx](#tymethod.wzx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#807)

#### fn [with\_wzx](#tymethod.with_wzx)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#810)

#### fn [wzy](#tymethod.wzy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#813)

#### fn [with\_wzy](#tymethod.with_wzy)(self, rhs: Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#816)

#### fn [wzz](#tymethod.wzz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#819)

#### fn [wzw](#tymethod.wzw)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#822)

#### fn [wwx](#tymethod.wwx)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#825)

#### fn [wwy](#tymethod.wwy)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#828)

#### fn [wwz](#tymethod.wwz)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#831)

#### fn [www](#tymethod.www)(self) -> Self::[Vec3](trait.Vec4Swizzles.html#associatedtype.Vec3 "type bevy::prelude::Vec4Swizzles::Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#834)

#### fn [xxxx](#tymethod.xxxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#837)

#### fn [xxxy](#tymethod.xxxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#840)

#### fn [xxxz](#tymethod.xxxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#843)

#### fn [xxxw](#tymethod.xxxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#846)

#### fn [xxyx](#tymethod.xxyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#849)

#### fn [xxyy](#tymethod.xxyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#852)

#### fn [xxyz](#tymethod.xxyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#855)

#### fn [xxyw](#tymethod.xxyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#858)

#### fn [xxzx](#tymethod.xxzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#861)

#### fn [xxzy](#tymethod.xxzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#864)

#### fn [xxzz](#tymethod.xxzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#867)

#### fn [xxzw](#tymethod.xxzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#870)

#### fn [xxwx](#tymethod.xxwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#873)

#### fn [xxwy](#tymethod.xxwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#876)

#### fn [xxwz](#tymethod.xxwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#879)

#### fn [xxww](#tymethod.xxww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#882)

#### fn [xyxx](#tymethod.xyxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#885)

#### fn [xyxy](#tymethod.xyxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#888)

#### fn [xyxz](#tymethod.xyxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#891)

#### fn [xyxw](#tymethod.xyxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#894)

#### fn [xyyx](#tymethod.xyyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#897)

#### fn [xyyy](#tymethod.xyyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#900)

#### fn [xyyz](#tymethod.xyyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#903)

#### fn [xyyw](#tymethod.xyyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#906)

#### fn [xyzx](#tymethod.xyzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#909)

#### fn [xyzy](#tymethod.xyzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#912)

#### fn [xyzz](#tymethod.xyzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#915)

#### fn [xywx](#tymethod.xywx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#918)

#### fn [xywy](#tymethod.xywy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#921)

#### fn [xywz](#tymethod.xywz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#924)

#### fn [xyww](#tymethod.xyww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#927)

#### fn [xzxx](#tymethod.xzxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#930)

#### fn [xzxy](#tymethod.xzxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#933)

#### fn [xzxz](#tymethod.xzxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#936)

#### fn [xzxw](#tymethod.xzxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#939)

#### fn [xzyx](#tymethod.xzyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#942)

#### fn [xzyy](#tymethod.xzyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#945)

#### fn [xzyz](#tymethod.xzyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#948)

#### fn [xzyw](#tymethod.xzyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#951)

#### fn [xzzx](#tymethod.xzzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#954)

#### fn [xzzy](#tymethod.xzzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#957)

#### fn [xzzz](#tymethod.xzzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#960)

#### fn [xzzw](#tymethod.xzzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#963)

#### fn [xzwx](#tymethod.xzwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#966)

#### fn [xzwy](#tymethod.xzwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#969)

#### fn [xzwz](#tymethod.xzwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#972)

#### fn [xzww](#tymethod.xzww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#975)

#### fn [xwxx](#tymethod.xwxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#978)

#### fn [xwxy](#tymethod.xwxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#981)

#### fn [xwxz](#tymethod.xwxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#984)

#### fn [xwxw](#tymethod.xwxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#987)

#### fn [xwyx](#tymethod.xwyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#990)

#### fn [xwyy](#tymethod.xwyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#993)

#### fn [xwyz](#tymethod.xwyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#996)

#### fn [xwyw](#tymethod.xwyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#999)

#### fn [xwzx](#tymethod.xwzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1002)

#### fn [xwzy](#tymethod.xwzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1005)

#### fn [xwzz](#tymethod.xwzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1008)

#### fn [xwzw](#tymethod.xwzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1011)

#### fn [xwwx](#tymethod.xwwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1014)

#### fn [xwwy](#tymethod.xwwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1017)

#### fn [xwwz](#tymethod.xwwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1020)

#### fn [xwww](#tymethod.xwww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1023)

#### fn [yxxx](#tymethod.yxxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1026)

#### fn [yxxy](#tymethod.yxxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1029)

#### fn [yxxz](#tymethod.yxxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1032)

#### fn [yxxw](#tymethod.yxxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1035)

#### fn [yxyx](#tymethod.yxyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1038)

#### fn [yxyy](#tymethod.yxyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1041)

#### fn [yxyz](#tymethod.yxyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1044)

#### fn [yxyw](#tymethod.yxyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1047)

#### fn [yxzx](#tymethod.yxzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1050)

#### fn [yxzy](#tymethod.yxzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1053)

#### fn [yxzz](#tymethod.yxzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1056)

#### fn [yxzw](#tymethod.yxzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1059)

#### fn [yxwx](#tymethod.yxwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1062)

#### fn [yxwy](#tymethod.yxwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1065)

#### fn [yxwz](#tymethod.yxwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1068)

#### fn [yxww](#tymethod.yxww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1071)

#### fn [yyxx](#tymethod.yyxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1074)

#### fn [yyxy](#tymethod.yyxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1077)

#### fn [yyxz](#tymethod.yyxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1080)

#### fn [yyxw](#tymethod.yyxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1083)

#### fn [yyyx](#tymethod.yyyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1086)

#### fn [yyyy](#tymethod.yyyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1089)

#### fn [yyyz](#tymethod.yyyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1092)

#### fn [yyyw](#tymethod.yyyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1095)

#### fn [yyzx](#tymethod.yyzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1098)

#### fn [yyzy](#tymethod.yyzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1101)

#### fn [yyzz](#tymethod.yyzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1104)

#### fn [yyzw](#tymethod.yyzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1107)

#### fn [yywx](#tymethod.yywx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1110)

#### fn [yywy](#tymethod.yywy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1113)

#### fn [yywz](#tymethod.yywz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1116)

#### fn [yyww](#tymethod.yyww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1119)

#### fn [yzxx](#tymethod.yzxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1122)

#### fn [yzxy](#tymethod.yzxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1125)

#### fn [yzxz](#tymethod.yzxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1128)

#### fn [yzxw](#tymethod.yzxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1131)

#### fn [yzyx](#tymethod.yzyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1134)

#### fn [yzyy](#tymethod.yzyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1137)

#### fn [yzyz](#tymethod.yzyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1140)

#### fn [yzyw](#tymethod.yzyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1143)

#### fn [yzzx](#tymethod.yzzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1146)

#### fn [yzzy](#tymethod.yzzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1149)

#### fn [yzzz](#tymethod.yzzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1152)

#### fn [yzzw](#tymethod.yzzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1155)

#### fn [yzwx](#tymethod.yzwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1158)

#### fn [yzwy](#tymethod.yzwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1161)

#### fn [yzwz](#tymethod.yzwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1164)

#### fn [yzww](#tymethod.yzww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1167)

#### fn [ywxx](#tymethod.ywxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1170)

#### fn [ywxy](#tymethod.ywxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1173)

#### fn [ywxz](#tymethod.ywxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1176)

#### fn [ywxw](#tymethod.ywxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1179)

#### fn [ywyx](#tymethod.ywyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1182)

#### fn [ywyy](#tymethod.ywyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1185)

#### fn [ywyz](#tymethod.ywyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1188)

#### fn [ywyw](#tymethod.ywyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1191)

#### fn [ywzx](#tymethod.ywzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1194)

#### fn [ywzy](#tymethod.ywzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1197)

#### fn [ywzz](#tymethod.ywzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1200)

#### fn [ywzw](#tymethod.ywzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1203)

#### fn [ywwx](#tymethod.ywwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1206)

#### fn [ywwy](#tymethod.ywwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1209)

#### fn [ywwz](#tymethod.ywwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1212)

#### fn [ywww](#tymethod.ywww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1215)

#### fn [zxxx](#tymethod.zxxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1218)

#### fn [zxxy](#tymethod.zxxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1221)

#### fn [zxxz](#tymethod.zxxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1224)

#### fn [zxxw](#tymethod.zxxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1227)

#### fn [zxyx](#tymethod.zxyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1230)

#### fn [zxyy](#tymethod.zxyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1233)

#### fn [zxyz](#tymethod.zxyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1236)

#### fn [zxyw](#tymethod.zxyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1239)

#### fn [zxzx](#tymethod.zxzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1242)

#### fn [zxzy](#tymethod.zxzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1245)

#### fn [zxzz](#tymethod.zxzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1248)

#### fn [zxzw](#tymethod.zxzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1251)

#### fn [zxwx](#tymethod.zxwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1254)

#### fn [zxwy](#tymethod.zxwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1257)

#### fn [zxwz](#tymethod.zxwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1260)

#### fn [zxww](#tymethod.zxww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1263)

#### fn [zyxx](#tymethod.zyxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1266)

#### fn [zyxy](#tymethod.zyxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1269)

#### fn [zyxz](#tymethod.zyxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1272)

#### fn [zyxw](#tymethod.zyxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1275)

#### fn [zyyx](#tymethod.zyyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1278)

#### fn [zyyy](#tymethod.zyyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1281)

#### fn [zyyz](#tymethod.zyyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1284)

#### fn [zyyw](#tymethod.zyyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1287)

#### fn [zyzx](#tymethod.zyzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1290)

#### fn [zyzy](#tymethod.zyzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1293)

#### fn [zyzz](#tymethod.zyzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1296)

#### fn [zyzw](#tymethod.zyzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1299)

#### fn [zywx](#tymethod.zywx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1302)

#### fn [zywy](#tymethod.zywy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1305)

#### fn [zywz](#tymethod.zywz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1308)

#### fn [zyww](#tymethod.zyww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1311)

#### fn [zzxx](#tymethod.zzxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1314)

#### fn [zzxy](#tymethod.zzxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1317)

#### fn [zzxz](#tymethod.zzxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1320)

#### fn [zzxw](#tymethod.zzxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1323)

#### fn [zzyx](#tymethod.zzyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1326)

#### fn [zzyy](#tymethod.zzyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1329)

#### fn [zzyz](#tymethod.zzyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1332)

#### fn [zzyw](#tymethod.zzyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1335)

#### fn [zzzx](#tymethod.zzzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1338)

#### fn [zzzy](#tymethod.zzzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1341)

#### fn [zzzz](#tymethod.zzzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1344)

#### fn [zzzw](#tymethod.zzzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1347)

#### fn [zzwx](#tymethod.zzwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1350)

#### fn [zzwy](#tymethod.zzwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1353)

#### fn [zzwz](#tymethod.zzwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1356)

#### fn [zzww](#tymethod.zzww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1359)

#### fn [zwxx](#tymethod.zwxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1362)

#### fn [zwxy](#tymethod.zwxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1365)

#### fn [zwxz](#tymethod.zwxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1368)

#### fn [zwxw](#tymethod.zwxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1371)

#### fn [zwyx](#tymethod.zwyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1374)

#### fn [zwyy](#tymethod.zwyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1377)

#### fn [zwyz](#tymethod.zwyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1380)

#### fn [zwyw](#tymethod.zwyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1383)

#### fn [zwzx](#tymethod.zwzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1386)

#### fn [zwzy](#tymethod.zwzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1389)

#### fn [zwzz](#tymethod.zwzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1392)

#### fn [zwzw](#tymethod.zwzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1395)

#### fn [zwwx](#tymethod.zwwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1398)

#### fn [zwwy](#tymethod.zwwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1401)

#### fn [zwwz](#tymethod.zwwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1404)

#### fn [zwww](#tymethod.zwww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1407)

#### fn [wxxx](#tymethod.wxxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1410)

#### fn [wxxy](#tymethod.wxxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1413)

#### fn [wxxz](#tymethod.wxxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1416)

#### fn [wxxw](#tymethod.wxxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1419)

#### fn [wxyx](#tymethod.wxyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1422)

#### fn [wxyy](#tymethod.wxyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1425)

#### fn [wxyz](#tymethod.wxyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1428)

#### fn [wxyw](#tymethod.wxyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1431)

#### fn [wxzx](#tymethod.wxzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1434)

#### fn [wxzy](#tymethod.wxzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1437)

#### fn [wxzz](#tymethod.wxzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1440)

#### fn [wxzw](#tymethod.wxzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1443)

#### fn [wxwx](#tymethod.wxwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1446)

#### fn [wxwy](#tymethod.wxwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1449)

#### fn [wxwz](#tymethod.wxwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1452)

#### fn [wxww](#tymethod.wxww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1455)

#### fn [wyxx](#tymethod.wyxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1458)

#### fn [wyxy](#tymethod.wyxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1461)

#### fn [wyxz](#tymethod.wyxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1464)

#### fn [wyxw](#tymethod.wyxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1467)

#### fn [wyyx](#tymethod.wyyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1470)

#### fn [wyyy](#tymethod.wyyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1473)

#### fn [wyyz](#tymethod.wyyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1476)

#### fn [wyyw](#tymethod.wyyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1479)

#### fn [wyzx](#tymethod.wyzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1482)

#### fn [wyzy](#tymethod.wyzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1485)

#### fn [wyzz](#tymethod.wyzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1488)

#### fn [wyzw](#tymethod.wyzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1491)

#### fn [wywx](#tymethod.wywx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1494)

#### fn [wywy](#tymethod.wywy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1497)

#### fn [wywz](#tymethod.wywz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1500)

#### fn [wyww](#tymethod.wyww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1503)

#### fn [wzxx](#tymethod.wzxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1506)

#### fn [wzxy](#tymethod.wzxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1509)

#### fn [wzxz](#tymethod.wzxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1512)

#### fn [wzxw](#tymethod.wzxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1515)

#### fn [wzyx](#tymethod.wzyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1518)

#### fn [wzyy](#tymethod.wzyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1521)

#### fn [wzyz](#tymethod.wzyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1524)

#### fn [wzyw](#tymethod.wzyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1527)

#### fn [wzzx](#tymethod.wzzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1530)

#### fn [wzzy](#tymethod.wzzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1533)

#### fn [wzzz](#tymethod.wzzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1536)

#### fn [wzzw](#tymethod.wzzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1539)

#### fn [wzwx](#tymethod.wzwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1542)

#### fn [wzwy](#tymethod.wzwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1545)

#### fn [wzwz](#tymethod.wzwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1548)

#### fn [wzww](#tymethod.wzww)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1551)

#### fn [wwxx](#tymethod.wwxx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1554)

#### fn [wwxy](#tymethod.wwxy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1557)

#### fn [wwxz](#tymethod.wwxz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1560)

#### fn [wwxw](#tymethod.wwxw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1563)

#### fn [wwyx](#tymethod.wwyx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1566)

#### fn [wwyy](#tymethod.wwyy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1569)

#### fn [wwyz](#tymethod.wwyz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1572)

#### fn [wwyw](#tymethod.wwyw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1575)

#### fn [wwzx](#tymethod.wwzx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1578)

#### fn [wwzy](#tymethod.wwzy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1581)

#### fn [wwzz](#tymethod.wwzz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1584)

#### fn [wwzw](#tymethod.wwzw)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1587)

#### fn [wwwx](#tymethod.wwwx)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1590)

#### fn [wwwy](#tymethod.wwwy)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1593)

#### fn [wwwz](#tymethod.wwwz)(self) -> Self

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#1596)

#### fn [wwww](#tymethod.wwww)(self) -> Self

## Provided Methods

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/vec_traits.rs.html#481)

#### fn [xyzw](#method.xyzw)(self) -> Self

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [DVec2](../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/dvec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [DVec3](../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [I8Vec2](../math/struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i8vec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [I8Vec3](../math/struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [I16Vec2](../math/struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i16vec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [I16Vec3](../math/struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [I64Vec2](../math/struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/i64vec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [I64Vec3](../math/struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [ISizeVec4](../math/struct.ISizeVec4.html "struct bevy::math::ISizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [ISizeVec2](../math/struct.ISizeVec2.html "struct bevy::math::ISizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/isizevec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [ISizeVec3](../math/struct.ISizeVec3.html "struct bevy::math::ISizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [IVec2](struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/ivec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [IVec3](struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [U8Vec2](../math/struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u8vec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [U8Vec3](../math/struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [U16Vec2](../math/struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u16vec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [U16Vec3](../math/struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [U64Vec2](../math/struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/u64vec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [U64Vec3](../math/struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [USizeVec4](../math/struct.USizeVec4.html "struct bevy::math::USizeVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [USizeVec2](../math/struct.USizeVec2.html "struct bevy::math::USizeVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/usizevec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [USizeVec3](../math/struct.USizeVec3.html "struct bevy::math::USizeVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#5)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#6)

#### type [Vec2](#associatedtype.Vec2) = [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/uvec4_impl.rs.html#8)

#### type [Vec3](#associatedtype.Vec3) = [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#12)

### impl [Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles") for [Vec4](struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#13)

#### type [Vec2](#associatedtype.Vec2) = [Vec2](struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/glam/0.32.1/x86_64-unknown-linux-gnu/src/glam/swizzles/sse2/vec4_impl.rs.html#15)

#### type [Vec3](#associatedtype.Vec3) = [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")