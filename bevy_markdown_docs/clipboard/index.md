[bevy](../index.html)

# Crate clipboard 

[Source](https://docs.rs/bevy_clipboard/0.19.0/x86_64-unknown-linux-gnu/src/bevy_clipboard/lib.rs.html#1-427)

This crate provides a platform-agnostic interface for accessing the clipboard.

Read (and write) to the [`Clipboard`](../prelude/struct.Clipboard.html "struct bevy::prelude::Clipboard") resource to interact with the system clipboard.

Note that this crate is deliberately low-level with minimal dependencies: it does not provide any input integration for clipboard operations, such as Ctrl+C/Ctrl+V support.

This should be provided by other crates (or your own systems) which depend on `bevy_clipboard`, such as `bevy_ui_widgets` in the case of text editing.

`bevy_clipboard`’s primary advantage over using [`arboard`](https://crates.io/crates/arboard) directly is that it provides a consistent API across all platforms, with a simple but robust fallback when `arboard` is not available or clipboard permissions are not granted.

### Platform support

On Android and iOS, `arboard` is not available and the `system_clipboard` feature has no effect. The [`Clipboard`](../prelude/struct.Clipboard.html "struct bevy::prelude::Clipboard") resource still works, but reads and writes go to an in-process buffer that is invisible to other applications and does not survive process exit.

On Windows and Unix, clipboard operations are performed synchronously and results are available immediately. On wasm32, results are accessed via [`ClipboardRead`](../prelude/enum.ClipboardRead.html "enum bevy::prelude::ClipboardRead"), which can be polled for completion.

Images are supported on Windows and Unix when the `image` feature is enabled, which depends on `system_clipboard`. Image support is not available on wasm32, Android, or iOS.

## Modules

[prelude](prelude/index.html "mod bevy::clipboard::prelude")

Commonly used types and traits from `bevy_clipboard`.

## Structs

[Clipboard](struct.Clipboard.html "struct bevy::clipboard::Clipboard")

A resource which provides access to the system clipboard.

[ClipboardPlugin](struct.ClipboardPlugin.html "struct bevy::clipboard::ClipboardPlugin")

Adds clipboard support to a Bevy app.

## Enums

[ClipboardError](enum.ClipboardError.html "enum bevy::clipboard::ClipboardError")

An error that might happen during a clipboard operation.

[ClipboardRead](enum.ClipboardRead.html "enum bevy::clipboard::ClipboardRead")

Represents an attempt to read from the clipboard.