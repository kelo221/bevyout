[bevy](../../index.html)::[remote](../index.html)::[builtin\_methods](index.html)

# Function parse\_some 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/builtin_methods.rs.html#586)

```rust
pub fn parse_some<T>(value: Option<Value>) -> Result<T, BrpError>where
    T: for<'de> Deserialize<'de>,
```

A helper function used to parse a `serde_json::Value` wrapped in an `Option`.