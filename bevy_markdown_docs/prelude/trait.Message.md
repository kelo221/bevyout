[bevy](../index.html)::[prelude](index.html)

# Trait Message 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/mod.rs.html#100)

```rust
pub trait Message:
    Send
    + Sync
    + 'static { }
```

A buffered message for pull-based event handling.

Messages can be written with [`MessageWriter`](struct.MessageWriter.html "struct bevy::prelude::MessageWriter") and read using the [`MessageReader`](struct.MessageReader.html "struct bevy::prelude::MessageReader") system parameter. Messages are stored in the [`Messages<M>`](struct.Messages.html "struct bevy::prelude::Messages") resource, and require periodically polling the world for new messages, typically in a system that runs as part of a schedule.

A [`MessageReader`](struct.MessageReader.html "struct bevy::prelude::MessageReader") system parameter tracks the consumption of these events on a per-system basis using a [`Local<MessageCursor>`](struct.Local.html "struct bevy::prelude::Local"), which will guarantee each system an opportunity to read the event once.

While the polling imposes a small overhead, messages are useful for efficiently batch processing a large number of messages at once. For cases like these, messages can be more efficient than [`Event`](trait.Event.html "trait bevy::prelude::Event")s (which are handled via [`Observer`](struct.Observer.html "struct bevy::prelude::Observer")s).

Unlike [`Event`](trait.Event.html "trait bevy::prelude::Event")s triggered for observers, messages are evaluated at fixed points in the schedule rather than immediately when they are sent. This allows for more predictable scheduling, and deferring message processing to a later point in time.

Messages must be thread-safe.

## Usage

The [`Message`](trait.Message.html "trait bevy::prelude::Message") trait can be derived:

```rust
#[derive(Message)]
struct Greeting(String);
```

The message can then be written to the message buffer using a [`MessageWriter`](struct.MessageWriter.html "struct bevy::prelude::MessageWriter"):

```rust
fn write_hello(mut writer: MessageWriter<Greeting>) {
    writer.write(Greeting("Hello!".to_string()));
}
```

Messages can be efficiently read using a [`MessageReader`](struct.MessageReader.html "struct bevy::prelude::MessageReader"):

