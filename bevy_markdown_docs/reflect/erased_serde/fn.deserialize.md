[bevy](../../index.html)::[reflect](../index.html)::[erased\_serde](index.html)

# Function deserialize 

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/de.rs.html#40-42)

```rust
pub fn deserialize<'de, T>(
    deserializer: &mut dyn Deserializer<'de>,
) -> Result<T, Error>where
    T: Deserialize<'de>,
```

Deserialize a value of type `T` from the given trait object.

```rust
use erased_serde::Deserializer;
use std::collections::BTreeMap as Map;

fn main() {
    static JSON: &'static [u8] = br#"{"A": 65, "B": 66}"#;
    static CBOR: &'static [u8] = &[162, 97, 65, 24, 65, 97, 66, 24, 66];

    // Construct some deserializers.
    let json = &mut serde_json::Deserializer::from_slice(JSON);
    let cbor = &mut serde_cbor::Deserializer::from_slice(CBOR);

    // The values in this map are boxed trait objects, which is not possible
    // with the normal serde::Deserializer because of object safety.
    let mut formats: Map<&str, Box<dyn Deserializer>> = Map::new();
    formats.insert("json", Box::new(<dyn Deserializer>::erase(json)));
    formats.insert("cbor", Box::new(<dyn Deserializer>::erase(cbor)));

    // Pick a Deserializer out of the formats map.
    let format = formats.get_mut("json").unwrap();

    let data: Map<String, usize> = erased_serde::deserialize(format).unwrap();

    println!("{}", data["A"] + data["B"]);
}
```