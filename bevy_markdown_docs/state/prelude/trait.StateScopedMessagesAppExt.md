[bevy](../../index.html)::[state](../index.html)::[prelude](index.html)

# Trait StateScopedMessagesAppExt 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#128)

```rust
pub trait StateScopedMessagesAppExt {
    // Required methods
    fn clear_messages_on_exit<M>(&mut self, state: impl States) -> &mut Self
       where M: Message;
    fn clear_messages_on_enter<M>(&mut self, state: impl States) -> &mut Self
       where M: Message;
}
```

Extension trait for [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") adding methods for registering state scoped messages.

## Required Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#137)

#### fn [clear\_messages\_on\_exit](#tymethod.clear_messages_on_exit)<M>(&mut self, state: impl [States](../../prelude/trait.States.html "trait bevy::prelude::States")) -> &mut Self

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

Clears a [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") when exiting the specified `state`.

Note that message cleanup is ambiguously ordered relative to [`DespawnOnExit`](../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit") entity cleanup, and the [`OnExit`](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit") schedule for the target state. All of these (state scoped entities and messages cleanup, and `OnExit`) occur within schedule [`StateTransition`](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition") and system set `StateTransitionSystems::ExitSchedules`.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#147)

#### fn [clear\_messages\_on\_enter](#tymethod.clear_messages_on_enter)<M>(&mut self, state: impl [States](../../prelude/trait.States.html "trait bevy::prelude::States")) -> &mut Self

where M: [Message](../../prelude/trait.Message.html "trait bevy::prelude::Message"),

Clears a [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") when entering the specified `state`.

Note that message cleanup is ambiguously ordered relative to [`DespawnOnEnter`](../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter") entity cleanup, and the [`OnEnter`](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter") schedule for the target state. All of these (state scoped entities and messages cleanup, and `OnEnter`) occur within schedule [`StateTransition`](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition") and system set `StateTransitionSystems::EnterSchedules`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#150)

### impl [StateScopedMessagesAppExt](../../prelude/trait.StateScopedMessagesAppExt.html "trait bevy::prelude::StateScopedMessagesAppExt") for [App](../../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped_events.rs.html#172)

### impl [StateScopedMessagesAppExt](../../prelude/trait.StateScopedMessagesAppExt.html "trait bevy::prelude::StateScopedMessagesAppExt") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")