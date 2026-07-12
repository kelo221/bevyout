[bevy](../../index.html)::[remote](../index.html)

# Module builtin\_methods 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#559)

Built-in verbs for the Bevy Remote Protocol.

## Structs

[BrpDespawnEntityParams](struct.BrpDespawnEntityParams.html "struct bevy::remote::builtin_methods::BrpDespawnEntityParams")

`world.despawn_entity`: Given an ID, despawns the entity with that ID.

[BrpEventObservers](struct.BrpEventObservers.html "struct bevy::remote::builtin_methods::BrpEventObservers")

Stores observer state for `world.observe+watch` requests.

[BrpGetComponentsParams](struct.BrpGetComponentsParams.html "struct bevy::remote::builtin_methods::BrpGetComponentsParams")

`world.get_components`: Retrieves one or more components from the entity with the given ID.

[BrpGetResourcesParams](struct.BrpGetResourcesParams.html "struct bevy::remote::builtin_methods::BrpGetResourcesParams")

`world.get_resources`: Retrieves the value of a given resource.

[BrpGetResourcesResponse](struct.BrpGetResourcesResponse.html "struct bevy::remote::builtin_methods::BrpGetResourcesResponse")

The response to a `world.get_resources` request.

[BrpInsertComponentsParams](struct.BrpInsertComponentsParams.html "struct bevy::remote::builtin_methods::BrpInsertComponentsParams")

`world.insert_components`: Adds one or more components to an entity.

[BrpInsertResourcesParams](struct.BrpInsertResourcesParams.html "struct bevy::remote::builtin_methods::BrpInsertResourcesParams")

`world.insert_resources`: Inserts a resource into the world with a given value.

[BrpJsonSchemaQueryFilter](struct.BrpJsonSchemaQueryFilter.html "struct bevy::remote::builtin_methods::BrpJsonSchemaQueryFilter")

Constraints that can be placed on a query to include or exclude certain definitions.

[BrpListComponentsParams](struct.BrpListComponentsParams.html "struct bevy::remote::builtin_methods::BrpListComponentsParams")

`world.list_components`: Returns a list of all type names of registered components in the system (no params provided), or those on an entity (params provided).

[BrpListComponentsWatchingResponse](struct.BrpListComponentsWatchingResponse.html "struct bevy::remote::builtin_methods::BrpListComponentsWatchingResponse")

A single response from a `world.list_components+watch` request.

[BrpMutateComponentsParams](struct.BrpMutateComponentsParams.html "struct bevy::remote::builtin_methods::BrpMutateComponentsParams")

`world.mutate_components`:

[BrpMutateResourcesParams](struct.BrpMutateResourcesParams.html "struct bevy::remote::builtin_methods::BrpMutateResourcesParams")

`world.mutate_resources`:

[BrpObserveParams](struct.BrpObserveParams.html "struct bevy::remote::builtin_methods::BrpObserveParams")

`world.observe+watch`: Registers an observer for the given event type and streams event data back to the client each time the event is triggered.

[BrpQuery](struct.BrpQuery.html "struct bevy::remote::builtin_methods::BrpQuery")

Describes the data that is to be fetched in a query.

[BrpQueryFilter](struct.BrpQueryFilter.html "struct bevy::remote::builtin_methods::BrpQueryFilter")

Additional constraints that can be placed on a query to include or exclude certain entities.

[BrpQueryParams](struct.BrpQueryParams.html "struct bevy::remote::builtin_methods::BrpQueryParams")

`world.query`: Performs a query over components in the ECS, returning entities and component values that match.

[BrpQueryRow](struct.BrpQueryRow.html "struct bevy::remote::builtin_methods::BrpQueryRow")

One query match result: a single entity paired with the requested components.

[BrpRemoveComponentsParams](struct.BrpRemoveComponentsParams.html "struct bevy::remote::builtin_methods::BrpRemoveComponentsParams")

`world.remove_components`: Deletes one or more components from an entity.

[BrpRemoveResourcesParams](struct.BrpRemoveResourcesParams.html "struct bevy::remote::builtin_methods::BrpRemoveResourcesParams")

`world.remove_resources`: Removes the given resource from the world.

[BrpReparentEntitiesParams](struct.BrpReparentEntitiesParams.html "struct bevy::remote::builtin_methods::BrpReparentEntitiesParams")

`world.reparent_entities`: Assign a new parent to one or more entities.

[BrpScheduleGraphResponse](struct.BrpScheduleGraphResponse.html "struct bevy::remote::builtin_methods::BrpScheduleGraphResponse")

The response to a `schedule.graph` request.