```rust
fn read_messages(mut reader: MessageReader<Greeting>) {
    // Process all messages of type `Greeting`.
    for Greeting(greeting) in reader.read() {
        println!("{greeting}");
    }
}
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#89)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [ActionRequest](../a11y/struct.ActionRequest.html "struct bevy::a11y::ActionRequest")

where [ActionRequest](../a11y/struct.ActionRequest.html "struct bevy::a11y::ActionRequest"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1562)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [AppExit](enum.AppExit.html "enum bevy::prelude::AppExit")

where [AppExit](enum.AppExit.html "enum bevy::prelude::AppExit"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#450)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [AppLifecycle](../window/enum.AppLifecycle.html "enum bevy::window::AppLifecycle")

where [AppLifecycle](../window/enum.AppLifecycle.html "enum bevy::window::AppLifecycle"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/ci_testing/config.rs.html#67)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [CiTestingCustomEvent](../dev_tools/ci_testing/struct.CiTestingCustomEvent.html "struct bevy::dev_tools::ci_testing::CiTestingCustomEvent")

where [CiTestingCustomEvent](../dev_tools/ci_testing/struct.CiTestingCustomEvent.html "struct bevy::dev_tools::ci_testing::CiTestingCustomEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#206)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [CursorEntered](struct.CursorEntered.html "struct bevy::prelude::CursorEntered")

where [CursorEntered](struct.CursorEntered.html "struct bevy::prelude::CursorEntered"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#223)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [CursorLeft](struct.CursorLeft.html "struct bevy::prelude::CursorLeft")

where [CursorLeft](struct.CursorLeft.html "struct bevy::prelude::CursorLeft"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#181)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [CursorMoved](struct.CursorMoved.html "struct bevy::prelude::CursorMoved")

where [CursorMoved](struct.CursorMoved.html "struct bevy::prelude::CursorMoved"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#63)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [DoubleTapGesture](../input/gestures/struct.DoubleTapGesture.html "struct bevy::input::gestures::DoubleTapGesture")

where [DoubleTapGesture](../input/gestures/struct.DoubleTapGesture.html "struct bevy::input::gestures::DoubleTapGesture"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#373)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [FileDragAndDrop](enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop")

where [FileDragAndDrop](enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#254)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [GamepadAxisChangedEvent](../input/gamepad/struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent")

where [GamepadAxisChangedEvent](../input/gamepad/struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#219)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [GamepadButtonChangedEvent](../input/gamepad/struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent")

where [GamepadButtonChangedEvent](../input/gamepad/struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#187)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [GamepadButtonStateChangedEvent](../input/gamepad/struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent")

where [GamepadButtonStateChangedEvent](../input/gamepad/struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#148)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [GamepadConnectionEvent](../input/gamepad/struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")

where [GamepadConnectionEvent](../input/gamepad/struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#35)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [GamepadEvent](../input/gamepad/enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent")

where [GamepadEvent](../input/gamepad/enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1777)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [GamepadRumbleRequest](../input/gamepad/enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest")

where [GamepadRumbleRequest](../input/gamepad/enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#142)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [HotPatched](../ecs/struct.HotPatched.html "struct bevy::ecs::HotPatched")

where [HotPatched](../ecs/struct.HotPatched.html "struct bevy::ecs::HotPatched"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#244)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [Ime](enum.Ime.html "enum bevy::prelude::Ime")

where [Ime](enum.Ime.html "enum bevy::prelude::Ime"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#149)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [KeyboardFocusLost](../input/keyboard/struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost")

where [KeyboardFocusLost](../input/keyboard/struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#100)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [KeyboardInput](../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

where [KeyboardInput](../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#31)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [MouseButtonInput](../input/mouse/struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput")

where [MouseButtonInput](../input/mouse/struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#96)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [MouseMotion](../input/mouse/struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion")

where [MouseMotion](../input/mouse/struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#157)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [MouseWheel](../input/mouse/struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel")

where [MouseWheel](../input/mouse/struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#81)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [PanGesture](../input/gestures/struct.PanGesture.html "struct bevy::input::gestures::PanGesture")

where [PanGesture](../input/gestures/struct.PanGesture.html "struct bevy::input::gestures::PanGesture"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#22)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [PinchGesture](../input/gestures/struct.PinchGesture.html "struct bevy::input::gestures::PinchGesture")

where [PinchGesture](../input/gestures/struct.PinchGesture.html "struct bevy::input::gestures::PinchGesture"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#91)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [PointerHits](../picking/backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits")

where [PointerHits](../picking/backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#278)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [PointerInput](../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput")

where [PointerInput](../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#115)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RawGamepadAxisChangedEvent](../input/gamepad/struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent")

where [RawGamepadAxisChangedEvent](../input/gamepad/struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#83)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RawGamepadButtonChangedEvent](../input/gamepad/struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent")

where [RawGamepadButtonChangedEvent](../input/gamepad/struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#62)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RawGamepadEvent](../input/gamepad/enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent")

where [RawGamepadEvent](../input/gamepad/enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#191)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RawWinitWindowEvent](../winit/struct.RawWinitWindowEvent.html "struct bevy::winit::RawWinitWindowEvent")

where [RawWinitWindowEvent](../winit/struct.RawWinitWindowEvent.html "struct bevy::winit::RawWinitWindowEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#398)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RemovedComponentEntity](../ecs/lifecycle/struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity")

where [RemovedComponentEntity](../ecs/lifecycle/struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#262)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RenderDebugOverlayEvent](../dev_tools/render_debug/enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent")

where [RenderDebugOverlayEvent](../dev_tools/render_debug/enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#50)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RequestRedraw](../window/struct.RequestRedraw.html "struct bevy::window::RequestRedraw")

where [RequestRedraw](../window/struct.RequestRedraw.html "struct bevy::window::RequestRedraw"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#44)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [RotationGesture](../input/gestures/struct.RotationGesture.html "struct bevy::input::gestures::RotationGesture")

where [RotationGesture](../input/gestures/struct.RotationGesture.html "struct bevy::input::gestures::RotationGesture"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#42)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [TouchInput](struct.TouchInput.html "struct bevy::prelude::TouchInput")

where [TouchInput](struct.TouchInput.html "struct bevy::prelude::TouchInput"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#27)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [UntypedAssetLoadFailedEvent](../asset/struct.UntypedAssetLoadFailedEvent.html "struct bevy::asset::UntypedAssetLoadFailedEvent")

where [UntypedAssetLoadFailedEvent](../asset/struct.UntypedAssetLoadFailedEvent.html "struct bevy::asset::UntypedAssetLoadFailedEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#354)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowBackendScaleFactorChanged](../window/struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged")

where [WindowBackendScaleFactorChanged](../window/struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#92)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowCloseRequested](../window/struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested")

where [WindowCloseRequested](../window/struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#110)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowClosed](../window/struct.WindowClosed.html "struct bevy::window::WindowClosed")

where [WindowClosed](../window/struct.WindowClosed.html "struct bevy::window::WindowClosed"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#131)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowClosing](../window/struct.WindowClosing.html "struct bevy::window::WindowClosing")

where [WindowClosing](../window/struct.WindowClosing.html "struct bevy::window::WindowClosing"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#66)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowCreated](../window/struct.WindowCreated.html "struct bevy::window::WindowCreated")

where [WindowCreated](../window/struct.WindowCreated.html "struct bevy::window::WindowCreated"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#151)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowDestroyed](../window/struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed")

where [WindowDestroyed](../window/struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#493)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowEvent](../window/enum.WindowEvent.html "enum bevy::window::WindowEvent")

where [WindowEvent](../window/enum.WindowEvent.html "enum bevy::window::WindowEvent"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#289)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowFocused](../window/struct.WindowFocused.html "struct bevy::window::WindowFocused")

where [WindowFocused](../window/struct.WindowFocused.html "struct bevy::window::WindowFocused"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#409)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowMoved](struct.WindowMoved.html "struct bevy::prelude::WindowMoved")

where [WindowMoved](struct.WindowMoved.html "struct bevy::prelude::WindowMoved"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#316)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowOccluded](../window/struct.WindowOccluded.html "struct bevy::window::WindowOccluded")

where [WindowOccluded](../window/struct.WindowOccluded.html "struct bevy::window::WindowOccluded"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#28)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowResized](../window/struct.WindowResized.html "struct bevy::window::WindowResized")

where [WindowResized](../window/struct.WindowResized.html "struct bevy::window::WindowResized"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#335)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowScaleFactorChanged](../window/struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged")

where [WindowScaleFactorChanged](../window/struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#431)

### impl [Message](trait.Message.html "trait bevy::prelude::Message") for [WindowThemeChanged](../window/struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged")

where [WindowThemeChanged](../window/struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged"): [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#49)

### impl<A> [Message](trait.Message.html "trait bevy::prelude::Message") for [AssetEvent](enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>

where A: [Asset](trait.Asset.html "trait bevy::prelude::Asset"), [AssetEvent](enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#9)

### impl<A> [Message](trait.Message.html "trait bevy::prelude::Message") for [AssetLoadFailedEvent](../asset/struct.AssetLoadFailedEvent.html "struct bevy::asset::AssetLoadFailedEvent")<A>

where A: [Asset](trait.Asset.html "trait bevy::prelude::Asset"), [AssetLoadFailedEvent](../asset/struct.AssetLoadFailedEvent.html "struct bevy::asset::AssetLoadFailedEvent")<A>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [Message](trait.Message.html "trait bevy::prelude::Message") for [Pointer](struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"), [Pointer](struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/transitions.rs.html#67)

### impl<S> [Message](trait.Message.html "trait bevy::prelude::Message") for [StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>

where S: [States](trait.States.html "trait bevy::prelude::States"), [StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,