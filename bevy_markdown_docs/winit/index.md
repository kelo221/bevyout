[bevy](../index.html)

# Crate winit 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#1-248)

`bevy_winit` provides utilities to handle window creation and the eventloop through [`winit`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/index.html "mod winit")

Most commonly, the [`WinitPlugin`](struct.WinitPlugin.html "struct bevy::winit::WinitPlugin") is used as part of [`DefaultPlugins`](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html). The app’s [runner](../prelude/struct.App.html#structfield.runner "field bevy::prelude::App::runner") is set by `WinitPlugin` and handles the `winit` [`EventLoop`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event_loop/struct.EventLoop.html "struct winit::event_loop::EventLoop"). See `winit_runner` for details.

## Modules

[accessibility](accessibility/index.html "mod bevy::winit::accessibility")

Helpers for mapping window entities to accessibility types

[converters](converters/index.html "mod bevy::winit::converters")

Helpers for mapping between winit and bevy types

## Structs

[CustomCursorSource](struct.CustomCursorSource.html "struct bevy::winit::CustomCursorSource")

Source for [`CustomCursor`](struct.WinitCustomCursor.html "struct bevy::winit::WinitCustomCursor").

[DisplayHandleWrapper](struct.DisplayHandleWrapper.html "struct bevy::winit::DisplayHandleWrapper")

A wrapper around [`winit::event_loop::OwnedDisplayHandle`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event_loop/struct.OwnedDisplayHandle.html "struct winit::event_loop::OwnedDisplayHandle")

[EventLoopProxy](struct.EventLoopProxy.html "struct bevy::winit::EventLoopProxy")

Used to send custom events to [`EventLoop`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event_loop/struct.EventLoop.html "struct winit::event_loop::EventLoop").

[EventLoopProxyWrapper](struct.EventLoopProxyWrapper.html "struct bevy::winit::EventLoopProxyWrapper")

A wrapper type around [`winit::event_loop::EventLoopProxy`](struct.EventLoopProxy.html "struct bevy::winit::EventLoopProxy") with the specific [`winit::event::Event::UserEvent`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/enum.Event.html#variant.UserEvent "variant winit::event::Event::UserEvent") used in the [`WinitPlugin`](struct.WinitPlugin.html "struct bevy::winit::WinitPlugin").

[RawWinitWindowEvent](struct.RawWinitWindowEvent.html "struct bevy::winit::RawWinitWindowEvent")

The original window event as produced by Winit. This is meant as an escape hatch for power users that wish to add custom Winit integrations. If you want to process events for your app or game, you should instead use `bevy::window::WindowEvent`, or one of its sub-events.

[WinitCustomCursor](struct.WinitCustomCursor.html "struct bevy::winit::WinitCustomCursor")

Use a custom image as a cursor (mouse pointer).

[WinitMonitors](struct.WinitMonitors.html "struct bevy::winit::WinitMonitors")

Stores [`winit`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/index.html "mod winit") monitors and their corresponding entities

[WinitPlugin](struct.WinitPlugin.html "struct bevy::winit::WinitPlugin")

A [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that uses `winit` to create and manage windows, and receive window and input events.

[WinitSettings](struct.WinitSettings.html "struct bevy::winit::WinitSettings")

Settings for the [`WinitPlugin`](struct.WinitPlugin.html "struct bevy::winit::WinitPlugin").

[WinitWindows](struct.WinitWindows.html "struct bevy::winit::WinitWindows")

A resource mapping window entities to their `winit`\-backend [`Window`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/window/struct.Window.html "struct winit::window::Window") states.

## Enums

[UpdateMode](enum.UpdateMode.html "enum bevy::winit::UpdateMode")

Determines how frequently an [`App`](../prelude/struct.App.html "struct bevy::prelude::App") should update.

[WinitUserEvent](enum.WinitUserEvent.html "enum bevy::winit::WinitUserEvent")

Events that can be sent to perform actions inside the winit event loop.

## Constants

[WINIT\_WINDOWS](constant.WINIT_WINDOWS.html "constant bevy::winit::WINIT_WINDOWS")

Temporary storage of WinitWindows data to replace usage of `!Send` resources. This will be replaced with proper storage of `!Send` data after issue #17667 is complete.

## Functions

[create\_monitors](fn.create_monitors.html "fn bevy::winit::create_monitors")

Synchronize available monitors as reported by [`winit`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/index.html "mod winit") with [`Monitor`](../window/struct.Monitor.html "struct bevy::window::Monitor") entities in the world.

[create\_windows](fn.create_windows.html "fn bevy::winit::create_windows")

Creates new windows on the [`winit`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/index.html "mod winit") backend for each entity with a newly-added [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") component.

[get\_selected\_videomode](fn.get_selected_videomode.html "fn bevy::winit::get_selected_videomode")

Returns some [`winit::monitor::VideoModeHandle`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/monitor/struct.VideoModeHandle.html "struct winit::monitor::VideoModeHandle") given a [`MonitorHandle`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/monitor/struct.MonitorHandle.html "struct winit::monitor::MonitorHandle") and a [`VideoModeSelection`](../prelude/enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection") or None if no valid matching video mode was found.

[select\_monitor](fn.select_monitor.html "fn bevy::winit::select_monitor")

Selects a monitor based on the given [`MonitorSelection`](../prelude/enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection").

[winit\_window\_position](fn.winit_window_position.html "fn bevy::winit::winit_window_position")

Compute the physical window position for a given [`WindowPosition`](../prelude/enum.WindowPosition.html "enum bevy::prelude::WindowPosition").

## Type Aliases

[CreateMonitorParams](type.CreateMonitorParams.html "type bevy::winit::CreateMonitorParams")

The parameters of the [`create_monitors`](fn.create_monitors.html "fn bevy::winit::create_monitors") system.

[CreateWindowParams](type.CreateWindowParams.html "type bevy::winit::CreateWindowParams")

The parameters of the [`create_windows`](fn.create_windows.html "fn bevy::winit::create_windows") system.