[bevy](../index.html)

# Crate a11y 

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#1-283)

Reusable accessibility primitives

This crate provides accessibility integration for the engine. It exposes the [`AccessibilityPlugin`](struct.AccessibilityPlugin.html "struct bevy::a11y::AccessibilityPlugin"). This plugin integrates `AccessKit`, a Rust crate providing OS-agnostic accessibility primitives, with Bevy’s ECS.

### Some notes on utility

While this crate defines useful types for accessibility, it does not actually power accessibility features in Bevy.

Instead, it helps other interfaces coordinate their approach to accessibility. Binary authors should add the [`AccessibilityPlugin`](struct.AccessibilityPlugin.html "struct bevy::a11y::AccessibilityPlugin"), while library maintainers may use the [`AccessibilityRequested`](struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested") and [`ManageAccessibilityUpdates`](struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates") resources.

The [`AccessibilityNode`](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode") component is useful in both cases. It helps describe an entity in terms of its accessibility factors through an `AccessKit` “node”.

Typical UI concepts, like buttons, checkboxes, and textboxes, are easily described by this component, though, technically, it can represent any kind of Bevy [`Entity`](../prelude/struct.Entity.html "struct bevy::prelude::Entity").

### This crate no longer re-exports `AccessKit`

As of Bevy version 0.15, [the `accesskit` crate](https://crates.io/crates/accesskit) is no longer re-exported from this crate.[1](#fn1) If you need to use `AccessKit` yourself, you’ll have to add it as a separate dependency in your project’s `Cargo.toml`.

Make sure to use the same version of the `accesskit` crate as Bevy. Otherwise, you may experience errors similar to: “Perhaps two different versions of crate `accesskit` are being used?”

* * *

1.  Some users were confused about `AccessKit`’s `Node` type, sometimes thinking it was Bevy UI’s primary way to define nodes!
    
    For this reason, its re-export was removed by default. Users who need its types can instead manually depend on the `accesskit` crate. [↩](#fnref1)
    

## Structs

[AccessibilityNode](struct.AccessibilityNode.html "struct bevy::a11y::AccessibilityNode")

Represents an entity to `AccessKit` through an [`accesskit::Node`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.Node.html "struct accesskit::Node").

[AccessibilityPlugin](struct.AccessibilityPlugin.html "struct bevy::a11y::AccessibilityPlugin")

Plugin managing integration with accessibility APIs.

[AccessibilityRequested](struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested")

Tracks whether an assistive technology has requested accessibility information.

[ActionRequest](struct.ActionRequest.html "struct bevy::a11y::ActionRequest")

Wrapper struct for [`accesskit::ActionRequest`](https://docs.rs/accesskit/0.24.1/x86_64-unknown-linux-gnu/accesskit/struct.ActionRequest.html "struct accesskit::ActionRequest").

[ManageAccessibilityUpdates](struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates")

Determines whether Bevy’s ECS updates the accessibility tree.

## Enums

[AccessibilitySystems](enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems")

A system set relating to accessibility.