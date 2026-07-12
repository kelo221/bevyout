[bevy](../index.html)

# Crate window 

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/lib.rs.html#1-174)

`bevy_window` provides a platform-agnostic interface for windowing in Bevy.

This crate contains types for window management and events, used by windowing implementors such as `bevy_winit`. The [`WindowPlugin`](../prelude/struct.WindowPlugin.html "struct bevy::prelude::WindowPlugin") sets up some global window-related parameters and is part of the [`DefaultPlugins`](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html).

## Modules

[prelude](prelude/index.html "mod bevy::window::prelude")

The windowing prelude.

## Structs

[ClosingWindow](struct.ClosingWindow.html "struct bevy::window::ClosingWindow")

Marker component for a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") that has been requested to close and is in the process of closing (on the next frame).

[CursorEntered](struct.CursorEntered.html "struct bevy::window::CursorEntered")

An event that is sent whenever the user’s cursor enters a window.

[CursorLeft](struct.CursorLeft.html "struct bevy::window::CursorLeft")

An event that is sent whenever the user’s cursor leaves a window.

[CursorMoved](struct.CursorMoved.html "struct bevy::window::CursorMoved")

An event reporting that the mouse cursor has moved inside a window.

[CursorOptions](struct.CursorOptions.html "struct bevy::window::CursorOptions")

Cursor data for a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window").