[BrpScheduleListResponse](struct.BrpScheduleListResponse.html "struct bevy::remote::builtin_methods::BrpScheduleListResponse")

The response to a `schedule.list` request.

[BrpSpawnEntityParams](struct.BrpSpawnEntityParams.html "struct bevy::remote::builtin_methods::BrpSpawnEntityParams")

`world.spawn_entity`: Creates a new entity with the given components and responds with its ID.

[BrpSpawnEntityResponse](struct.BrpSpawnEntityResponse.html "struct bevy::remote::builtin_methods::BrpSpawnEntityResponse")

A response from the world to the client that specifies a single entity.

[BrpTriggerEventParams](struct.BrpTriggerEventParams.html "struct bevy::remote::builtin_methods::BrpTriggerEventParams")

`world.trigger_event`:

[BrpWriteMessageParams](struct.BrpWriteMessageParams.html "struct bevy::remote::builtin_methods::BrpWriteMessageParams")

`world.write_message`:

[JsonSchemaTypeLimit](struct.JsonSchemaTypeLimit.html "struct bevy::remote::builtin_methods::JsonSchemaTypeLimit")

Additional [`BrpJsonSchemaQueryFilter`](struct.BrpJsonSchemaQueryFilter.html "struct bevy::remote::builtin_methods::BrpJsonSchemaQueryFilter") constraints that can be placed on a query to include or exclude certain definitions.

## Enums

[BrpGetComponentsResponse](enum.BrpGetComponentsResponse.html "enum bevy::remote::builtin_methods::BrpGetComponentsResponse")

The response to a `world.get_components` request.

[BrpGetComponentsWatchingResponse](enum.BrpGetComponentsWatchingResponse.html "enum bevy::remote::builtin_methods::BrpGetComponentsWatchingResponse")

A single response from a `world.get_components+watch` request.

[ComponentSelector](enum.ComponentSelector.html "enum bevy::remote::builtin_methods::ComponentSelector")

A selector for components in a query.

## Constants

[BRP\_DESPAWN\_COMPONENTS\_METHOD](constant.BRP_DESPAWN_COMPONENTS_METHOD.html "constant bevy::remote::builtin_methods::BRP_DESPAWN_COMPONENTS_METHOD")

The method path for a `world.despawn_entity` request.

[BRP\_GET\_COMPONENTS\_AND\_WATCH\_METHOD](constant.BRP_GET_COMPONENTS_AND_WATCH_METHOD.html "constant bevy::remote::builtin_methods::BRP_GET_COMPONENTS_AND_WATCH_METHOD")

The method path for a `world.get_components+watch` request.

[BRP\_GET\_COMPONENTS\_METHOD](constant.BRP_GET_COMPONENTS_METHOD.html "constant bevy::remote::builtin_methods::BRP_GET_COMPONENTS_METHOD")

The method path for a `world.get_components` request.

[BRP\_GET\_RESOURCE\_METHOD](constant.BRP_GET_RESOURCE_METHOD.html "constant bevy::remote::builtin_methods::BRP_GET_RESOURCE_METHOD")

The method path for a `world.get_resources` request.

[BRP\_INSERT\_COMPONENTS\_METHOD](constant.BRP_INSERT_COMPONENTS_METHOD.html "constant bevy::remote::builtin_methods::BRP_INSERT_COMPONENTS_METHOD")

The method path for a `world.insert_components` request.

[BRP\_INSERT\_RESOURCE\_METHOD](constant.BRP_INSERT_RESOURCE_METHOD.html "constant bevy::remote::builtin_methods::BRP_INSERT_RESOURCE_METHOD")

The method path for a `world.insert_resources` request.

[BRP\_LIST\_COMPONENTS\_AND\_WATCH\_METHOD](constant.BRP_LIST_COMPONENTS_AND_WATCH_METHOD.html "constant bevy::remote::builtin_methods::BRP_LIST_COMPONENTS_AND_WATCH_METHOD")

The method path for a `world.list_components+watch` request.

[BRP\_LIST\_COMPONENTS\_METHOD](constant.BRP_LIST_COMPONENTS_METHOD.html "constant bevy::remote::builtin_methods::BRP_LIST_COMPONENTS_METHOD")

The method path for a `world.list_components` request.

[BRP\_LIST\_RESOURCES\_METHOD](constant.BRP_LIST_RESOURCES_METHOD.html "constant bevy::remote::builtin_methods::BRP_LIST_RESOURCES_METHOD")

The method path for a `world.list_resources` request.

