[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)

# Module common\_conditions 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#648)

A collection of [run conditions](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that may be useful in any bevy app.

## Functions

[any\_component\_removed](fn.any_component_removed.html "fn bevy::ecs::schedule::common_conditions::any_component_removed")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entity with a component of the given type removed.

[any\_match\_filter](fn.any_match_filter.html "fn bevy::ecs::schedule::common_conditions::any_match_filter")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities that match the given [`QueryFilter`](../../query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

[any\_with\_component](fn.any_with_component.html "fn bevy::ecs::schedule::common_conditions::any_with_component")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities with the given component type.

[condition\_changed](fn.condition_changed.html "fn bevy::ecs::schedule::common_conditions::condition_changed")

Generates a [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the passed one changes.

[condition\_changed\_to](fn.condition_changed_to.html "fn bevy::ecs::schedule::common_conditions::condition_changed_to")

Generates a [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the result of the passed one went from false to true since the last time this was called.

[not](fn.not.html "fn bevy::ecs::schedule::common_conditions::not")

Generates a [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that inverses the result of passed one.

[on\_message](fn.on_message.html "fn bevy::ecs::schedule::common_conditions::on_message")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any new messages of the given type since it was last called.

[resource\_added](fn.resource_added.html "fn bevy::ecs::schedule::common_conditions::resource_added")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added since the condition was last checked.

[resource\_changed](fn.resource_changed.html "fn bevy::ecs::schedule::common_conditions::resource_changed")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added or mutably dereferenced since the condition was last checked.

[resource\_changed\_or\_removed](fn.resource_changed_or_removed.html "fn bevy::ecs::schedule::common_conditions::resource_changed_or_removed")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added, removed or mutably dereferenced since the condition was last checked.

[resource\_equals](fn.resource_equals.html "fn bevy::ecs::schedule::common_conditions::resource_equals")

Generates a [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the resource is equal to `value`.

[resource\_exists](fn.resource_exists.html "fn bevy::ecs::schedule::common_conditions::resource_exists")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource exists.

[resource\_exists\_and\_changed](fn.resource_exists_and_changed.html "fn bevy::ecs::schedule::common_conditions::resource_exists_and_changed")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added or mutably dereferenced since the condition was last checked.

[resource\_exists\_and\_equals](fn.resource_exists_and_equals.html "fn bevy::ecs::schedule::common_conditions::resource_exists_and_equals")

Generates a [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the resource exists and is equal to `value`.

[resource\_removed](fn.resource_removed.html "fn bevy::ecs::schedule::common_conditions::resource_removed")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been removed since the condition was last checked.

[run\_once](fn.run_once.html "fn bevy::ecs::schedule::common_conditions::run_once")

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` on the first time the condition is run and false every time after.