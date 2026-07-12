[bevy](../../../index.html)::[reflect](../../index.html)::[func](../index.html)

# Module signature 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/mod.rs.html#188)

Available on **crate feature `functions`** only.

Function signature types.

Function signatures differ from [`FunctionInfo`](../struct.FunctionInfo.html "struct bevy::reflect::func::FunctionInfo") and [`SignatureInfo`](../struct.SignatureInfo.html "struct bevy::reflect::func::SignatureInfo") in that they are only concerned about the types and order of the arguments and return type of a function.

The names of arguments do not matter, nor does any other information about the function such as its name or other attributes.

This makes signatures useful for comparing or hashing functions strictly based on their arguments and return type.

## Structs

[ArgumentSignature](struct.ArgumentSignature.html "struct bevy::reflect::func::signature::ArgumentSignature")

The argument-portion of a function signature.

[Signature](struct.Signature.html "struct bevy::reflect::func::signature::Signature")

The signature of a function.