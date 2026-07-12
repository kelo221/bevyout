[bevy](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_internal/0.19.0/x86_64-unknown-linux-gnu/src/bevy_internal/lib.rs.html#12)

`use bevy::prelude::*;` to import common components, bundles, and plugins.

## Modules

[adaptors](adaptors/index.html "mod bevy::prelude::adaptors")

Adaptors used by the Curve API for transforming and combining curves together.

[cores](cores/index.html "mod bevy::prelude::cores")

Core data structures to be used internally in Curve implementations, encapsulating storage and access patterns for reuse.

[derivatives](derivatives/index.html "mod bevy::prelude::derivatives")

This module holds traits related to extracting derivatives from curves. In applications, the derivatives of interest are chiefly the first and second; in this module, these are provided by the traits [`CurveWithDerivative`](derivatives/trait.CurveWithDerivative.html "trait bevy::prelude::derivatives::CurveWithDerivative") and [`CurveWithTwoDerivatives`](derivatives/trait.CurveWithTwoDerivatives.html "trait bevy::prelude::derivatives::CurveWithTwoDerivatives").

[easing](easing/index.html "mod bevy::prelude::easing")

Module containing different easing functions.

[interval](interval/index.html "mod bevy::prelude::interval")

The [`Interval`](struct.Interval.html "struct bevy::prelude::Interval") type for nonempty intervals used by the [`Curve`](trait.Curve.html "trait bevy::prelude::Curve") trait.

[iterable](iterable/index.html "mod bevy::prelude::iterable")

Iterable curves, which sample in the form of an iterator in order to support `Vec`\-like output whose length cannot be known statically.

[light\_consts](light_consts/index.html "mod bevy::prelude::light_consts")

Constants for operating with the light units: lumens, and lux.

[ops](ops/index.html "mod bevy::prelude::ops")

This mod re-exports the correct versions of floating-point operations with unspecified precision in the standard library depending on whether the `libm` crate feature is enabled.

[sample\_curves](sample_curves/index.html "mod bevy::prelude::sample_curves")`alloc`

Sample-interpolated curves constructed using the [`Curve`](trait.Curve.html "trait bevy::prelude::Curve") API.

[vec](vec/index.html "mod bevy::prelude::vec")

A contiguous growable array type with heap-allocated contents, written `Vec<T>`.

## Macros

[bsn\_list](macro.bsn_list.html "macro bevy::prelude::bsn_list")

Creates a `SceneList` using BSN (Bevy Scene Notation) syntax.

[children](macro.children.html "macro bevy::prelude::children")

Returns a [`SpawnRelatedBundle`](../ecs/spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle") that will insert the [`Children`](struct.Children.html "struct bevy::prelude::Children") component, spawn a [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") of entities with given bundles that relate to the [`Children`](struct.Children.html "struct bevy::prelude::Children") entity via the [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") component, and reserve space in the [`Children`](struct.Children.html "struct bevy::prelude::Children") for each spawned entity.

[debug](macro.debug.html "macro bevy::prelude::debug")

Constructs an event at the debug level.

[debug\_once](macro.debug_once.html "macro bevy::prelude::debug_once")

Call [`debug!`](macro.debug.html "macro bevy::prelude::debug") once per call site.

[debug\_span](macro.debug_span.html "macro bevy::prelude::debug_span")

Constructs a span at the debug level.

[error](macro.error.html "macro bevy::prelude::error")

Constructs an event at the error level.

[error\_once](macro.error_once.html "macro bevy::prelude::error_once")

Call [`error!`](macro.error.html "macro bevy::prelude::error") once per call site.

[error\_span](macro.error_span.html "macro bevy::prelude::error_span")

Constructs a span at the error level.

[format](macro.format.html "macro bevy::prelude::format")

Creates a `String` using interpolation of runtime expressions.

[info](macro.info.html "macro bevy::prelude::info")

Constructs an event at the info level.

[info\_once](macro.info_once.html "macro bevy::prelude::info_once")

Call [`info!`](macro.info.html "macro bevy::prelude::info") once per call site.

[info\_span](macro.info_span.html "macro bevy::prelude::info_span")

Constructs a span at the info level.

[once](macro.once.html "macro bevy::prelude::once")

Call some expression only once per call site.

[related](macro.related.html "macro bevy::prelude::related")

Returns a [`SpawnRelatedBundle`](../ecs/spawn/struct.SpawnRelatedBundle.html "struct bevy::ecs::spawn::SpawnRelatedBundle") that will insert the given [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget"), spawn a [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") of entities with given bundles that relate to the [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") entity via the [`RelationshipTarget::Relationship`](trait.RelationshipTarget.html#associatedtype.Relationship "associated type bevy::prelude::RelationshipTarget::Relationship") component, and reserve space in the [`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") for each spawned entity.

[trace](macro.trace.html "macro bevy::prelude::trace")

Constructs an event at the trace level.

[trace\_once](macro.trace_once.html "macro bevy::prelude::trace_once")

Call [`trace!`](macro.trace.html "macro bevy::prelude::trace") once per call site.

[trace\_span](macro.trace_span.html "macro bevy::prelude::trace_span")

Constructs a span at the trace level.

[vec](macro.vec.html "macro bevy::prelude::vec")Non-`no_global_oom_handling`

Creates a [`Vec`](struct.Vec.html "struct bevy::prelude::Vec") containing the arguments.

[warn](macro.warn.html "macro bevy::prelude::warn")

Constructs an event at the warn level.

[warn\_once](macro.warn_once.html "macro bevy::prelude::warn_once")

Call [`warn!`](macro.warn.html "macro bevy::prelude::warn") once per call site.

[warn\_span](macro.warn_span.html "macro bevy::prelude::warn_span")

Constructs a span at the warn level.

## Structs

[AabbGizmoConfigGroup](struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup")

The [`GizmoConfigGroup`](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used for debug visualizations of [`Aabb`](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") components on entities

[AccessibleLabel](struct.AccessibleLabel.html "struct bevy::prelude::AccessibleLabel")

A component which permits the a11y label to be specified independently from other a11y attributes.

[Add](struct.Add.html "struct bevy::prelude::Add")

Trigger emitted when a component is inserted onto an entity that does not already have that component. Runs before `Insert`. See [`ComponentHooks::on_add`](../ecs/lifecycle/struct.ComponentHooks.html#method.on_add "method bevy::ecs::lifecycle::ComponentHooks::on_add") for more information.

[Added](struct.Added.html "struct bevy::prelude::Added")

A filter on a component that only retains results the first time after they have been added.

[Allow](struct.Allow.html "struct bevy::prelude::Allow")

Allows a query to contain entities with the component `T`, bypassing [`DefaultQueryFilters`](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters").

[AmbientLight](struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

An ambient light, which lights the entire scene equally.

[AngularColorStop](struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

An angular color stop for a conic gradient

[AnimatableCurve](struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")

This type allows the conversion of a [curve](trait.Curve.html "trait bevy::prelude::Curve") valued in the [property type](trait.AnimatableProperty.html#associatedtype.Property "associated type bevy::prelude::AnimatableProperty::Property") of an [`AnimatableProperty`](trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") into an [`AnimationCurve`](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") which animates that property.

[AnimatableCurveEvaluator](struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")

An [`AnimatableCurveEvaluator`](struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator") for [`AnimatableProperty`](trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") instances.

[AnimatableKeyframeCurve](struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")

A [curve](trait.Curve.html "trait bevy::prelude::Curve") defined by keyframes with values in an [animatable](trait.Animatable.html "trait bevy::prelude::Animatable") type.

[AnimatedField](struct.AnimatedField.html "struct bevy::prelude::AnimatedField")

A [`Component`](trait.Component.html "trait bevy::prelude::Component") field that can be animated, defined by a function that reads the component and returns the accessed field / property.

[AnimationClip](struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

A list of [`VariableCurve`](struct.VariableCurve.html "struct bevy::prelude::VariableCurve")s and the [`AnimationTargetId`](../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")s to which they apply.

[AnimationGraph](struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

A graph structure that describes how animation clips are to be blended together.

[AnimationGraphAssetLoader](struct.AnimationGraphAssetLoader.html "struct bevy::prelude::AnimationGraphAssetLoader")

An [`AssetLoader`](../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") that can load [`AnimationGraph`](struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")s as assets.

[AnimationGraphHandle](struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

A [`Handle`](enum.Handle.html "enum bevy::prelude::Handle") to the [`AnimationGraph`](struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph") to be used by the [`AnimationPlayer`](struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") on the same entity.

[AnimationGraphHandleTemplate](struct.AnimationGraphHandleTemplate.html "struct bevy::prelude::AnimationGraphHandleTemplate")

[AnimationGraphNode](struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode")

An individual node within an animation graph.

[AnimationPlayer](struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer")

Animation controls.

[AnimationPlugin](struct.AnimationPlugin.html "struct bevy::prelude::AnimationPlugin")

Adds animation support to an app

[AnimationTransition](struct.AnimationTransition.html "struct bevy::prelude::AnimationTransition")

An animation that is being faded out as part of a transition

[AnimationTransitions](struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions")

Manages fade-out of animation blend factors, allowing for smooth transitions between animations.

[Annulus](struct.Annulus.html "struct bevy::prelude::Annulus")

A primitive shape formed by the region between two circles, also known as a ring.

[AnyOf](struct.AnyOf.html "struct bevy::prelude::AnyOf")

The `AnyOf` query parameter fetches entities with any of the component types included in T.

[App](struct.App.html "struct bevy::prelude::App")

[`App`](struct.App.html "struct bevy::prelude::App") is the primary API for writing user applications. It automates the setup of a [standard lifecycle](struct.Main.html "struct bevy::prelude::Main") and provides interface glue for [plugins](trait.Plugin.html "trait bevy::prelude::Plugin").

[AppFunctionRegistry](struct.AppFunctionRegistry.html "struct bevy::prelude::AppFunctionRegistry")`reflect_functions`

A [`Resource`](trait.Resource.html "trait bevy::prelude::Resource") storing [`FunctionRegistry`](../reflect/func/struct.FunctionRegistry.html "struct bevy::reflect::func::FunctionRegistry") for function registrations relevant to a whole app.

[AppTypeRegistry](struct.AppTypeRegistry.html "struct bevy::prelude::AppTypeRegistry")

A [`Resource`](trait.Resource.html "trait bevy::prelude::Resource") storing [`TypeRegistry`](../reflect/struct.TypeRegistry.html "struct bevy::reflect::TypeRegistry") for type registrations relevant to a whole app.

[ApplyDeferred](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred")

A special [`System`](trait.System.html "trait bevy::prelude::System") that instructs the executor to call [`System::apply_deferred`](trait.System.html#tymethod.apply_deferred "method bevy::prelude::System::apply_deferred") on the systems that have run but not applied their [`Deferred`](struct.Deferred.html "struct bevy::prelude::Deferred") system parameters (like [`Commands`](struct.Commands.html "struct bevy::prelude::Commands")) or other system buffers.

[Arc2d](struct.Arc2d.html "struct bevy::prelude::Arc2d")

A primitive representing an arc between two points on a circle.

[AssetChanged](struct.AssetChanged.html "struct bevy::prelude::AssetChanged")

Filter that selects entities with an `A` for an asset that changed after the system last ran, where `A` is a component that implements [`AsAssetId`](../asset/trait.AsAssetId.html "trait bevy::asset::AsAssetId").

[AssetPlugin](struct.AssetPlugin.html "struct bevy::prelude::AssetPlugin")

Provides “asset” loading and processing functionality. An [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") is a “runtime value” that is loaded from an [`AssetSource`](../asset/io/struct.AssetSource.html "struct bevy::asset::io::AssetSource"), which can be something like a filesystem, a network, etc.

[AssetServer](struct.AssetServer.html "struct bevy::prelude::AssetServer")

Loads and tracks the state of [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") values from a configured [`AssetReader`](../asset/io/trait.AssetReader.html "trait bevy::asset::io::AssetReader"). This can be used to kick off new asset loads and retrieve their current load states.

[Assets](struct.Assets.html "struct bevy::prelude::Assets")

Stores [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") values identified by their [`AssetId`](enum.AssetId.html "enum bevy::prelude::AssetId").

[AudioPlayer](struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")

A component for playing a sound.

[AudioSink](struct.AudioSink.html "struct bevy::prelude::AudioSink")

Used to control audio during playback.

[AudioSource](struct.AudioSource.html "struct bevy::prelude::AudioSource")

A source of audio data

[Axis](struct.Axis.html "struct bevy::prelude::Axis")

Stores the position data of the input devices of type `T`.

[BVec2](struct.BVec2.html "struct bevy::prelude::BVec2")

A 2-dimensional `bool` vector mask.

[BVec3](struct.BVec3.html "struct bevy::prelude::BVec3")

A 3-dimensional `bool` vector mask.

[BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

A 4-dimensional `bool` vector mask.

[BVec3A](struct.BVec3A.html "struct bevy::prelude::BVec3A")

A 3-dimensional SIMD vector mask.

[BVec4A](struct.BVec4A.html "struct bevy::prelude::BVec4A")

A 4-dimensional SIMD vector mask.

[BackInCurve](struct.BackInCurve.html "struct bevy::prelude::BackInCurve")

`f(t) = 2.70158 * t³ - 1.70158 * t²`

[BackInOutCurve](struct.BackInOutCurve.html "struct bevy::prelude::BackInOutCurve")

Behaves as `BackIn` for t < 0.5 and as `BackOut` for t >= 0.5

[BackOutCurve](struct.BackOutCurve.html "struct bevy::prelude::BackOutCurve")

`f(t) = 1.0 + 2.70158 * (t - 1.0)³ + 1.70158 * (t - 1.0)²`

[BackgroundColor](struct.BackgroundColor.html "struct bevy::prelude::BackgroundColor")

The background color of the node

[BackgroundGradient](struct.BackgroundGradient.html "struct bevy::prelude::BackgroundGradient")

A UI node that displays a gradient

[BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")

The built in “universal” Bevy error type. This has a blanket [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From") impl for any type that implements Rust’s [`Error`](https://doc.rust-lang.org/nightly/core/error/trait.Error.html "trait core::error::Error"), meaning it can be used as a “catch all” error.

[BlendInput](struct.BlendInput.html "struct bevy::prelude::BlendInput")

An individual input for [`Animatable::blend`](trait.Animatable.html#tymethod.blend "associated function bevy::prelude::Animatable::blend").

[BorderColor](struct.BorderColor.html "struct bevy::prelude::BorderColor")

The border color of the UI node.

[BorderGradient](struct.BorderGradient.html "struct bevy::prelude::BorderGradient")

A UI node border that displays a gradient

[BorderRadius](struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

Used to add rounded corners to a UI node. You can set a UI node to have uniformly rounded corners or specify different radii for each corner. If a given radius exceeds half the length of the smallest dimension between the node’s height or width, the radius will calculated as half the smallest dimension.

[BorderRect](struct.BorderRect.html "struct bevy::prelude::BorderRect")

Defines border insets that shrink a rectangle from its minimum and maximum corners.

[BounceInCurve](struct.BounceInCurve.html "struct bevy::prelude::BounceInCurve")

bouncy at the start!

[BounceInOutCurve](struct.BounceInOutCurve.html "struct bevy::prelude::BounceInOutCurve")

Behaves as `BounceIn` for t < 0.5 and as `BounceOut` for t >= 0.5

[BounceOutCurve](struct.BounceOutCurve.html "struct bevy::prelude::BounceOutCurve")

bouncy at the end!

[Box](struct.Box.html "struct bevy::prelude::Box")

A pointer type that uniquely owns a heap allocation of type `T`.

[BoxShadow](struct.BoxShadow.html "struct bevy::prelude::BoxShadow")

List of shadows to draw for a [`Node`](struct.Node.html "struct bevy::prelude::Node").

[BoxShadowSamples](struct.BoxShadowSamples.html "struct bevy::prelude::BoxShadowSamples")

Number of shadow samples. A larger value will result in higher quality shadows. Default is 4, values higher than ~10 offer diminishing returns.

[Button](struct.Button.html "struct bevy::prelude::Button")

Marker struct for buttons

[ButtonInput](struct.ButtonInput.html "struct bevy::prelude::ButtonInput")

A “press-able” input of type `T`.

[CalculatedClip](struct.CalculatedClip.html "struct bevy::prelude::CalculatedClip")

The calculated clip of the node

[Camera](struct.Camera.html "struct bevy::prelude::Camera")

The defining [`Component`](trait.Component.html "trait bevy::prelude::Component") for camera entities, storing information about how and what to render through this camera.

[Camera2d](struct.Camera2d.html "struct bevy::prelude::Camera2d")

A 2D camera component. Enables the 2D render graph for a [`Camera`](struct.Camera.html "struct bevy::prelude::Camera").

[Camera3d](struct.Camera3d.html "struct bevy::prelude::Camera3d")

A 3D camera component. Enables the main 3D render graph for a [`Camera`](struct.Camera.html "struct bevy::prelude::Camera").

[Cancel](struct.Cancel.html "struct bevy::prelude::Cancel")

Fires when a pointer is canceled, and its current interaction state is dropped.

[Capsule2d](struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

A 2D capsule primitive, also known as a stadium or pill shape.

[Capsule3d](struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

A 3D capsule primitive centered on the origin A three-dimensional capsule is defined as a surface at a distance (radius) from a line

[ChainCurve](struct.ChainCurve.html "struct bevy::prelude::ChainCurve")

The curve that results from chaining one curve with another. The second curve is effectively reparametrized so that its start is at the end of the first.

[Changed](struct.Changed.html "struct bevy::prelude::Changed")

A filter on a component that only retains results the first time after they have been added or mutably dereferenced.

[ChildOf](struct.ChildOf.html "struct bevy::prelude::ChildOf")

Stores the parent entity of this child entity with this component.

[Children](struct.Children.html "struct bevy::prelude::Children")

Tracks which entities are children of this parent entity.

[Circle](struct.Circle.html "struct bevy::prelude::Circle")

A circle primitive, representing the set of points some distance from the origin

[CircularInCurve](struct.CircularInCurve.html "struct bevy::prelude::CircularInCurve")

`f(t) = 1.0 - sqrt(1.0 - t²)`

[CircularInOutCurve](struct.CircularInOutCurve.html "struct bevy::prelude::CircularInOutCurve")

Behaves as `CircularIn` for t < 0.5 and as `CircularOut` for t >= 0.5

[CircularOutCurve](struct.CircularOutCurve.html "struct bevy::prelude::CircularOutCurve")

`f(t) = sqrt((2.0 - t) * t)`

[CircularSector](struct.CircularSector.html "struct bevy::prelude::CircularSector")

A primitive representing a circular sector: a pie slice of a circle.

[CircularSegment](struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

A primitive representing a circular segment: the area enclosed by the arc of a circle and its chord (the line between its endpoints).

[ClearColor](struct.ClearColor.html "struct bevy::prelude::ClearColor")

A [`Resource`](trait.Resource.html "trait bevy::prelude::Resource") that stores the default color that cameras use to clear the screen between frames.

[Click](struct.Click.html "struct bevy::prelude::Click")

Fires when a pointer sends a pointer pressed event followed by a pointer released event, with the same [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") for both events.

[Clipboard](struct.Clipboard.html "struct bevy::prelude::Clipboard")

A resource which provides access to the system clipboard.

[ClipboardPlugin](struct.ClipboardPlugin.html "struct bevy::prelude::ClipboardPlugin")

Adds clipboard support to a Bevy app.

[ColorMaterial](struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

A [2d material](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d") that renders [2d meshes](struct.Mesh2d.html "struct bevy::prelude::Mesh2d") with a texture tinted by a uniform color

[ColorStop](struct.ColorStop.html "struct bevy::prelude::ColorStop")

A color stop for a gradient

[Commands](struct.Commands.html "struct bevy::prelude::Commands")

A [`Command`](trait.Command.html "trait bevy::prelude::Command") queue to perform structural changes to the [`World`](struct.World.html "struct bevy::prelude::World").

[ComputedNode](struct.ComputedNode.html "struct bevy::prelude::ComputedNode")

Provides the computed size and layout properties of the node.

[ComputedUiRenderTargetInfo](struct.ComputedUiRenderTargetInfo.html "struct bevy::prelude::ComputedUiRenderTargetInfo")

Derived information about the render target for this UI node.

[ComputedUiTargetCamera](struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera")

Derived information about the camera target for this UI node.

[Cone](struct.Cone.html "struct bevy::prelude::Cone")

A cone primitive centered on the midpoint between the tip of the cone and the center of its base.

[ConicGradient](struct.ConicGradient.html "struct bevy::prelude::ConicGradient")

A conic gradient

[ConicalFrustum](struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

A conical frustum primitive. A conical frustum can be created by slicing off a section of a cone.

[ConstantCurve](struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")

A curve with a constant value over its domain.

[ContactShadowsPlugin](struct.ContactShadowsPlugin.html "struct bevy::prelude::ContactShadowsPlugin")

Enables contact shadows for a camera.

[ContiguousMut](struct.ContiguousMut.html "struct bevy::prelude::ContiguousMut")

Data type returned by [`ContiguousQueryData::fetch_contiguous`](../ecs/query/trait.ContiguousQueryData.html#tymethod.fetch_contiguous "associated function bevy::ecs::query::ContiguousQueryData::fetch_contiguous") for [`Mut<T>`](struct.Mut.html "struct bevy::prelude::Mut") and `&mut T`

[ContiguousRef](struct.ContiguousRef.html "struct bevy::prelude::ContiguousRef")

Contiguous equivalent of [`Ref<T>`](struct.Ref.html "struct bevy::prelude::Ref").

[ContinuationCurve](struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")

The curve that results from chaining two curves.

[ConvexPolygon](struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")`alloc`

A convex polygon with `N` vertices.

[CubicBSpline](struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")`alloc`

A spline interpolated continuously across the nearest four control points. The curve does not necessarily pass through any of the control points.

[CubicBezier](struct.CubicBezier.html "struct bevy::prelude::CubicBezier")`alloc`

A spline composed of a single cubic Bezier curve.

[CubicCardinalSpline](struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")`alloc`

A spline interpolated continuously across the nearest four control points, with the position of the curve specified at every control point and the tangents computed automatically. The associated [`CubicCurve`](struct.CubicCurve.html "struct bevy::prelude::CubicCurve") has one segment between each pair of adjacent control points.

[CubicCurve](struct.CubicCurve.html "struct bevy::prelude::CubicCurve")`alloc`

A collection of [`CubicSegment`](struct.CubicSegment.html "struct bevy::prelude::CubicSegment")s chained into a single parametric curve. It is a [`Curve`](trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, N]`, where `N` is its number of segments.

[CubicHermite](struct.CubicHermite.html "struct bevy::prelude::CubicHermite")`alloc`

A spline interpolated continuously between the nearest two control points, with the position and velocity of the curve specified at both control points. This curve passes through all control points, with the specified velocity which includes direction and parametric speed.

[CubicInCurve](struct.CubicInCurve.html "struct bevy::prelude::CubicInCurve")

`f(t) = t³`

[CubicInOutCurve](struct.CubicInOutCurve.html "struct bevy::prelude::CubicInOutCurve")

Behaves as `CubicIn` for t < 0.5 and as `CubicOut` for t >= 0.5

[CubicNurbs](struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")`alloc`

Non-uniform Rational B-Splines (NURBS) are a powerful generalization of the [`CubicBSpline`](struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline") which can represent a much more diverse class of curves (like perfect circles and ellipses).

[CubicOutCurve](struct.CubicOutCurve.html "struct bevy::prelude::CubicOutCurve")

`f(t) = (t - 1.0)³ + 1.0`

[CubicSegment](struct.CubicSegment.html "struct bevy::prelude::CubicSegment")

A segment of a cubic curve, used to hold precomputed coefficients for fast interpolation. It is a [`Curve`](trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, 1]`.

[Cuboid](struct.Cuboid.html "struct bevy::prelude::Cuboid")

A cuboid primitive, which is like a cube, except that the x, y, and z dimensions are not required to be the same.

[CursorEntered](struct.CursorEntered.html "struct bevy::prelude::CursorEntered")

An event that is sent whenever the user’s cursor enters a window.

[CursorLeft](struct.CursorLeft.html "struct bevy::prelude::CursorLeft")

An event that is sent whenever the user’s cursor leaves a window.

[CursorMoved](struct.CursorMoved.html "struct bevy::prelude::CursorMoved")

An event reporting that the mouse cursor has moved inside a window.

[CurveReparamCurve](struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")

A curve that has been reparametrized by another curve, using that curve to transform the sample times before sampling. Curves of this type are produced by [`CurveExt::reparametrize_by_curve`](trait.CurveExt.html#method.reparametrize_by_curve "method bevy::prelude::CurveExt::reparametrize_by_curve").

[Cylinder](struct.Cylinder.html "struct bevy::prelude::Cylinder")

A cylinder primitive centered on the origin

[DebugName](struct.DebugName.html "struct bevy::prelude::DebugName")

Wrapper to help debugging ECS issues. This is used to display the names of systems, components, …

[DefaultGizmoConfigGroup](struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup")

The default gizmo config group.

[DefaultPickingPlugins](struct.DefaultPickingPlugins.html "struct bevy::prelude::DefaultPickingPlugins")

One plugin that contains the [`PointerInputPlugin`](struct.PointerInputPlugin.html "struct bevy::prelude::PointerInputPlugin"), [`PickingPlugin`](struct.PickingPlugin.html "struct bevy::prelude::PickingPlugin") and the [`InteractionPlugin`](struct.InteractionPlugin.html "struct bevy::prelude::InteractionPlugin"), this is probably the plugin that will be most used.

[DefaultPlugins](struct.DefaultPlugins.html "struct bevy::prelude::DefaultPlugins")

This plugin group will add all the default plugins for a _Bevy_ application:

[DefaultUiCamera](struct.DefaultUiCamera.html "struct bevy::prelude::DefaultUiCamera")

[Deferred](struct.Deferred.html "struct bevy::prelude::Deferred")

A [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that stores a buffer which gets applied to the [`World`](struct.World.html "struct bevy::prelude::World") during [`ApplyDeferred`](struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred"). This is used internally by [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") to defer `World` mutations.

[Despawn](struct.Despawn.html "struct bevy::prelude::Despawn")

[`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") emitted for each component on an entity when it is despawned. See [`ComponentHooks::on_despawn`](../ecs/lifecycle/struct.ComponentHooks.html#method.on_despawn "method bevy::ecs::lifecycle::ComponentHooks::on_despawn") for more information.

[DespawnOnEnter](struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")

Entities marked with this component will be despawned upon entering the given state.

[DespawnOnExit](struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")

Entities marked with this component will be despawned upon exiting the given state.

[DespawnWhen](struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")

Entities marked with this component will be despawned when a [`StateTransitionEvent<S>`](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[Dir2](struct.Dir2.html "struct bevy::prelude::Dir2")

A normalized vector pointing in a direction in 2D space

[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

A normalized vector pointing in a direction in 3D space

[Dir3A](struct.Dir3A.html "struct bevy::prelude::Dir3A")

A normalized SIMD vector pointing in a direction in 3D space.

[DirectionalLight](struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")

A Directional light.

[DisableOnEnter](struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")

Entities marked with this component will be disabled upon entering the given state.

[DisableOnExit](struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")

Entities marked with this component will be disabled upon exiting the given state.

[DisableWhen](struct.DisableWhen.html "struct bevy::prelude::DisableWhen")

Entities marked with this component will be disabled when a [`StateTransitionEvent<S>`](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[Discard](struct.Discard.html "struct bevy::prelude::Discard")

Trigger emitted when a component is removed from an entity, regardless of whether or not it is later replaced.

[DistanceFog](struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

Configures the “classic” computer graphics [distance fog](https://en.wikipedia.org/wiki/Distance_fog) effect, in which objects appear progressively more covered in atmospheric haze the further away they are from the camera. Affects meshes rendered via the PBR [`StandardMaterial`](struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial").

[Drag](struct.Drag.html "struct bevy::prelude::Drag")

Fires while the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") is being dragged.

[DragDrop](struct.DragDrop.html "struct bevy::prelude::DragDrop")

Fires when a pointer drops the `dropped` entity onto the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragEnd](struct.DragEnd.html "struct bevy::prelude::DragEnd")

Fires when a pointer is dragging the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") and a pointer released event is received.

[DragEnter](struct.DragEnter.html "struct bevy::prelude::DragEnter")

Fires when a pointer dragging the `dragged` entity enters the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target")

[DragEntry](struct.DragEntry.html "struct bevy::prelude::DragEntry")

Dragging state.

[DragLeave](struct.DragLeave.html "struct bevy::prelude::DragLeave")

Fires when a pointer dragging the `dragged` entity leaves the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragOver](struct.DragOver.html "struct bevy::prelude::DragOver")

Fires while the `dragged` entity is being dragged over the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[DragStart](struct.DragStart.html "struct bevy::prelude::DragStart")

Fires when the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") receives a pointer pressed event followed by a pointer move event.

[DynamicTextureAtlasBuilder](struct.DynamicTextureAtlasBuilder.html "struct bevy::prelude::DynamicTextureAtlasBuilder")

Helper utility to update [`TextureAtlasLayout`](struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout") on the fly.

[DynamicWorld](struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld")

A collection of serializable resources and dynamic entities.

[DynamicWorldBuilder](struct.DynamicWorldBuilder.html "struct bevy::prelude::DynamicWorldBuilder")

A [`DynamicWorld`](struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld") builder, used to build a [`DynamicWorld`](struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld") from a [`World`](struct.World.html "struct bevy::prelude::World") by extracting some entities and resources.

[DynamicWorldRoot](struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

Adding this component will spawn the world as a child of that entity. Once it’s spawned, the entity will have a [`WorldInstance`](../world_serialization/struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance") component.

[EasingCurve](struct.EasingCurve.html "struct bevy::prelude::EasingCurve")

A [`Curve`](trait.Curve.html "trait bevy::prelude::Curve") that is defined by

[ElasticCurve](struct.ElasticCurve.html "struct bevy::prelude::ElasticCurve")

`f(omega,t) = 1 - (1 - t)²(2sin(omega * t) / omega + cos(omega * t))`, parametrized by `omega`

[ElasticInCurve](struct.ElasticInCurve.html "struct bevy::prelude::ElasticInCurve")

`f(t) = -2.0^(10.0 * t - 10.0) * sin((t * 10.0 - 10.75) * 2.0 * π / 3.0)`

[ElasticInOutCurve](struct.ElasticInOutCurve.html "struct bevy::prelude::ElasticInOutCurve")

Behaves as `ElasticIn` for t < 0.5 and as `ElasticOut` for t >= 0.5

[ElasticOutCurve](struct.ElasticOutCurve.html "struct bevy::prelude::ElasticOutCurve")

`f(t) = 2.0^(-10.0 * t) * sin((t * 10.0 - 0.75) * 2.0 * π / 3.0) + 1.0`

[Ellipse](struct.Ellipse.html "struct bevy::prelude::Ellipse")

An ellipse primitive, which is like a circle, but the width and height can be different

[EnableOnEnter](struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")

Entities marked with this component will be enabled upon entering the given state.

[EnableOnExit](struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")

Entities marked with this component will be enabled upon exiting the given state.

[EnableWhen](struct.EnableWhen.html "struct bevy::prelude::EnableWhen")

Entities marked with this component will be enabled when a [`StateTransitionEvent<S>`](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[Enter](struct.Enter.html "struct bevy::prelude::Enter")

Fires when a pointer crosses into the bounds of a [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Over`](struct.Over.html "struct bevy::prelude::Over"), this event bubbles up through a subset of the [target entity’s](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship).

[EnterSchedules](struct.EnterSchedules.html "struct bevy::prelude::EnterSchedules")

System set that runs enter schedule(s) for state `S`.

[Entity](struct.Entity.html "struct bevy::prelude::Entity")

Unique identifier for an entity in a [`World`](struct.World.html "struct bevy::prelude::World"). Note that this is just an id, not the entity itself. Further, the entity this id refers to may no longer exist in the [`World`](struct.World.html "struct bevy::prelude::World"). For more information about entities, their ids, and how to use them, see the module [docs](../ecs/entity/index.html "mod bevy::ecs::entity").

[EntityCommands](struct.EntityCommands.html "struct bevy::prelude::EntityCommands")

A list of commands that will be run to modify an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

[EntityMut](struct.EntityMut.html "struct bevy::prelude::EntityMut")

Provides mutable access to a single entity and all of its components.

[EntityRef](struct.EntityRef.html "struct bevy::prelude::EntityRef")

A read-only reference to a particular [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") and all of its components.

[EntityWorldMut](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")

A mutable reference to a particular [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"), and the entire world.

[EnvironmentMapLight](struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

A pair of cubemap textures that represent the surroundings of a specific area in space.

[EvenCore](struct.EvenCore.html "struct bevy::prelude::EvenCore")`alloc`

The data core of a curve derived from evenly-spaced samples. The intention is to use this in addition to explicit or inferred interpolation information in user-space in order to implement curves using [`domain`](struct.EvenCore.html#method.domain "method bevy::prelude::EvenCore::domain") and [`sample_with`](struct.EvenCore.html#method.sample_with "method bevy::prelude::EvenCore::sample_with").

[ExitSchedules](struct.ExitSchedules.html "struct bevy::prelude::ExitSchedules")

System set that runs exit schedule(s) for state `S`.

[ExponentialInCurve](struct.ExponentialInCurve.html "struct bevy::prelude::ExponentialInCurve")

`f(t) ≈ 2.0^(10.0 * (t - 1.0))`

[ExponentialInOutCurve](struct.ExponentialInOutCurve.html "struct bevy::prelude::ExponentialInOutCurve")

Behaves as `ExponentialIn` for t < 0.5 and as `ExponentialOut` for t >= 0.5

[ExponentialOutCurve](struct.ExponentialOutCurve.html "struct bevy::prelude::ExponentialOutCurve")

`f(t) ≈ 1.0 - 2.0^(-10.0 * t)`

[ExtractSchedule](struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule")

Schedule in which data from the main world is ‘extracted’ into the render world.

[Extrusion](struct.Extrusion.html "struct bevy::prelude::Extrusion")

A 3D shape representing an extruded 2D `base_shape`.

[FilteredResources](struct.FilteredResources.html "struct bevy::prelude::FilteredResources")

Provides read-only access to a set of [`Resource`](trait.Resource.html "trait bevy::prelude::Resource")s defined by the contained [`Access`](../ecs/query/struct.Access.html "struct bevy::ecs::query::Access").

[FilteredResourcesMut](struct.FilteredResourcesMut.html "struct bevy::prelude::FilteredResourcesMut")

Provides mutable access to a set of [`Resource`](trait.Resource.html "trait bevy::prelude::Resource")s defined by the contained [`Access`](../ecs/query/struct.Access.html "struct bevy::ecs::query::Access").

[First](struct.First.html "struct bevy::prelude::First")

Runs first in the schedule.

[Fixed](struct.Fixed.html "struct bevy::prelude::Fixed")

The fixed timestep game clock following virtual time.

[FixedFirst](struct.FixedFirst.html "struct bevy::prelude::FixedFirst")

Runs first in the [`FixedMain`](../app/struct.FixedMain.html "struct bevy::app::FixedMain") schedule.

[FixedLast](struct.FixedLast.html "struct bevy::prelude::FixedLast")

The schedule that runs last in [`FixedMain`](../app/struct.FixedMain.html "struct bevy::app::FixedMain")

[FixedPostUpdate](struct.FixedPostUpdate.html "struct bevy::prelude::FixedPostUpdate")

The schedule that runs after the [`FixedUpdate`](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") schedule, for reacting to changes made in the main update logic.

[FixedPreUpdate](struct.FixedPreUpdate.html "struct bevy::prelude::FixedPreUpdate")

The schedule that contains logic that must run before [`FixedUpdate`](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate").

[FixedUpdate](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate")

The schedule that contains most gameplay logic, which runs at a fixed rate rather than every render frame. For logic that should run once per render frame, use the [`Update`](struct.Update.html "struct bevy::prelude::Update") schedule instead.

[Font](struct.Font.html "struct bevy::prelude::Font")

An [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") that contains the data for a loaded font, if loaded as an asset.

[FontWeight](struct.FontWeight.html "struct bevy::prelude::FontWeight")

How thick or bold the strokes of a font appear.

[FontWidth](struct.FontWidth.html "struct bevy::prelude::FontWidth")

The visual width of a font as a ratio of its normal width, typically 0.5 to 2.0. `<https://docs.microsoft.com/en-us/typography/opentype/spec/os2#uswidthclass>`

[ForeverCurve](struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")

The curve that results from repeating a curve forever.

[FrustumGizmoConfigGroup](struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup")

The [`GizmoConfigGroup`](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used for debug visualizations of [`Frustum`](../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum") components on entities

[FunctionCurve](struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")

A curve defined by a function together with a fixed domain.

[Gamepad](struct.Gamepad.html "struct bevy::prelude::Gamepad")

Stores a connected gamepad’s metadata such as the name and its [`GamepadButton`](enum.GamepadButton.html "enum bevy::prelude::GamepadButton") and [`GamepadAxis`](enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis").

[GamepadSettings](struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings")

Gamepad settings component.

[GeneratedEnvironmentMapLight](struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

A generated environment map that is filtered at runtime.

[GilrsPlugin](struct.GilrsPlugin.html "struct bevy::prelude::GilrsPlugin")

Plugin that provides gamepad handling to an [`App`](struct.App.html "struct bevy::prelude::App").

[Gizmo](struct.Gizmo.html "struct bevy::prelude::Gizmo")

A component that draws the gizmos of a [`GizmoAsset`](struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset").

[GizmoAsset](struct.GizmoAsset.html "struct bevy::prelude::GizmoAsset")

A collection of gizmos.

[GizmoConfig](struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig")

A struct that stores configuration for gizmos.

[GizmoConfigStore](struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")

A [`Resource`](trait.Resource.html "trait bevy::prelude::Resource") storing [`GizmoConfig`](struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig") and [`GizmoConfigGroup`](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") structs

[GizmoLineConfig](struct.GizmoLineConfig.html "struct bevy::prelude::GizmoLineConfig")

A struct that stores configuration for gizmos.

[Gizmos](struct.Gizmos.html "struct bevy::prelude::Gizmos")

A [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") for drawing gizmos.

[GlobalAmbientLight](struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight")

The global ambient light, which lights the entire scene equally.

[GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform") is an affine transformation from entity-local coordinates to worldspace coordinates.

[GlobalUiDebugOptions](struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions")

Configuration for the UI debug overlay

[GlobalVolume](struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume")

Use this [`Resource`](trait.Resource.html "trait bevy::prelude::Resource") to control the global volume of all audio.

[GlobalZIndex](struct.GlobalZIndex.html "struct bevy::prelude::GlobalZIndex")

`GlobalZIndex` allows a [`Node`](struct.Node.html "struct bevy::prelude::Node") entity anywhere in the UI hierarchy to escape the implicit draw ordering of the UI’s layout tree and be rendered above or below other UI nodes. Nodes with a `GlobalZIndex` of greater than 0 will be drawn on top of nodes without a `GlobalZIndex` or nodes with a lower `GlobalZIndex`. Nodes with a `GlobalZIndex` of less than 0 will be drawn below nodes without a `GlobalZIndex` or nodes with a greater `GlobalZIndex`.

[Gltf](struct.Gltf.html "struct bevy::prelude::Gltf")

Representation of a loaded glTF file.

[GltfExtras](struct.GltfExtras.html "struct bevy::prelude::GltfExtras")

Additional untyped data that can be present on most glTF types at the primitive level.

[GraphCurve](struct.GraphCurve.html "struct bevy::prelude::GraphCurve")

A curve that is the graph of another curve over its parameter space. Curves of this type are produced by [`CurveExt::graph`](trait.CurveExt.html#method.graph "method bevy::prelude::CurveExt::graph").

[GridPlacement](struct.GridPlacement.html "struct bevy::prelude::GridPlacement")

Represents the position of a grid item in a single axis.

[GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

A [`GridTrack`](struct.GridTrack.html "struct bevy::prelude::GridTrack") is a Row or Column of a CSS Grid. This struct specifies what size the track should be. See below for the different “track sizing functions” you can specify.

[HalfSpace](struct.HalfSpace.html "struct bevy::prelude::HalfSpace")

A region of 3D space, specifically an open set whose border is a bisecting 2D plane.

[Has](struct.Has.html "struct bevy::prelude::Has")

Returns a bool that describes if an entity has the component `T`.

[HoveredEntityAncestors](struct.HoveredEntityAncestors.html "struct bevy::prelude::HoveredEntityAncestors")

A cache map containing the ancestry of hovered entities

[Hsla](struct.Hsla.html "struct bevy::prelude::Hsla")

Color in Hue-Saturation-Lightness (HSL) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).

[Hsva](struct.Hsva.html "struct bevy::prelude::Hsva")

Color in Hue-Saturation-Value (HSV) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HSL_and_HSV).

[Hwba](struct.Hwba.html "struct bevy::prelude::Hwba")

Color in Hue-Whiteness-Blackness (HWB) color space with alpha. Further information on this color model can be found on [Wikipedia](https://en.wikipedia.org/wiki/HWB_color_model).

[IRect](struct.IRect.html "struct bevy::prelude::IRect")

A rectangle defined by two opposite corners.

[IVec2](struct.IVec2.html "struct bevy::prelude::IVec2")

A 2-dimensional vector.

[IVec3](struct.IVec3.html "struct bevy::prelude::IVec3")

A 3-dimensional vector.

[IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

A 4-dimensional vector.

[If](struct.If.html "struct bevy::prelude::If")

A [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that wraps another parameter and causes its system to skip instead of failing when the parameter is invalid.

[IgnoreScroll](struct.IgnoreScroll.html "struct bevy::prelude::IgnoreScroll")

Controls whether a UI element ignores its parent’s [`ScrollPosition`](struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition") along specific axes.

[Image](struct.Image.html "struct bevy::prelude::Image")

An image, optimized for usage in rendering.

[ImageNode](struct.ImageNode.html "struct bevy::prelude::ImageNode")

A UI Node that renders an image.

[ImagePlugin](struct.ImagePlugin.html "struct bevy::prelude::ImagePlugin")

Adds the [`Image`](struct.Image.html "struct bevy::prelude::Image") as an asset and makes sure that they are extracted and prepared for the GPU.

[In](struct.In.html "struct bevy::prelude::In")

A [`SystemInput`](trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](trait.System.html "trait bevy::prelude::System") receives an input value of type `T` from its caller.

[InMut](struct.InMut.html "struct bevy::prelude::InMut")

A [`SystemInput`](trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](trait.System.html "trait bevy::prelude::System") receives a mutable reference to a value of type `T` from its caller.

[InRef](struct.InRef.html "struct bevy::prelude::InRef")

A [`SystemInput`](trait.SystemInput.html "trait bevy::prelude::SystemInput") type which denotes that a [`System`](trait.System.html "trait bevy::prelude::System") receives a read-only reference to a value of type `T` from its caller.

[InfinitePlane3d](struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

An unbounded plane in 3D space. It forms a separating surface through the origin, stretching infinitely far

[InheritedVisibility](struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")

Whether or not an entity is visible in the hierarchy.

[Insert](struct.Insert.html "struct bevy::prelude::Insert")

Trigger emitted when a component is inserted, regardless of whether or not the entity already had that component. Runs after `Add`, if it ran. See [`ComponentHooks::on_insert`](../ecs/lifecycle/struct.ComponentHooks.html#method.on_insert "method bevy::ecs::lifecycle::ComponentHooks::on_insert") for more information.

[InteractionPlugin](struct.InteractionPlugin.html "struct bevy::prelude::InteractionPlugin")

Generates [`Pointer`](struct.Pointer.html "struct bevy::prelude::Pointer") events and handles event bubbling.

[Interval](struct.Interval.html "struct bevy::prelude::Interval")

A nonempty closed interval, possibly unbounded in either direction.

[IsDefaultUiCamera](struct.IsDefaultUiCamera.html "struct bevy::prelude::IsDefaultUiCamera")

Marker used to identify default cameras, they will have priority over the [`PrimaryWindow`](../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow") camera.

[Isometry2d](struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

An isometry in two dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

[Isometry3d](struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

An isometry in three dimensions, representing a rotation followed by a translation. This can often be useful for expressing relative positions and transformations from one position to another.

[Laba](struct.Laba.html "struct bevy::prelude::Laba")

Color in LAB color space, with alpha

[Label](struct.Label.html "struct bevy::prelude::Label")

Marker struct for labels

[Last](struct.Last.html "struct bevy::prelude::Last")

Runs last in the schedule.

[LayoutConfig](struct.LayoutConfig.html "struct bevy::prelude::LayoutConfig")

This component can be added to any UI node to modify its layout behavior.

[Lcha](struct.Lcha.html "struct bevy::prelude::Lcha")

Color in LCH color space, with alpha

[Leave](struct.Leave.html "struct bevy::prelude::Leave")

Fires when a pointer crosses out of the bounds of a [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Out`](struct.Out.html "struct bevy::prelude::Out"), this event bubbles up through a subset of the [target entity’s](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship).

[LightGizmoConfigGroup](struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup")

The [`GizmoConfigGroup`](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used to configure the visualization of lights.

[LightProbe](struct.LightProbe.html "struct bevy::prelude::LightProbe")

A marker component for a light probe, which is a cuboid region that provides global illumination to all fragments inside it.

[Line2d](struct.Line2d.html "struct bevy::prelude::Line2d")

An infinite line going through the origin along a direction in 2D space.

[Line3d](struct.Line3d.html "struct bevy::prelude::Line3d")

An infinite line going through the origin along a direction in 3D space.

[LinearCurve](struct.LinearCurve.html "struct bevy::prelude::LinearCurve")

`f(t) = t`

[LinearGradient](struct.LinearGradient.html "struct bevy::prelude::LinearGradient")

A linear gradient

[LinearReparamCurve](struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")

A curve that has had its domain changed by a linear reparameterization (stretching and scaling). Curves of this type are produced by [`CurveExt::reparametrize_linear`](trait.CurveExt.html#method.reparametrize_linear "method bevy::prelude::CurveExt::reparametrize_linear").

[LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

Linear RGB color with alpha.

[Local](struct.Local.html "struct bevy::prelude::Local")

A [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides a system-private value of `T` that persists across system calls.

[Main](struct.Main.html "struct bevy::prelude::Main")

The schedule that contains the app logic that is evaluated each tick of [`App::update()`](struct.App.html#method.update "method bevy::prelude::App::update").

[ManualTextureViews](struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews")

Resource that stores manually managed [`ManualTextureView`](../render/texture/struct.ManualTextureView.html "struct bevy::render::texture::ManualTextureView")s for use as a [`RenderTarget`](../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget"). This type dereferences to a `HashMap<ManualTextureViewHandle, ManualTextureView>`. To add a new texture view, pick a new [`ManualTextureViewHandle`](../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle") and insert it into the map. Then, to render to the view, set a [`Camera`](struct.Camera.html "struct bevy::prelude::Camera")s `target` to `RenderTarget::TextureView(handle)`.

[MapCurve](struct.MapCurve.html "struct bevy::prelude::MapCurve")

A curve whose samples are defined by mapping samples from another curve through a given function. Curves of this type are produced by [`CurveExt::map`](trait.CurveExt.html#method.map "method bevy::prelude::CurveExt::map").

[Mat2](struct.Mat2.html "struct bevy::prelude::Mat2")

A 2x2 column major matrix.

[Mat3](struct.Mat3.html "struct bevy::prelude::Mat3")

A 3x3 column major matrix.

[Mat4](struct.Mat4.html "struct bevy::prelude::Mat4")

A 4x4 column major matrix.

[Mat3A](struct.Mat3A.html "struct bevy::prelude::Mat3A")

A 3x3 column major matrix.

[MaterialNode](struct.MaterialNode.html "struct bevy::prelude::MaterialNode")

[MaterialNodeTemplate](struct.MaterialNodeTemplate.html "struct bevy::prelude::MaterialNodeTemplate")

[MaterialPlugin](struct.MaterialPlugin.html "struct bevy::prelude::MaterialPlugin")

Adds the necessary ECS resources and render logic to enable rendering entities using the given [`Material`](trait.Material.html "trait bevy::prelude::Material") asset type.

[Mesh](struct.Mesh.html "struct bevy::prelude::Mesh")

A 3D object made out of vertices representing triangles, lines, or points, with “attribute” values for each vertex.

[Mesh2d](struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

A component for 2D meshes. Requires a [`MeshMaterial2d`](https://docs.rs/bevy/latest/bevy/prelude/struct.MeshMaterial2d.html) to be rendered, commonly using a [`ColorMaterial`](https://docs.rs/bevy/latest/bevy/prelude/struct.ColorMaterial.html).

[Mesh3d](struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

A component for 3D meshes. Requires a [`MeshMaterial3d`](https://docs.rs/bevy/latest/bevy/pbr/struct.MeshMaterial3d.html) to be rendered, commonly using a [`StandardMaterial`](https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html).

[MeshMaterial2d](struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")

A [material](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d") used for rendering a [`Mesh2d`](struct.Mesh2d.html "struct bevy::prelude::Mesh2d").

[MeshMaterial3d](struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")

A [material](trait.Material.html "trait bevy::prelude::Material") used for rendering a [`Mesh3d`](struct.Mesh3d.html "struct bevy::prelude::Mesh3d").

[MeshPickingCamera](struct.MeshPickingCamera.html "struct bevy::prelude::MeshPickingCamera")

An optional component that marks cameras that should be used in the [`MeshPickingPlugin`](struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin").

[MeshPickingPlugin](struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin")

Adds the mesh picking backend to your app.

[MeshPickingSettings](struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings")

Runtime settings for the [`MeshPickingPlugin`](struct.MeshPickingPlugin.html "struct bevy::prelude::MeshPickingPlugin").

[MeshRayCast](struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast")

Add this ray casting [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") to your system to cast rays into the world with an immediate-mode API. Call `cast_ray` to immediately perform a ray cast and get a result.

[MeshRayCastSettings](struct.MeshRayCastSettings.html "struct bevy::prelude::MeshRayCastSettings")

Settings for a ray cast.

[MessageMutator](struct.MessageMutator.html "struct bevy::prelude::MessageMutator")

Reads and writes [`Message`](trait.Message.html "trait bevy::prelude::Message")s of type `T`, keeping track of which messages have already been read.

[MessageReader](struct.MessageReader.html "struct bevy::prelude::MessageReader")

Reads [`Message`](trait.Message.html "trait bevy::prelude::Message")s of type `T` in order and tracks which messages have already been read.

[MessageWriter](struct.MessageWriter.html "struct bevy::prelude::MessageWriter")

Writes [`Message`](trait.Message.html "trait bevy::prelude::Message")s of type `T`.

[Messages](struct.Messages.html "struct bevy::prelude::Messages")

A message collection that represents the messages that occurred within the last two [`Messages::update`](struct.Messages.html#method.update "method bevy::prelude::Messages::update") calls. Messages can be written to using a [`MessageWriter`](struct.MessageWriter.html "struct bevy::prelude::MessageWriter") and are typically cheaply read using a [`MessageReader`](struct.MessageReader.html "struct bevy::prelude::MessageReader").

[MinimalPlugins](struct.MinimalPlugins.html "struct bevy::prelude::MinimalPlugins")

This plugin group will add the minimal plugins for a _Bevy_ application:

[MorphWeights](struct.MorphWeights.html "struct bevy::prelude::MorphWeights")

Controls the [morph targets](https://en.wikipedia.org/wiki/Morph_target_animation) for all child [`Mesh3d`](struct.Mesh3d.html "struct bevy::prelude::Mesh3d") entities. In most cases, [`MorphWeights`](struct.MorphWeights.html "struct bevy::prelude::MorphWeights") should be considered the “source of truth” when writing [morph targets](https://en.wikipedia.org/wiki/Morph_target_animation) for meshes. However you can choose to write child [`MeshMorphWeights`](../mesh/morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights") if your situation requires more granularity. Just note that if you set [`MorphWeights`](struct.MorphWeights.html "struct bevy::prelude::MorphWeights"), it will overwrite child [`MeshMorphWeights`](../mesh/morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights") values.

[Move](struct.Move.html "struct bevy::prelude::Move")

Fires while a pointer is moving over the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[Mut](struct.Mut.html "struct bevy::prelude::Mut")

Unique mutable borrow of an entity’s component or of a resource.

[Name](struct.Name.html "struct bevy::prelude::Name")

Component used to identify an entity. Stores a hash for faster comparisons.

[NameOrEntity](struct.NameOrEntity.html "struct bevy::prelude::NameOrEntity")

Convenient query for giving a human friendly name to an entity.

[Node](struct.Node.html "struct bevy::prelude::Node")

The base component for UI entities. It describes UI layout and style properties.

[NonPathHandleError](struct.NonPathHandleError.html "struct bevy::prelude::NonPathHandleError")

Error for when only path [`Handle`](enum.Handle.html "enum bevy::prelude::Handle")s are supported.

[NonSend](struct.NonSend.html "struct bevy::prelude::NonSend")

Shared borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[NonSendMut](struct.NonSendMut.html "struct bevy::prelude::NonSendMut")

Unique borrow of a non-[`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") resource.

[Observer](struct.Observer.html "struct bevy::prelude::Observer")

An [`Observer`](struct.Observer.html "struct bevy::prelude::Observer") system. Add this [`Component`](trait.Component.html "trait bevy::prelude::Component") to an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") to turn it into an “observer”.

[Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

Color in Oklab color space, with alpha

[Oklcha](struct.Oklcha.html "struct bevy::prelude::Oklcha")

Color in Oklch color space, with alpha

[On](struct.On.html "struct bevy::prelude::On")

A [system parameter](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") used by an observer to process events. See [`Observer`](struct.Observer.html "struct bevy::prelude::Observer") and [`Event`](trait.Event.html "trait bevy::prelude::Event") for examples.

[OnEnter](struct.OnEnter.html "struct bevy::prelude::OnEnter")

The label of a [`Schedule`](struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](struct.State.html "struct bevy::prelude::State") enters the provided state.

[OnExit](struct.OnExit.html "struct bevy::prelude::OnExit")

The label of a [`Schedule`](struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](struct.State.html "struct bevy::prelude::State") exits the provided state.

[OnTransition](struct.OnTransition.html "struct bevy::prelude::OnTransition")

The label of a [`Schedule`](struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](struct.State.html "struct bevy::prelude::State") exits AND enters the provided `exited` and `entered` states.

[Or](struct.Or.html "struct bevy::prelude::Or")

A filter that tests if any of the given filters apply.

[OrthographicProjection](struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

Project a 3D space onto a 2D surface using parallel lines, i.e., unlike [`PerspectiveProjection`](struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection"), the size of objects remains the same regardless of their distance to the camera.

[Out](struct.Out.html "struct bevy::prelude::Out")

Fires when a pointer crosses out of the bounds of a [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Leave`](struct.Leave.html "struct bevy::prelude::Leave"), this event bubbles up to all of the [target entity’s](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) without restriction. Refer to [`pointer_events`](fn.pointer_events.html "fn bevy::prelude::pointer_events") for more information on how these events are triggered. Refer to [`PointerTraversal`](struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal") for how [`Pointer`](struct.Pointer.html "struct bevy::prelude::Pointer") events are propagated.

[OuterColor](struct.OuterColor.html "struct bevy::prelude::OuterColor")

Sets a color to fill the regions outside the Node’s border created when a border radius is set.

[Outline](struct.Outline.html "struct bevy::prelude::Outline")

The [`Outline`](struct.Outline.html "struct bevy::prelude::Outline") component adds an outline outside the edge of a UI node. Outlines do not take up space in the layout.

[Over](struct.Over.html "struct bevy::prelude::Over")

Fires when a pointer crosses into the bounds of a [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target"). Unlike [`Enter`](struct.Enter.html "struct bevy::prelude::Enter"), this event bubbles up to all of the [target entity’s](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") ancestors (traversed via the [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship) without restriction. Refer to [`pointer_events`](fn.pointer_events.html "fn bevy::prelude::pointer_events") for more information on how these events are triggered. Refer to [`PointerTraversal`](struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal") for how [`Pointer`](struct.Pointer.html "struct bevy::prelude::Pointer") events are propagated.

[Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

Whether to show or hide overflowing items

[OverflowClipMargin](struct.OverflowClipMargin.html "struct bevy::prelude::OverflowClipMargin")

The bounds of the visible area when a UI node is clipped.

[OverrideClip](struct.OverrideClip.html "struct bevy::prelude::OverrideClip")

UI node entities with this component will ignore any clipping rect they inherit, the node will not be clipped regardless of its ancestors’ `Overflow` setting.

[ParallelCommands](struct.ParallelCommands.html "struct bevy::prelude::ParallelCommands")

An alternative to [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") that can be used in parallel contexts, such as those in [`Query::par_iter`](struct.Query.html#method.par_iter "method bevy::prelude::Query::par_iter").

[ParamSet](struct.ParamSet.html "struct bevy::prelude::ParamSet")

A collection of potentially conflicting [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam")s allowed by disjoint access.

[PerspectiveProjection](struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection")

A 3D camera projection in which distant objects appear smaller than close objects.

[Pickable](struct.Pickable.html "struct bevy::prelude::Pickable")

An optional component that marks an entity as usable by a backend, and overrides default picking behavior for an entity.

[PickingMessageWriters](struct.PickingMessageWriters.html "struct bevy::prelude::PickingMessageWriters")

A helper system param for accessing the picking event writers.

[PickingPlugin](struct.PickingPlugin.html "struct bevy::prelude::PickingPlugin")

This plugin sets up the core picking infrastructure. It receives input events, and provides the shared types used by other picking plugins.

[PingPongCurve](struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")

The curve that results from chaining a curve with its reversed version. The transition point is guaranteed to make no jump.

[Pitch](struct.Pitch.html "struct bevy::prelude::Pitch")

A source of sine wave sound

[Plane2d](struct.Plane2d.html "struct bevy::prelude::Plane2d")

An unbounded plane in 2D space. It forms a separating surface through the origin, stretching infinitely far

[Plane3d](struct.Plane3d.html "struct bevy::prelude::Plane3d")

A bounded plane in 3D space. It forms a surface starting from the origin with a defined height and width.

[PlaybackSettings](struct.PlaybackSettings.html "struct bevy::prelude::PlaybackSettings")

Initial settings to be used when audio starts playing.

[PointLight](struct.PointLight.html "struct bevy::prelude::PointLight")

A light that emits light in all directions from a central point.

[Pointer](struct.Pointer.html "struct bevy::prelude::Pointer")

Stores the common data needed for all pointer events.

[PointerButtonState](struct.PointerButtonState.html "struct bevy::prelude::PointerButtonState")

An entry in the cache that drives the `pointer_events` system, storing additional data about pointer button presses.

[PointerInputPlugin](struct.PointerInputPlugin.html "struct bevy::prelude::PointerInputPlugin")

Adds mouse and touch inputs for picking pointers to your app. This is a default input plugin, that you can replace with your own plugin as needed.

[PointerState](struct.PointerState.html "struct bevy::prelude::PointerState")

State for all pointers.

[PointerTraversal](struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal")

A traversal query (i.e. it implements [`Traversal`](../ecs/traversal/trait.Traversal.html "trait bevy::ecs::traversal::Traversal")) intended for use with [`Pointer`](struct.Pointer.html "struct bevy::prelude::Pointer") events.

[PointerTraversalItem](struct.PointerTraversalItem.html "struct bevy::prelude::PointerTraversalItem")

Automatically generated [`WorldQuery`](../ecs/query/trait.WorldQuery.html "trait bevy::ecs::query::WorldQuery") item type for [`PointerTraversal`](struct.PointerTraversal.html "struct bevy::prelude::PointerTraversal"), returned when iterating over query results.

[Polygon](struct.Polygon.html "struct bevy::prelude::Polygon")`alloc`

A polygon with N vertices.

[Polyline2d](struct.Polyline2d.html "struct bevy::prelude::Polyline2d")`alloc`

A series of connected line segments in 2D space.

[Polyline3d](struct.Polyline3d.html "struct bevy::prelude::Polyline3d")`alloc`

A series of connected line segments in 3D space.

[Populated](struct.Populated.html "struct bevy::prelude::Populated")

[System parameter](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that works very much like [`Query`](struct.Query.html "struct bevy::prelude::Query") except it always contains at least one matching entity.

[PopulatedMessageReader](struct.PopulatedMessageReader.html "struct bevy::prelude::PopulatedMessageReader")

Reads [`Message`](trait.Message.html "trait bevy::prelude::Message")s of type `T` in order and tracks which messages have already been read. Skips the system if there no messages.

[PostStartup](struct.PostStartup.html "struct bevy::prelude::PostStartup")

The schedule that runs once after [`Startup`](struct.Startup.html "struct bevy::prelude::Startup").

[PostUpdate](struct.PostUpdate.html "struct bevy::prelude::PostUpdate")

The schedule that contains logic that must run after [`Update`](struct.Update.html "struct bevy::prelude::Update"). For example, synchronizing “local transforms” in a hierarchy to “global” absolute transforms. This enables the [`PostUpdate`](struct.PostUpdate.html "struct bevy::prelude::PostUpdate") transform-sync system to react to “local transform” changes in [`Update`](struct.Update.html "struct bevy::prelude::Update") without the [`Update`](struct.Update.html "struct bevy::prelude::Update") systems needing to know about (or add scheduler dependencies for) the “global transform sync system”.

[PreStartup](struct.PreStartup.html "struct bevy::prelude::PreStartup")

The schedule that runs before [`Startup`](struct.Startup.html "struct bevy::prelude::Startup").

[PreUpdate](struct.PreUpdate.html "struct bevy::prelude::PreUpdate")

The schedule that contains logic that must run before [`Update`](struct.Update.html "struct bevy::prelude::Update"). For example, a system that reads raw keyboard input OS events into a `Messages` resource. This enables systems in [`Update`](struct.Update.html "struct bevy::prelude::Update") to consume the messages from the `Messages` resource without actually knowing about (or taking a direct scheduler dependency on) the “os-level keyboard event system”.

[Press](struct.Press.html "struct bevy::prelude::Press")

Fires when a pointer button is pressed over the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[PreviousState](struct.PreviousState.html "struct bevy::prelude::PreviousState")

The previous state of [`State<S>`](struct.State.html "struct bevy::prelude::State").

[QuadraticInCurve](struct.QuadraticInCurve.html "struct bevy::prelude::QuadraticInCurve")

`f(t) = t²`

[QuadraticInOutCurve](struct.QuadraticInOutCurve.html "struct bevy::prelude::QuadraticInOutCurve")

Behaves as `QuadraticIn` for t < 0.5 and as `QuadraticOut` for t >= 0.5

[QuadraticOutCurve](struct.QuadraticOutCurve.html "struct bevy::prelude::QuadraticOutCurve")

`f(t) = -(t * (t - 2.0))`

[QuarticInCurve](struct.QuarticInCurve.html "struct bevy::prelude::QuarticInCurve")

`f(t) = t⁴`

[QuarticInOutCurve](struct.QuarticInOutCurve.html "struct bevy::prelude::QuarticInOutCurve")

Behaves as `QuarticIn` for t < 0.5 and as `QuarticOut` for t >= 0.5

[QuarticOutCurve](struct.QuarticOutCurve.html "struct bevy::prelude::QuarticOutCurve")

`f(t) = 1.0 - (1.0 - t)⁴`

[Quat](struct.Quat.html "struct bevy::prelude::Quat")

A quaternion representing an orientation.

[Query](struct.Query.html "struct bevy::prelude::Query")

A [system parameter](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides selective access to the [`Component`](trait.Component.html "trait bevy::prelude::Component") data stored in a [`World`](struct.World.html "struct bevy::prelude::World").

[QueryBuilder](struct.QueryBuilder.html "struct bevy::prelude::QueryBuilder")

Builder struct to create [`QueryState`](struct.QueryState.html "struct bevy::prelude::QueryState") instances at runtime.

[QueryState](struct.QueryState.html "struct bevy::prelude::QueryState")

Provides scoped access to a [`World`](struct.World.html "struct bevy::prelude::World") state according to a given [`QueryData`](../ecs/query/trait.QueryData.html "trait bevy::ecs::query::QueryData") and [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

[QuinticInCurve](struct.QuinticInCurve.html "struct bevy::prelude::QuinticInCurve")

`f(t) = t⁵`

[QuinticInOutCurve](struct.QuinticInOutCurve.html "struct bevy::prelude::QuinticInOutCurve")

Behaves as `QuinticIn` for t < 0.5 and as `QuinticOut` for t >= 0.5

[QuinticOutCurve](struct.QuinticOutCurve.html "struct bevy::prelude::QuinticOutCurve")

`f(t) = (t - 1.0)⁵ + 1.0`

[RadialGradient](struct.RadialGradient.html "struct bevy::prelude::RadialGradient")

A radial gradient

[RationalCurve](struct.RationalCurve.html "struct bevy::prelude::RationalCurve")`alloc`

A collection of [`RationalSegment`](struct.RationalSegment.html "struct bevy::prelude::RationalSegment")s chained into a single parametric curve. It is a [`Curve`](trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, N]`, where `N` is the number of segments.

[RationalSegment](struct.RationalSegment.html "struct bevy::prelude::RationalSegment")

A segment of a rational cubic curve, used to hold precomputed coefficients for fast interpolation. It is a [`Curve`](trait.Curve.html "trait bevy::prelude::Curve") with domain `[0, 1]`.

[Ray2d](struct.Ray2d.html "struct bevy::prelude::Ray2d")

An infinite half-line starting at `origin` and going in `direction` in 2D space.

[Ray3d](struct.Ray3d.html "struct bevy::prelude::Ray3d")

An infinite half-line starting at `origin` and going in `direction` in 3D space.

[RayCastBackfaces](struct.RayCastBackfaces.html "struct bevy::prelude::RayCastBackfaces")

Disables backface culling for [ray casts](struct.MeshRayCast.html "struct bevy::prelude::MeshRayCast") on this entity.

[Real](struct.Real.html "struct bevy::prelude::Real")

Real time clock representing elapsed wall clock time.

[Rect](struct.Rect.html "struct bevy::prelude::Rect")

A rectangle defined by two opposite corners.

[RectLight](struct.RectLight.html "struct bevy::prelude::RectLight")

A rectangular area light.

[Rectangle](struct.Rectangle.html "struct bevy::prelude::Rectangle")

A rectangle primitive, which is like a square, except that the width and height can be different

[Ref](struct.Ref.html "struct bevy::prelude::Ref")

Shared borrow of an entity’s component with access to change detection. Similar to [`Mut`](struct.Mut.html "struct bevy::prelude::Mut") but is immutable and so doesn’t require unique access.

[ReflectAdd](struct.ReflectAdd.html "struct bevy::prelude::ReflectAdd")

A struct used to perform addition on reflected values.

[ReflectAddAssign](struct.ReflectAddAssign.html "struct bevy::prelude::ReflectAddAssign")

A struct used to perform addition assignment on reflected values.

[ReflectComponent](struct.ReflectComponent.html "struct bevy::prelude::ReflectComponent")

A struct used to operate on reflected [`Component`](trait.Component.html "trait bevy::prelude::Component") trait of a type.

[ReflectDefault](struct.ReflectDefault.html "struct bevy::prelude::ReflectDefault")

A struct used to provide the default value of a type.

[ReflectDeserialize](struct.ReflectDeserialize.html "struct bevy::prelude::ReflectDeserialize")

A struct used to deserialize reflected instances of a type.

[ReflectDiv](struct.ReflectDiv.html "struct bevy::prelude::ReflectDiv")

A struct used to perform division on reflected values.

[ReflectDivAssign](struct.ReflectDivAssign.html "struct bevy::prelude::ReflectDivAssign")

A struct used to perform division assignment on reflected values.

[ReflectEvent](struct.ReflectEvent.html "struct bevy::prelude::ReflectEvent")

A struct used to operate on reflected [`Event`](trait.Event.html "trait bevy::prelude::Event") trait of a type.

[ReflectFreelyMutableState](struct.ReflectFreelyMutableState.html "struct bevy::prelude::ReflectFreelyMutableState")

A struct used to operate on the reflected [`FreelyMutableState`](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") trait of a type.

[ReflectFromReflect](struct.ReflectFromReflect.html "struct bevy::prelude::ReflectFromReflect")

Type data that represents the [`FromReflect`](trait.FromReflect.html "trait bevy::prelude::FromReflect") trait and allows it to be used dynamically.

[ReflectFromWorld](struct.ReflectFromWorld.html "struct bevy::prelude::ReflectFromWorld")

A struct used to operate on the reflected [`FromWorld`](trait.FromWorld.html "trait bevy::prelude::FromWorld") trait of a type.

[ReflectMessage](struct.ReflectMessage.html "struct bevy::prelude::ReflectMessage")

A struct used to operate on reflected [`Message`](trait.Message.html "trait bevy::prelude::Message") trait of a type.

[ReflectMul](struct.ReflectMul.html "struct bevy::prelude::ReflectMul")

A struct used to perform multiplication on reflected values.

[ReflectMulAssign](struct.ReflectMulAssign.html "struct bevy::prelude::ReflectMulAssign")

A struct used to perform multiplication assignment on reflected values.

[ReflectRem](struct.ReflectRem.html "struct bevy::prelude::ReflectRem")

A struct used to perform remainder on reflected values.

[ReflectRemAssign](struct.ReflectRemAssign.html "struct bevy::prelude::ReflectRemAssign")

A struct used to perform remainder assignment on reflected values.

[ReflectResource](struct.ReflectResource.html "struct bevy::prelude::ReflectResource")

A struct that marks a reflected [`Resource`](trait.Resource.html "trait bevy::prelude::Resource") of a type.

[ReflectSerialize](struct.ReflectSerialize.html "struct bevy::prelude::ReflectSerialize")

A struct used to serialize reflected instances of a type.

[ReflectState](struct.ReflectState.html "struct bevy::prelude::ReflectState")

A struct used to operate on the reflected [`States`](trait.States.html "trait bevy::prelude::States") trait of a type.

[ReflectSub](struct.ReflectSub.html "struct bevy::prelude::ReflectSub")

A struct used to perform subtraction on reflected values.

[ReflectSubAssign](struct.ReflectSubAssign.html "struct bevy::prelude::ReflectSubAssign")

A struct used to perform subtraction assignment on reflected values.

[RegularPolygon](struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

A polygon centered on the origin where all vertices lie on a circle, equally far apart.

[Release](struct.Release.html "struct bevy::prelude::Release")

Fires when a pointer button is released over the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[Remove](struct.Remove.html "struct bevy::prelude::Remove")

Trigger emitted when a component is removed from an entity, and runs before the component is removed, so you can still access the component data. See [`ComponentHooks::on_remove`](../ecs/lifecycle/struct.ComponentHooks.html#method.on_remove "method bevy::ecs::lifecycle::ComponentHooks::on_remove") for more information.

[RemovedComponents](struct.RemovedComponents.html "struct bevy::prelude::RemovedComponents")

A [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that yields entities that had their `T` [`Component`](trait.Component.html "trait bevy::prelude::Component") removed or have been despawned with it.

[RenderGraph](struct.RenderGraph.html "struct bevy::prelude::RenderGraph")

Schedule label for the root render graph schedule. This schedule runs once per frame in the [`render_system`](../render/renderer/fn.render_system.html "fn bevy::render::renderer::render_system") system and is responsible for driving the entire rendering process.

[ReparamCurve](struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")

A curve whose sample space is mapped onto that of some base curve’s before sampling. Curves of this type are produced by [`CurveExt::reparametrize`](trait.CurveExt.html#method.reparametrize "method bevy::prelude::CurveExt::reparametrize").

[RepeatCurve](struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")

The curve that results from repeating a curve `N` times.

[RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

Represents a _possibly_ repeated [`GridTrack`](struct.GridTrack.html "struct bevy::prelude::GridTrack").

[Res](struct.Res.html "struct bevy::prelude::Res")

Shared borrow of a [`Resource`](trait.Resource.html "trait bevy::prelude::Resource").

[ResMut](struct.ResMut.html "struct bevy::prelude::ResMut")

Unique mutable borrow of a [`Resource`](trait.Resource.html "trait bevy::prelude::Resource").

[ResolvedBorderRadius](struct.ResolvedBorderRadius.html "struct bevy::prelude::ResolvedBorderRadius")

Represents the resolved border radius values for a UI node.

[ReverseCurve](struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")

The curve that results from reversing another.

[Rhombus](struct.Rhombus.html "struct bevy::prelude::Rhombus")

A rhombus primitive, also known as a diamond shape. A four sided polygon, centered on the origin, where opposite sides are parallel but without requiring right angles.

[Ring](struct.Ring.html "struct bevy::prelude::Ring")

A 2D shape representing the ring version of a base shape.

[Rot2](struct.Rot2.html "struct bevy::prelude::Rot2")

A 2D rotation.

[RumbleSystems](struct.RumbleSystems.html "struct bevy::prelude::RumbleSystems")

Updates the running gamepad rumble effects.

[RunFixedMainLoop](struct.RunFixedMainLoop.html "struct bevy::prelude::RunFixedMainLoop")

Runs the [`FixedMain`](../app/struct.FixedMain.html "struct bevy::app::FixedMain") schedule in a loop according until all relevant elapsed time has been “consumed”.

[SampleAutoCurve](struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")

A curve that is defined by neighbor interpolation over a set of evenly-spaced samples, interpolated automatically using [a particularly well-behaved interpolation](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[SampleCurve](struct.SampleCurve.html "struct bevy::prelude::SampleCurve")

A curve that is defined by explicit neighbor interpolation over a set of evenly-spaced samples.

[ScenePatchInstance](struct.ScenePatchInstance.html "struct bevy::prelude::ScenePatchInstance")

A component that, when added, will queue applying the given [`ScenePatch`](../scene/struct.ScenePatch.html "struct bevy::scene::ScenePatch") after the scene and its dependencies have been loaded and resolved.

[Schedule](struct.Schedule.html "struct bevy::prelude::Schedule")

A collection of systems, and the metadata and executor needed to run them in a certain order under certain conditions.

[Schedules](struct.Schedules.html "struct bevy::prelude::Schedules")

Resource that stores [`Schedule`](struct.Schedule.html "struct bevy::prelude::Schedule")s mapped to [`ScheduleLabel`](../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")s excluding the current running [`Schedule`](struct.Schedule.html "struct bevy::prelude::Schedule").

[ScreenSpaceAmbientOcclusionPlugin](struct.ScreenSpaceAmbientOcclusionPlugin.html "struct bevy::prelude::ScreenSpaceAmbientOcclusionPlugin")

Plugin for screen space ambient occlusion.

[Scroll](struct.Scroll.html "struct bevy::prelude::Scroll")

Fires while a pointer is scrolling over the [target entity](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target").

[ScrollPosition](struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

The scroll position of the node. Values are in logical pixels, increasing from top-left to bottom-right.

[Segment2d](struct.Segment2d.html "struct bevy::prelude::Segment2d")

A line segment defined by two endpoints in 2D space.

[Segment3d](struct.Segment3d.html "struct bevy::prelude::Segment3d")

A line segment defined by two endpoints in 3D space.

[SerializedAnimationGraph](struct.SerializedAnimationGraph.html "struct bevy::prelude::SerializedAnimationGraph")

A version of [`AnimationGraph`](struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph") suitable for serializing as an asset.

[SerializedAnimationGraphNode](struct.SerializedAnimationGraphNode.html "struct bevy::prelude::SerializedAnimationGraphNode")

A version of [`AnimationGraphNode`](struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode") suitable for serializing as an asset.

[Shader](struct.Shader.html "struct bevy::prelude::Shader")

An “unprocessed” shader. It can contain preprocessor directives and imports.

[ShadowStyle](struct.ShadowStyle.html "struct bevy::prelude::ShadowStyle")

[ShortName](struct.ShortName.html "struct bevy::prelude::ShortName")

Lazily shortens a type name to remove all module paths.

[ShowAabbGizmo](struct.ShowAabbGizmo.html "struct bevy::prelude::ShowAabbGizmo")

Add this [`Component`](trait.Component.html "trait bevy::prelude::Component") to an entity to draw its [`Aabb`](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component.

[ShowFrustumGizmo](struct.ShowFrustumGizmo.html "struct bevy::prelude::ShowFrustumGizmo")

Add this [`Component`](trait.Component.html "trait bevy::prelude::Component") to an entity to draw its [`Frustum`](../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum") component.

[ShowLightGizmo](struct.ShowLightGizmo.html "struct bevy::prelude::ShowLightGizmo")

Add this [`Component`](trait.Component.html "trait bevy::prelude::Component") to an entity to draw any of its lights components ([`PointLight`](struct.PointLight.html "struct bevy::prelude::PointLight"), [`SpotLight`](struct.SpotLight.html "struct bevy::prelude::SpotLight"), [`DirectionalLight`](struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight") and [`RectLight`](struct.RectLight.html "struct bevy::prelude::RectLight")).

[ShowSkinnedMeshBoundsGizmo](struct.ShowSkinnedMeshBoundsGizmo.html "struct bevy::prelude::ShowSkinnedMeshBoundsGizmo")

Add this [`Component`](trait.Component.html "trait bevy::prelude::Component") to an entity to draw its [`DynamicSkinnedMeshBounds`](../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds") component.

[SineInCurve](struct.SineInCurve.html "struct bevy::prelude::SineInCurve")

`f(t) = 1.0 - cos(t * π / 2.0)`

[SineInOutCurve](struct.SineInOutCurve.html "struct bevy::prelude::SineInOutCurve")

Behaves as `SineIn` for t < 0.5 and as `SineOut` for t >= 0.5

[SineOutCurve](struct.SineOutCurve.html "struct bevy::prelude::SineOutCurve")

`f(t) = sin(t * π / 2.0)`

[Single](struct.Single.html "struct bevy::prelude::Single")

[System parameter](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam") that provides access to single entity’s components, much like [`Query::single`](struct.Query.html#method.single "method bevy::prelude::Query::single")/[`Query::single_mut`](struct.Query.html#method.single_mut "method bevy::prelude::Query::single_mut").

[SkinnedMeshBoundsGizmoConfigGroup](struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup")

The [`GizmoConfigGroup`](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") used for debug visualizations of entities with [`DynamicSkinnedMeshBounds`](../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

[SmoothStepCurve](struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve")

`f(t) = 3t² - 2t³`

[SmoothStepInCurve](struct.SmoothStepInCurve.html "struct bevy::prelude::SmoothStepInCurve")

Behaves as the first half of [`SmoothStepCurve`](struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmoothStepOutCurve](struct.SmoothStepOutCurve.html "struct bevy::prelude::SmoothStepOutCurve")

Behaves as the second half of [`SmoothStepCurve`](struct.SmoothStepCurve.html "struct bevy::prelude::SmoothStepCurve").

[SmootherStepCurve](struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve")

`f(t) = 6t⁵ - 15t⁴ + 10t³`

[SmootherStepInCurve](struct.SmootherStepInCurve.html "struct bevy::prelude::SmootherStepInCurve")

Behaves as the first half of [`SmootherStepCurve`](struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[SmootherStepOutCurve](struct.SmootherStepOutCurve.html "struct bevy::prelude::SmootherStepOutCurve")

Behaves as the second half of [`SmootherStepCurve`](struct.SmootherStepCurve.html "struct bevy::prelude::SmootherStepCurve").

[SpatialAudioSink](struct.SpatialAudioSink.html "struct bevy::prelude::SpatialAudioSink")

Used to control spatial audio during playback.

[SpatialListener](struct.SpatialListener.html "struct bevy::prelude::SpatialListener")

Settings for the listener for spatial audio sources.

[Spawn](struct.Spawn.html "struct bevy::prelude::Spawn")

A wrapper over a [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle") indicating that an entity should be spawned with that [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle"). This is intended to be used for hierarchical spawning via traits like [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") and [`SpawnRelated`](trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated").

[SpawnIter](struct.SpawnIter.html "struct bevy::prelude::SpawnIter")

A [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that spawns entities using an iterator of a given [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle"):

[SpawnScene](struct.SpawnScene.html "struct bevy::prelude::SpawnScene")

The schedule that contains scene spawning.

[SpawnWith](struct.SpawnWith.html "struct bevy::prelude::SpawnWith")

A [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that spawns entities using a [`FnOnce`](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce") with a [`RelatedSpawner`](../ecs/relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner") as an argument:

[Sphere](struct.Sphere.html "struct bevy::prelude::Sphere")

A sphere primitive, representing the set of all points some distance from the origin

[SpotLight](struct.SpotLight.html "struct bevy::prelude::SpotLight")

A light that emits light in a given direction from a central point.

[Sprite](struct.Sprite.html "struct bevy::prelude::Sprite")

Describes a sprite to be rendered to a 2D camera

[SpriteMaterial](struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[SpriteMesh](struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

This is a carbon copy of [`Sprite`](struct.Sprite.html "struct bevy::prelude::Sprite") that uses the Mesh backend instead of the Sprite backend.

[SpritePickingCamera](struct.SpritePickingCamera.html "struct bevy::prelude::SpritePickingCamera")

An optional component that marks cameras that should be used in the [`SpritePickingPlugin`](struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin").

[SpritePickingPlugin](struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin")

Enables the sprite picking backend, allowing you to click on, hover over and drag sprites.

[SpritePickingSettings](struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings")

Runtime settings for the [`SpritePickingPlugin`](struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin").

[Srgba](struct.Srgba.html "struct bevy::prelude::Srgba")

Non-linear standard RGB with alpha.

[StandardMaterial](struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

A material with “standard” properties used in PBR lighting. Standard property values with pictures here: [https://google.github.io/filament/notes/material\_properties.html](https://google.github.io/filament/notes/material_properties.html).

[Startup](struct.Startup.html "struct bevy::prelude::Startup")

The schedule that runs once when the app starts.

[State](struct.State.html "struct bevy::prelude::State")

A finite-state machine whose transitions have associated schedules ([`OnEnter(state)`](struct.OnEnter.html "struct bevy::prelude::OnEnter") and [`OnExit(state)`](struct.OnExit.html "struct bevy::prelude::OnExit")).

[StateTransition](struct.StateTransition.html "struct bevy::prelude::StateTransition")

Runs [state transitions](trait.States.html "trait bevy::prelude::States").

[StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")

A [`Message`](trait.Message.html "trait bevy::prelude::Message") sent when any state transition of `S` happens. This includes identity transitions, where `exited` and `entered` have the same value.

[StepsCurve](struct.StepsCurve.html "struct bevy::prelude::StepsCurve")

`n` steps connecting the start and the end. Jumping behavior is customizable via [`JumpAt`](enum.JumpAt.html "enum bevy::prelude::JumpAt"). See [`JumpAt`](enum.JumpAt.html "enum bevy::prelude::JumpAt") for all the options and visual examples.

[Strikethrough](struct.Strikethrough.html "struct bevy::prelude::Strikethrough")

A text entity with this component is drawn with strikethrough.

[StrikethroughColor](struct.StrikethroughColor.html "struct bevy::prelude::StrikethroughColor")

Color for the text’s strikethrough. If this component is not present, its `TextColor` will be used.

[String](struct.String.html "struct bevy::prelude::String")

A UTF-8–encoded, growable string.

[SubApp](struct.SubApp.html "struct bevy::prelude::SubApp")

A secondary application with its own [`World`](struct.World.html "struct bevy::prelude::World"). These can run independently of each other.

[TaskPoolOptions](struct.TaskPoolOptions.html "struct bevy::prelude::TaskPoolOptions")

Helper for configuring and creating the default task pools. For end-users who want full control, set up [`TaskPoolPlugin`](struct.TaskPoolPlugin.html "struct bevy::prelude::TaskPoolPlugin")

[TaskPoolPlugin](struct.TaskPoolPlugin.html "struct bevy::prelude::TaskPoolPlugin")

Setup of default task pools: [`AsyncComputeTaskPool`](../tasks/struct.AsyncComputeTaskPool.html "struct bevy::tasks::AsyncComputeTaskPool"), [`ComputeTaskPool`](../tasks/struct.ComputeTaskPool.html "struct bevy::tasks::ComputeTaskPool"), [`IoTaskPool`](../tasks/struct.IoTaskPool.html "struct bevy::tasks::IoTaskPool").

[Tetrahedron](struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

A tetrahedron primitive.

[Text](struct.Text.html "struct bevy::prelude::Text")

The top-level UI text component.

[Text2d](struct.Text2d.html "struct bevy::prelude::Text2d")

The top-level 2D text component.

[TextBackgroundColor](struct.TextBackgroundColor.html "struct bevy::prelude::TextBackgroundColor")

The background color of the text for this section.

[TextColor](struct.TextColor.html "struct bevy::prelude::TextColor")

The color of the text for this section.

[TextFont](struct.TextFont.html "struct bevy::prelude::TextFont")

`TextFont` determines the style of a text span within a [`ComputedTextBlock`](../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock"), specifically the font face, the font size, the line height, and the antialiasing method.

[TextLayout](struct.TextLayout.html "struct bevy::prelude::TextLayout")

Component with text format settings for a block of text.

[TextShadow](struct.TextShadow.html "struct bevy::prelude::TextShadow")

Adds a shadow behind text

[TextSpan](struct.TextSpan.html "struct bevy::prelude::TextSpan")

A span of text in a tree of spans.

[TextureAtlas](struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

An index into a [`TextureAtlasLayout`](struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout"), which corresponds to a specific section of a texture.

[TextureAtlasBuilder](struct.TextureAtlasBuilder.html "struct bevy::prelude::TextureAtlasBuilder")

A builder which is used to create a texture atlas from many individual sprites.

[TextureAtlasLayout](struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

Stores a map used to lookup the position of a texture in a [`TextureAtlas`](struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas"). This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.

[TextureAtlasSources](struct.TextureAtlasSources.html "struct bevy::prelude::TextureAtlasSources")

Stores a mapping from sub texture handles to the related area index.

[TextureSlice](struct.TextureSlice.html "struct bevy::prelude::TextureSlice")

Single texture slice, representing a texture rect to draw in a given area

[TextureSlicer](struct.TextureSlicer.html "struct bevy::prelude::TextureSlicer")

Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes without needing to prepare multiple assets. The associated texture will be split into nine portions, so that on resize the different portions scale or tile in different ways to keep the texture in proportion.

[ThreadedAnimationGraph](struct.ThreadedAnimationGraph.html "struct bevy::prelude::ThreadedAnimationGraph")

An acceleration structure for an animation graph that allows Bevy to evaluate it quickly.

[ThreadedAnimationGraphs](struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs")

Acceleration structures for animation graphs that allows Bevy to evaluate them quickly.

[Time](struct.Time.html "struct bevy::prelude::Time")

A generic clock resource that tracks how much it has advanced since its previous update and since its creation.

[Timer](struct.Timer.html "struct bevy::prelude::Timer")

Tracks elapsed time. Enters the finished state once `duration` is reached.

[Torus](struct.Torus.html "struct bevy::prelude::Torus")

A torus primitive, often representing a ring or donut shape The set of points some distance from a circle centered at the origin

[TouchInput](struct.TouchInput.html "struct bevy::prelude::TouchInput")

A touch input event.

[Touches](struct.Touches.html "struct bevy::prelude::Touches")

A collection of [`Touch`](../input/touch/struct.Touch.html "struct bevy::input::touch::Touch")es.

[Transform](struct.Transform.html "struct bevy::prelude::Transform")

Describe the position of an entity. If the entity has a parent, the position is relative to its parent position.

[TransformGizmoCamera](struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera")

Marker component for the camera the transform gizmo should use.

[TransformGizmoFocus](struct.TransformGizmoFocus.html "struct bevy::prelude::TransformGizmoFocus")

Component that marks the entity the transform gizmo operates on.

[TransformGizmoPlugin](struct.TransformGizmoPlugin.html "struct bevy::prelude::TransformGizmoPlugin")

Opt-in plugin that adds the interactive transform gizmo.

[TransformGizmoSettings](struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings")

Configuration and preferences for the transform gizmo.

[TransformGizmoState](struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState")

Runtime state of the transform gizmo (drag and hover).

[TransformGizmoSystems](struct.TransformGizmoSystems.html "struct bevy::prelude::TransformGizmoSystems")

System set for the transform gizmo. All transform gizmo systems run in [`PostUpdate`](struct.PostUpdate.html "struct bevy::prelude::PostUpdate") within this set.

[TransformHelper](struct.TransformHelper.html "struct bevy::prelude::TransformHelper")

System parameter for computing up-to-date [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")s.

[TransformPlugin](struct.TransformPlugin.html "struct bevy::prelude::TransformPlugin")

The base plugin for handling [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") components

[TransformTreeChanged](struct.TransformTreeChanged.html "struct bevy::prelude::TransformTreeChanged")

An optimization for transform propagation. This ZST marker component uses change detection to mark all entities of the hierarchy as “dirty” if any of their descendants have a changed `Transform`. If this component is _not_ marked `is_changed()`, propagation will halt.

[TransitionSchedules](struct.TransitionSchedules.html "struct bevy::prelude::TransitionSchedules")

System set that runs transition schedule(s) for state `S`.

[Triangle2d](struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

A triangle in 2D space

[Triangle3d](struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

A 3D triangle primitive.

[URect](struct.URect.html "struct bevy::prelude::URect")

A rectangle defined by two opposite corners.

[UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

A 2-dimensional vector.

[UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

A 3-dimensional vector.

[UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

A 4-dimensional vector.

[UiDebugOptions](struct.UiDebugOptions.html "struct bevy::prelude::UiDebugOptions")

Configuration for the UI debug overlay

[UiGlobalTransform](struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

Absolute 2D transform for UI nodes

[UiMaterialKey](struct.UiMaterialKey.html "struct bevy::prelude::UiMaterialKey")

[UiMaterialPlugin](struct.UiMaterialPlugin.html "struct bevy::prelude::UiMaterialPlugin")

Adds the necessary ECS resources and render logic to enable rendering entities using the given [`UiMaterial`](trait.UiMaterial.html "trait bevy::prelude::UiMaterial") asset type (which includes [`UiMaterial`](trait.UiMaterial.html "trait bevy::prelude::UiMaterial") types).

[UiPickingCamera](struct.UiPickingCamera.html "struct bevy::prelude::UiPickingCamera")

An optional component that marks cameras that should be used in the [`UiPickingPlugin`](struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin").

[UiPickingPlugin](struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin")

A plugin that adds picking support for UI nodes.

[UiPickingSettings](struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings")

Runtime settings for the [`UiPickingPlugin`](struct.UiPickingPlugin.html "struct bevy::prelude::UiPickingPlugin").

[UiPosition](struct.UiPosition.html "struct bevy::prelude::UiPosition")

Responsive position relative to a UI node.

[UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

A type which is commonly used to define margins, paddings and borders.

[UiScale](struct.UiScale.html "struct bevy::prelude::UiScale")

The current scale of the UI.

[UiTargetCamera](struct.UiTargetCamera.html "struct bevy::prelude::UiTargetCamera")

Indicates that this root [`Node`](struct.Node.html "struct bevy::prelude::Node") entity should be rendered to a specific camera.

[UiTransform](struct.UiTransform.html "struct bevy::prelude::UiTransform")

Relative 2D transform for UI nodes

[Underline](struct.Underline.html "struct bevy::prelude::Underline")

Add to a text entity to draw its text with underline.

[UnderlineColor](struct.UnderlineColor.html "struct bevy::prelude::UnderlineColor")

Color for the text’s underline. If this component is not present, its `TextColor` will be used.

[UnevenCore](struct.UnevenCore.html "struct bevy::prelude::UnevenCore")`alloc`

The data core of a curve defined by unevenly-spaced samples or keyframes. The intention is to use this in concert with implicitly or explicitly-defined interpolation in user-space in order to implement the curve interface using [`domain`](struct.UnevenCore.html#method.domain "method bevy::prelude::UnevenCore::domain") and [`sample_with`](struct.UnevenCore.html#method.sample_with "method bevy::prelude::UnevenCore::sample_with").

[UnevenSampleAutoCurve](struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")

A curve that is defined by interpolation over unevenly spaced samples, interpolated automatically using [a particularly well-behaved interpolation](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[UnevenSampleCurve](struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")

A curve that is defined by interpolation over unevenly spaced samples with explicit interpolation.

[Update](struct.Update.html "struct bevy::prelude::Update")

The schedule that contains any app logic that must run once per render frame. For most gameplay logic, consider using [`FixedUpdate`](struct.FixedUpdate.html "struct bevy::prelude::FixedUpdate") instead.

[Val2](struct.Val2.html "struct bevy::prelude::Val2")

A pair of [`Val`](enum.Val.html "enum bevy::prelude::Val")s used to represent a 2-dimensional size or offset.

[VariableCurve](struct.VariableCurve.html "struct bevy::prelude::VariableCurve")

Contains an [animation curve](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") which is used to animate a property of an entity.

[Vec](struct.Vec.html "struct bevy::prelude::Vec")

A contiguous growable array type, written as `Vec<T>`, short for ‘vector’.

[Vec2](struct.Vec2.html "struct bevy::prelude::Vec2")

A 2-dimensional vector.

[Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")

A 3-dimensional vector.

[Vec4](struct.Vec4.html "struct bevy::prelude::Vec4")

A 4-dimensional vector.

[Vec3A](struct.Vec3A.html "struct bevy::prelude::Vec3A")

A 3-dimensional vector.

[ViewFrustum](struct.ViewFrustum.html "struct bevy::prelude::ViewFrustum")

A region of 3D space defined by the intersection of 6 [`HalfSpace`](struct.HalfSpace.html "struct bevy::prelude::HalfSpace")s.

[ViewVisibility](struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")

Algorithmically computed indication of whether an entity is visible and should be extracted for rendering.

[ViewportNode](struct.ViewportNode.html "struct bevy::prelude::ViewportNode")

Component used to render a [`RenderTarget`](../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget") to a node.

[Virtual](struct.Virtual.html "struct bevy::prelude::Virtual")

The virtual game clock representing game time.

[WeightsCurve](struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")

This type allows an [`IterableCurve`](iterable/trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve") valued in `f32` to be used as an [`AnimationCurve`](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") that animates [morph weights](struct.MorphWeights.html "struct bevy::prelude::MorphWeights").

[WeightsCurveSample](struct.WeightsCurveSample.html "struct bevy::prelude::WeightsCurveSample")

Type indicating that the sampled value from an animation curve is coming from a [`WeightsCurve`](struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve").

[Window](struct.Window.html "struct bevy::prelude::Window")

The defining [`Component`](trait.Component.html "trait bevy::prelude::Component") for window entities, storing information about how it should appear and behave.

[WindowMoved](struct.WindowMoved.html "struct bevy::prelude::WindowMoved")

An event that is sent when a window is repositioned in physical pixels.

[WindowPlugin](struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin")

A [`Plugin`](trait.Plugin.html "trait bevy::prelude::Plugin") that defines an interface for windowing support in Bevy.

[WindowResizeConstraints](struct.WindowResizeConstraints.html "struct bevy::prelude::WindowResizeConstraints")

The size limits on a [`Window`](struct.Window.html "struct bevy::prelude::Window").

[With](struct.With.html "struct bevy::prelude::With")

Filter that selects entities with a component `T`.

[WithOneRelated](struct.WithOneRelated.html "struct bevy::prelude::WithOneRelated")

A wrapper over an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") indicating that an entity should be added. This is intended to be used for hierarchical spawning via traits like [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") and [`SpawnRelated`](trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated").

[WithRelated](struct.WithRelated.html "struct bevy::prelude::WithRelated")

A [`SpawnableList`](../ecs/spawn/trait.SpawnableList.html "trait bevy::ecs::spawn::SpawnableList") that links already spawned entities to the root entity via relations of type `I`.

[Without](struct.Without.html "struct bevy::prelude::Without")

Filter that selects entities without a component `T`.

[World](struct.World.html "struct bevy::prelude::World")

Stores and exposes operations on [entities](struct.Entity.html "struct bevy::prelude::Entity"), [components](trait.Component.html "trait bevy::prelude::Component"), resources, and their associated metadata.

[WorldAsset](struct.WorldAsset.html "struct bevy::prelude::WorldAsset")

A composition of [`World`](struct.World.html "struct bevy::prelude::World") objects.

[WorldAssetRoot](struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

Adding this component will spawn the world as a child of that entity. Once it’s spawned, the entity will have a [`WorldInstance`](../world_serialization/struct.WorldInstance.html "struct bevy::world_serialization::WorldInstance") component.

[WorldInstanceSpawner](struct.WorldInstanceSpawner.html "struct bevy::prelude::WorldInstanceSpawner")

Handles spawning and despawning world instances, either synchronously or batched through the [`world_instance_spawner_system`](../world_serialization/fn.world_instance_spawner_system.html "fn bevy::world_serialization::world_instance_spawner_system").

[Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")

[CIE 1931](https://en.wikipedia.org/wiki/CIE_1931_color_space) color space, also known as XYZ, with an alpha channel.

[ZIndex](struct.ZIndex.html "struct bevy::prelude::ZIndex")

Indicates that this [`Node`](struct.Node.html "struct bevy::prelude::Node") entity’s front-to-back ordering is not controlled solely by its location in the UI hierarchy. A node with a higher z-index will appear on top of sibling nodes with a lower z-index.

[ZipCurve](struct.ZipCurve.html "struct bevy::prelude::ZipCurve")

A curve that combines the output data from two constituent curves into a tuple output. Curves of this type are produced by [`CurveExt::zip`](trait.CurveExt.html#method.zip "method bevy::prelude::CurveExt::zip").

## Enums

[AlignContent](enum.AlignContent.html "enum bevy::prelude::AlignContent")

Used to control how items are distributed.

[AlignItems](enum.AlignItems.html "enum bevy::prelude::AlignItems")

Used to control how each individual item is aligned by default within the space they’re given.

[AlignSelf](enum.AlignSelf.html "enum bevy::prelude::AlignSelf")

Used to control how the specified item is aligned within the space it’s given.

[AlphaMode](enum.AlphaMode.html "enum bevy::prelude::AlphaMode")

Sets how a material’s base color alpha channel is used for transparency.

[AnimationGraphLoadError](enum.AnimationGraphLoadError.html "enum bevy::prelude::AnimationGraphLoadError")

Errors that can occur when deserializing animation graphs from RON.

[AnimationGraphSaveError](enum.AnimationGraphSaveError.html "enum bevy::prelude::AnimationGraphSaveError")

Errors that can occur when serializing animation graphs to RON.

[AnimationNodeType](enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType")

Animation node data specific to the type of node (clip, blend, or add).

[AppExit](enum.AppExit.html "enum bevy::prelude::AppExit")

A [`Message`](trait.Message.html "trait bevy::prelude::Message") that indicates the [`App`](struct.App.html "struct bevy::prelude::App") should exit. If one or more of these are present at the end of an update, the [runner](struct.App.html#method.set_runner "method bevy::prelude::App::set_runner") will end and ([maybe](struct.App.html#method.run "method bevy::prelude::App::run")) return control to the caller.

[AssetEvent](enum.AssetEvent.html "enum bevy::prelude::AssetEvent")

[`Message`](trait.Message.html "trait bevy::prelude::Message")s that occur for a specific loaded [`Asset`](trait.Asset.html "trait bevy::prelude::Asset"), such as “value changed” events and “dependency” events.

[AssetId](enum.AssetId.html "enum bevy::prelude::AssetId")

A unique runtime-only identifier for an [`Asset`](trait.Asset.html "trait bevy::prelude::Asset"). This is cheap to [`Copy`](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html "trait core::marker::Copy")/[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") and is not directly tied to the lifetime of the Asset. This means it _can_ point to an [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") that no longer exists.

[AssetMode](enum.AssetMode.html "enum bevy::prelude::AssetMode")

Controls whether or not assets are pre-processed before being loaded.

[BoxSizing](enum.BoxSizing.html "enum bevy::prelude::BoxSizing")

Which part of a Node’s box length styles like width and height control

[ChainError](enum.ChainError.html "enum bevy::prelude::ChainError")

An error indicating that an end-to-end composition couldn’t be performed because of malformed inputs.

[ClearColorConfig](enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

For a camera, specifies the color used to clear the viewport [before rendering](struct.Camera.html#structfield.clear_color "field bevy::prelude::Camera::clear_color") or when [writing to the final render target texture](struct.Camera.html#structfield.output_mode "field bevy::prelude::Camera::output_mode").

[ClipboardRead](enum.ClipboardRead.html "enum bevy::prelude::ClipboardRead")

Represents an attempt to read from the clipboard.

[Color](enum.Color.html "enum bevy::prelude::Color")

An enumerated type that can represent any of the color types in this crate.

[CompositingSpace](enum.CompositingSpace.html "enum bevy::prelude::CompositingSpace")

Color space for alpha compositing. Affects how overlapping semi-transparent layers blend.

[ConvexPolygonError](enum.ConvexPolygonError.html "enum bevy::prelude::ConvexPolygonError")`alloc`

An error that happens when creating a [`ConvexPolygon`](struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon").

[CubicNurbsError](enum.CubicNurbsError.html "enum bevy::prelude::CubicNurbsError")

Error during construction of [`CubicNurbs`](struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")

[Display](enum.Display.html "enum bevy::prelude::Display")

Defines the layout model used by this node.

[EaseFunction](enum.EaseFunction.html "enum bevy::prelude::EaseFunction")

Curve functions over the [unit interval](struct.Interval.html#associatedconstant.UNIT "associated constant bevy::prelude::Interval::UNIT"), commonly used for easing transitions.

[EulerRot](enum.EulerRot.html "enum bevy::prelude::EulerRot")

Euler rotation sequences.

[EvaluatorId](enum.EvaluatorId.html "enum bevy::prelude::EvaluatorId")

The [`EvaluatorId`](enum.EvaluatorId.html "enum bevy::prelude::EvaluatorId") is used to look up the [`AnimationCurveEvaluator`](trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator") for an [`AnimatableProperty`](trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty"). For a given animated property, this ID should always be the same to allow things like animation blending to occur.

[FileDragAndDrop](enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop")

Events related to files being dragged and dropped on a window.

[FlexDirection](enum.FlexDirection.html "enum bevy::prelude::FlexDirection")

Defines how flexbox items are ordered within a flexbox

[FlexWrap](enum.FlexWrap.html "enum bevy::prelude::FlexWrap")

Defines if flexbox items appear on a single line or on multiple lines

[FogFalloff](enum.FogFalloff.html "enum bevy::prelude::FogFalloff")

Allows switching between different fog falloff modes, and configuring their parameters.

[FontHinting](enum.FontHinting.html "enum bevy::prelude::FontHinting")

Font hinting strategy, which controls the rasterization for fonts.

[FontSize](enum.FontSize.html "enum bevy::prelude::FontSize")

The vertical height of rasterized glyphs in the font atlas in pixels.

[FontSmoothing](enum.FontSmoothing.html "enum bevy::prelude::FontSmoothing")

Determines which antialiasing method to use when rendering text. By default, text is rendered with grayscale antialiasing, but this can be changed to achieve a pixelated look.

[FontSource](enum.FontSource.html "enum bevy::prelude::FontSource")

Determines how the font face for a text sections is selected.

[FontStyle](enum.FontStyle.html "enum bevy::prelude::FontStyle")

The slant style of a font face: normal, italic, or oblique.

[GamepadAxis](enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis")

Represents gamepad input types that are mapped in the range \[-1.0, 1.0\].

[GamepadButton](enum.GamepadButton.html "enum bevy::prelude::GamepadButton")

Represents gamepad input types that are mapped in the range \[0.0, 1.0\].

[GizmoLineJoint](enum.GizmoLineJoint.html "enum bevy::prelude::GizmoLineJoint")

An enum configuring how line joints will be drawn.

[GizmoLineStyle](enum.GizmoLineStyle.html "enum bevy::prelude::GizmoLineStyle")

An enum used to configure the style of gizmo lines, similar to CSS line-style

[GltfAssetLabel](enum.GltfAssetLabel.html "enum bevy::prelude::GltfAssetLabel")

Labels that can be used to load part of a glTF

[Gradient](enum.Gradient.html "enum bevy::prelude::Gradient")

[GridAutoFlow](enum.GridAutoFlow.html "enum bevy::prelude::GridAutoFlow")

Controls whether grid items are placed row-wise or column-wise as well as whether the sparse or dense packing algorithm is used.

[GridPlacementError](enum.GridPlacementError.html "enum bevy::prelude::GridPlacementError")

Errors that occur when setting constraints for a `GridPlacement`

[GridTrackRepetition](enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")

How many times to repeat a repeated grid track

[Handle](enum.Handle.html "enum bevy::prelude::Handle")

A handle to a specific [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") of type `A`. Handles act as abstract “references” to assets, whose data are stored in the [`Assets<A>`](struct.Assets.html "struct bevy::prelude::Assets") resource, avoiding the need to store multiple copies of the same data.

[HexColorError](enum.HexColorError.html "enum bevy::prelude::HexColorError")

Error returned if a hex string could not be parsed as a color.

[ImageFormat](enum.ImageFormat.html "enum bevy::prelude::ImageFormat")

The format of an on-disk image asset.

[Ime](enum.Ime.html "enum bevy::prelude::Ime")

An Input Method Editor event.

[InlineDirection](enum.InlineDirection.html "enum bevy::prelude::InlineDirection")

Sets the inline axis direction (LTR or RTL) used for layout.

[Interaction](enum.Interaction.html "enum bevy::prelude::Interaction")

Describes what type of input interaction has occurred for a UI node.

[InterpolationColorSpace](enum.InterpolationColorSpace.html "enum bevy::prelude::InterpolationColorSpace")

The color space used for interpolation.

[JumpAt](enum.JumpAt.html "enum bevy::prelude::JumpAt")

Configuration options for the [`EaseFunction::Steps`](enum.EaseFunction.html#variant.Steps "variant bevy::prelude::EaseFunction::Steps") curves. This closely replicates the [CSS step function specification](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function/steps#description).

[Justify](enum.Justify.html "enum bevy::prelude::Justify")

Describes the horizontal alignment of multiple lines of text relative to each other.

[JustifyContent](enum.JustifyContent.html "enum bevy::prelude::JustifyContent")

Used to control how items are distributed.

[JustifyItems](enum.JustifyItems.html "enum bevy::prelude::JustifyItems")

Used to control how each individual item is aligned by default within the space they’re given.

[JustifySelf](enum.JustifySelf.html "enum bevy::prelude::JustifySelf")

Used to control how the specified item is aligned within the space it’s given.

[KeyCode](enum.KeyCode.html "enum bevy::prelude::KeyCode")

The key code of a [`KeyboardInput`](../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput").

[LightGizmoColor](enum.LightGizmoColor.html "enum bevy::prelude::LightGizmoColor")

Configures how a color is attributed to a light gizmo.

[LineBreak](enum.LineBreak.html "enum bevy::prelude::LineBreak")

Determines how lines will be broken when preventing text from running out of bounds.

[LinearReparamError](enum.LinearReparamError.html "enum bevy::prelude::LinearReparamError")

An error indicating that a linear reparameterization couldn’t be performed because of malformed inputs.

[MaxTrackSizingFunction](enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction")

[MinTrackSizingFunction](enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction")

[MonitorSelection](enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection")

References a screen monitor.

[MouseButton](enum.MouseButton.html "enum bevy::prelude::MouseButton")

A button on a mouse device.

[Msaa](enum.Msaa.html "enum bevy::prelude::Msaa")

Component for configuring the number of samples for [Multi-Sample Anti-Aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing) for a [`Camera`](struct.Camera.html "struct bevy::prelude::Camera").

[MsaaWriteback](enum.MsaaWriteback.html "enum bevy::prelude::MsaaWriteback")

Controls when MSAA writeback occurs for a camera.

[NextState](enum.NextState.html "enum bevy::prelude::NextState")

The next state of [`State<S>`](struct.State.html "struct bevy::prelude::State").

[NodeImageMode](enum.NodeImageMode.html "enum bevy::prelude::NodeImageMode")

Controls how the image is altered to fit within the layout and how the layout algorithm determines the space in the layout for the image

[OverflowAxis](enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")

Whether to show or hide overflowing items

[ParallaxMappingMethod](enum.ParallaxMappingMethod.html "enum bevy::prelude::ParallaxMappingMethod")

The [parallax mapping](https://en.wikipedia.org/wiki/Parallax_mapping) method to use to compute depth based on the material’s [`depth_map`](struct.StandardMaterial.html#structfield.depth_map "field bevy::prelude::StandardMaterial::depth_map").

[PingPongError](enum.PingPongError.html "enum bevy::prelude::PingPongError")

An error indicating that a ping ponging of a curve couldn’t be performed because of malformed inputs.

[PointerButton](enum.PointerButton.html "enum bevy::prelude::PointerButton")

The button that was just pressed or released

[PositionType](enum.PositionType.html "enum bevy::prelude::PositionType")

The strategy used to position this node

[Projection](enum.Projection.html "enum bevy::prelude::Projection")

Component that defines how to compute a [`Camera`](struct.Camera.html "struct bevy::prelude::Camera")’s projection matrix.

[RadialGradientShape](enum.RadialGradientShape.html "enum bevy::prelude::RadialGradientShape")

[RayCastVisibility](enum.RayCastVisibility.html "enum bevy::prelude::RayCastVisibility")

How a ray cast should handle [`Visibility`](enum.Visibility.html "enum bevy::prelude::Visibility").

[RepeatError](enum.RepeatError.html "enum bevy::prelude::RepeatError")

An error indicating that a repetition of a curve couldn’t be performed because of malformed inputs.

[ResamplingError](enum.ResamplingError.html "enum bevy::prelude::ResamplingError")

An error indicating that a resampling operation could not be performed because of malformed inputs.

[ReverseError](enum.ReverseError.html "enum bevy::prelude::ReverseError")

An error indicating that a reversion of a curve couldn’t be performed because of malformed inputs.

[RunFixedMainLoopSystems](enum.RunFixedMainLoopSystems.html "enum bevy::prelude::RunFixedMainLoopSystems")

Set enum for the systems that want to run inside [`RunFixedMainLoop`](struct.RunFixedMainLoop.html "struct bevy::prelude::RunFixedMainLoop"), but before or after the fixed update logic. Systems in this set will run exactly once per frame, regardless of the number of fixed updates. They will also run under a variable timestep.

[SerializedAnimationNodeType](enum.SerializedAnimationNodeType.html "enum bevy::prelude::SerializedAnimationNodeType")

A version of [`AnimationNodeType`](enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType") suitable for serializing as part of a [`SerializedAnimationGraphNode`](struct.SerializedAnimationGraphNode.html "struct bevy::prelude::SerializedAnimationGraphNode") asset.

[Severity](enum.Severity.html "enum bevy::prelude::Severity")

Indicates how severe a [`BevyError`](struct.BevyError.html "struct bevy::prelude::BevyError") is.

[SliceScaleMode](enum.SliceScaleMode.html "enum bevy::prelude::SliceScaleMode")

Defines how a texture slice scales when resized

[SpriteImageMode](enum.SpriteImageMode.html "enum bevy::prelude::SpriteImageMode")

Controls how the image is altered when scaled.

[SpritePickingMode](enum.SpritePickingMode.html "enum bevy::prelude::SpritePickingMode")

How should the [`SpritePickingPlugin`](struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin") handle picking and how should it handle transparent pixels

[SpriteScalingMode](enum.SpriteScalingMode.html "enum bevy::prelude::SpriteScalingMode")

Represents various modes for proportional scaling of a texture.

[StaticTransformOptimizations](enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations")

Configure the behavior of static scene optimizations for [`Transform`](struct.Transform.html "struct bevy::prelude::Transform") propagation.

[TextError](enum.TextError.html "enum bevy::prelude::TextError")

Errors related to the textsystem

[TextureError](enum.TextureError.html "enum bevy::prelude::TextureError")

An error that occurs when loading a texture.

[TimerMode](enum.TimerMode.html "enum bevy::prelude::TimerMode")

Specifies [`Timer`](struct.Timer.html "struct bevy::prelude::Timer") behavior.

[TorusKind](enum.TorusKind.html "enum bevy::prelude::TorusKind")

The type of torus determined by the minor and major radii

[TransformGizmoAxis](enum.TransformGizmoAxis.html "enum bevy::prelude::TransformGizmoAxis")

Which axis the user is interacting with.

[TransformGizmoMode](enum.TransformGizmoMode.html "enum bevy::prelude::TransformGizmoMode")

Which manipulation mode the gizmo is in.

[TransformGizmoSpace](enum.TransformGizmoSpace.html "enum bevy::prelude::TransformGizmoSpace")

Whether the gizmo transforms the object using world or local space axes.

[TransformSystems](enum.TransformSystems.html "enum bevy::prelude::TransformSystems")

Set enum for the systems relating to transform propagation

[UiAntiAlias](enum.UiAntiAlias.html "enum bevy::prelude::UiAntiAlias")

Marker for controlling whether UI is rendered with or without anti-aliasing in a camera. By default, UI is always anti-aliased.

[UntypedHandle](enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

An untyped variant of [`Handle`](enum.Handle.html "enum bevy::prelude::Handle"), which internally stores the [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") type information at runtime as a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") instead of encoding it in the compile-time type. This allows handles across [`Asset`](trait.Asset.html "trait bevy::prelude::Asset") types to be stored together and compared.

[Val](enum.Val.html "enum bevy::prelude::Val")

Represents the possible value types for layout properties.

[ValArithmeticError](enum.ValArithmeticError.html "enum bevy::prelude::ValArithmeticError")

[ValParseError](enum.ValParseError.html "enum bevy::prelude::ValParseError")

[VideoModeSelection](enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection")

References an exclusive fullscreen video mode.

[Visibility](enum.Visibility.html "enum bevy::prelude::Visibility")

User indication of whether an entity is visible. Propagates down the entity hierarchy.

[VisualBox](enum.VisualBox.html "enum bevy::prelude::VisualBox")

Used to determine which region of a UI node is used for visual bounds.

[WindingOrder](enum.WindingOrder.html "enum bevy::prelude::WindingOrder")

The winding order for a set of points

[WindowPosition](enum.WindowPosition.html "enum bevy::prelude::WindowPosition")

Defines where a [`Window`](struct.Window.html "struct bevy::prelude::Window") should be placed on the screen.

[WorldFilter](enum.WorldFilter.html "enum bevy::prelude::WorldFilter")

A filter used to control which types can be added to a [`DynamicWorld`](struct.DynamicWorld.html "struct bevy::prelude::DynamicWorld").

## Traits

[Alpha](trait.Alpha.html "trait bevy::prelude::Alpha")

Methods for manipulating alpha values.

[Animatable](trait.Animatable.html "trait bevy::prelude::Animatable")

An animatable value type.

[AnimatableProperty](trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty")

A trait for exposing a value in an entity so that it can be animated.

[AnimationCompatibleCurve](trait.AnimationCompatibleCurve.html "trait bevy::prelude::AnimationCompatibleCurve")

This trait collects the additional requirements on top of [`Curve<T>`](trait.Curve.html "trait bevy::prelude::Curve") needed for a curve to be used as an [`AnimationCurve`](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve").

[AnimationCurve](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve")

A low-level trait that provides control over how curves are actually applied to entities by the animation system.

[AnimationCurveEvaluator](trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")

A low-level trait for use in [`VariableCurve`](struct.VariableCurve.html "struct bevy::prelude::VariableCurve") that provides fine control over how animations are evaluated.

[AppExtStates](trait.AppExtStates.html "trait bevy::prelude::AppExtStates")

State installation methods for [`App`](struct.App.html "struct bevy::prelude::App") and [`SubApp`](struct.SubApp.html "struct bevy::prelude::SubApp").

[AppGizmoBuilder](trait.AppGizmoBuilder.html "trait bevy::prelude::AppGizmoBuilder")

A extension trait adding `App::init_gizmo_group` and `App::insert_gizmo_config`.

[Asset](trait.Asset.html "trait bevy::prelude::Asset")

Declares that this type is an asset, which can be loaded and managed by the [`AssetServer`](struct.AssetServer.html "struct bevy::prelude::AssetServer") and stored in [`Assets`](struct.Assets.html "struct bevy::prelude::Assets") collections.

[AssetApp](trait.AssetApp.html "trait bevy::prelude::AssetApp")

Adds asset-related builder methods to [`App`](struct.App.html "struct bevy::prelude::App").

[AudioSinkPlayback](trait.AudioSinkPlayback.html "trait bevy::prelude::AudioSinkPlayback")

Common interactions with an audio sink.

[BuildChildrenTransformExt](trait.BuildChildrenTransformExt.html "trait bevy::prelude::BuildChildrenTransformExt")

Collection of methods similar to the built-in parenting methods on [`EntityWorldMut`](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut") and [`EntityCommands`](struct.EntityCommands.html "struct bevy::prelude::EntityCommands"), but preserving each entity’s [`GlobalTransform`](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").

[Bundle](trait.Bundle.html "trait bevy::prelude::Bundle")

The `Bundle` trait enables insertion and removal of [`Component`](trait.Component.html "trait bevy::prelude::Component")s from an entity.

[ColorToComponents](trait.ColorToComponents.html "trait bevy::prelude::ColorToComponents")

Trait with methods for converting colors to non-color types

[ColorToPacked](trait.ColorToPacked.html "trait bevy::prelude::ColorToPacked")

Trait with methods for converting colors to packed non-color types

[Command](trait.Command.html "trait bevy::prelude::Command")

A [`World`](struct.World.html "struct bevy::prelude::World") mutation.

[CommandsSceneExt](trait.CommandsSceneExt.html "trait bevy::prelude::CommandsSceneExt")

Adds scene spawning functionality to [`Commands`](struct.Commands.html "struct bevy::prelude::Commands").

[CommandsStatesExt](trait.CommandsStatesExt.html "trait bevy::prelude::CommandsStatesExt")

Extension trait for [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") adding `bevy_state` helpers.

[Component](trait.Component.html "trait bevy::prelude::Component")

A data type that can be used to store data for an [entity](../ecs/entity/index.html "mod bevy::ecs::entity").

[ComputedStates](trait.ComputedStates.html "trait bevy::prelude::ComputedStates")

A state whose value is automatically computed based on the values of other [`States`](trait.States.html "trait bevy::prelude::States").

[ContainsEntity](trait.ContainsEntity.html "trait bevy::prelude::ContainsEntity")

A trait for types that contain an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

[CubicGenerator](trait.CubicGenerator.html "trait bevy::prelude::CubicGenerator")`alloc`

Implement this on cubic splines that can generate a cubic curve from their spline parameters.

[Curve](trait.Curve.html "trait bevy::prelude::Curve")

A trait for a type that can represent values of type `T` parametrized over a fixed interval.

[CurveExt](trait.CurveExt.html "trait bevy::prelude::CurveExt")

Extension trait implemented by [curves](trait.Curve.html "trait bevy::prelude::Curve"), allowing access to a number of adaptors and convenience methods.

[CurveResampleExt](trait.CurveResampleExt.html "trait bevy::prelude::CurveResampleExt")`alloc`

Extension trait implemented by [curves](trait.Curve.html "trait bevy::prelude::Curve"), allowing access to generic resampling methods as well as those based on [stable interpolation](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate").

[CyclicCubicGenerator](trait.CyclicCubicGenerator.html "trait bevy::prelude::CyclicCubicGenerator")`alloc`

Implement this on cubic splines that can generate a cyclic cubic curve from their spline parameters.

[Decodable](trait.Decodable.html "trait bevy::prelude::Decodable")

A type implementing this trait can be converted to a [`rodio::Source`](../audio/trait.Source.html "trait bevy::audio::Source") type.

[DelayedCommandsExt](trait.DelayedCommandsExt.html "trait bevy::prelude::DelayedCommandsExt")

Extension trait for [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") that provides delayed command functionality.

[DetectChanges](trait.DetectChanges.html "trait bevy::prelude::DetectChanges")

Types that can read change detection information. This change detection is controlled by [`DetectChangesMut`](trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut") types such as [`ResMut`](struct.ResMut.html "struct bevy::prelude::ResMut").

[DetectChangesMut](trait.DetectChangesMut.html "trait bevy::prelude::DetectChangesMut")

Types that implement reliable change detection.

[DirectAssetAccessExt](trait.DirectAssetAccessExt.html "trait bevy::prelude::DirectAssetAccessExt")

An extension trait for methods for working with assets directly from a [`World`](struct.World.html "struct bevy::prelude::World").

[Ease](trait.Ease.html "trait bevy::prelude::Ease")

A type whose values can be eased between.

[EntityCommand](trait.EntityCommand.html "trait bevy::prelude::EntityCommand")

A command which gets executed for a given [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

[EntityCommandsSceneExt](trait.EntityCommandsSceneExt.html "trait bevy::prelude::EntityCommandsSceneExt")

Adds scene functionality to [`EntityWorldMut`](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

[EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent")

An [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") is an [`Event`](trait.Event.html "trait bevy::prelude::Event") that is triggered for a specific [`EntityEvent::event_target`](trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") entity:

[EntityMapper](trait.EntityMapper.html "trait bevy::prelude::EntityMapper")

An implementor of this trait knows how to map an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") into another [`Entity`](struct.Entity.html "struct bevy::prelude::Entity").

[EntityWorldMutSceneExt](trait.EntityWorldMutSceneExt.html "trait bevy::prelude::EntityWorldMutSceneExt")

Adds scene functionality to [`EntityWorldMut`](struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut").

[Event](trait.Event.html "trait bevy::prelude::Event")

An [`Event`](trait.Event.html "trait bevy::prelude::Event") is something that “happens” at a given moment.

[FloatExt](trait.FloatExt.html "trait bevy::prelude::FloatExt")

A trait for extending [`f32`](https://doc.rust-lang.org/nightly/std/primitive.f32.html "primitive f32") and [`f64`](https://doc.rust-lang.org/nightly/std/primitive.f64.html "primitive f64") with extra methods.

[FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect")

A trait that enables types to be dynamically constructed from reflected data.

[FromRng](trait.FromRng.html "trait bevy::prelude::FromRng")

Ergonomics trait for a type with a [`StandardUniform`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/distr/struct.StandardUniform.html "struct rand::distr::StandardUniform") distribution, allowing values to be generated uniformly from an [`RngExt`](https://docs.rs/rand/0.9.4/x86_64-unknown-linux-gnu/rand/rng/trait.RngExt.html "trait rand::rng::RngExt") by a method in its own namespace.

[FromTemplate](trait.FromTemplate.html "trait bevy::prelude::FromTemplate")

[`FromTemplate`](trait.FromTemplate.html "trait bevy::prelude::FromTemplate") is implemented for types that can be produced by a specific, canonical [`Template`](trait.Template.html "trait bevy::prelude::Template"). This creates a way to correlate to the [`Template`](trait.Template.html "trait bevy::prelude::Template") using the desired template output type. This is used by Bevy’s scene system.

[FromWorld](trait.FromWorld.html "trait bevy::prelude::FromWorld")

Creates an instance of the type this trait is implemented for using data from the supplied [`World`](struct.World.html "struct bevy::prelude::World").

[Function](trait.Function.html "trait bevy::prelude::Function")

A trait used to power [function-like](../reflect/func/index.html "mod bevy::reflect::func") operations via [reflection](trait.Reflect.html "trait bevy::prelude::Reflect").

[GetField](trait.GetField.html "trait bevy::prelude::GetField")

A convenience trait which combines fetching and downcasting of struct fields.

[GetPath](trait.GetPath.html "trait bevy::prelude::GetPath")

A trait which allows nested [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") values to be retrieved with path strings.

[GetTupleStructField](trait.GetTupleStructField.html "trait bevy::prelude::GetTupleStructField")

A convenience trait which combines fetching and downcasting of tuple struct fields.

[GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup")

A trait used to create gizmo configs groups.

[GizmoPrimitive2d](trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")

A trait for rendering 2D geometric primitives (`P`) with [`GizmoBuffer`](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer").

[GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")

A trait for rendering 3D geometric primitives (`P`) with [`GizmoBuffer`](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer").

[Gray](trait.Gray.html "trait bevy::prelude::Gray")

Trait for returning a grayscale color of a provided lightness.

[Hue](trait.Hue.html "trait bevy::prelude::Hue")

Trait for manipulating the hue of a color.

[InColorSpace](trait.InColorSpace.html "trait bevy::prelude::InColorSpace")

Set the color space used for interpolation.

[Inset](trait.Inset.html "trait bevy::prelude::Inset")

A primitive that can be resized uniformly.

[IntoFunction](trait.IntoFunction.html "trait bevy::prelude::IntoFunction")

A trait for types that can be converted into a [`DynamicFunction`](../reflect/func/struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction").

[IntoFunctionMut](trait.IntoFunctionMut.html "trait bevy::prelude::IntoFunctionMut")

A trait for types that can be converted into a [`DynamicFunctionMut`](../reflect/func/struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

[IntoScheduleConfigs](trait.IntoScheduleConfigs.html "trait bevy::prelude::IntoScheduleConfigs")

Types that can convert into a [`ScheduleConfigs`](../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs").

[IntoSystem](trait.IntoSystem.html "trait bevy::prelude::IntoSystem")

Conversion trait to turn something into a [`System`](trait.System.html "trait bevy::prelude::System").

[IntoSystemSet](trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")

Types that can be converted into a [`SystemSet`](trait.SystemSet.html "trait bevy::prelude::SystemSet").

[Luminance](trait.Luminance.html "trait bevy::prelude::Luminance")

Methods for changing the luminance of a color. Note that these methods are not guaranteed to produce consistent results across color spaces, but will be within a given space.

[Material](trait.Material.html "trait bevy::prelude::Material")

Materials are used alongside [`MaterialPlugin`](struct.MaterialPlugin.html "struct bevy::prelude::MaterialPlugin"), [`Mesh3d`](struct.Mesh3d.html "struct bevy::prelude::Mesh3d"), and [`MeshMaterial3d`](struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d") to spawn entities that are rendered with a specific [`Material`](trait.Material.html "trait bevy::prelude::Material") type. They serve as an easy to use high level way to render [`Mesh3d`](struct.Mesh3d.html "struct bevy::prelude::Mesh3d") entities with custom shader logic.

[Measured2d](trait.Measured2d.html "trait bevy::prelude::Measured2d")

A trait for getting measurements of 2D shapes

[Measured3d](trait.Measured3d.html "trait bevy::prelude::Measured3d")

A trait for getting measurements of 3D shapes

[MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder")

A trait used to build [`Mesh`](struct.Mesh.html "struct bevy::prelude::Mesh")es from a configuration

[Meshable](trait.Meshable.html "trait bevy::prelude::Meshable")

A trait for shapes that can be turned into a [`Mesh`](struct.Mesh.html "struct bevy::prelude::Mesh").

[Message](trait.Message.html "trait bevy::prelude::Message")

A buffered message for pull-based event handling.

[Mix](trait.Mix.html "trait bevy::prelude::Mix")

Linear interpolation of two colors within a given color space.

[ObserverSystemExt](trait.ObserverSystemExt.html "trait bevy::prelude::ObserverSystemExt")

Extension trait for adding run conditions to observer systems.

[PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect")

The foundational trait of [`bevy_reflect`](../reflect/index.html "mod bevy::reflect"), used for accessing and modifying data dynamically.

[PatchFromTemplate](trait.PatchFromTemplate.html "trait bevy::prelude::PatchFromTemplate")

A helper function that returns a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`FromTemplate`](trait.FromTemplate.html "trait bevy::prelude::FromTemplate"). It will use [`FromTemplate::Template`](trait.FromTemplate.html#associatedtype.Template "associated type bevy::prelude::FromTemplate::Template") as the “patched template”.

[PatchTemplate](trait.PatchTemplate.html "trait bevy::prelude::PatchTemplate")

A helper function that returns a [`TemplatePatch`](../scene/struct.TemplatePatch.html "struct bevy::scene::TemplatePatch") [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") for something that implements [`Template`](trait.Template.html "trait bevy::prelude::Template").

[Plugin](trait.Plugin.html "trait bevy::prelude::Plugin")

A collection of Bevy app logic and configuration.

[PluginGroup](trait.PluginGroup.html "trait bevy::prelude::PluginGroup")

Combines multiple [`Plugin`](trait.Plugin.html "trait bevy::prelude::Plugin")s into a single unit.

[Primitive2d](trait.Primitive2d.html "trait bevy::prelude::Primitive2d")

A marker trait for 2D primitives

[Primitive3d](trait.Primitive3d.html "trait bevy::prelude::Primitive3d")

A marker trait for 3D primitives

[RationalGenerator](trait.RationalGenerator.html "trait bevy::prelude::RationalGenerator")`alloc`

Implement this on cubic splines that can generate a rational cubic curve from their spline parameters.

[ReadOnlySystem](trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem")

[`System`](trait.System.html "trait bevy::prelude::System") types that do not modify the [`World`](struct.World.html "struct bevy::prelude::World") when run. This is implemented for any systems whose parameters all implement [`ReadOnlySystemParam`](../ecs/system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam").

[Reflect](trait.Reflect.html "trait bevy::prelude::Reflect")

A core trait of [`bevy_reflect`](../reflect/index.html "mod bevy::reflect"), used for downcasting to concrete types.

[ReflectPath](trait.ReflectPath.html "trait bevy::prelude::ReflectPath")

Something that can be interpreted as a reflection path in [`GetPath`](trait.GetPath.html "trait bevy::prelude::GetPath").

[RelationshipTarget](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget")

A [`Component`](trait.Component.html "trait bevy::prelude::Component") containing the collection of entities that relate to this [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") via the associated `Relationship` type. See the [`Relationship`](../ecs/relationship/trait.Relationship.html "trait bevy::ecs::relationship::Relationship") documentation for more information.

[Resource](trait.Resource.html "trait bevy::prelude::Resource")

A type that can be inserted into a [`World`](struct.World.html "struct bevy::prelude::World") as a singleton.

[ResultSeverityExt](trait.ResultSeverityExt.html "trait bevy::prelude::ResultSeverityExt")

Extension methods for annotating errors with a [`Severity`](enum.Severity.html "enum bevy::prelude::Severity").

[Saturation](trait.Saturation.html "trait bevy::prelude::Saturation")

Trait for manipulating the saturation of a color.

[Scene](trait.Scene.html "trait bevy::prelude::Scene")

Conceptually, a [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") describes what a spawned [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") should look like. This often describes what [`Component`](trait.Component.html "trait bevy::prelude::Component")s the entity should have.

[SceneComponent](trait.SceneComponent.html "trait bevy::prelude::SceneComponent")

Implemented for [`Component`](trait.Component.html "trait bevy::prelude::Component")s that have an associated [`Scene`](trait.Scene.html "trait bevy::prelude::Scene"), which can be constructed with [`Self::Props`](trait.SceneComponent.html#associatedtype.Props "associated type bevy::prelude::SceneComponent::Props").

[SceneList](trait.SceneList.html "trait bevy::prelude::SceneList")

This behaves like a list of [`Scene`](trait.Scene.html "trait bevy::prelude::Scene"), where each entry in the list is a new entity (see [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") for more details).

[ShapeSample](trait.ShapeSample.html "trait bevy::prelude::ShapeSample")

Exposes methods to uniformly sample a variety of primitive shapes.

[SpawnListSystem](trait.SpawnListSystem.html "trait bevy::prelude::SpawnListSystem")

Returns a system that spawns the given [`SceneList`](trait.SceneList.html "trait bevy::prelude::SceneList"). This should generally only be added to schedules that run once, such as [`Startup`](struct.Startup.html "struct bevy::prelude::Startup").

[SpawnRelated](trait.SpawnRelated.html "trait bevy::prelude::SpawnRelated")

[`RelationshipTarget`](trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") methods that create a [`Bundle`](trait.Bundle.html "trait bevy::prelude::Bundle") with a [`DynamicBundle::Effect`](../ecs/bundle/trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect") that:

[SpawnSystem](trait.SpawnSystem.html "trait bevy::prelude::SpawnSystem")

Returns a system that spawns the given [`Scene`](trait.Scene.html "trait bevy::prelude::Scene"). This should generally only be added to schedules that run once, such as [`Startup`](struct.Startup.html "struct bevy::prelude::Startup").

[StableInterpolate](trait.StableInterpolate.html "trait bevy::prelude::StableInterpolate")

A type with a natural interpolation that provides strong subdivision guarantees.

[StateScopedMessagesAppExt](trait.StateScopedMessagesAppExt.html "trait bevy::prelude::StateScopedMessagesAppExt")

Extension trait for [`App`](struct.App.html "struct bevy::prelude::App") adding methods for registering state scoped messages.

[StateSet](trait.StateSet.html "trait bevy::prelude::StateSet")

A [`States`](trait.States.html "trait bevy::prelude::States") type or tuple of types which implement [`States`](trait.States.html "trait bevy::prelude::States").

[States](trait.States.html "trait bevy::prelude::States")

Types that can define world-wide states in a finite-state machine.

[Struct](trait.Struct.html "trait bevy::prelude::Struct")

A trait used to power [struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) operations via [reflection](../reflect/index.html "mod bevy::reflect").

[SubStates](trait.SubStates.html "trait bevy::prelude::SubStates")

A sub-state is a state that exists only when the source state meet certain conditions, but unlike [`ComputedStates`](trait.ComputedStates.html "trait bevy::prelude::ComputedStates") - while they exist they can be manually modified.

[System](trait.System.html "trait bevy::prelude::System")

An ECS system that can be added to a [`Schedule`](struct.Schedule.html "struct bevy::prelude::Schedule")

[SystemCondition](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")

A system that determines if one or more scheduled systems should run.

[SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput")

Trait for types that can be used as input to [`System`](trait.System.html "trait bevy::prelude::System")s.

[SystemParamBuilder](trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")

A builder that can create a [`SystemParam`](../ecs/system/trait.SystemParam.html "trait bevy::ecs::system::SystemParam").

[SystemParamFunction](trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")

A trait implemented for all functions that can be used as [`System`](trait.System.html "trait bevy::prelude::System")s.

[SystemSet](trait.SystemSet.html "trait bevy::prelude::SystemSet")

System sets are tag-like labels that can be used to group systems together.

[Template](trait.Template.html "trait bevy::prelude::Template")

A [`Template`](trait.Template.html "trait bevy::prelude::Template") is something that, given a spawn context (target [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"), [`World`](struct.World.html "struct bevy::prelude::World"), etc), can produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[ToOwned](trait.ToOwned.html "trait bevy::prelude::ToOwned")

A generalization of `Clone` to borrowed data.

[ToRing](trait.ToRing.html "trait bevy::prelude::ToRing")

Provides a convenience method for converting a primitive to a [`Ring`](struct.Ring.html "struct bevy::prelude::Ring"), with a given thickness.

[ToString](trait.ToString.html "trait bevy::prelude::ToString")

A trait for converting a value to a `String`.

[TransformPoint](trait.TransformPoint.html "trait bevy::prelude::TransformPoint")

A trait for point transformation methods.

[TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct")

A trait used to power [tuple struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) operations via [reflection](../reflect/index.html "mod bevy::reflect").

[TypePath](trait.TypePath.html "trait bevy::prelude::TypePath")

A static accessor to type paths and names.

[UiMaterial](trait.UiMaterial.html "trait bevy::prelude::UiMaterial")

Materials are used alongside [`UiMaterialPlugin`](struct.UiMaterialPlugin.html "struct bevy::prelude::UiMaterialPlugin") and [`MaterialNode`](struct.MaterialNode.html "struct bevy::prelude::MaterialNode") to spawn entities that are rendered with a specific [`UiMaterial`](trait.UiMaterial.html "trait bevy::prelude::UiMaterial") type. They serve as an easy to use high level way to render `Node` entities with custom shader logic.

[ValNum](trait.ValNum.html "trait bevy::prelude::ValNum")

All the types that should be able to be used in the [`Val`](enum.Val.html "enum bevy::prelude::Val") enum should implement this trait.

[Vec2Swizzles](trait.Vec2Swizzles.html "trait bevy::prelude::Vec2Swizzles")

[Vec3Swizzles](trait.Vec3Swizzles.html "trait bevy::prelude::Vec3Swizzles")

[Vec4Swizzles](trait.Vec4Swizzles.html "trait bevy::prelude::Vec4Swizzles")

[WorldSceneExt](trait.WorldSceneExt.html "trait bevy::prelude::WorldSceneExt")

Adds scene spawning functionality to [`World`](struct.World.html "struct bevy::prelude::World").

[\_](trait._.html "trait bevy::prelude::_")

## Functions

[advance\_transitions](fn.advance_transitions.html "fn bevy::prelude::advance_transitions")

A system that alters the weight of currently-playing transitions based on the current time and decline amount.

[any\_component\_removed](fn.any_component_removed.html "fn bevy::prelude::any_component_removed")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entity with a component of the given type removed.

[any\_match\_filter](fn.any_match_filter.html "fn bevy::prelude::any_match_filter")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities that match the given [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

[any\_with\_component](fn.any_with_component.html "fn bevy::prelude::any_with_component")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities with the given component type.

[asset\_value](fn.asset_value.html "fn bevy::prelude::asset_value")

This will create a new [`HandleTemplate`](../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate") for the given `asset` value. This makes it possible to define assets “inline” in templates / scenes that produce a [`Handle`](enum.Handle.html "enum bevy::prelude::Handle").

[auto](fn.auto.html "fn bevy::prelude::auto")

Returns a [`Val::Auto`](enum.Val.html#variant.Auto "variant bevy::prelude::Val::Auto") where the value is automatically determined based on the context and other [`Node`](struct.Node.html "struct bevy::prelude::Node") properties.

[bvec2](fn.bvec2.html "fn bevy::prelude::bvec2")

Creates a 2-dimensional `bool` vector mask.

[bvec3](fn.bvec3.html "fn bevy::prelude::bvec3")

Creates a 3-dimensional `bool` vector mask.

[bvec4](fn.bvec4.html "fn bevy::prelude::bvec4")

Creates a 4-dimensional `bool` vector mask.

[bvec3a](fn.bvec3a.html "fn bevy::prelude::bvec3a")

Creates a 3-dimensional `bool` vector mask.

[bvec4a](fn.bvec4a.html "fn bevy::prelude::bvec4a")

Creates a 4-dimensional `bool` vector mask.

[condition\_changed](fn.condition_changed.html "fn bevy::prelude::condition_changed")

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the passed one changes.

[condition\_changed\_to](fn.condition_changed_to.html "fn bevy::prelude::condition_changed_to")

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the result of the passed one went from false to true since the last time this was called.

[default](fn.default.html "fn bevy::prelude::default")

An ergonomic abbreviation for [`Default::default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default") to make initializing structs easier.

[expire\_completed\_transitions](fn.expire_completed_transitions.html "fn bevy::prelude::expire_completed_transitions")

A system that removed transitions that have completed from the [`AnimationTransitions`](struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions") object.

[gizmo](fn.gizmo.html "fn bevy::prelude::gizmo")

A global gizmo context for use outside of bevy systems.

[in\_state](fn.in_state.html "fn bevy::prelude::in_state")

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the state machine is currently in `state`.

[interpolate\_with\_cubic\_bezier](fn.interpolate_with_cubic_bezier.html "fn bevy::prelude::interpolate_with_cubic_bezier")

Evaluates a cubic Bézier curve at a value `t`, given two endpoints and the derivatives at those endpoints.

[interval](fn.interval.html "fn bevy::prelude::interval")

Create an [`Interval`](struct.Interval.html "struct bevy::prelude::Interval") with a given `start` and `end`. Alias of [`Interval::new`](struct.Interval.html#method.new "associated function bevy::prelude::Interval::new").

[ivec2](fn.ivec2.html "fn bevy::prelude::ivec2")

Creates a 2-dimensional vector.

[ivec3](fn.ivec3.html "fn bevy::prelude::ivec3")

Creates a 3-dimensional vector.

[ivec4](fn.ivec4.html "fn bevy::prelude::ivec4")

Creates a 4-dimensional vector.

[last\_transition](fn.last_transition.html "fn bevy::prelude::last_transition")

Returns the latest state transition event of type `S`, if any are available.

[mat2](fn.mat2.html "fn bevy::prelude::mat2")

Creates a 2x2 matrix from two column vectors.

[mat3](fn.mat3.html "fn bevy::prelude::mat3")

Creates a 3x3 matrix from three column vectors.

[mat4](fn.mat4.html "fn bevy::prelude::mat4")

Creates a 4x4 matrix from four column vectors.

[mat3a](fn.mat3a.html "fn bevy::prelude::mat3a")

Creates a 3x3 matrix from three column vectors.

[not](fn.not.html "fn bevy::prelude::not")

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that inverses the result of passed one.

[on](fn.on.html "fn bevy::prelude::on")

Returns an [`OnTemplate`](../scene/struct.OnTemplate.html "struct bevy::scene::OnTemplate") that will create an [`Observer`](struct.Observer.html "struct bevy::prelude::Observer") of a given [`EntityEvent`](trait.EntityEvent.html "trait bevy::prelude::EntityEvent") on the current [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") entity.

[on\_message](fn.on_message.html "fn bevy::prelude::on_message")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any new messages of the given type since it was last called.

[percent](fn.percent.html "fn bevy::prelude::percent")

Returns a [`Val::Percent`](enum.Val.html#variant.Percent "variant bevy::prelude::Val::Percent") representing a percentage of the parent node’s length along a specific axis.

[pointer\_events](fn.pointer_events.html "fn bevy::prelude::pointer_events")

Dispatches interaction events to the target entities.

[px](fn.px.html "fn bevy::prelude::px")

Returns a [`Val::Px`](enum.Val.html#variant.Px "variant bevy::prelude::Val::Px") representing a value in logical pixels.

[quat](fn.quat.html "fn bevy::prelude::quat")

Creates a quaternion from `x`, `y`, `z` and `w` values.

[resource\_added](fn.resource_added.html "fn bevy::prelude::resource_added")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added since the condition was last checked.

[resource\_changed](fn.resource_changed.html "fn bevy::prelude::resource_changed")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added or mutably dereferenced since the condition was last checked.

[resource\_changed\_or\_removed](fn.resource_changed_or_removed.html "fn bevy::prelude::resource_changed_or_removed")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added, removed or mutably dereferenced since the condition was last checked.

[resource\_equals](fn.resource_equals.html "fn bevy::prelude::resource_equals")

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the resource is equal to `value`.

[resource\_exists](fn.resource_exists.html "fn bevy::prelude::resource_exists")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource exists.

[resource\_exists\_and\_changed](fn.resource_exists_and_changed.html "fn bevy::prelude::resource_exists_and_changed")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added or mutably dereferenced since the condition was last checked.

[resource\_exists\_and\_equals](fn.resource_exists_and_equals.html "fn bevy::prelude::resource_exists_and_equals")

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the resource exists and is equal to `value`.

[resource\_removed](fn.resource_removed.html "fn bevy::prelude::resource_removed")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been removed since the condition was last checked.

[run\_once](fn.run_once.html "fn bevy::prelude::run_once")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` on the first time the condition is run and false every time after.

[state\_changed](fn.state_changed.html "fn bevy::prelude::state_changed")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine changed state.

[state\_exists](fn.state_exists.html "fn bevy::prelude::state_exists")

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine exists.

[template](fn.template.html "fn bevy::prelude::template")

Returns a “free floating” template for a given `func`. This prevents the need to define a custom type for one-off templates.

[template\_value](fn.template_value.html "fn bevy::prelude::template_value")

Returns a [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") that completely overwrites the current value of a [`Template`](trait.Template.html "trait bevy::prelude::Template") `T` with the given `value`. The `value` is cloned each time the [`Template`](trait.Template.html "trait bevy::prelude::Template") is built.

[uvec2](fn.uvec2.html "fn bevy::prelude::uvec2")

Creates a 2-dimensional vector.

[uvec3](fn.uvec3.html "fn bevy::prelude::uvec3")

Creates a 3-dimensional vector.

[uvec4](fn.uvec4.html "fn bevy::prelude::uvec4")

Creates a 4-dimensional vector.

[vec2](fn.vec2.html "fn bevy::prelude::vec2")

Creates a 2-dimensional vector.

[vec3](fn.vec3.html "fn bevy::prelude::vec3")

Creates a 3-dimensional vector.

[vec4](fn.vec4.html "fn bevy::prelude::vec4")

Creates a 4-dimensional vector.

[vec3a](fn.vec3a.html "fn bevy::prelude::vec3a")

Creates a 3-dimensional vector.

[vh](fn.vh.html "fn bevy::prelude::vh")

Returns a [`Val::Vh`](enum.Val.html#variant.Vh "variant bevy::prelude::Val::Vh") representing a percentage of the viewport height.

[vmax](fn.vmax.html "fn bevy::prelude::vmax")

Returns a [`Val::VMax`](enum.Val.html#variant.VMax "variant bevy::prelude::Val::VMax") representing a percentage of the viewport’s larger dimension.

[vmin](fn.vmin.html "fn bevy::prelude::vmin")

Returns a [`Val::VMin`](enum.Val.html#variant.VMin "variant bevy::prelude::Val::VMin") representing a percentage of the viewport’s smaller dimension.

[vw](fn.vw.html "fn bevy::prelude::vw")

Returns a [`Val::Vw`](enum.Val.html#variant.Vw "variant bevy::prelude::Val::Vw") representing a percentage of the viewport width.

## Type Aliases

[AnimationDiGraph](type.AnimationDiGraph.html "type bevy::prelude::AnimationDiGraph")

A type alias for the `petgraph` data structure that defines the animation graph.

[AnimationMask](type.AnimationMask.html "type bevy::prelude::AnimationMask")

The type of an animation mask bitfield.

[AnimationNodeIndex](type.AnimationNodeIndex.html "type bevy::prelude::AnimationNodeIndex")

The index of either an animation or blend node in the animation graph.

[ChildSpawner](type.ChildSpawner.html "type bevy::prelude::ChildSpawner")

A type alias over [`RelatedSpawner`](../ecs/relationship/struct.RelatedSpawner.html "struct bevy::ecs::relationship::RelatedSpawner") used to spawn child entities containing a [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.

[ChildSpawnerCommands](type.ChildSpawnerCommands.html "type bevy::prelude::ChildSpawnerCommands")

A type alias over [`RelatedSpawnerCommands`](../ecs/relationship/struct.RelatedSpawnerCommands.html "struct bevy::ecs::relationship::RelatedSpawnerCommands") used to spawn child entities containing a [`ChildOf`](struct.ChildOf.html "struct bevy::prelude::ChildOf") relationship.

[Result](type.Result.html "type bevy::prelude::Result")

A result type for use in fallible systems, commands and observers.

[SystemIn](type.SystemIn.html "type bevy::prelude::SystemIn")

Shorthand way to get the [`System::In`](trait.System.html#associatedtype.In "associated type bevy::prelude::System::In") for a [`System`](trait.System.html "trait bevy::prelude::System") as a [`SystemInput::Inner`](trait.SystemInput.html#associatedtype.Inner "associated type bevy::prelude::SystemInput::Inner").

[Text2dReader](type.Text2dReader.html "type bevy::prelude::Text2dReader")

2d alias for [`TextReader`](../text/struct.TextReader.html "struct bevy::text::TextReader").

[Text2dWriter](type.Text2dWriter.html "type bevy::prelude::Text2dWriter")

2d alias for [`TextWriter`](../text/struct.TextWriter.html "struct bevy::text::TextWriter").

[TextUiReader](type.TextUiReader.html "type bevy::prelude::TextUiReader")

UI alias for [`TextReader`](../text/struct.TextReader.html "struct bevy::text::TextReader").

[TextUiWriter](type.TextUiWriter.html "type bevy::prelude::TextUiWriter")

UI alias for [`TextWriter`](../text/struct.TextWriter.html "struct bevy::text::TextWriter").

## Attribute Macros

[bevy\_main](attr.bevy_main.html "attr bevy::prelude::bevy_main")

Generates the required main function boilerplate for Android.

[reflect\_trait](attr.reflect_trait.html "attr bevy::prelude::reflect_trait")

A macro that automatically generates type data for traits, which their implementors can then register.

## Derive Macros

[Asset](derive.Asset.html "derive bevy::prelude::Asset")

Implement the `Asset` trait.

[Bundle](derive.Bundle.html "derive bevy::prelude::Bundle")

Implement the `Bundle` trait.

[Component](derive.Component.html "derive bevy::prelude::Component")

Cheat sheet for derive syntax, see full explanation and examples on the `Component` trait doc.

[Deref](derive.Deref.html "derive bevy::prelude::Deref")

Implements [`Deref`](std::ops::Deref) for structs. This is especially useful when utilizing the [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern.

[DerefMut](derive.DerefMut.html "derive bevy::prelude::DerefMut")

Implements [`DerefMut`](std::ops::DerefMut) for structs. This is especially useful when utilizing the [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) pattern.

[EntityEvent](derive.EntityEvent.html "derive bevy::prelude::EntityEvent")

Cheat sheet for derive syntax, see full explanation on `EntityEvent` trait docs.

[Event](derive.Event.html "derive bevy::prelude::Event")

Implement the `Event` trait.

[FromReflect](derive.FromReflect.html "derive bevy::prelude::FromReflect")

Derives the `FromReflect` trait.

[FromTemplate](derive.FromTemplate.html "derive bevy::prelude::FromTemplate")

Derives `FromTemplate`.

[FromWorld](derive.FromWorld.html "derive bevy::prelude::FromWorld")

Implement the `FromWorld` trait.

[GizmoConfigGroup](derive.GizmoConfigGroup.html "derive bevy::prelude::GizmoConfigGroup")

Implements the [`GizmoConfigGroup`](derive.GizmoConfigGroup.html "derive bevy::prelude::GizmoConfigGroup") trait for a gizmo config group type.

[Message](derive.Message.html "derive bevy::prelude::Message")

Implement the `Message` trait.

[Reflect](derive.Reflect.html "derive bevy::prelude::Reflect")

The main derive macro used by `bevy_reflect` for deriving its `Reflect` trait.

[Resource](derive.Resource.html "derive bevy::prelude::Resource")

Implement the `Resource` trait.

[SceneComponent](derive.SceneComponent.html "derive bevy::prelude::SceneComponent")

[States](derive.States.html "derive bevy::prelude::States")

Implements the `States` trait for a type - see the trait docs for an example usage.

[SubStates](derive.SubStates.html "derive bevy::prelude::SubStates")

Implements the `SubStates` trait for a type - see the trait docs for an example usage.

[SystemSet](derive.SystemSet.html "derive bevy::prelude::SystemSet")

Derive macro generating an impl of the trait `SystemSet`.

[TypePath](derive.TypePath.html "derive bevy::prelude::TypePath")

Derives the `TypePath` trait, providing a stable alternative to \[`std::any::type_name`\].