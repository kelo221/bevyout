[bevy](../../index.html)::[picking](../index.html)::[prelude](index.html)

# Function pointer\_events 

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#667-684)

```rust
pub fn pointer_events(
    input_events: MessageReader<'_, '_, PointerInput>,
    pointers: Query<'_, '_, &PointerLocation>,
    ancestors_query: Query<'_, '_, &ChildOf>,
    pointer_map: Res<'_, PointerMap>,
    hover_map: Res<'_, HoverMap>,
    previous_hover_map: Res<'_, PreviousHoverMap>,
    picking_settings: Res<'_, PickingSettings>,
    pointer_state: ResMut<'_, PointerState>,
    hovered_entity_ancestors: Local<'_, HoveredEntityAncestors>,
    sent_leave: Local<'_, HashSet<(PointerId, Entity)>>,
    sent_enter: Local<'_, HashSet<(PointerId, Entity)>>,
    commands: Commands<'_, '_>,
    message_writers: PickingMessageWriters<'_>,
)
```

Dispatches interaction events to the target entities.

Within a single frame, events are dispatched in the following order:

*   [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") → [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave") → [`DragLeave`](../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave").
*   [`DragEnter`](../../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter") → [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter") → [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over").
*   Any number of any of the following:
    *   For each movement: [`DragStart`](../../prelude/struct.DragStart.html "struct bevy::prelude::DragStart") → [`Drag`](../../prelude/struct.Drag.html "struct bevy::prelude::Drag") → [`DragOver`](../../prelude/struct.DragOver.html "struct bevy::prelude::DragOver") → [`Move`](../../prelude/struct.Move.html "struct bevy::prelude::Move").
    *   For each button press: [`Press`](../../prelude/struct.Press.html "struct bevy::prelude::Press") or [`Click`](../../prelude/struct.Click.html "struct bevy::prelude::Click") → [`Release`](../../prelude/struct.Release.html "struct bevy::prelude::Release") → [`DragDrop`](../../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop") → [`DragEnd`](../../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd") → [`DragLeave`](../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave").
    *   For each pointer cancellation: [`Cancel`](../../prelude/struct.Cancel.html "struct bevy::prelude::Cancel").

Additionally, across multiple frames, the following are also strictly ordered by the interaction state machine:

*   When a pointer moves over the target: [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over"), [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter"), [`Move`](../../prelude/struct.Move.html "struct bevy::prelude::Move"), [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave"), [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out").
*   When a pointer presses buttons on the target: [`Press`](../../prelude/struct.Press.html "struct bevy::prelude::Press"), [`Click`](../../prelude/struct.Click.html "struct bevy::prelude::Click"), [`Release`](../../prelude/struct.Release.html "struct bevy::prelude::Release").
*   When a pointer drags the target: [`DragStart`](../../prelude/struct.DragStart.html "struct bevy::prelude::DragStart"), [`Drag`](../../prelude/struct.Drag.html "struct bevy::prelude::Drag"), [`DragEnd`](../../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd").
*   When a pointer drags something over the target: [`DragEnter`](../../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter"), [`DragOver`](../../prelude/struct.DragOver.html "struct bevy::prelude::DragOver"), [`DragDrop`](../../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop"), [`DragLeave`](../../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave").
*   When a pointer is canceled: No other events will follow the [`Cancel`](../../prelude/struct.Cancel.html "struct bevy::prelude::Cancel") event for that pointer.

Four events – [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over"), [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter"), [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave") and [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") – are driven only by the [`HoverMap`](../hover/struct.HoverMap.html "struct bevy::picking::hover::HoverMap"). The rest rely on additional data from the [`PointerInput`](../pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") event stream. To receive these events for a custom pointer, you must add [`PointerInput`](../pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") events.

When the pointer goes from hovering entity A to entity B, entity A will receive [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") and [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter") and then entity B will receive [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave") and [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over"). No entity will ever receive both an [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over") and an [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") or an [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter") and a [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave") event during the same frame.

When we account for event bubbling, the two pairs of events, [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over") and [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter") [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave"), behave differently. When the hovering focus shifts between children, parent entities may receive redundant [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") → [`Over`](../../prelude/struct.Over.html "struct bevy::prelude::Over") pairs. In the case of [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter") → [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave"), shared parent entities will not receive [`Enter`](../../prelude/struct.Enter.html "struct bevy::prelude::Enter") or [`Leave`](../../prelude/struct.Leave.html "struct bevy::prelude::Leave").

Both [`Click`](../../prelude/struct.Click.html "struct bevy::prelude::Click") and [`Release`](../../prelude/struct.Release.html "struct bevy::prelude::Release") target the entity hovered in the _previous frame_, rather than the current frame. This is because touch pointers hover nothing on the frame they are released. The end effect is that these two events can be received sequentially after an [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") event (but always on the same frame as the [`Out`](../../prelude/struct.Out.html "struct bevy::prelude::Out") event).

Note: Though it is common for the [`PointerInput`](../pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") stream may contain multiple pointer movements and presses each frame, the hover state is determined only by the pointer’s _final position_. Since the hover state ultimately determines which entities receive events, this may mean that an entity can receive events from before or after it was actually hovered.