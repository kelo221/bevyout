[bevy](../index.html)

# Crate transform 

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/lib.rs.html#1-57)

## Bevy Transform

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license) [![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy_transform) [![Downloads](https://img.shields.io/crates/d/bevy_transform.svg)](https://crates.io/crates/bevy_transform) [![Docs](https://docs.rs/bevy_transform/badge.svg)](https://docs.rs/bevy_transform/latest/bevy_transform/) [![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/bevy)

This crate contains types and functions associated with the `Transform` component.

## Modules

[commands](commands/index.html "mod bevy::transform::commands")`bevy-support`

Extension to [`EntityCommands`](../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands") to modify [`bevy_ecs::hierarchy`](../ecs/hierarchy/index.html "mod bevy::ecs::hierarchy") hierarchies. while preserving [`GlobalTransform`](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform").

[components](components/index.html "mod bevy::transform::components")

The basic components of the transform crate

[helper](helper/index.html "mod bevy::transform::helper")`bevy-support`

Helpers related to computing global transforms System parameter for computing up-to-date [`GlobalTransform`](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")s.

[plugins](plugins/index.html "mod bevy::transform::plugins")`bevy-support`

Transform related plugins

[systems](systems/index.html "mod bevy::transform::systems")`bevy-support`

Systems responsible for transform propagation

[traits](traits/index.html "mod bevy::transform::traits")

Transform related traits

## Structs

[TransformPlugin](struct.TransformPlugin.html "struct bevy::transform::TransformPlugin")

The base plugin for handling [`Transform`](../prelude/struct.Transform.html "struct bevy::prelude::Transform") components

## Enums

[StaticTransformOptimizations](enum.StaticTransformOptimizations.html "enum bevy::transform::StaticTransformOptimizations")

Configure the behavior of static scene optimizations for [`Transform`](../prelude/struct.Transform.html "struct bevy::prelude::Transform") propagation.

[TransformSystems](enum.TransformSystems.html "enum bevy::transform::TransformSystems")

Set enum for the systems relating to transform propagation

## Traits

[TransformPoint](trait.TransformPoint.html "trait bevy::transform::TransformPoint")

A trait for point transformation methods.