[BRP\_MUTATE\_COMPONENTS\_METHOD](constant.BRP_MUTATE_COMPONENTS_METHOD.html "constant bevy::remote::builtin_methods::BRP_MUTATE_COMPONENTS_METHOD")

The method path for a `world.mutate_components` request.

[BRP\_MUTATE\_RESOURCE\_METHOD](constant.BRP_MUTATE_RESOURCE_METHOD.html "constant bevy::remote::builtin_methods::BRP_MUTATE_RESOURCE_METHOD")

The method path for a `world.mutate_resources` request.

[BRP\_OBSERVE\_METHOD](constant.BRP_OBSERVE_METHOD.html "constant bevy::remote::builtin_methods::BRP_OBSERVE_METHOD")

The method path for a `world.observe+watch` request.

[BRP\_QUERY\_METHOD](constant.BRP_QUERY_METHOD.html "constant bevy::remote::builtin_methods::BRP_QUERY_METHOD")

The method path for a `world.query` request.

[BRP\_REGISTRY\_SCHEMA\_METHOD](constant.BRP_REGISTRY_SCHEMA_METHOD.html "constant bevy::remote::builtin_methods::BRP_REGISTRY_SCHEMA_METHOD")

The method path for a `registry.schema` request.

[BRP\_REMOVE\_COMPONENTS\_METHOD](constant.BRP_REMOVE_COMPONENTS_METHOD.html "constant bevy::remote::builtin_methods::BRP_REMOVE_COMPONENTS_METHOD")

The method path for a `world.remove_components` request.

[BRP\_REMOVE\_RESOURCE\_METHOD](constant.BRP_REMOVE_RESOURCE_METHOD.html "constant bevy::remote::builtin_methods::BRP_REMOVE_RESOURCE_METHOD")

The method path for a `world.remove_resources` request.

[BRP\_REPARENT\_ENTITIES\_METHOD](constant.BRP_REPARENT_ENTITIES_METHOD.html "constant bevy::remote::builtin_methods::BRP_REPARENT_ENTITIES_METHOD")

The method path for a `world.reparent_entities` request.

[BRP\_SCHEDULE\_GRAPH](constant.BRP_SCHEDULE_GRAPH.html "constant bevy::remote::builtin_methods::BRP_SCHEDULE_GRAPH")

The method path for a `schedule.graph` request.

[BRP\_SCHEDULE\_LIST](constant.BRP_SCHEDULE_LIST.html "constant bevy::remote::builtin_methods::BRP_SCHEDULE_LIST")

The method path for a `schedule.list` request.

[BRP\_SPAWN\_ENTITY\_METHOD](constant.BRP_SPAWN_ENTITY_METHOD.html "constant bevy::remote::builtin_methods::BRP_SPAWN_ENTITY_METHOD")

The method path for a `world.spawn_entity` request.

[BRP\_TRIGGER\_EVENT\_METHOD](constant.BRP_TRIGGER_EVENT_METHOD.html "constant bevy::remote::builtin_methods::BRP_TRIGGER_EVENT_METHOD")

The method path for a `world.trigger_event` request.

[BRP\_WRITE\_MESSAGE\_METHOD](constant.BRP_WRITE_MESSAGE_METHOD.html "constant bevy::remote::builtin_methods::BRP_WRITE_MESSAGE_METHOD")

The method path for a `world.write_message` request.

[RPC\_DISCOVER\_METHOD](constant.RPC_DISCOVER_METHOD.html "constant bevy::remote::builtin_methods::RPC_DISCOVER_METHOD")

The method path for a `rpc.discover` request.

## Functions

[export\_registry\_types](fn.export_registry_types.html "fn bevy::remote::builtin_methods::export_registry_types")

Handles a `registry.schema` request (list all registry types in form of schema) coming from a client.

[parse](fn.parse.html "fn bevy::remote::builtin_methods::parse")

A helper function used to parse a `serde_json::Value`.

[parse\_some](fn.parse_some.html "fn bevy::remote::builtin_methods::parse_some")

A helper function used to parse a `serde_json::Value` wrapped in an `Option`.

[process\_remote\_despawn\_entity\_request](fn.process_remote_despawn_entity_request.html "fn bevy::remote::builtin_methods::process_remote_despawn_entity_request")

Handles a `world.despawn_entity` (despawn entity) request coming from a client.

[process\_remote\_get\_components\_request](fn.process_remote_get_components_request.html "fn bevy::remote::builtin_methods::process_remote_get_components_request")

Handles a `world.get_components` request coming from a client.

[process\_remote\_get\_components\_watching\_request](fn.process_remote_get_components_watching_request.html "fn bevy::remote::builtin_methods::process_remote_get_components_watching_request")

