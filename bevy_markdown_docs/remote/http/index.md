[bevy](../../index.html)::[remote](../index.html)

# Module http 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/lib.rs.html#561)

Available on **crate feature `http` and non-`target_family=wasm`** only.

The BRP transport using JSON-RPC over HTTP.

Adding the [`RemoteHttpPlugin`](struct.RemoteHttpPlugin.html "struct bevy::remote::http::RemoteHttpPlugin") to your [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") causes Bevy to accept connections over HTTP (by default, on port 15702) while your app is running.

When `bevy_render` is enabled, a second port is available to query the render subapp.

Clients are expected to `POST` JSON requests to the root URL; see the `client` example for a trivial example of use.

## Structs

[Headers](struct.Headers.html "struct bevy::remote::http::Headers")

A struct that holds a collection of HTTP headers.

[HostAddress](struct.HostAddress.html "struct bevy::remote::http::HostAddress")

A resource containing the IP address that Bevy will host on.

[HostPort](struct.HostPort.html "struct bevy::remote::http::HostPort")

A resource containing the port number that Bevy will listen on.

[RemoteHttpPlugin](struct.RemoteHttpPlugin.html "struct bevy::remote::http::RemoteHttpPlugin")

Add this plugin to your [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") to allow remote connections over HTTP to inspect and modify entities. It requires the [`RemotePlugin`](../struct.RemotePlugin.html "struct bevy::remote::RemotePlugin").

## Constants

[DEFAULT\_ADDR](constant.DEFAULT_ADDR.html "constant bevy::remote::http::DEFAULT_ADDR")

The default host address that Bevy will use for its server.

[DEFAULT\_PORT](constant.DEFAULT_PORT.html "constant bevy::remote::http::DEFAULT_PORT")

The default port that Bevy will listen on.

[DEFAULT\_RENDER\_PORT](constant.DEFAULT_RENDER_PORT.html "constant bevy::remote::http::DEFAULT_RENDER_PORT")

The default port that Bevy will listen on for the render subapp.