[CustomCursorImage](struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

A custom cursor created from an image.

[CustomCursorImageTemplate](struct.CustomCursorImageTemplate.html "struct bevy::window::CustomCursorImageTemplate")

[CustomCursorUrl](struct.CustomCursorUrl.html "struct bevy::window::CustomCursorUrl")

A custom cursor created from a URL. Note that this currently only works on the web.

[EnabledButtons](struct.EnabledButtons.html "struct bevy::window::EnabledButtons")

Specifies which [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") control buttons should be enabled.

[ExitSystems](struct.ExitSystems.html "struct bevy::window::ExitSystems")

A [`SystemSet`](../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") for the system that exits the application. Which can be either [`exit_on_all_closed`](fn.exit_on_all_closed.html "fn bevy::window::exit_on_all_closed") or [`exit_on_primary_closed`](fn.exit_on_primary_closed.html "fn bevy::window::exit_on_primary_closed").

[HasWindows](struct.HasWindows.html "struct bevy::window::HasWindows")

A relationship for all Windows on a specific Monitor.

[InternalWindowState](struct.InternalWindowState.html "struct bevy::window::InternalWindowState")

Stores internal [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") state that isn’t directly accessible.

[Monitor](struct.Monitor.html "struct bevy::window::Monitor")

Represents an available monitor as reported by the user’s operating system, which can be used to query information about the display, such as its size, position, and video modes.

[NormalizedWindowRef](struct.NormalizedWindowRef.html "struct bevy::window::NormalizedWindowRef")

A flattened representation of a window reference for equality/hashing purposes.

[OnMonitor](struct.OnMonitor.html "struct bevy::window::OnMonitor")

Represents the relationship between a Window and the Monitor it is currently on.

[PrimaryMonitor](struct.PrimaryMonitor.html "struct bevy::window::PrimaryMonitor")

A marker component for the primary monitor

[PrimaryWindow](struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow")

Marker [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") for the window considered the primary window.

[RawHandleWrapper](struct.RawHandleWrapper.html "struct bevy::window::RawHandleWrapper")

A wrapper over [`RawWindowHandle`](https://docs.rs/raw-window-handle/0.6.2/x86_64-unknown-linux-gnu/raw_window_handle/enum.RawWindowHandle.html "enum raw_window_handle::RawWindowHandle") and [`RawDisplayHandle`](https://docs.rs/raw-window-handle/0.6.2/x86_64-unknown-linux-gnu/raw_window_handle/enum.RawDisplayHandle.html "enum raw_window_handle::RawDisplayHandle") that allows us to safely pass it across threads.

[RawHandleWrapperHolder](struct.RawHandleWrapperHolder.html "struct bevy::window::RawHandleWrapperHolder")

Holder of the [`RawHandleWrapper`](struct.RawHandleWrapper.html "struct bevy::window::RawHandleWrapper") with wrappers, to allow use in asynchronous context

[RequestRedraw](struct.RequestRedraw.html "struct bevy::window::RequestRedraw")

An event that indicates all of the application’s windows should be redrawn, even if their control flow is set to `Wait` and there have been no window events.

[ThreadLockedRawWindowHandleWrapper](struct.ThreadLockedRawWindowHandleWrapper.html "struct bevy::window::ThreadLockedRawWindowHandleWrapper")

A [`RawHandleWrapper`](struct.RawHandleWrapper.html "struct bevy::window::RawHandleWrapper") that cannot be sent across threads.

[VideoMode](struct.VideoMode.html "struct bevy::window::VideoMode")

Represents a video mode that a monitor supports

[Window](struct.Window.html "struct bevy::window::Window")

The defining [`Component`](../prelude/trait.Component.html "trait bevy::prelude::Component") for window entities, storing information about how it should appear and behave.

[WindowBackendScaleFactorChanged](struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged")

An event that indicates a window’s OS-reported scale factor has changed.

[WindowCloseRequested](struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested")

An event that is sent whenever the operating systems requests that a window be closed. This will be sent when the close button of the window is pressed.

[WindowClosed](struct.WindowClosed.html "struct bevy::window::WindowClosed")

An event that is sent whenever a window is closed. This will be sent when the window entity loses its [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") component or is despawned.

[WindowClosing](struct.WindowClosing.html "struct bevy::window::WindowClosing")

An event that is sent whenever a window is closing. This will be sent when after a [`WindowCloseRequested`](struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested") event is received and the window is in the process of closing.

[WindowCreated](struct.WindowCreated.html "struct bevy::window::WindowCreated")

An event that is sent whenever a new window is created.

[WindowDestroyed](struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed")

An event that is sent whenever a window is destroyed by the underlying window system.

[WindowFocused](struct.WindowFocused.html "struct bevy::window::WindowFocused")

An event that indicates a window has received or lost focus.

[WindowMoved](struct.WindowMoved.html "struct bevy::window::WindowMoved")

An event that is sent when a window is repositioned in physical pixels.

[WindowOccluded](struct.WindowOccluded.html "struct bevy::window::WindowOccluded")

The window has been occluded (completely hidden from view).

[WindowPlugin](struct.WindowPlugin.html "struct bevy::window::WindowPlugin")

A [`Plugin`](../prelude/trait.Plugin.html "trait bevy::prelude::Plugin") that defines an interface for windowing support in Bevy.

[WindowResizeConstraints](struct.WindowResizeConstraints.html "struct bevy::window::WindowResizeConstraints")

The size limits on a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window").

[WindowResized](struct.WindowResized.html "struct bevy::window::WindowResized")

A window event that is sent whenever a window’s logical size has changed.

[WindowResolution](struct.WindowResolution.html "struct bevy::window::WindowResolution")

Controls the size of a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window")

[WindowScaleFactorChanged](struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged")

An event that indicates a window’s scale factor has changed.

[WindowThemeChanged](struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged")

An event sent when the system theme changes for a window.

[WindowWrapper](struct.WindowWrapper.html "struct bevy::window::WindowWrapper")

A wrapper over a window.

## Enums

[AppLifecycle](enum.AppLifecycle.html "enum bevy::window::AppLifecycle")

Application lifetime events

[CompositeAlphaMode](enum.CompositeAlphaMode.html "enum bevy::window::CompositeAlphaMode")

Specifies how the alpha channel of the textures should be handled during compositing, for a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window").

[CursorGrabMode](enum.CursorGrabMode.html "enum bevy::window::CursorGrabMode")

Defines if and how the cursor is grabbed by a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window").

[CursorIcon](enum.CursorIcon.html "enum bevy::window::CursorIcon")

Insert into a window entity to set the cursor for that window.

[CustomCursor](enum.CustomCursor.html "enum bevy::window::CustomCursor")

Custom cursor image data.

[CustomCursorTemplate](enum.CustomCursorTemplate.html "enum bevy::window::CustomCursorTemplate")

[ExitCondition](enum.ExitCondition.html "enum bevy::window::ExitCondition")

Defines the specific conditions the application should exit on

[FileDragAndDrop](enum.FileDragAndDrop.html "enum bevy::window::FileDragAndDrop")

Events related to files being dragged and dropped on a window.

[Ime](enum.Ime.html "enum bevy::window::Ime")

An Input Method Editor event.

[MonitorSelection](enum.MonitorSelection.html "enum bevy::window::MonitorSelection")

References a screen monitor.

[PresentMode](enum.PresentMode.html "enum bevy::window::PresentMode")

Presentation mode for a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window").

[ScreenEdge](enum.ScreenEdge.html "enum bevy::window::ScreenEdge")

The edges of a screen. Corresponds to [`winit::platform::ios::ScreenEdge`](https://docs.rs/winit/latest/x86_64-apple-darwin/winit/platform/ios/struct.ScreenEdge.html).

[SystemCursorIcon](enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon")

The icon to display for a window.

[VideoModeSelection](enum.VideoModeSelection.html "enum bevy::window::VideoModeSelection")

References an exclusive fullscreen video mode.

[WindowEvent](enum.WindowEvent.html "enum bevy::window::WindowEvent")

Wraps all `bevy_window` and `bevy_input` events in a common enum.

[WindowLevel](enum.WindowLevel.html "enum bevy::window::WindowLevel")

Specifies where a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") should appear relative to other overlapping windows (on top or under) .

[WindowMode](enum.WindowMode.html "enum bevy::window::WindowMode")

Defines the way a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") is displayed.

[WindowPosition](enum.WindowPosition.html "enum bevy::window::WindowPosition")

Defines where a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") should be placed on the screen.

[WindowRef](enum.WindowRef.html "enum bevy::window::WindowRef")

Reference to a [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window"), whether it be a direct link to a specific entity or a more vague defaulting choice.

[WindowTheme](enum.WindowTheme.html "enum bevy::window::WindowTheme")

The [`Window`](../prelude/struct.Window.html "struct bevy::prelude::Window") theme variant to use.

## Functions

[close\_when\_requested](fn.close_when_requested.html "fn bevy::window::close_when_requested")

Close windows in response to [`WindowCloseRequested`](struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested") (e.g. when the close button is pressed).

[exit\_on\_all\_closed](fn.exit_on_all_closed.html "fn bevy::window::exit_on_all_closed")

Exit the application when there are no open windows.

[exit\_on\_primary\_closed](fn.exit_on_primary_closed.html "fn bevy::window::exit_on_primary_closed")

Exit the application when the primary window has been closed