Handles a `world.get_components+watch` request coming from a client.

[process\_remote\_get\_resources\_request](fn.process_remote_get_resources_request.html "fn bevy::remote::builtin_methods::process_remote_get_resources_request")

Handles a `world.get_resources` request coming from a client.

[process\_remote\_insert\_components\_request](fn.process_remote_insert_components_request.html "fn bevy::remote::builtin_methods::process_remote_insert_components_request")

Handles a `world.insert_components` request (insert components) coming from a client.

[process\_remote\_insert\_resources\_request](fn.process_remote_insert_resources_request.html "fn bevy::remote::builtin_methods::process_remote_insert_resources_request")

Handles a `world.insert_resources` request coming from a client.

[process\_remote\_list\_components\_request](fn.process_remote_list_components_request.html "fn bevy::remote::builtin_methods::process_remote_list_components_request")

Handles a `world.list_components` request (list all components) coming from a client.

[process\_remote\_list\_components\_watching\_request](fn.process_remote_list_components_watching_request.html "fn bevy::remote::builtin_methods::process_remote_list_components_watching_request")

Handles a `world.list_components+watch` request coming from a client.

[process\_remote\_list\_methods\_request](fn.process_remote_list_methods_request.html "fn bevy::remote::builtin_methods::process_remote_list_methods_request")

Handles a `rpc.discover` request coming from a client.

[process\_remote\_list\_resources\_request](fn.process_remote_list_resources_request.html "fn bevy::remote::builtin_methods::process_remote_list_resources_request")

Handles a `world.list_resources` request coming from a client.

[process\_remote\_mutate\_components\_request](fn.process_remote_mutate_components_request.html "fn bevy::remote::builtin_methods::process_remote_mutate_components_request")

Handles a `world.mutate_components` request coming from a client.

[process\_remote\_mutate\_resources\_request](fn.process_remote_mutate_resources_request.html "fn bevy::remote::builtin_methods::process_remote_mutate_resources_request")

Handles a `world.mutate_resources` request coming from a client.

[process\_remote\_observe\_watching\_request](fn.process_remote_observe_watching_request.html "fn bevy::remote::builtin_methods::process_remote_observe_watching_request")

Handles a `world.observe+watch` request coming from a client.

[process\_remote\_query\_request](fn.process_remote_query_request.html "fn bevy::remote::builtin_methods::process_remote_query_request")

Handles a `world.query` request coming from a client.

[process\_remote\_remove\_components\_request](fn.process_remote_remove_components_request.html "fn bevy::remote::builtin_methods::process_remote_remove_components_request")

Handles a `world.remove_components` request (remove components) coming from a client.

[process\_remote\_remove\_resources\_request](fn.process_remote_remove_resources_request.html "fn bevy::remote::builtin_methods::process_remote_remove_resources_request")

Handles a `world.remove_resources` request coming from a client.

[process\_remote\_reparent\_entities\_request](fn.process_remote_reparent_entities_request.html "fn bevy::remote::builtin_methods::process_remote_reparent_entities_request")

Handles a `world.reparent_entities` request coming from a client.

[process\_remote\_spawn\_entity\_request](fn.process_remote_spawn_entity_request.html "fn bevy::remote::builtin_methods::process_remote_spawn_entity_request")

Handles a `world.spawn_entity` request coming from a client.

[process\_remote\_trigger\_event\_request](fn.process_remote_trigger_event_request.html "fn bevy::remote::builtin_methods::process_remote_trigger_event_request")

Handles a `world.trigger_event` request coming from a client.

[process\_remote\_write\_message\_request](fn.process_remote_write_message_request.html "fn bevy::remote::builtin_methods::process_remote_write_message_request")

Handles a `world.write_message` request coming from a client.

[schedule\_graph](fn.schedule_graph.html "fn bevy::remote::builtin_methods::schedule_graph")

Handles a `schedule.graph` request coming from a client.

[schedule\_list](fn.schedule_list.html "fn bevy::remote::builtin_methods::schedule_list")

Handles a `schedule.list` request coming from a client.

## Type Aliases

[BrpListComponentsResponse](type.BrpListComponentsResponse.html "type bevy::remote::builtin_methods::BrpListComponentsResponse")

The response to a `world.list_components` request.

[BrpListResourcesResponse](type.BrpListResourcesResponse.html "type bevy::remote::builtin_methods::BrpListResourcesResponse")

The response to a `world.list_resources` request.

[BrpQueryResponse](type.BrpQueryResponse.html "type bevy::remote::builtin_methods::BrpQueryResponse")

The response to a `world.query` request.