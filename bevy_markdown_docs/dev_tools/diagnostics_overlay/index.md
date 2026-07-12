[bevy](../../index.html)::[dev\_tools](../index.html)

# Module diagnostics\_overlay 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/lib.rs.html#16)

Overlay showing diagnostics

The window can be created using the [`DiagnosticsOverlay`](struct.DiagnosticsOverlay.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlay") component

## Structs

[DiagnosticsOverlay](struct.DiagnosticsOverlay.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlay")

Diagnostics overlay displays on a draggable and collapsible window statistics stored on the [`DiagnosticsStore`](../../diagnostic/struct.DiagnosticsStore.html "struct bevy::diagnostic::DiagnosticsStore"). Spawning an entity with this component will create the window for you. Some presets are also provided.

[DiagnosticsOverlayItem](struct.DiagnosticsOverlayItem.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayItem")

An item to be displayed on the overlay.

[DiagnosticsOverlayPlane](struct.DiagnosticsOverlayPlane.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayPlane")

Marker for the UI root that will hold all of the [`DiagnosticsOverlay`](struct.DiagnosticsOverlay.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlay") entities.

[DiagnosticsOverlayPlugin](struct.DiagnosticsOverlayPlugin.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayPlugin")

Plugin that builds a visual overlay to present diagnostics.

[DiagnosticsOverlayStyle](struct.DiagnosticsOverlayStyle.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayStyle")

Configures the style of diagnostic overlays

## Enums

[DiagnosticsOverlayStatistic](enum.DiagnosticsOverlayStatistic.html "enum bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayStatistic")

The statistic to use when displaying a diagnostic

[DiagnosticsOverlaySystems](enum.DiagnosticsOverlaySystems.html "enum bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlaySystems")

System set for the systems of the [`DiagnosticsOverlayPlugin`](struct.DiagnosticsOverlayPlugin.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayPlugin")

## Constants

[INITIAL\_DIAGNOSTICS\_OVERLAY\_PLANE\_Z\_INDEX](constant.INITIAL_DIAGNOSTICS_OVERLAY_PLANE_Z_INDEX.html "constant bevy::dev_tools::diagnostics_overlay::INITIAL_DIAGNOSTICS_OVERLAY_PLANE_Z_INDEX")

Initial Z-index for the [`DiagnosticsOverlayPlane`](struct.DiagnosticsOverlayPlane.html "struct bevy::dev_tools::diagnostics_overlay::DiagnosticsOverlayPlane")