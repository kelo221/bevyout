# DynamicLighting port boundary

This directory is the isolated Rust port boundary for concepts from
AlpacaIT.DynamicLighting by Henry de Jongh. The original Unity package is kept
unchanged at the repository root under `DynamicLighting/` and is not compiled
or included in the Rust crate.

Original project: <https://github.com/Henry00IS/DynamicLighting>

The port is being rebuilt from documented behavior and small, reviewable Rust
contracts. No Unity C# source is copied into this directory. The core effect
model is Bevy-free; only `bevy_bridge/` imports Bevy types.
