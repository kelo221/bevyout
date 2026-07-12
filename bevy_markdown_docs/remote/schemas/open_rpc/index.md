[bevy](../../../index.html)::[remote](../../index.html)::[schemas](../index.html)

# Module open\_rpc 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#14)

Module with trimmed down `OpenRPC` document structs. It tries to follow this standard: [https://spec.open-rpc.org](https://spec.open-rpc.org)

## Structs

[InfoObject](struct.InfoObject.html "struct bevy::remote::schemas::open_rpc::InfoObject")

Contains metadata information about the `OpenRPC` document.

[MethodObject](struct.MethodObject.html "struct bevy::remote::schemas::open_rpc::MethodObject")

Represents an RPC method in the `OpenRPC` document.

[OpenRpcDocument](struct.OpenRpcDocument.html "struct bevy::remote::schemas::open_rpc::OpenRpcDocument")

Represents an `OpenRPC` document as defined by the `OpenRPC` specification.

[Parameter](struct.Parameter.html "struct bevy::remote::schemas::open_rpc::Parameter")

Represents an RPC method parameter in the `OpenRPC` document.

[ServerObject](struct.ServerObject.html "struct bevy::remote::schemas::open_rpc::ServerObject")

Describes a server hosting the API as specified in the `OpenRPC` document.