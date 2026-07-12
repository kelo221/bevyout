[bevy](../index.html)

# Crate remote 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#1-1626)

An implementation of the Bevy Remote Protocol, to allow for remote control of a Bevy app.

Adding the [`RemotePlugin`](struct.RemotePlugin.html "struct bevy::remote::RemotePlugin") to your [`App`](../prelude/struct.App.html "struct bevy::prelude::App") will setup everything needed without starting any transports. To start accepting remote connections you will need to add a second plugin like the [`RemoteHttpPlugin`](http/struct.RemoteHttpPlugin.html "struct bevy::remote::http::RemoteHttpPlugin") to enable communication over HTTP. These _remote clients_ can inspect and alter the state of the entity-component system.

The Bevy Remote Protocol is based on the JSON-RPC 2.0 protocol.

### Request objects

A typical client request might look like this:

```json
{
    "method": "world.get_components",
    "id": 0,
    "params": {
        "entity": 4294967298,
        "components": [
            "bevy_transform::components::transform::Transform"
        ]
    }
}
```

The `id` and `method` fields are required. The `params` field may be omitted for certain methods:

*   `id` is arbitrary JSON data. The server completely ignores its contents, and the client may use it for any purpose. It will be copied via serialization and deserialization (so object property order, etc. can’t be relied upon to be identical) and sent back to the client as part of the response.
    
*   `method` is a string that specifies one of the possible [`BrpRequest`](struct.BrpRequest.html "struct bevy::remote::BrpRequest") variants: `world.query`, `world.get_components`, `world.insert_components`, etc. It’s case-sensitive.
    
*   `params` is parameter data specific to the request.
    

