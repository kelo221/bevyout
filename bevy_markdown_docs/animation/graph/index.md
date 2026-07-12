[bevy](../../index.html)::[animation](../index.html)

# Module graph 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#15)

The animation graph, which allows animations to be blended together.

## Structs

[AnimationGraph](struct.AnimationGraph.html "struct bevy::animation::graph::AnimationGraph")

A graph structure that describes how animation clips are to be blended together.

[AnimationGraphAssetLoader](struct.AnimationGraphAssetLoader.html "struct bevy::animation::graph::AnimationGraphAssetLoader")

An [`AssetLoader`](../../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") that can load [`AnimationGraph`](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")s as assets.

[AnimationGraphHandle](struct.AnimationGraphHandle.html "struct bevy::animation::graph::AnimationGraphHandle")

A [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") to the [`AnimationGraph`](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph") to be used by the [`AnimationPlayer`](../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") on the same entity.

[AnimationGraphHandleTemplate](struct.AnimationGraphHandleTemplate.html "struct bevy::animation::graph::AnimationGraphHandleTemplate")

[AnimationGraphNode](struct.AnimationGraphNode.html "struct bevy::animation::graph::AnimationGraphNode")

An individual node within an animation graph.

[NonPathHandleError](struct.NonPathHandleError.html "struct bevy::animation::graph::NonPathHandleError")

Error for when only path [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")s are supported.

[SerializedAnimationGraph](struct.SerializedAnimationGraph.html "struct bevy::animation::graph::SerializedAnimationGraph")

A version of [`AnimationGraph`](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph") suitable for serializing as an asset.

[SerializedAnimationGraphNode](struct.SerializedAnimationGraphNode.html "struct bevy::animation::graph::SerializedAnimationGraphNode")

A version of [`AnimationGraphNode`](../../prelude/struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode") suitable for serializing as an asset.

[ThreadedAnimationGraph](struct.ThreadedAnimationGraph.html "struct bevy::animation::graph::ThreadedAnimationGraph")

An acceleration structure for an animation graph that allows Bevy to evaluate it quickly.

[ThreadedAnimationGraphs](struct.ThreadedAnimationGraphs.html "struct bevy::animation::graph::ThreadedAnimationGraphs")

Acceleration structures for animation graphs that allows Bevy to evaluate them quickly.

## Enums

[AnimationGraphLoadError](enum.AnimationGraphLoadError.html "enum bevy::animation::graph::AnimationGraphLoadError")

Errors that can occur when deserializing animation graphs from RON.

[AnimationGraphSaveError](enum.AnimationGraphSaveError.html "enum bevy::animation::graph::AnimationGraphSaveError")

Errors that can occur when serializing animation graphs to RON.

[AnimationNodeType](enum.AnimationNodeType.html "enum bevy::animation::graph::AnimationNodeType")

Animation node data specific to the type of node (clip, blend, or add).

[SerializedAnimationNodeType](enum.SerializedAnimationNodeType.html "enum bevy::animation::graph::SerializedAnimationNodeType")

A version of [`AnimationNodeType`](../../prelude/enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType") suitable for serializing as part of a [`SerializedAnimationGraphNode`](../../prelude/struct.SerializedAnimationGraphNode.html "struct bevy::prelude::SerializedAnimationGraphNode") asset.

## Type Aliases

[AnimationDiGraph](type.AnimationDiGraph.html "type bevy::animation::graph::AnimationDiGraph")

A type alias for the `petgraph` data structure that defines the animation graph.

[AnimationMask](type.AnimationMask.html "type bevy::animation::graph::AnimationMask")

The type of an animation mask bitfield.

[AnimationNodeIndex](type.AnimationNodeIndex.html "type bevy::animation::graph::AnimationNodeIndex")

The index of either an animation or blend node in the animation graph.