[bevy](../index.html)

# Crate diagnostic 

[Source](https://docs.rs/bevy_diagnostic/0.19.0/x86_64-unknown-linux-gnu/src/bevy_diagnostic/lib.rs.html#1-51)

This crate provides a straightforward solution for integrating diagnostics in the [Bevy game engine](https://bevy.org/). It allows users to easily add diagnostic functionality to their Bevy applications, enhancing their ability to monitor and optimize their games.

## Structs

[Diagnostic](struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic")

A timeline of [`DiagnosticMeasurement`](struct.DiagnosticMeasurement.html "struct bevy::diagnostic::DiagnosticMeasurement")s of a specific type. Diagnostic examples: frames per second, CPU usage, network latency

[DiagnosticMeasurement](struct.DiagnosticMeasurement.html "struct bevy::diagnostic::DiagnosticMeasurement")

A single measurement of a [`Diagnostic`](struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic").

[DiagnosticPath](struct.DiagnosticPath.html "struct bevy::diagnostic::DiagnosticPath")

Unique diagnostic path, separated by `/`.

[Diagnostics](struct.Diagnostics.html "struct bevy::diagnostic::Diagnostics")

Record new [`DiagnosticMeasurement`](struct.DiagnosticMeasurement.html "struct bevy::diagnostic::DiagnosticMeasurement")’s.

[DiagnosticsPlugin](struct.DiagnosticsPlugin.html "struct bevy::diagnostic::DiagnosticsPlugin")

Adds core diagnostics resources to an App.

[DiagnosticsStore](struct.DiagnosticsStore.html "struct bevy::diagnostic::DiagnosticsStore")

A collection of [`Diagnostic`](struct.Diagnostic.html "struct bevy::diagnostic::Diagnostic")s.

[EntityCountDiagnosticsPlugin](struct.EntityCountDiagnosticsPlugin.html "struct bevy::diagnostic::EntityCountDiagnosticsPlugin")

Adds “entity count” diagnostic to an App.

[FrameCount](struct.FrameCount.html "struct bevy::diagnostic::FrameCount")

Maintains a count of frames rendered since the start of the application.

[FrameCountPlugin](struct.FrameCountPlugin.html "struct bevy::diagnostic::FrameCountPlugin")

Adds frame counting functionality to Apps.

[FrameTimeDiagnosticsPlugin](struct.FrameTimeDiagnosticsPlugin.html "struct bevy::diagnostic::FrameTimeDiagnosticsPlugin")

Adds “frame time” diagnostic to an App, specifically “frame time”, “fps” and “frame count”

[LogDiagnosticsPlugin](struct.LogDiagnosticsPlugin.html "struct bevy::diagnostic::LogDiagnosticsPlugin")

An App Plugin that logs diagnostics to the console.

[LogDiagnosticsState](struct.LogDiagnosticsState.html "struct bevy::diagnostic::LogDiagnosticsState")

State used by the [`LogDiagnosticsPlugin`](struct.LogDiagnosticsPlugin.html "struct bevy::diagnostic::LogDiagnosticsPlugin")

[SystemInfo](struct.SystemInfo.html "struct bevy::diagnostic::SystemInfo")

A resource that stores diagnostic information about the system. This information can be useful for debugging and profiling purposes.

[SystemInformationDiagnosticsPlugin](struct.SystemInformationDiagnosticsPlugin.html "struct bevy::diagnostic::SystemInformationDiagnosticsPlugin")

Adds a System Information Diagnostic, specifically `cpu_usage` (in %) and `mem_usage` (in %)

## Constants

[DEFAULT\_MAX\_HISTORY\_LENGTH](constant.DEFAULT_MAX_HISTORY_LENGTH.html "constant bevy::diagnostic::DEFAULT_MAX_HISTORY_LENGTH")

Default max history length for new diagnostics.

## Traits

[RegisterDiagnostic](trait.RegisterDiagnostic.html "trait bevy::diagnostic::RegisterDiagnostic")

Extend [`App`](../prelude/struct.App.html "struct bevy::prelude::App") with new `register_diagnostic` function.

## Functions

[update\_frame\_count](fn.update_frame_count.html "fn bevy::diagnostic::update_frame_count")

A system used to increment [`FrameCount`](struct.FrameCount.html "struct bevy::diagnostic::FrameCount") with wrapping addition.