For more information, see the documentation for [`BrpRequest`](struct.BrpRequest.html "struct bevy::remote::BrpRequest"). [`BrpRequest`](struct.BrpRequest.html "struct bevy::remote::BrpRequest") is serialized to JSON via `serde`, so [the `serde` documentation](https://serde.rs/) may be useful to clarify the correspondence between the Rust structure and the JSON format.

### Response objects

A response from the server to the client might look like this:

```json
{
    "jsonrpc": "2.0",
    "id": 0,
    "result": {
        "bevy_transform::components::transform::Transform": {
            "rotation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0 },
            "scale": { "x": 1.0, "y": 1.0, "z": 1.0 },
            "translation": { "x": 0.0, "y": 0.5, "z": 0.0 }
        }
    }
}
```

The `id` field will always be present. The `result` field will be present if the request was successful. Otherwise, an `error` field will replace it.

*   `id` is the arbitrary JSON data that was sent as part of the request. It will be identical to the `id` data sent during the request, modulo serialization and deserialization. If there’s an error reading the `id` field, it will be `null`.
    
*   `result` will be present if the request succeeded and will contain the response specific to the request.
    
*   `error` will be present if the request failed and will contain an error object with more information about the cause of failure.
    

### Error objects

An error object might look like this:

```json
{
    "code": -32602,
    "message": "Missing \"entity\" field"
}
```

The `code` and `message` fields will always be present. There may also be a `data` field.

*   `code` is an integer representing the kind of an error that happened. Error codes documented in the [`error_codes`](error_codes/index.html "mod bevy::remote::error_codes") module.
    
*   `message` is a short, one-sentence human-readable description of the error.
    
*   `data` is an optional field of arbitrary type containing additional information about the error.
    

### Built-in methods

The Bevy Remote Protocol includes a number of built-in methods for accessing and modifying data in the ECS.

#### `world.get_components`

Retrieve the values of one or more components from an entity.

`params`:

*   `entity`: The ID of the entity whose components will be fetched.
*   `components`: An array of [fully-qualified type names](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of components to fetch.
*   `strict` (optional): A flag to enable strict mode which will fail if any one of the components is not present or can not be reflected. Defaults to false.

If `strict` is false:

`result`:

*   `components`: A map associating each type name to its value on the requested entity.
*   `errors`: A map associating each type name with an error if it was not on the entity or could not be reflected.

If `strict` is true:

`result`: A map associating each type name to its value on the requested entity.

#### `world.query`

Perform a query over components in the ECS, returning all matching entities and their associated component values.

All of the arrays that comprise this request are optional, and when they are not provided, they will be treated as if they were empty.

`params`:

*   `data`:
    *   `components` (optional): An array of [fully-qualified type names](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of components to fetch, see _below_ example for a query to list all the type names in **your** project.
    *   `option` (optional): An array of fully-qualified type names of components to fetch optionally. to fetch all reflectable components, you can pass in the string `"all"`.
    *   `has` (optional): An array of fully-qualified type names of components whose presence will be reported as boolean values.
*   `filter` (optional):
    *   `with` (optional): An array of fully-qualified type names of components that must be present on entities in order for them to be included in results.
    *   `without` (optional): An array of fully-qualified type names of components that must _not_ be present on entities in order for them to be included in results.
*   `strict` (optional): A flag to enable strict mode which will fail if any one of the components is not present or can not be reflected. Defaults to false.

`result`: An array, each of which is an object containing:

*   `entity`: The ID of a query-matching entity.
*   `components`: A map associating each type name from `components`/`option` to its value on the matching entity if the component is present.
*   `has`: A map associating each type name from `has` to a boolean value indicating whether or not the entity has that component. If `has` was empty or omitted, this key will be omitted in the response.

##### Example

To use the query API and retrieve Transform data for all entities that have a Transform use this query:

```json
{
    "jsonrpc": "2.0",
    "method": "world.query",
    "id": 0,
    "params": {
        "data": {
            "components": ["bevy_transform::components::transform::Transform"]
            "option": [],
            "has": []
        },
        "filter": {
          "with": [],
          "without": []
        },
        "strict": false
    }
}
```

To query all entities and all of their Reflectable components (and retrieve their values), you can pass in “all” for the option field:

```json
{
    "jsonrpc": "2.0",
    "method": "world.query",
    "id": 0,
    "params": {
        "data": {
            "components": []
            "option": "all",
            "has": []
        },
        "filter": {
           "with": [],
          "without": []
        },
        "strict": false
    }
}
```

This should return you something like the below (in a larger list):

```json
{
  "components": {
    "bevy_camera::Camera3d": {
      "depth_load_op": {
        "Clear": 0.0
      },
      "depth_texture_usages": 16,
    },
    "bevy_core_pipeline::tonemapping::DebandDither": "Enabled",
    "bevy_core_pipeline::tonemapping::Tonemapping": "TonyMcMapface",
    "bevy_light::cluster::ClusterConfig": {
      "FixedZ": {
     "dynamic_resizing": true,
        "total": 4096,
        "z_config": {
          "far_z_mode": "MaxClusterableObjectRange",
          "first_slice_depth": 5.0
        },
        "z_slices": 24
      }
    },
    "bevy_camera::Camera": {
      "clear_color": "Default",
      "is_active": true,
      "msaa_writeback": true,
      "order": 0,
      "sub_camera_view": null,
      "target": {
        "Window": "Primary"
      },
   "viewport": null
    },
    "bevy_camera::Projection": {
      "Perspective": {
        "aspect_ratio": 1.7777777910232544,
        "far": 1000.0,
        "fov": 0.7853981852531433,
        "near": 0.10000000149011612
      }
    },
    "bevy_camera::primitives::Frustum": {},
 "bevy_render::sync_world::RenderEntity": 4294967291,
    "bevy_render::sync_world::SyncToRenderWorld": {},
    "bevy_render::view::Msaa": "Sample4",
    "bevy_camera::visibility::InheritedVisibility": true,
    "bevy_camera::visibility::ViewVisibility": false,
    "bevy_camera::visibility::Visibility": "Inherited",
    "bevy_camera::visibility::VisibleEntities": {},
    "bevy_transform::components::global_transform::GlobalTransform": [
      0.9635179042816162,
      -3.725290298461914e-9,
      0.26764383912086487,
      0.11616238951683044,
      0.9009039402008056,
      -0.4181846082210541,
      -0.24112138152122495,
      0.4340185225009918,
      0.8680371046066284,
      -2.5,
      4.5,
      9.0
    ],
    "bevy_transform::components::transform::Transform": {
   "rotation": [
        -0.22055435180664065,
        -0.13167093694210052,
        -0.03006339818239212,
        0.9659786224365234
      ],
      "scale": [
        1.0,
        1.0,
        1.0
   ],
      "translation": [
        -2.5,
      4.5,
        9.0
      ]
    },
    "bevy_transform::components::transform::TransformTreeChanged": null
  },
  "entity": 4294967261
},
```

#### `world.spawn_entity`

Create a new entity with the provided components and return the resulting entity ID.

`params`:

*   `components`: A map associating each component’s [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") with its value.

`result`:

*   `entity`: The ID of the newly spawned entity.

#### `world.despawn_entity`

Despawn the entity with the given ID.

`params`:

*   `entity`: The ID of the entity to be despawned.

`result`: null.

#### `world.remove_components`

Delete one or more components from an entity.

`params`:

*   `entity`: The ID of the entity whose components should be removed.
*   `components`: An array of [fully-qualified type names](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of components to be removed.

`result`: null.

#### `world.insert_components`

Insert one or more components into an entity.

`params`:

*   `entity`: The ID of the entity to insert components into.
*   `components`: A map associating each component’s fully-qualified type name with its value.

`result`: null.

#### `world.mutate_components`

Mutate a field in a component.

`params`:

*   `entity`: The ID of the entity with the component to mutate.
*   `component`: The component’s [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path").
*   `path`: The path of the field within the component. See [`GetPath`](../prelude/trait.GetPath.html#syntax "trait bevy::prelude::GetPath") for more information on formatting this string.
*   `value`: The value to insert at `path`.

`result`: null.

#### `world.reparent_entities`

Assign a new parent to one or more entities.

`params`:

*   `entities`: An array of entity IDs of entities that will be made children of the `parent`.
*   `parent` (optional): The entity ID of the parent to which the child entities will be assigned. If excluded, the given entities will be removed from their parents.

`result`: null.

#### `world.list_components`

List all registered components or all components present on an entity.

When `params` is not provided, this lists all registered components. If `params` is provided, this lists only those components present on the provided entity.

`params` (optional):

*   `entity`: The ID of the entity whose components will be listed.

`result`: An array of fully-qualified type names of components.

#### `world.get_components+watch`

Watch the values of one or more components from an entity.

`params`:

*   `entity`: The ID of the entity whose components will be fetched.
*   `components`: An array of [fully-qualified type names](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of components to fetch.
*   `strict` (optional): A flag to enable strict mode which will fail if any one of the components is not present or can not be reflected. Defaults to false.

If `strict` is false:

`result`:

*   `components`: A map of components added or changed in the last tick associating each type name to its value on the requested entity.
*   `removed`: An array of fully-qualified type names of components removed from the entity in the last tick.
*   `errors`: A map associating each type name with an error if it was not on the entity or could not be reflected.

If `strict` is true:

`result`:

*   `components`: A map of components added or changed in the last tick associating each type name to its value on the requested entity.
*   `removed`: An array of fully-qualified type names of components removed from the entity in the last tick.

#### `world.list_components+watch`

Watch all components present on an entity.

When `params` is not provided, this lists all registered components. If `params` is provided, this lists only those components present on the provided entity.

`params`:

*   `entity`: The ID of the entity whose components will be listed.

`result`:

*   `added`: An array of fully-qualified type names of components added to the entity in the last tick.
*   `removed`: An array of fully-qualified type names of components removed from the entity in the last tick.

#### `world.get_resources`

Extract the value of a given resource from the world.

`params`:

*   `resource`: The [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of the resource to get.

`result`:

*   `value`: The value of the resource in the world.

#### `world.insert_resources`

Insert the given resource into the world with the given value.

`params`:

*   `resource`: The [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of the resource to insert.
*   `value`: The value of the resource to be inserted.

`result`: null.

#### `world.remove_resources`

Remove the given resource from the world.

`params`

*   `resource`: The [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of the resource to remove.

`result`: null.

#### `world.mutate_resources`

Mutate a field in a resource.

`params`:

*   `resource`: The [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of the resource to mutate.
*   `path`: The path of the field within the resource. See [`GetPath`](../prelude/trait.GetPath.html#syntax "trait bevy::prelude::GetPath") for more information on formatting this string.
*   `value`: The value to be inserted at `path`.

`result`: null.

#### `world.list_resources`

List all reflectable registered resource types. This method has no parameters.

`result`: An array of [fully-qualified type names](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of registered resource types.

#### `world.trigger_event`

Triggers an event.

`params`:

*   `event`: The [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") of the event to trigger.
*   `value`: The value of the event to trigger.

`result`: null.

#### `registry.schema`

Retrieve schema information about registered types in the Bevy app’s type registry.

`params` (optional):

*   `with_crates`: An array of crate names to include in the results. When empty or omitted, types from all crates will be included.
*   `without_crates`: An array of crate names to exclude from the results. When empty or omitted, no crates will be excluded.
*   `type_limit`: Additional type constraints:
    *   `with`: An array of [fully-qualified type names](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") that must be present for a type to be included
    *   `without`: An array of [fully-qualified type names](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") that must not be present for a type to be excluded

`result`: A map associating each type’s [fully-qualified type name](../prelude/trait.TypePath.html#tymethod.type_path "associated function bevy::prelude::TypePath::type_path") to a [`JsonSchemaBevyType`](schemas/json_schema/struct.JsonSchemaBevyType.html "struct bevy::remote::schemas::json_schema::JsonSchemaBevyType"). This contains schema information about that type, including field definitions, type information, reflect type information, and other metadata helpful for understanding the structure of the type.

#### `rpc.discover`

Discover available remote methods and server information. This follows the [`OpenRPC` specification for service discovery](https://spec.open-rpc.org/#service-discovery-method).

This method takes no parameters.

`result`: An `OpenRPC` document containing:

*   Information about all available remote methods
*   Server connection information (when using HTTP transport)
*   `OpenRPC` specification version

### Custom methods

In addition to the provided methods, the Bevy Remote Protocol can be extended to include custom methods. This is primarily done during the initialization of [`RemotePlugin`](struct.RemotePlugin.html "struct bevy::remote::RemotePlugin"), although the methods may also be extended at runtime using the [`RemoteMethods`](struct.RemoteMethods.html "struct bevy::remote::RemoteMethods") resource.

#### Example

[ⓘ](# "This example is not tested")

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            // `default` adds all of the built-in methods, while `with_method` extends them
            RemotePlugin::default()
                .with_method("super_user/cool_method", path::to::my::cool::handler)
                // ... more methods can be added by chaining `with_method`
        )
        .add_systems(
            // ... standard application setup
        )
        .run();
}
```

The handler is expected to be a system-convertible function which takes optional JSON parameters as input and returns a [`BrpResult`](type.BrpResult.html "type bevy::remote::BrpResult"). This means that it should have a type signature which looks something like this:

```rust
fn handler(In(params): In<Option<Value>>, world: &mut World) -> BrpResult {
    todo!()
}
```

Arbitrary system parameters can be used in conjunction with the optional `Value` input. The handler system will always run with exclusive `World` access.

## Modules

[builtin\_methods](builtin_methods/index.html "mod bevy::remote::builtin_methods")

Built-in verbs for the Bevy Remote Protocol.

[error\_codes](error_codes/index.html "mod bevy::remote::error_codes")

Error codes used by BRP.

[http](http/index.html "mod bevy::remote::http")`http` and non-`target_family=wasm`

The BRP transport using JSON-RPC over HTTP.

[schemas](schemas/index.html "mod bevy::remote::schemas")

Module with schemas used for various BRP endpoints

## Structs

[BrpError](struct.BrpError.html "struct bevy::remote::BrpError")

An error a request might return.

[BrpMessage](struct.BrpMessage.html "struct bevy::remote::BrpMessage")

A message from the Bevy Remote Protocol server thread to the main world.

[BrpReceiver](struct.BrpReceiver.html "struct bevy::remote::BrpReceiver")

A resource that receives messages sent by Bevy Remote Protocol clients.

[BrpRequest](struct.BrpRequest.html "struct bevy::remote::BrpRequest")

A single request from a Bevy Remote Protocol client to the server, serialized in JSON.

[BrpResponse](struct.BrpResponse.html "struct bevy::remote::BrpResponse")

A response according to BRP.

[BrpSender](struct.BrpSender.html "struct bevy::remote::BrpSender")

A resource holding the matching sender for the [`BrpReceiver`](struct.BrpReceiver.html "struct bevy::remote::BrpReceiver")’s receiver.

[RemoteLast](struct.RemoteLast.html "struct bevy::remote::RemoteLast")

Schedule that contains all systems to process Bevy Remote Protocol requests

[RemoteMethods](struct.RemoteMethods.html "struct bevy::remote::RemoteMethods")

Holds all implementations of methods known to the server.

[RemotePlugin](struct.RemotePlugin.html "struct bevy::remote::RemotePlugin")

Add this plugin to your [`App`](../prelude/struct.App.html "struct bevy::prelude::App") to allow remote connections to inspect and modify entities.

[RemoteWatchingRequests](struct.RemoteWatchingRequests.html "struct bevy::remote::RemoteWatchingRequests")

Holds the [`BrpMessage`](struct.BrpMessage.html "struct bevy::remote::BrpMessage")’s of all ongoing watching requests along with their handlers.

## Enums

[BrpBatch](enum.BrpBatch.html "enum bevy::remote::BrpBatch")

The requests may occur on their own or in batches. Actual parsing is deferred for the sake of proper error reporting.

[BrpPayload](enum.BrpPayload.html "enum bevy::remote::BrpPayload")

A result/error payload present in every response.

[RemoteMethodHandler](enum.RemoteMethodHandler.html "enum bevy::remote::RemoteMethodHandler")

A type to hold the allowed types of systems to be used as method handlers.

[RemoteMethodSystemId](enum.RemoteMethodSystemId.html "enum bevy::remote::RemoteMethodSystemId")

The [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") of a function that can be used as a remote method.

[RemoteSystems](enum.RemoteSystems.html "enum bevy::remote::RemoteSystems")

The systems sets of the [`RemoteLast`](struct.RemoteLast.html "struct bevy::remote::RemoteLast") schedule.

## Type Aliases

[BrpResult](type.BrpResult.html "type bevy::remote::BrpResult")

The result of a request.

[RemoteInstantMethodSystemId](type.RemoteInstantMethodSystemId.html "type bevy::remote::RemoteInstantMethodSystemId")

The [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") of a function that implements a remote instant method (`world.get_components`, `world.query`, etc.)

[RemoteWatchingMethodSystemId](type.RemoteWatchingMethodSystemId.html "type bevy::remote::RemoteWatchingMethodSystemId")

The [`SystemId`](../ecs/system/struct.SystemId.html "struct bevy::ecs::system::SystemId") of a function that implements a remote watching method (`world.get_components+watch`, `world.list_components+watch`, etc.)