[bevy](../index.html)

# Crate dev\_tools 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/lib.rs.html#1-32)

This crate provides additional utilities for the [Bevy game engine](https://bevy.org), focused on improving developer experience.

## Modules

[ci\_testing](ci_testing/index.html "mod bevy::dev_tools::ci_testing")`bevy_ci_testing`

Utilities for testing in CI environments.

[diagnostics\_overlay](diagnostics_overlay/index.html "mod bevy::dev_tools::diagnostics_overlay")

Overlay showing diagnostics

[fps\_overlay](fps_overlay/index.html "mod bevy::dev_tools::fps_overlay")

Module containing logic for FPS overlay.

[frame\_time\_graph](frame_time_graph/index.html "mod bevy::dev_tools::frame_time_graph")

Module containing logic for the frame time graph

[infinite\_grid](infinite_grid/index.html "mod bevy::dev_tools::infinite_grid")

This module implements an infinite grid with colored major axis.

[picking\_debug](picking_debug/index.html "mod bevy::dev_tools::picking_debug")

Text and on-screen debugging tools

[render\_debug](render_debug/index.html "mod bevy::dev_tools::render_debug")

Renderer debugging overlay

[schedule\_data](schedule_data/index.html "mod bevy::dev_tools::schedule_data")`schedule_data`

Tools for extracting schedule data from an app, and interpreting that data for use with visualization tools (for example).

[states](states/index.html "mod bevy::dev_tools::states")

Tools for debugging states.

## Structs

[CameraMovement](struct.CameraMovement.html "struct bevy::dev_tools::CameraMovement")

Move the camera to the given position

[EasyCameraMovementPlugin](struct.EasyCameraMovementPlugin.html "struct bevy::dev_tools::EasyCameraMovementPlugin")

Plugin to move the camera smoothly according to the current time

[EasyScreenshotPlugin](struct.EasyScreenshotPlugin.html "struct bevy::dev_tools::EasyScreenshotPlugin")

Add this plugin to your app to enable easy screenshotting.

## Enums

[ScreenshotFormat](enum.ScreenshotFormat.html "enum bevy::dev_tools::ScreenshotFormat")

File format the screenshot will be saved in