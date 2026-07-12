[bevy](../../index.html)::[ecs](../index.html)

# Module bundle 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#31)

Types for handling [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle")s.

This module contains the [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") trait and some other helper types.

## Structs

[BundleId](struct.BundleId.html "struct bevy::ecs::bundle::BundleId")

For a specific [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), this stores a unique value identifying a type of a registered [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").

[BundleInfo](struct.BundleInfo.html "struct bevy::ecs::bundle::BundleInfo")

Stores metadata associated with a specific type of [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") for a given [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[BundleScratch](struct.BundleScratch.html "struct bevy::ecs::bundle::BundleScratch")

Enables pushing components to internal scratch space (uses a bump allocator), which can then be written as a dynamic bundle. The contents are cleared after each write and the allocated scratch space is reused across writes.

[BundleWriter](struct.BundleWriter.html "struct bevy::ecs::bundle::BundleWriter")

Enables pushing components to the internal [`BundleScratch`](struct.BundleScratch.html "struct bevy::ecs::bundle::BundleScratch"), which can then be written as a dynamic bundle.

[Bundles](struct.Bundles.html "struct bevy::ecs::bundle::Bundles")

Metadata for bundles. Stores a [`BundleInfo`](struct.BundleInfo.html "struct bevy::ecs::bundle::BundleInfo") for each type of [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") in a given world.

## Enums

[InsertMode](enum.InsertMode.html "enum bevy::ecs::bundle::InsertMode")

What to do on insertion if a component already exists.

## Traits

[Bundle](trait.Bundle.html "trait bevy::ecs::bundle::Bundle")

The `Bundle` trait enables insertion and removal of [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component")s from an entity.

[BundleFromComponents](trait.BundleFromComponents.html "trait bevy::ecs::bundle::BundleFromComponents")

Creates a [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") by taking it from internal storage.

[DynamicBundle](trait.DynamicBundle.html "trait bevy::ecs::bundle::DynamicBundle")

The parts from [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") that don’t require statically knowing the components of the bundle.

[NoBundleEffect](trait.NoBundleEffect.html "trait bevy::ecs::bundle::NoBundleEffect")

A trait implemented for [`DynamicBundle::Effect`](trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect") implementations that do nothing. This is used as a type constraint for [`Bundle`](../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") APIs that do not / cannot run [`DynamicBundle::Effect`](trait.DynamicBundle.html#associatedtype.Effect "associated type bevy::ecs::bundle::DynamicBundle::Effect"), such as “batch spawn” APIs.

## Derive Macros

[Bundle](derive.Bundle.html "derive bevy::ecs::bundle::Bundle")

Implement the `Bundle` trait.