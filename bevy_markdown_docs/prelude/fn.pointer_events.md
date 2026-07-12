[bevy](../index.html)::[prelude](index.html)

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

*   [`Out`](struct.Out.html "struct bevy::prelude::Out") → [`Leave`](struct.Leave.html "struct bevy::prelude::Leave") → [`DragLeave`](struct.DragLeave.html "struct bevy::prelude::DragLeave").
*   [`DragEnter`](struct.DragEnter.html "struct bevy::prelude::DragEnter") → [`Enter`](struct.Enter.html "struct bevy::prelude::Enter") → [`Over`](struct.Over.html "struct bevy::prelude::Over").
*   Any number of any of the following:
    *   For each movement: [`DragStart`](struct.DragStart.html "struct bevy::prelude::DragStart") → [`Drag`](struct.Drag.html "struct bevy::prelude::Drag") → [`DragOver`](struct.DragOver.html "struct bevy::prelude::DragOver") → [`Move`](struct.Move.html "struct bevy::prelude::Move").
    *   For each button press: [`Press`](struct.Press.html "struct bevy::prelude::Press") or [`Click`](struct.Click.html "struct bevy::prelude::Click") → [`Release`](struct.Release.html "struct bevy::prelude::Release") → [`DragDrop`](struct.DragDrop.html "struct bevy::prelude::DragDrop") → [`DragEnd`](struct.DragEnd.html "struct bevy::prelude::DragEnd") → [`DragLeave`](struct.DragLeave.html "struct bevy::prelude::DragLeave").
    *   For each pointer cancellation: [`Cancel`](struct.Cancel.html "struct bevy::prelude::Cancel").

Additionally, across multiple frames, the following are also strictly ordered by the interaction state machine:

*   When a pointer moves over the target: [`Over`](struct.Over.html "struct bevy::prelude::Over"), [`Enter`](struct.Enter.html "struct bevy::prelude::Enter"), [`Move`](struct.Move.html "struct bevy::prelude::Move"), [`Leave`](struct.Leave.html "struct bevy::prelude::Leave"), [`Out`](struct.Out.html "struct bevy::prelude::Out").
*   When a pointer presses buttons on the target: [`Press`](struct.Press.html "struct bevy::prelude::Press"), [`Click`](struct.Click.html "struct bevy::prelude::Click"), [`Release`](struct.Release.html "struct bevy::prelude::Release").
*   When a pointer drags the target: [`DragStart`](struct.DragStart.html "struct bevy::prelude::DragStart"), [`Drag`](struct.Drag.html "struct bevy::prelude::Drag"), [`DragEnd`](struct.DragEnd.html "struct bevy::prelude::DragEnd").
*   When a pointer drags something over the target: [`DragEnter`](struct.DragEnter.html "struct bevy::prelude::DragEnter"), [`DragOver`](struct.DragOver.html "struct bevy::prelude::DragOver"), [`DragDrop`](struct.DragDrop.html "struct bevy::prelude::DragDrop"), [`DragLeave`](struct.DragLeave.html "struct bevy::prelude::DragLeave").
*   When a pointer is canceled: No other events will follow the [`Cancel`](struct.Cancel.html "struct bevy::prelude::Cancel") event for that pointer.

Four events – [`Over`](struct.Over.html "struct bevy::prelude::Over"), [`Enter`](struct.Enter.html "struct bevy::prelude::Enter"), [`Leave`](struct.Leave.html "struct bevy::prelude::Leave") and [`Out`](struct.Out.html "struct bevy::prelude::Out") – are driven only by the [`HoverMap`](../picking/hover/struct.HoverMap.html "struct bevy::picking::hover::HoverMap"). The rest rely on additional data from the [`PointerInput`](../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") event stream. To receive these events for a custom pointer, you must add [`PointerInput`](../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") events.

When the pointer goes from hovering entity A to entity B, entity A will receive [`Out`](struct.Out.html "struct bevy::prelude::Out") and [`Enter`](struct.Enter.html "struct bevy::prelude::Enter") and then entity B will receive [`Leave`](struct.Leave.html "struct bevy::prelude::Leave") and [`Over`](struct.Over.html "struct bevy::prelude::Over"). No entity will ever receive both an [`Over`](struct.Over.html "struct bevy::prelude::Over") and an [`Out`](struct.Out.html "struct bevy::prelude::Out") or an [`Enter`](struct.Enter.html "struct bevy::prelude::Enter") and a [`Leave`](struct.Leave.html "struct bevy::prelude::Leave") event during the same frame.

When we account for event bubbling, the two pairs of events, [`Out`](struct.Out.html "struct bevy::prelude::Out") [`Over`](struct.Over.html "struct bevy::prelude::Over") and [`Enter`](struct.Enter.html "struct bevy::prelude::Enter") [`Leave`](struct.Leave.html "struct bevy::prelude::Leave"), behave differently. When the hovering focus shifts between children, parent entities may receive redundant [`Out`](struct.Out.html "struct bevy::prelude::Out") → [`Over`](struct.Over.html "struct bevy::prelude::Over") pairs. In the case of [`Enter`](struct.Enter.html "struct bevy::prelude::Enter") → [`Leave`](struct.Leave.html "struct bevy::prelude::Leave"), shared parent entities will not receive [`Enter`](struct.Enter.html "struct bevy::prelude::Enter") or [`Leave`](struct.Leave.html "struct bevy::prelude::Leave").

Both [`Click`](struct.Click.html "struct bevy::prelude::Click") and [`Release`](struct.Release.html "struct bevy::prelude::Release") target the entity hovered in the _previous frame_, rather than the current frame. This is because touch pointers hover nothing on the frame they are released. The end effect is that these two events can be received sequentially after an [`Out`](struct.Out.html "struct bevy::prelude::Out") event (but always on the same frame as the [`Out`](struct.Out.html "struct bevy::prelude::Out") event).

Note: Though it is common for the [`PointerInput`](../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput") stream may contain multiple pointer movements and presses each frame, the hover state is determined only by the pointer’s _final position_. Since the hover state ultimately determines which entities receive events, this may mean that an entity can receive events from before or after it was actually hovered.