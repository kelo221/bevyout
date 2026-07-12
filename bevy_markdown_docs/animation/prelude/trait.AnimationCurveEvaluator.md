[bevy](../../index.html)::[animation](../index.html)::[prelude](index.html)

# Trait AnimationCurveEvaluator 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#650)

```rust
pub trait AnimationCurveEvaluator:
    Downcast
    + Send
    + Sync
    + 'static {
    // Required methods
    fn blend(
        &mut self,
        graph_node: NodeIndex,
    ) -> Result<(), AnimationEvaluationError>;
    fn add(
        &mut self,
        graph_node: NodeIndex,
    ) -> Result<(), AnimationEvaluationError>;
    fn push_blend_register(
        &mut self,
        weight: f32,
        graph_node: NodeIndex,
    ) -> Result<(), AnimationEvaluationError>;
    fn commit(
        &mut self,
        entity: EntityMutExcept<'_, '_, (AnimationTargetId, AnimatedBy, AnimationPlayer, AnimationGraphHandle)>,
    ) -> Result<(), AnimationEvaluationError>;
}
```

A low-level trait for use in [`VariableCurve`](../../prelude/struct.VariableCurve.html "struct bevy::prelude::VariableCurve") that provides fine control over how animations are evaluated.

You can implement this trait when the generic [`AnimatableCurveEvaluator`](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator") isn’t sufficiently-expressive for your needs. For example, `MorphWeights` implements this trait instead of using [`AnimatableCurveEvaluator`](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator") because it needs to animate arbitrarily many weights at once, which can’t be done with [`Animatable`](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") as that works on fixed-size values only.

If you implement this trait, you should also implement [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") on your curve type, as that trait allows creating instances of this one.

Implementations of [`AnimatableCurveEvaluator`](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator") should maintain a _stack_ of (value, weight, node index) triples, as well as a _blend register_, which is either a (value, weight) pair or empty. _Value_ here refers to an instance of the value being animated: for example, [`Vec3`](../../prelude/struct.Vec3.html "struct bevy::prelude::Vec3") in the case of translation keyframes. The stack stores intermediate values generated while evaluating the [`AnimationGraph`](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph"), while the blend register stores the result of a blend operation.

## Required Methods

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#667)

#### fn [blend](#tymethod.blend)( &mut self, graph\_node: [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AnimationEvaluationError](../enum.AnimationEvaluationError.html "enum bevy::animation::AnimationEvaluationError")\>

Blends the top element of the stack with the blend register.

The semantics of this method are as follows:

1.  Pop the top element of the stack. Call its value vₘ and its weight wₘ. If the stack was empty, return success.
    
2.  If the blend register is empty, set the blend register value to vₘ and the blend register weight to wₘ; then, return success.
    
3.  If the blend register is nonempty, call its current value vₙ and its current weight wₙ. Then, set the value of the blend register to `interpolate(vₙ, vₘ, wₘ / (wₘ + wₙ))`, and set the weight of the blend register to wₘ + wₙ.
    
4.  Return success.
    

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#683)

#### fn [add](#tymethod.add)(&mut self, graph\_node: [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex")) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AnimationEvaluationError](../enum.AnimationEvaluationError.html "enum bevy::animation::AnimationEvaluationError")\>

Additively blends the top element of the stack with the blend register.

The semantics of this method are as follows:

1.  Pop the top element of the stack. Call its value vₘ and its weight wₘ. If the stack was empty, return success.
    
2.  If the blend register is empty, set the blend register value to vₘ and the blend register weight to wₘ; then, return success.
    
3.  If the blend register is nonempty, call its current value vₙ. Then, set the value of the blend register to vₙ + vₘwₘ.
    
4.  Return success.
    

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#693-697)

#### fn [push\_blend\_register](#tymethod.push_blend_register)( &mut self, weight: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), graph\_node: [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AnimationEvaluationError](../enum.AnimationEvaluationError.html "enum bevy::animation::AnimationEvaluationError")\>

Pushes the current value of the blend register onto the stack.

If the blend register is empty, this method does nothing successfully. Otherwise, this method pushes the current value of the blend register onto the stack, alongside the weight and graph node supplied to this function. The weight present in the blend register is discarded; only the weight parameter to this function is pushed onto the stack. The blend register is emptied after this process.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#711)

#### fn [commit](#tymethod.commit)( &mut self, entity: [EntityMutExcept](../../ecs/world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, ([AnimationTargetId](../struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId"), [AnimatedBy](../struct.AnimatedBy.html "struct bevy::animation::AnimatedBy"), [AnimationPlayer](../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer"), [AnimationGraphHandle](../../prelude/struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle"))>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AnimationEvaluationError](../enum.AnimationEvaluationError.html "enum bevy::animation::AnimationEvaluationError")\>

Pops the top value off the stack and writes it into the appropriate component.

If the stack is empty, this method does nothing successfully. Otherwise, it pops the top value off the stack, fetches the associated component from either the `transform` or `entity` values as appropriate, and updates the appropriate property with the value popped from the stack. The weight and node index associated with the popped stack element are discarded. After doing this, the stack is emptied.

The property on the component must be overwritten with the value from the stack, not blended with it.

## Implementations

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#714)

### impl dyn [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#714)

#### pub fn [is](#method.is)<\_\_T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where \_\_T: [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator"),

Returns true if the trait object wraps an object of type `__T`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#714)

#### pub fn [downcast](#method.downcast)<\_\_T>( self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<\_\_T>, [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")\>>

where \_\_T: [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator"),

Returns a boxed object from a boxed trait object if the underlying object is of type `__T`. Returns the original boxed trait if it isn’t.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#714)

#### pub fn [downcast\_rc](#method.downcast_rc)<\_\_T>( self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<\_\_T>, [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")\>>

where \_\_T: [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator"),

Returns an `Rc`\-ed object from an `Rc`\-ed trait object if the underlying object is of type `__T`. Returns the original `Rc`\-ed trait if it isn’t.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#714)

#### pub fn [downcast\_ref](#method.downcast_ref)<\_\_T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&\_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator"),

Returns a reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#714)

#### pub fn [downcast\_mut](#method.downcast_mut)<\_\_T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut \_\_T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where \_\_T: [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator"),

Returns a mutable reference to the object within the trait object if it is of type `__T`, or `None` if it isn’t.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#399)

### impl<A> [AnimationCurveEvaluator](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator") for [AnimatableCurveEvaluator](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>

where A: [Animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable"),