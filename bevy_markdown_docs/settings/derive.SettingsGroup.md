[bevy](../index.html)::[settings](index.html)

# Derive Macro SettingsGroup 

[Source](https://docs.rs/bevy_ecs_macros/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs_macros/lib.rs.html#633)

```rust
#[derive(SettingsGroup)]
{
    // Attributes available to this derive:
    #[settings_group]
}
```

Cheat sheet for derive syntax,

### Group Override

[ⓘ](# "This example is not tested")

```rust
#[derive(SettingsGroup)]
#[settings_group(group = "my_group")]
struct MySettings {
    test: true
}
```

results in:

[ⓘ](# "This example is not tested")

```rust
[my_group]
test = true
```

### File Override

[ⓘ](# "This example is not tested")

```rust
#[derive(SettingsGroup)]
#[settings_group(file = "my_file")]
struct MySettings {
    test: true
}
```

results in a different file being used as the source of the settings.

### Key Override

Only valid for enums, as struct keys are always derived from the field name.

[ⓘ](# "This example is not tested")

```rust
#[derive(SettingsGroup)]
#[settings_group(key = "my_key")]
enum MySettingsEnum {
    Variant1,
    Variant2
};
```

results in:

[ⓘ](# "This example is not tested")

```rust
[my_settings_enum]
my_key = "variant1"
```