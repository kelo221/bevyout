[bevy](../index.html)::[reflect](index.html)

# Trait Reflect 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#421)

```rust
pub trait Reflect:
    PartialReflect
    + DynamicTyped
    + Any {
    // Required methods
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn as_any(&self) -> &(dyn Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static);
    fn into_reflect(self: Box<Self>) -> Box<dyn Reflect>;
    fn as_reflect(&self) -> &(dyn Reflect + 'static);
    fn as_reflect_mut(&mut self) -> &mut (dyn Reflect + 'static);
    fn set(&mut self, value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>>;
}
```

A core trait of [`bevy_reflect`](index.html "mod bevy::reflect"), used for downcasting to concrete types.

This is a subtrait of [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), meaning any type which implements `Reflect` implements `PartialReflect` by definition.

It’s recommended to use [the derive macro](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect") rather than manually implementing this trait. Doing so will automatically implement this trait, [`PartialReflect`](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"), and many other useful traits for reflection, including one of the appropriate subtraits: [`Struct`](../prelude/trait.Struct.html "trait bevy::prelude::Struct"), [`TupleStruct`](../prelude/trait.TupleStruct.html "trait bevy::prelude::TupleStruct") or [`Enum`](enums/trait.Enum.html "trait bevy::reflect::enums::Enum").

If you need to use this trait as a generic bound along with other reflection traits, for your convenience, consider using [`Reflectable`](trait.Reflectable.html "trait bevy::reflect::Reflectable") instead.

See the [crate-level documentation](index.html "mod bevy::reflect") to see how this trait can be used.

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#425)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<Self>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Returns the value as a [`Box<dyn Any>`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any").

For remote wrapper types, this will return the remote type instead.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#430)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any").

For remote wrapper types, this will return the remote type instead.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#435)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the value as a [`&mut dyn Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any").

For remote wrapper types, this will return the remote type instead.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#438)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<Self>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

Casts this type to a boxed, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#441)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#444)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

Casts this type to a mutable, fully-reflected value.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#450)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

Performs a type-checked assignment of a reflected value to this value.

If `value` does not contain a value of type `T`, returns an `Err` containing the trait object.

## Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#528)

### impl dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#534)

#### pub fn [downcast](#method.downcast)<T>(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<T>, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Downcasts the value to type `T`, consuming the trait object.

If the underlying value is not of type `T`, returns `Err(self)`.

For remote types, `T` should be the type itself rather than the wrapper type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#547)

#### pub fn [take](#method.take)<T>(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Downcasts the value to type `T`, unboxing and consuming the trait object.

If the underlying value is not of type `T`, returns `Err(self)`.

For remote types, `T` should be the type itself rather than the wrapper type.

##### [Examples found in repository](#scraped-examples)[?](../../scrape-examples-help.html)

examples/reflection/reflection\_types.rs ([line 140](../../src/reflection_types/reflection_types.rs.html#140))

```rust
67fn setup() {
68    let mut z = <HashMap<_, _>>::default();
69    z.insert("Hello".to_string(), 1.0);
70    let value: Box<dyn Reflect> = Box::new(A {
71        x: 1,
72        y: vec![1, 2],
73        z,
74    });
75
76    // There are a number of different "reflect traits", which each expose different operations on
77    // the underlying type
78    match value.reflect_ref() {
79        // `Struct` is a trait automatically implemented for structs that derive Reflect. This trait
80        // allows you to interact with fields via their string names or indices
81        ReflectRef::Struct(value) => {
82            info!(
83                "This is a 'struct' type with an 'x' value of {}",
84                value.get_field::<usize>("x").unwrap()
85            );
86        }
87        // `TupleStruct` is a trait automatically implemented for tuple structs that derive Reflect.
88        // This trait allows you to interact with fields via their indices
89        ReflectRef::TupleStruct(_) => {}
90        // `Tuple` is a special trait that can be manually implemented (instead of deriving
91        // Reflect). This exposes "tuple" operations on your type, allowing you to interact
92        // with fields via their indices. Tuple is automatically implemented for tuples of
93        // arity 12 or less.
94        ReflectRef::Tuple(_) => {}
95        // `Enum` is a trait automatically implemented for enums that derive Reflect. This trait allows you
96        // to interact with the current variant and its fields (if it has any)
97        ReflectRef::Enum(_) => {}
98        // `List` is a special trait that can be manually implemented (instead of deriving Reflect).
99        // This exposes "list" operations on your type, such as insertion. `List` is automatically
100        // implemented for relevant core types like Vec<T>.
101        ReflectRef::List(_) => {}
102        // `Array` is a special trait that can be manually implemented (instead of deriving Reflect).
103        // This exposes "array" operations on your type, such as indexing. `Array`
104        // is automatically implemented for relevant core types like [T; N].
105        ReflectRef::Array(_) => {}
106        // `Map` is a special trait that can be manually implemented (instead of deriving Reflect).
107        // This exposes "map" operations on your type, such as getting / inserting by key.
108        // Map is automatically implemented for relevant core types like HashMap<K, V>
109        ReflectRef::Map(_) => {}
110        // `Set` is a special trait that can be manually implemented (instead of deriving Reflect).
111        // This exposes "set" operations on your type, such as getting / inserting by value.
112        // Set is automatically implemented for relevant core types like HashSet<T>
113        ReflectRef::Set(_) => {}
114        // `Function` is a special trait that can be manually implemented (instead of deriving Reflect).
115        // This exposes "function" operations on your type, such as calling it with arguments.
116        // This trait is automatically implemented for types like DynamicFunction.
117        // This variant only exists if the `reflect_functions` feature is enabled.
118        #[cfg(feature = "reflect_functions")]
119        ReflectRef::Function(_) => {}
120        // `Opaque` types do not implement any of the other traits above. They are simply a Reflect
121        // implementation. Opaque is implemented for opaque types like String and Instant,
122        // but also include primitive types like i32, usize, and f32 (despite not technically being opaque).
123        ReflectRef::Opaque(_) => {}
124        #[expect(
125            clippy::allow_attributes,
126            reason = "`unreachable_patterns` is not always linted"
127        )]
128        #[allow(
129            unreachable_patterns,
130            reason = "This example cannot always detect when `bevy_reflect/functions` is enabled."
131        )]
132        _ => {}
133    }
134
135    let mut dynamic_list = DynamicList::default();
136    dynamic_list.push(3u32);
137    dynamic_list.push(4u32);
138    dynamic_list.push(5u32);
139
140    let mut value: A = value.take::<A>().unwrap();
141    value.y.apply(&dynamic_list);
142    assert_eq!(value.y, vec![3u32, 4u32, 5u32]);
143
144    // reference types defined above that are only used to demonstrate reflect
145    // derive functionality:
146    _ = || -> (A, B, C, D, E, F) { unreachable!() };
147}
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#564)

#### pub fn [is](#method.is)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Returns `true` if the underlying value is of type `T`, or `false` otherwise.

The underlying value is the concrete type that is stored in this `dyn` object; it can be downcast to. In the case that this underlying value “represents” a different type, like the Dynamic\*\*\* types do, you can call `represents` to determine what type they represent. Represented types cannot be downcast to, but you can use [`FromReflect`](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") to create a value of the represented type from them.

For remote types, `T` should be the type itself rather than the wrapper type.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#574)

#### pub fn [downcast\_ref](#method.downcast_ref)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Downcasts the value to type `T` by reference.

If the underlying value is not of type `T`, returns `None`.

For remote types, `T` should be the type itself rather than the wrapper type.

##### [Examples found in repository](#scraped-examples-1)[?](../../scrape-examples-help.html)

examples/reflection/custom\_attributes.rs ([line 63](../../src/custom_attributes/custom_attributes.rs.html#63))

```rust
6fn main() {
7    // Bevy supports statically registering custom attribute data on reflected types,
8    // which can then be accessed at runtime via the type's `TypeInfo`.
9    // Attributes are registered using the `#[reflect(@...)]` syntax,
10    // where the `...` is any expression that resolves to a value which implements `Reflect`.
11    // Note that these attributes are stored based on their type:
12    // if two attributes have the same type, the second one will overwrite the first.
13
14    // Here is an example of registering custom attributes on a type:
15    #[derive(Reflect)]
16    struct Slider {
17        #[reflect(@RangeInclusive::<f32>::new(0.0, 1.0))]
18        // Alternatively, we could have used the `0.0..=1.0` syntax,
19        // but remember to ensure the type is the one you want!
20        #[reflect(@0.0..=1.0_f32)]
21        value: f32,
22    }
23
24    // Now, we can access the custom attributes at runtime:
25    let TypeInfo::Struct(type_info) = Slider::type_info() else {
26        panic!("expected struct");
27    };
28
29    let field = type_info.field("value").unwrap();
30
31    let range = field.get_attribute::<RangeInclusive<f32>>().unwrap();
32    assert_eq!(*range, 0.0..=1.0);
33
34    // And remember that our attributes can be any type that implements `Reflect`:
35    #[derive(Reflect)]
36    struct Required;
37
38    #[derive(Reflect, PartialEq, Debug)]
39    struct Tooltip(String);
40
41    impl Tooltip {
42        fn new(text: &str) -> Self {
43            Self(text.to_string())
44        }
45    }
46
47    #[derive(Reflect)]
48    #[reflect(@Required, @Tooltip::new("An ID is required!"))]
49    struct Id(u8);
50
51    let TypeInfo::TupleStruct(type_info) = Id::type_info() else {
52        panic!("expected struct");
53    };
54
55    // We can check if an attribute simply exists on our type:
56    assert!(type_info.has_attribute::<Required>());
57
58    // We can also get attribute data dynamically:
59    let some_type_id = TypeId::of::<Tooltip>();
60
61    let tooltip: &dyn Reflect = type_info.get_attribute_by_id(some_type_id).unwrap();
62    assert_eq!(
63        tooltip.downcast_ref::<Tooltip>(),
64        Some(&Tooltip::new("An ID is required!"))
65    );
66
67    // And again, attributes of the same type will overwrite each other:
68    #[derive(Reflect)]
69    enum Status {
70        // This will result in `false` being stored:
71        #[reflect(@true)]
72        #[reflect(@false)]
73        Disabled,
74        // This will result in `true` being stored:
75        #[reflect(@false)]
76        #[reflect(@true)]
77        Enabled,
78    }
79
80    let TypeInfo::Enum(type_info) = Status::type_info() else {
81        panic!("expected enum");
82    };
83
84    let disabled = type_info.variant("Disabled").unwrap();
85    assert!(!disabled.get_attribute::<bool>().unwrap());
86
87    let enabled = type_info.variant("Enabled").unwrap();
88    assert!(enabled.get_attribute::<bool>().unwrap());
89}
```

Hide additional examples

examples/reflection/reflection.rs ([line 75](../../src/reflection/reflection.rs.html#75))

```rust
55fn setup(type_registry: Res<AppTypeRegistry>) {
56    let mut value = Foo {
57        a: 1,
58        _ignored: NonReflectedValue { _a: 10 },
59        nested: Bar { b: 8 },
60    };
61
62    // You can set field values like this. The type must match exactly or this will fail.
63    *value.get_field_mut("a").unwrap() = 2usize;
64    assert_eq!(value.a, 2);
65    assert_eq!(*value.get_field::<usize>("a").unwrap(), 2);
66
67    // You can also get the `&dyn PartialReflect` value of a field like this
68    let field = value.field("a").unwrap();
69
70    // But values introspected via `PartialReflect` will not return `dyn Reflect` trait objects
71    // (even if the containing type does implement `Reflect`), so we need to convert them:
72    let fully_reflected_field = field.try_as_reflect().unwrap();
73
74    // Now, you can downcast your `Reflect` value like this:
75    assert_eq!(*fully_reflected_field.downcast_ref::<usize>().unwrap(), 2);
76
77    // For this specific case, we also support the shortcut `try_downcast_ref`:
78    assert_eq!(*field.try_downcast_ref::<usize>().unwrap(), 2);
79
80    // `DynamicStruct` also implements the `Struct` and `Reflect` traits.
81    let mut patch = DynamicStruct::default();
82    patch.insert("a", 4usize);
83
84    // You can "apply" Reflect implementations on top of other Reflect implementations.
85    // This will only set fields with the same name, and it will fail if the types don't match.
86    // You can use this to "patch" your types with new values.
87    value.apply(&patch);
88    assert_eq!(value.a, 4);
89
90    let type_registry = type_registry.read();
91    // By default, all derived `Reflect` types can be Serialized using serde. No need to derive
92    // Serialize!
93    let serializer = ReflectSerializer::new(&value, &type_registry);
94    let ron_string =
95        ron::ser::to_string_pretty(&serializer, ron::ser::PrettyConfig::default()).unwrap();
96    info!("{}\n", ron_string);
97
98    // Dynamic properties can be deserialized
99    let reflect_deserializer = ReflectDeserializer::new(&type_registry);
100    let mut deserializer = ron::de::Deserializer::from_str(&ron_string).unwrap();
101    let reflect_value = reflect_deserializer.deserialize(&mut deserializer).unwrap();
102
103    // Deserializing returns a `Box<dyn PartialReflect>` value.
104    // Generally, deserializing a value will return the "dynamic" variant of a type.
105    // For example, deserializing a struct will return the DynamicStruct type.
106    // "Opaque types" will be deserialized as themselves.
107    assert_eq!(
108        reflect_value.reflect_type_path(),
109        DynamicStruct::type_path(),
110    );
111
112    // Reflect has its own `partial_eq` implementation, named `reflect_partial_eq`. This behaves
113    // like normal `partial_eq`, but it treats "dynamic" and "non-dynamic" types the same. The
114    // `Foo` struct and deserialized `DynamicStruct` are considered equal for this reason:
115    assert!(reflect_value.reflect_partial_eq(&value).unwrap());
116
117    // By "patching" `Foo` with the deserialized DynamicStruct, we can "Deserialize" Foo.
118    // This means we can serialize and deserialize with a single `Reflect` derive!
119    value.apply(&*reflect_value);
120}
```

examples/reflection/dynamic\_types.rs ([line 51](../../src/dynamic_types/dynamic_types.rs.html#51))

```rust
20fn main() {
21    #[derive(Reflect, Default, PartialEq, Debug)]
22    #[reflect(Identifiable, Default)]
23    struct Player {
24        id: u32,
25    }
26
27    #[reflect_trait]
28    trait Identifiable {
29        fn id(&self) -> u32;
30    }
31
32    impl Identifiable for Player {
33        fn id(&self) -> u32 {
34            self.id
35        }
36    }
37
38    // Normally, when instantiating a type, you get back exactly that type.
39    // This is because the type is known at compile time.
40    // We call this the "concrete" or "canonical" type.
41    let player: Player = Player { id: 123 };
42
43    // When working with reflected types, however, we often "erase" this type information
44    // using the `Reflect` trait object.
45    // This trait object also gives us access to all the methods in the `PartialReflect` trait too.
46    // The underlying type is still the same (in this case, `Player`),
47    // but now we've hidden that information from the compiler.
48    let reflected: Box<dyn Reflect> = Box::new(player);
49
50    // Because it's the same type under the hood, we can still downcast it back to the original type.
51    assert!(reflected.downcast_ref::<Player>().is_some());
52
53    // We can attempt to clone our value using `PartialReflect::reflect_clone`.
54    // This will recursively call `PartialReflect::reflect_clone` on all fields of the type.
55    // Or, if we had registered `ReflectClone` using `#[reflect(Clone)]`, it would simply call `Clone::clone` directly.
56    let cloned: Box<dyn Reflect> = reflected.reflect_clone().unwrap();
57    assert_eq!(cloned.downcast_ref::<Player>(), Some(&Player { id: 123 }));
58
59    // Another way we can "clone" our data is by converting it to a dynamic type.
60    // Notice here we bind it as a `dyn PartialReflect` instead of `dyn Reflect`.
61    // This is because it returns a dynamic type that simply represents the original type.
62    // In this case, because `Player` is a struct, it will return a `DynamicStruct`.
63    let dynamic: Box<dyn PartialReflect> = reflected.to_dynamic();
64    assert!(dynamic.is_dynamic());
65
66    // And if we try to convert it back to a `dyn Reflect` trait object, we'll get `None`.
67    // Dynamic types cannot be directly cast to `dyn Reflect` trait objects.
68    assert!(dynamic.try_as_reflect().is_none());
69
70    // Generally dynamic types are used to represent (or "proxy") the original type,
71    // so that we can continue to access its fields and overall structure.
72    let dynamic_ref = dynamic.reflect_ref().as_struct().unwrap();
73    let id = dynamic_ref.field("id").unwrap().try_downcast_ref::<u32>();
74    assert_eq!(id, Some(&123));
75
76    // It also enables us to create a representation of a type without having compile-time
77    // access to the actual type. This is how the reflection deserializers work.
78    // They generally can't know how to construct a type ahead of time,
79    // so they instead build and return these dynamic representations.
80    let input = "(id: 123)";
81    let mut registry = TypeRegistry::default();
82    registry.register::<Player>();
83    let registration = registry.get(std::any::TypeId::of::<Player>()).unwrap();
84    let deserialized = TypedReflectDeserializer::new(registration, &registry)
85        .deserialize(&mut ron::Deserializer::from_str(input).unwrap())
86        .unwrap();
87
88    // Our deserialized output is a `DynamicStruct` that proxies/represents a `Player`.
89    assert!(deserialized.represents::<Player>());
90
91    // And while this does allow us to access the fields and structure of the type,
92    // there may be instances where we need the actual type.
93    // For example, if we want to convert our `dyn Reflect` into a `dyn Identifiable`,
94    // we can't use the `DynamicStruct` proxy.
95    let reflect_identifiable = registration
96        .data::<ReflectIdentifiable>()
97        .expect("`ReflectIdentifiable` should be registered");
98
99    // Trying to access the registry with our `deserialized` will give a compile error
100    // since it doesn't implement `Reflect`, only `PartialReflect`.
101    // Similarly, trying to force the operation will fail.
102    // This fails since the underlying type of `deserialized` is `DynamicStruct` and not `Player`.
103    assert!(deserialized
104        .try_as_reflect()
105        .and_then(|reflect_trait_obj| reflect_identifiable.get(reflect_trait_obj))
106        .is_none());
107
108    // So how can we go from a dynamic type to a concrete type?
109    // There are two ways:
110
111    // 1. Using `PartialReflect::apply`.
112    {
113        // If you know the type at compile time, you can construct a new value and apply the dynamic
114        // value to it.
115        let mut value = Player::default();
116        value.apply(deserialized.as_ref());
117        assert_eq!(value.id, 123);
118
119        // If you don't know the type at compile time, you need a dynamic way of constructing
120        // an instance of the type. One such way is to use the `ReflectDefault` type data.
121        let reflect_default = registration
122            .data::<ReflectDefault>()
123            .expect("`ReflectDefault` should be registered");
124
125        let mut value: Box<dyn Reflect> = reflect_default.default();
126        value.apply(deserialized.as_ref());
127
128        let identifiable: &dyn Identifiable = reflect_identifiable.get(value.as_reflect()).unwrap();
129        assert_eq!(identifiable.id(), 123);
130    }
131
132    // 2. Using `FromReflect`
133    {
134        // If you know the type at compile time, you can use the `FromReflect` trait to convert the
135        // dynamic value into the concrete type directly.
136        let value: Player = Player::from_reflect(deserialized.as_ref()).unwrap();
137        assert_eq!(value.id, 123);
138
139        // If you don't know the type at compile time, you can use the `ReflectFromReflect` type data
140        // to perform the conversion dynamically.
141        let reflect_from_reflect = registration
142            .data::<ReflectFromReflect>()
143            .expect("`ReflectFromReflect` should be registered");
144
145        let value: Box<dyn Reflect> = reflect_from_reflect
146            .from_reflect(deserialized.as_ref())
147            .unwrap();
148        let identifiable: &dyn Identifiable = reflect_identifiable.get(value.as_reflect()).unwrap();
149        assert_eq!(identifiable.id(), 123);
150    }
151
152    // Lastly, while dynamic types are commonly generated via reflection methods like
153    // `PartialReflect::to_dynamic` or via the reflection deserializers,
154    // you can also construct them manually.
155    let mut my_dynamic_list = DynamicList::from_iter([1u32, 2u32, 3u32]);
156
157    // This is useful when you just need to apply some subset of changes to a type.
158    let mut my_list: Vec<u32> = Vec::new();
159    my_list.apply(&my_dynamic_list);
160    assert_eq!(my_list, vec![1, 2, 3]);
161
162    // And if you want it to actually proxy a type, you can configure it to do that as well:
163    assert!(!my_dynamic_list
164        .as_partial_reflect()
165        .represents::<Vec<u32>>());
166    my_dynamic_list.set_represented_type(Some(<Vec<u32>>::type_info()));
167    assert!(my_dynamic_list
168        .as_partial_reflect()
169        .represents::<Vec<u32>>());
170
171    // ============================= REFERENCE ============================= //
172    // For reference, here are all the available dynamic types:
173
174    // 1. `DynamicTuple`
175    {
176        let mut dynamic_tuple = DynamicTuple::default();
177        dynamic_tuple.insert(1u32);
178        dynamic_tuple.insert(2u32);
179        dynamic_tuple.insert(3u32);
180
181        let mut my_tuple: (u32, u32, u32) = (0, 0, 0);
182        my_tuple.apply(&dynamic_tuple);
183        assert_eq!(my_tuple, (1, 2, 3));
184    }
185
186    // 2. `DynamicArray`
187    {
188        let dynamic_array = DynamicArray::from_iter([1u32, 2u32, 3u32]);
189
190        let mut my_array = [0u32; 3];
191        my_array.apply(&dynamic_array);
192        assert_eq!(my_array, [1, 2, 3]);
193    }
194
195    // 3. `DynamicList`
196    {
197        let dynamic_list = DynamicList::from_iter([1u32, 2u32, 3u32]);
198
199        let mut my_list: Vec<u32> = Vec::new();
200        my_list.apply(&dynamic_list);
201        assert_eq!(my_list, vec![1, 2, 3]);
202    }
203
204    // 4. `DynamicSet`
205    {
206        let mut dynamic_set = DynamicSet::from_iter(["x", "y", "z"]);
207        assert!(dynamic_set.contains(&"x"));
208
209        dynamic_set.remove(&"y");
210
211        let mut my_set: HashSet<&str> = HashSet::default();
212        my_set.apply(&dynamic_set);
213        assert_eq!(my_set, HashSet::from_iter(["x", "z"]));
214    }
215
216    // 5. `DynamicMap`
217    {
218        let dynamic_map = DynamicMap::from_iter([("x", 1u32), ("y", 2u32), ("z", 3u32)]);
219
220        let mut my_map: HashMap<&str, u32> = HashMap::default();
221        my_map.apply(&dynamic_map);
222        assert_eq!(my_map.get("x"), Some(&1));
223        assert_eq!(my_map.get("y"), Some(&2));
224        assert_eq!(my_map.get("z"), Some(&3));
225    }
226
227    // 6. `DynamicStruct`
228    {
229        #[derive(Reflect, Default, Debug, PartialEq)]
230        struct MyStruct {
231            x: u32,
232            y: u32,
233            z: u32,
234        }
235
236        let mut dynamic_struct = DynamicStruct::default();
237        dynamic_struct.insert("x", 1u32);
238        dynamic_struct.insert("y", 2u32);
239        dynamic_struct.insert("z", 3u32);
240
241        let mut my_struct = MyStruct::default();
242        my_struct.apply(&dynamic_struct);
243        assert_eq!(my_struct, MyStruct { x: 1, y: 2, z: 3 });
244    }
245
246    // 7. `DynamicTupleStruct`
247    {
248        #[derive(Reflect, Default, Debug, PartialEq)]
249        struct MyTupleStruct(u32, u32, u32);
250
251        let mut dynamic_tuple_struct = DynamicTupleStruct::default();
252        dynamic_tuple_struct.insert(1u32);
253        dynamic_tuple_struct.insert(2u32);
254        dynamic_tuple_struct.insert(3u32);
255
256        let mut my_tuple_struct = MyTupleStruct::default();
257        my_tuple_struct.apply(&dynamic_tuple_struct);
258        assert_eq!(my_tuple_struct, MyTupleStruct(1, 2, 3));
259    }
260
261    // 8. `DynamicEnum`
262    {
263        #[derive(Reflect, Default, Debug, PartialEq)]
264        enum MyEnum {
265            #[default]
266            Empty,
267            Xyz(u32, u32, u32),
268        }
269
270        let mut values = DynamicTuple::default();
271        values.insert(1u32);
272        values.insert(2u32);
273        values.insert(3u32);
274
275        let dynamic_variant = DynamicVariant::Tuple(values);
276        let dynamic_enum = DynamicEnum::new("Xyz", dynamic_variant);
277
278        let mut my_enum = MyEnum::default();
279        my_enum.apply(&dynamic_enum);
280        assert_eq!(my_enum, MyEnum::Xyz(1, 2, 3));
281    }
282}
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#584)

#### pub fn [downcast\_mut](#method.downcast_mut)<T>(&mut self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Downcasts the value to type `T` by mutable reference.

If the underlying value is not of type `T`, returns `None`.

For remote types, `T` should be the type itself rather than the wrapper type.

## Trait Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#589)

### impl [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#590)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#604)

### impl [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") for dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#605)

#### fn [type\_path](../prelude/trait.TypePath.html#tymethod.type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns the fully qualified path of the underlying type. [Read more](../prelude/trait.TypePath.html#tymethod.type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#609)

#### fn [short\_type\_path](../prelude/trait.TypePath.html#tymethod.short_type_path)() -> &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

Returns a short, pretty-print enabled path to the type. [Read more](../prelude/trait.TypePath.html#tymethod.short_type_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#108)

#### fn [type\_ident](../prelude/trait.TypePath.html#method.type_ident)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the type, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.type_ident)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#117)

#### fn [crate\_name](../prelude/trait.TypePath.html#method.crate_name)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the name of the crate the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.crate_name)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/type_path.rs.html#126)

#### fn [module\_path](../prelude/trait.TypePath.html#method.module_path)() -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Returns the path to the module the type is in, or [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") if it is [anonymous](../prelude/trait.TypePath.html#anonymity "trait bevy::prelude::TypePath"). [Read more](../prelude/trait.TypePath.html#method.module_path)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#595)

### impl [Typed](trait.Typed.html "trait bevy::reflect::Typed") for dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#596)

#### fn [type\_info](trait.Typed.html#tymethod.type_info)() -> &'static [TypeInfo](enum.TypeInfo.html "enum bevy::reflect::TypeInfo")

Returns the compile-time [info](enum.TypeInfo.html "enum bevy::reflect::TypeInfo") for the underlying type.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#112)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for &'static [Location](https://doc.rust-lang.org/nightly/core/panic/location/struct.Location.html "struct core::panic::location::Location")<'static>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#113)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<&'static [Location](https://doc.rust-lang.org/nightly/core/panic/location/struct.Location.html "struct core::panic::location::Location")<'static>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#117)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#121)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#125)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<&'static [Location](https://doc.rust-lang.org/nightly/core/panic/location/struct.Location.html "struct core::panic::location::Location")<'static>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#129)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#133)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/panic.rs.html#137)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#119)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for &'static [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#120)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<&'static [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#124)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#128)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#132)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<&'static [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#136)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#140)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#144)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#406)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for &'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#407)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#411)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#415)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#419)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<&'static [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#423)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#427)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#431)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#688)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#176-179)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#168-171)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#160-163)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#152-155)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#143-146)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#134-137)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#172-175)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#164-167)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#156-159)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#148-151)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Atomic](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.Atomic.html "struct core::sync::atomic::Atomic")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/sync.rs.html#138-141)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#262)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#263)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#267)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#271)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#275)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [Path](https://doc.rust-lang.org/nightly/std/path/struct.Path.html "struct std::path::Path")\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#279)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#283)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#287)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#115)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/time.rs.html#7-20)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#3-11)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#103-111)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#76-84)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#67-75)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#40-48)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#4-12)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#22-30)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#94-102)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#85-93)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#58-66)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#49-57)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#13-21)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[NonZero](https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html "struct core::num::nonzero::NonZero")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#31-39)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[OsString](https://doc.rust-lang.org/nightly/std/ffi/os_str/struct.OsString.html "struct std::ffi::os_str::OsString")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/ffi.rs.html#8-16)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[PathBuf](https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html "struct std::path::PathBuf")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/path.rs.html#22-31)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeFull](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFull.html "struct core::ops::range::RangeFull")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#8)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[SmolStr](https://docs.rs/smol_str/0.2.2/x86_64-unknown-linux-gnu/smol_str/struct.SmolStr.html "struct smol_str::SmolStr")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smol_str.rs.html#4-13)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[SocketAddr](https://doc.rust-lang.org/nightly/core/net/socket_addr/enum.SocketAddr.html "enum core::net::socket_addr::SocketAddr")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/net.rs.html#4-12)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/any.rs.html#3)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#20-29)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [char](https://doc.rust-lang.org/nightly/std/primitive.char.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[char](https://doc.rust-lang.org/nightly/std/primitive.char.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#30-39)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#280-298)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[f64](https://doc.rust-lang.org/nightly/std/primitive.f64.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#299-317)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i8](https://doc.rust-lang.org/nightly/std/primitive.i8.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#160-179)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i16](https://doc.rust-lang.org/nightly/std/primitive.i16.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#180-199)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i32](https://doc.rust-lang.org/nightly/std/primitive.i32.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#200-219)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i64](https://doc.rust-lang.org/nightly/std/primitive.i64.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#220-239)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[i128](https://doc.rust-lang.org/nightly/std/primitive.i128.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#240-259)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[isize](https://doc.rust-lang.org/nightly/std/primitive.isize.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#260-279)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#40-59)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u16](https://doc.rust-lang.org/nightly/std/primitive.u16.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#60-79)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#80-99)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#100-119)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[u128](https://doc.rust-lang.org/nightly/std/primitive.u128.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#120-139)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#140-159)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

### impl<A, B, C, D, E, F, G, H, I, J, K, L> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E, F, G, H, I, J, K, L)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), J: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), K: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), L: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J, K, L)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [into\_reflect](#tymethod.into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J, K, L)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, ) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#712)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

### impl<A, B, C, D, E, F, G, H, I, J, K> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E, F, G, H, I, J, K)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), J: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), K: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J, K)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [into\_reflect](#tymethod.into_reflect)( self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J, K)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>, ) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#710)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

### impl<A, B, C, D, E, F, G, H, I, J> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E, F, G, H, I, J)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), J: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I, J)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#708)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

### impl<A, B, C, D, E, F, G, H, I> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E, F, G, H, I)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), I: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H, I)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#706)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

### impl<A, B, C, D, E, F, G, H> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E, F, G, H)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), H: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G, H)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#704)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

### impl<A, B, C, D, E, F, G> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E, F, G)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), G: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F, G)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#702)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

### impl<A, B, C, D, E, F> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E, F)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), F: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E, F)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#700)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

### impl<A, B, C, D, E> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D, E)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), E: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D, E)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#698)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

### impl<A, B, C, D> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C, D)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), D: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C, D)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#696)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

### impl<A, B, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B, C)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), C: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B, C)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#694)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

### impl<A, B> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), B: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A, B)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#692)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

### impl<A> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [(A,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)

where A: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[(A,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple.rs.html#690)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

### impl<K, V, S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>

where K: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[HashMap](https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html "struct std::collections::hash::map::HashMap")<K, V, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_map.rs.html#12)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#185-189)

### impl<K, V, S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>

where K: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#191)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#195)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#199)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#203)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<K, V, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#207)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#211)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#215)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

### impl<K, V> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>

where K: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord"), V: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[BTreeMap](https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html "struct alloc::collections::btree::map::BTreeMap")<K, V>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/map.rs.html#171-176)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

### impl<N, E, Ix> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>

where N: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), E: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), Ix: [IndexType](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/trait.IndexType.html "trait petgraph::graph_impl::IndexType") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Graph](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.Graph.html "struct petgraph::graph_impl::Graph")<N, E, [Directed](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/enum.Directed.html "enum petgraph::Directed"), Ix>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/petgraph.rs.html#12-16)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

### impl<T, E> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

where [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, E: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/result.rs.html#8-14)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#418-421)

### impl<T, S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<T, S>

where T: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#423)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<T, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#427)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#431)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#435)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[IndexSet](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/set/struct.IndexSet.html "struct indexmap::set::IndexSet")<T, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#439)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#443)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/indexmap.rs.html#447)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#566)

### impl<T, const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#568)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#573)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#578)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#583)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[\[T; N\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#588)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#593)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/primitives.rs.html#598)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>

where T: [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[BTreeSet](https://doc.rust-lang.org/nightly/alloc/collections/btree/set/struct.BTreeSet.html "struct alloc::collections::btree::set::BTreeSet")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/btree/set.rs.html#3)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[BinaryHeap](https://doc.rust-lang.org/nightly/alloc/collections/binary_heap/struct.BinaryHeap.html "struct alloc::collections::binary_heap::BinaryHeap")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/binary_heap.rs.html#3)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Bound](https://doc.rust-lang.org/nightly/core/ops/range/enum.Bound.html "enum core::ops::range::Bound")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#9)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>

where T: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Cow](https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html "enum alloc::borrow::Cow")<'static, [\[T\]](https://doc.rust-lang.org/nightly/std/primitive.slice.html)\>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/borrow.rs.html#276-280)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>

where [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/option.rs.html#8-14)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#3)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeFrom](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeFrom.html "struct core::ops::range::RangeFrom")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#5)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeInclusive.html "struct core::ops::range::RangeInclusive")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#4)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeTo](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeTo.html "struct core::ops::range::RangeTo")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#6)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/range/struct.RangeToInclusive.html "struct core::ops::range::RangeToInclusive")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/ops.rs.html#7)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Saturating](https://doc.rust-lang.org/nightly/core/num/saturating/struct.Saturating.html "struct core::num::saturating::Saturating")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#113)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#159-161)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>

where T: [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), <T as [Array](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html "trait smallvec::Array")\>::[Item](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/trait.Array.html#associatedtype.Item "type smallvec::Array::Item"): [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#163)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#167)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#171)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#175)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[SmallVec](https://docs.rs/smallvec/1.15.1/x86_64-unknown-linux-gnu/smallvec/struct.SmallVec.html "struct smallvec::SmallVec")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#179)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#183)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/smallvec.rs.html#187)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>

where T: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[VecDeque](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/struct.VecDeque.html "struct alloc::collections::vec_deque::VecDeque")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/collections/vec_deque.rs.html#10-17)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[Wrapping](https://doc.rust-lang.org/nightly/core/num/wrapping/struct.Wrapping.html "struct core::num::wrapping::Wrapping")<T>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/core/num.rs.html#112)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

### impl<V, S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>

where V: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [into\_any](#tymethod.into_any)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [as\_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [as\_any\_mut](#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [into\_reflect](#tymethod.into_reflect)(self: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<[HashSet](https://doc.rust-lang.org/nightly/std/collections/hash/set/struct.HashSet.html "struct std::collections::hash::set::HashSet")<V, S>>) -> [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [as\_reflect](#tymethod.as_reflect)(&self) -> &(dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [as\_reflect\_mut](#tymethod.as_reflect_mut)(&mut self) -> &mut (dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/std/collections/hash_set.rs.html#9)

#### fn [set](#tymethod.set)(&mut self, value: [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")\>>

## Implementors

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#62)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Aabb](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#42)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Aabb2d](../math/bounding/struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#48)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Aabb3d](../math/bounding/struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#112)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AabbCast2d](../math/bounding/struct.AabbCast2d.html "struct bevy::math::bounding::AabbCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#109)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AabbCast3d](../math/bounding/struct.AabbCast3d.html "struct bevy::math::bounding::AabbCast3d")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#43)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AabbGizmoConfigGroup](../prelude/struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#110)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AccessibilityRequested](../a11y/struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#251)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AccessibilitySystems](../a11y/enum.AccessibilitySystems.html "enum bevy::a11y::AccessibilitySystems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/accessibility.rs.html#185)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AccessibleLabel](../prelude/struct.AccessibleLabel.html "struct bevy::prelude::AccessibleLabel")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#210)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AccumulatedMouseMotion](../input/mouse/struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#231)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AccumulatedMouseScroll](../input/mouse/struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#208)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AcquireFocus](../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Activate](../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#33)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ActivateOnPress](../ui_widgets/struct.ActivateOnPress.html "struct bevy::ui_widgets::ActivateOnPress")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ActiveAnimation](../animation/struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#49)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ActiveDescendant](../ui_widgets/struct.ActiveDescendant.html "struct bevy::ui_widgets::ActiveDescendant")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#333)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Add](../prelude/struct.Add.html "struct bevy::prelude::Add")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Affine2](../math/struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#416-423)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Affine3](../math/struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#424-431)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Affine3A](../math/struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1055)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AlignContent](../prelude/enum.AlignContent.html "enum bevy::prelude::AlignContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#895)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AlignItems](../prelude/enum.AlignItems.html "enum bevy::prelude::AlignItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#975)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AlignSelf](../prelude/enum.AlignSelf.html "enum bevy::prelude::AlignSelf")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/alpha.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AlphaMode](../prelude/enum.AlphaMode.html "enum bevy::prelude::AlphaMode")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#245)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AlphaMode2d](../sprite_render/enum.AlphaMode2d.html "enum bevy::sprite_render::AlphaMode2d")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#9)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AmbientLight](../prelude/struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#254)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Anchor](../sprite/struct.Anchor.html "struct bevy::sprite::Anchor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#113)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AngularColorStop](../prelude/struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#213)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimatedBy](../animation/struct.AnimatedBy.html "struct bevy::animation::AnimatedBy")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#103)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationClip](../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#112)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationGraph](../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationGraphHandle](../prelude/struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#169)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationGraphNode](../prelude/struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#211)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationNodeType](../prelude/enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#730)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationPlayer](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#184)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationTargetId](../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#54)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationTransition](../prelude/struct.AnimationTransition.html "struct bevy::prelude::AnimationTransition")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#31)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimationTransitions](../prelude/struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#955)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Annulus](../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#745)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnnulusMeshBuilder](../mesh/struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#1565)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AppExit](../prelude/enum.AppExit.html "enum bevy::prelude::AppExit")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#453)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AppLifecycle](../window/enum.AppLifecycle.html "enum bevy::window::AppLifecycle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#117)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Arc2d](../prelude/struct.Arc2d.html "struct bevy::prelude::Arc2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#14)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AspectRatio](../math/struct.AspectRatio.html "struct bevy::math::AspectRatio")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AssetIndex](../asset/struct.AssetIndex.html "struct bevy::asset::AssetIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#414)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AtmosphereMode](../pbr/enum.AtmosphereMode.html "enum bevy::pbr::AtmosphereMode")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#286)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AtmosphereSettings](../pbr/struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#105)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AutoDirectionalNavigation](../ui/auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AutoExposure](../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/compensation_curve.rs.html#20)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AutoExposureCompensationCurve](../post_process/auto_exposure/struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/autofocus.rs.html#20)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AutoFocus](../input_focus/struct.AutoFocus.html "struct bevy::input_focus::AutoFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#90)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AutoNavigationConfig](../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#984)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AxisSettings](../input/gamepad/struct.AxisSettings.html "struct bevy::input::gamepad::AxisSettings")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#287-294)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BVec2](../prelude/struct.BVec2.html "struct bevy::prelude::BVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#295-303)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BVec3](../prelude/struct.BVec3.html "struct bevy::prelude::BVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#304-313)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BVec4](../prelude/struct.BVec4.html "struct bevy::prelude::BVec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#502-508)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BVec3A](../prelude/struct.BVec3A.html "struct bevy::prelude::BVec3A")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#509-515)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BVec4A](../prelude/struct.BVec4A.html "struct bevy::prelude::BVec4A")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#95)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Backfaces](../picking/mesh_picking/ray_cast/enum.Backfaces.html "enum bevy::picking::mesh_picking::ray_cast::Backfaces")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2222)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BackgroundColor](../prelude/struct.BackgroundColor.html "struct bevy::prelude::BackgroundColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#526)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BackgroundGradient](../prelude/struct.BackgroundGradient.html "struct bevy::prelude::BackgroundGradient")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#11-18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BlendState](../render/render_resource/struct.BlendState.html "struct bevy::render::render_resource::BlendState")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#30)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Bloom](../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#216)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BloomCompositeMode](../post_process/bloom/enum.BloomCompositeMode.html "enum bevy::post_process::bloom::BloomCompositeMode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#199)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BloomPrefilter](../post_process/bloom/struct.BloomPrefilter.html "struct bevy::post_process::bloom::BloomPrefilter")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2249)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BorderColor](../prelude/struct.BorderColor.html "struct bevy::prelude::BorderColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#542)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BorderGradient](../prelude/struct.BorderGradient.html "struct bevy::prelude::BorderGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BorderRadius](../prelude/struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/border_rect.rs.html#8)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BorderRect](../prelude/struct.BorderRect.html "struct bevy::prelude::BorderRect")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#478)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BoundingCircle](../math/bounding/struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#150)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BoundingCircleCast](../math/bounding/struct.BoundingCircleCast.html "struct bevy::math::bounding::BoundingCircleCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#504)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BoundingSphere](../math/bounding/struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#154)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BoundingSphereCast](../math/bounding/struct.BoundingSphereCast.html "struct bevy::math::bounding::BoundingSphereCast")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2831)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BoxShadow](../prelude/struct.BoxShadow.html "struct bevy::prelude::BoxShadow")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#186)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BoxShadowSamples](../prelude/struct.BoxShadowSamples.html "struct bevy::prelude::BoxShadowSamples")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1181)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [BoxSizing](../prelude/enum.BoxSizing.html "enum bevy::prelude::BoxSizing")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/button.rs.html#6)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for bevy::prelude::[Button](../prelude/struct.Button.html "struct bevy::prelude::Button")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for bevy::ui\_widgets::[Button](../ui_widgets/struct.Button.html "struct bevy::ui_widgets::Button")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1412)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ButtonAxisSettings](../input/gamepad/struct.ButtonAxisSettings.html "struct bevy::input::gamepad::ButtonAxisSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#820)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ButtonSettings](../input/gamepad/struct.ButtonSettings.html "struct bevy::input::gamepad::ButtonSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#172)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ButtonState](../input/enum.ButtonState.html "enum bevy::input::ButtonState")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#34)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ButtonVariant](../feathers/controls/enum.ButtonVariant.html "enum bevy::feathers::controls::ButtonVariant")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2407)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CalculatedClip](../prelude/struct.CalculatedClip.html "struct bevy::prelude::CalculatedClip")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#374)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Camera](../prelude/struct.Camera.html "struct bevy::prelude::Camera")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#9)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Camera2d](../prelude/struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Camera3d](../prelude/struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#58)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Camera3dDepthLoadOp](../camera/enum.Camera3dDepthLoadOp.html "enum bevy::camera::Camera3dDepthLoadOp")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#41)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Camera3dDepthTextureUsage](../camera/struct.Camera3dDepthTextureUsage.html "struct bevy::camera::Camera3dDepthTextureUsage")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#1044)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CameraMainTextureUsages](../camera/struct.CameraMainTextureUsages.html "struct bevy::camera::CameraMainTextureUsages")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#860)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CameraOutputMode](../camera/enum.CameraOutputMode.html "enum bevy::camera::CameraOutputMode")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#176)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CameraRenderGraph](../render/camera/struct.CameraRenderGraph.html "struct bevy::render::camera::CameraRenderGraph")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#178)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cancel](../prelude/struct.Cancel.html "struct bevy::prelude::Cancel")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2183)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Capsule2d](../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1121)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Capsule2dMeshBuilder](../mesh/struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#856)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Capsule3d](../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Capsule3dMeshBuilder](../mesh/struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CapsuleUvProfile](../mesh/enum.CapsuleUvProfile.html "enum bevy::mesh::CapsuleUvProfile")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#179)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cascade](../light/cascade/struct.Cascade.html "struct bevy::light::cascade::Cascade")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#24)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CascadeShadowConfig](../light/struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#167)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cascades](../light/struct.Cascades.html "struct bevy::light::Cascades")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#443)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CascadesFrusta](../camera/primitives/struct.CascadesFrusta.html "struct bevy::camera::primitives::CascadesFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#460)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CascadesVisibleEntities](../camera/visibility/struct.CascadesVisibleEntities.html "struct bevy::camera::visibility::CascadesVisibleEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#49)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Checkable](../ui/struct.Checkable.html "struct bevy::ui::Checkable")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#36)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Checkbox](../ui_widgets/struct.Checkbox.html "struct bevy::ui_widgets::Checkbox")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#54)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Checked](../ui/struct.Checked.html "struct bevy::ui::Checked")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#95)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ChildOf](../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#149)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Children](../prelude/struct.Children.html "struct bevy::prelude::Children")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#43)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ChromaticAberration](../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#29)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Circle](../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CircleMeshBuilder](../mesh/struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#106)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CircularMeshUvMode](../mesh/enum.CircularMeshUvMode.html "enum bevy::mesh::CircularMeshUvMode")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#285)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CircularSector](../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#128)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CircularSectorMeshBuilder](../mesh/struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#437)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CircularSegment](../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#266)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CircularSegmentMeshBuilder](../mesh/struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#53)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ClearColor](../prelude/struct.ClearColor.html "struct bevy::prelude::ClearColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#11)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ClearColorConfig](../prelude/enum.ClearColorConfig.html "enum bevy::prelude::ClearColorConfig")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#309)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Click](../prelude/struct.Click.html "struct bevy::prelude::Click")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#105)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ClusterConfig](../light/cluster/enum.ClusterConfig.html "enum bevy::light::cluster::ClusterConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#82)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ClusterFarZMode](../light/cluster/enum.ClusterFarZMode.html "enum bevy::light::cluster::ClusterFarZMode")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#95)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ClusterZConfig](../light/cluster/struct.ClusterZConfig.html "struct bevy::light::cluster::ClusterZConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#229)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ClusteredDecal](../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color.rs.html#48)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Color](../prelude/enum.Color.html "enum bevy::prelude::Color")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#47)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorChannel](../feathers/controls/enum.ColorChannel.html "enum bevy::feathers::controls::ColorChannel")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#399)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorGrading](../render/view/struct.ColorGrading.html "struct bevy::render::view::ColorGrading")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#428)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorGradingGlobal](../render/view/struct.ColorGradingGlobal.html "struct bevy::render::view::ColorGradingGlobal")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#494)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorGradingSection](../render/view/struct.ColorGradingSection.html "struct bevy::render::view::ColorGradingSection")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorMaterial](../prelude/struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#68)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorPlaneValue](../feathers/controls/struct.ColorPlaneValue.html "struct bevy::feathers::controls::ColorPlaneValue")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#187)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorSlider](../feathers/controls/struct.ColorSlider.html "struct bevy::feathers::controls::ColorSlider")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#10)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorStop](../prelude/struct.ColorStop.html "struct bevy::prelude::ColorStop")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#40)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorSwatchFg](../feathers/controls/struct.ColorSwatchFg.html "struct bevy::feathers::controls::ColorSwatchFg")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#33)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorSwatchValue](../feathers/controls/struct.ColorSwatchValue.html "struct bevy::feathers::controls::ColorSwatchValue")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#132)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CompassOctant](../math/enum.CompassOctant.html "enum bevy::math::CompassOctant")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/compass.rs.html#25)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CompassQuadrant](../math/enum.CompassQuadrant.html "enum bevy::math::CompassQuadrant")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/info.rs.html#178)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#136)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComponentTicks](../ecs/change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1295)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CompositeAlphaMode](../window/enum.CompositeAlphaMode.html "enum bevy::window::CompositeAlphaMode")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#92)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CompositingSpace](../prelude/enum.CompositingSpace.html "enum bevy::prelude::CompositingSpace")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#217)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComputedCameraValues](../camera/struct.ComputedCameraValues.html "struct bevy::camera::ComputedCameraValues")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#26)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComputedNode](../prelude/struct.ComputedNode.html "struct bevy::prelude::ComputedNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#17)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComputedStackIndex](../ui/struct.ComputedStackIndex.html "struct bevy::ui::ComputedStackIndex")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#37)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComputedTextBlock](../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3036)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComputedUiRenderTargetInfo](../prelude/struct.ComputedUiRenderTargetInfo.html "struct bevy::prelude::ComputedUiRenderTargetInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3014)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ComputedUiTargetCamera](../prelude/struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#927)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cone](../prelude/struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConeAnchor](../mesh/enum.ConeAnchor.html "enum bevy::mesh::ConeAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#20)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConeMeshBuilder](../mesh/struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#410)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConicGradient](../prelude/struct.ConicGradient.html "struct bevy::prelude::ConicGradient")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1010)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConicalFrustum](../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConicalFrustumMeshBuilder](../mesh/struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#34)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ContactShadows](../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#139)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ContentSize](../ui/struct.ContentSize.html "struct bevy::ui::ContentSize")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#37)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ContrastAdaptiveSharpening](../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ControlOrientation](../ui_widgets/enum.ControlOrientation.html "enum bevy::ui_widgets::ControlOrientation")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1950)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConvexPolygon](../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#413)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConvexPolygonMeshBuilder](../mesh/struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#392)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubemapFrusta](../camera/primitives/struct.CubemapFrusta.html "struct bevy::camera::primitives::CubemapFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#408)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubemapLayout](../camera/primitives/enum.CubemapLayout.html "enum bevy::camera::primitives::CubemapLayout")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#435)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubemapVisibleEntities](../camera/visibility/struct.CubemapVisibleEntities.html "struct bevy::camera::visibility::CubemapVisibleEntities")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#113)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicRotationCurve](../animation/gltf_curves/struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#684)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cuboid](../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CuboidMeshBuilder](../mesh/struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#209)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CursorEntered](../prelude/struct.CursorEntered.html "struct bevy::prelude::CursorEntered")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1076)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CursorGrabMode](../window/enum.CursorGrabMode.html "enum bevy::window::CursorGrabMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/mod.rs.html#24)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CursorIcon](../window/enum.CursorIcon.html "enum bevy::window::CursorIcon")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#226)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CursorLeft](../prelude/struct.CursorLeft.html "struct bevy::prelude::CursorLeft")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#184)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CursorMoved](../prelude/struct.CursorMoved.html "struct bevy::prelude::CursorMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#744)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CursorOptions](../window/struct.CursorOptions.html "struct bevy::window::CursorOptions")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#71)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CustomCursor](../window/enum.CustomCursor.html "enum bevy::window::CustomCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#15)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CustomCursorImage](../window/struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#55)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CustomCursorUrl](../window/struct.CustomCursorUrl.html "struct bevy::window::CustomCursorUrl")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#109)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CustomProjection](../camera/struct.CustomProjection.html "struct bevy::camera::CustomProjection")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#777)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Cylinder](../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CylinderAnchor](../mesh/enum.CylinderAnchor.html "enum bevy::mesh::CylinderAnchor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#20)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CylinderMeshBuilder](../mesh/struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#433-440)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DAffine2](../math/struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#441-448)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DAffine3](../math/struct.DAffine3.html "struct bevy::math::DAffine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#380-387)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DMat2](../math/struct.DMat2.html "struct bevy::math::DMat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DMat3](../math/struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#397-406)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DMat4](../math/struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#460-469)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DQuat](../math/struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#315-322)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DVec2](../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DVec3](../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#379)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DebandDither](../core_pipeline/tonemapping/enum.DebandDither.html "enum bevy::core_pipeline::tonemapping::DebandDither")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#23)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DefaultCursor](../feathers/cursor/struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#84)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DefaultGizmoConfigGroup](../prelude/struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DefaultOpaqueRendererMethod](../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#172)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DefaultQueryFilters](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#232)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DefaultSpatialScale](../audio/struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#82)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DeferredPrepass](../core_pipeline/prepass/struct.DeferredPrepass.html "struct bevy::core_pipeline::prepass::DeferredPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#93)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DeferredPrepassDoubleBuffer](../core_pipeline/prepass/struct.DeferredPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#133)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DelayedCommandQueue](../time/struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#66)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DenoiseCas](../anti_alias/contrast_adaptive_sharpening/struct.DenoiseCas.html "struct bevy::anti_alias::contrast_adaptive_sharpening::DenoiseCas")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#75)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DepthOfField](../post_process/dof/struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#119)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DepthOfFieldMode](../post_process/dof/enum.DepthOfFieldMode.html "enum bevy::post_process::dof::DepthOfFieldMode")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#62)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DepthPrepass](../core_pipeline/prepass/struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#87)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DepthPrepassDoubleBuffer](../core_pipeline/prepass/struct.DepthPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#388)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Despawn](../prelude/struct.Despawn.html "struct bevy::prelude::Despawn")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#88)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Dir2](../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#399)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Dir3](../prelude/struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1053)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Dir4](../math/struct.Dir4.html "struct bevy::math::Dir4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#803)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Dir3A](../prelude/struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#61)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DirectionalLight](../prelude/struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#191)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DirectionalLightShadowMap](../light/struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#173)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DirectionalLightTexture](../light/struct.DirectionalLightTexture.html "struct bevy::light::DirectionalLightTexture")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#251)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DirectionalNavigationMap](../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#354)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DirectlyHovered](../picking/hover/struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#131)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Disabled](../ecs/entity_disabling/struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#361)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Discard](../prelude/struct.Discard.html "struct bevy::prelude::Discard")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1147)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Display](../prelude/enum.Display.html "enum bevy::prelude::Display")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DistanceFog](../prelude/struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#66)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DoubleTapGesture](../input/gestures/struct.DoubleTapGesture.html "struct bevy::input::gestures::DoubleTapGesture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#348)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Drag](../prelude/struct.Drag.html "struct bevy::prelude::Drag")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#421)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DragDrop](../prelude/struct.DragDrop.html "struct bevy::prelude::DragDrop")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#370)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DragEnd](../prelude/struct.DragEnd.html "struct bevy::prelude::DragEnd")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#385)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DragEnter](../prelude/struct.DragEnter.html "struct bevy::prelude::DragEnter")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#433)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DragEntry](../prelude/struct.DragEntry.html "struct bevy::prelude::DragEntry")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#409)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DragLeave](../prelude/struct.DragLeave.html "struct bevy::prelude::DragLeave")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#397)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DragOver](../prelude/struct.DragOver.html "struct bevy::prelude::DragOver")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#338)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DragStart](../prelude/struct.DragStart.html "struct bevy::prelude::DragStart")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#329)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DynamicSkinnedMeshBounds](../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DynamicWorldRoot](../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#431)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EaseFunction](../prelude/enum.EaseFunction.html "enum bevy::prelude::EaseFunction")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#804)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Ellipse](../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#556)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EllipseMeshBuilder](../mesh/struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1432)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EnabledButtons](../window/struct.EnabledButtons.html "struct bevy::window::EnabledButtons")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#223)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Enter](../prelude/struct.Enter.html "struct bevy::prelude::Enter")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#414)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityCursor](../feathers/cursor/enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#248)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityGeneration](../ecs/entity/struct.EntityGeneration.html "struct bevy::ecs::entity::EntityGeneration")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash.rs.html#8)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityHash](../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityHashSet](../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/mod.rs.html#147)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityIndex](../ecs/entity/struct.EntityIndex.html "struct bevy::ecs::entity::EntityIndex")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#29)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityIndexSet](../ecs/entity/struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#105)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EnvironmentMapLight](../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#90)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ErasedGizmoConfigGroup](../gizmos/config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#471-500)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EulerRot](../prelude/enum.EulerRot.html "enum bevy::prelude::EulerRot")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#229)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Exposure](../camera/struct.Exposure.html "struct bevy::camera::Exposure")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#59)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersButton](../feathers/controls/struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#48)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersCheckbox](../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersColorPlane](../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#162)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersColorSlider](../feathers/controls/struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersColorSwatch](../feathers/controls/struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#33)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersDisclosureToggle](../feathers/controls/struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#106)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersListRow](../feathers/controls/struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#36)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersListView](../feathers/controls/struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#48)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersMenu](../feathers/controls/struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#141)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersMenuButton](../feathers/controls/struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#439)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersMenuDivider](../feathers/controls/struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#250)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersMenuItem](../feathers/controls/struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#195)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersMenuPopup](../feathers/controls/struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#55)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersNumberInput](../feathers/controls/struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#47)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersRadio](../feathers/controls/struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersScrollbar](../feathers/controls/struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#50)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersSlider](../feathers/controls/struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#85)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersTextInput](../feathers/controls/struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#38)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersTextInputContainer](../feathers/controls/struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#42)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersToggleSwitch](../feathers/controls/struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#126)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FeathersToolButton](../feathers/controls/struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#376)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FileDragAndDrop](../prelude/enum.FileDragAndDrop.html "enum bevy::prelude::FileDragAndDrop")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#68)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Fixed](../prelude/struct.Fixed.html "struct bevy::prelude::Fixed")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1206)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FlexDirection](../prelude/enum.FlexDirection.html "enum bevy::prelude::FlexDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1478)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FlexWrap](../prelude/enum.FlexWrap.html "enum bevy::prelude::FlexWrap")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/float_ord.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FloatOrd](../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#15)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusCause](../input_focus/enum.FocusCause.html "enum bevy::input_focus::FocusCause")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#33)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusGained](../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#23)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusIndicator](../feathers/focus/struct.FocusIndicator.html "struct bevy::feathers::focus::FocusIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#50)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusLost](../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#101)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusPolicy](../ui/enum.FocusPolicy.html "enum bevy::ui::FocusPolicy")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#30)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusWithinIndicator](../feathers/focus/struct.FocusWithinIndicator.html "struct bevy::feathers::focus::FocusWithinIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#469)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusableArea](../input_focus/directional_navigation/struct.FocusableArea.html "struct bevy::input_focus::directional_navigation::FocusableArea")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#100)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FogFalloff](../prelude/enum.FogFalloff.html "enum bevy::prelude::FogFalloff")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#75)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FogVolume](../light/struct.FogVolume.html "struct bevy::light::FogVolume")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#728)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontFeatureTag](../text/struct.FontFeatureTag.html "struct bevy::text::FontFeatureTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#839)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontFeatures](../text/struct.FontFeatures.html "struct bevy::text::FontFeatures")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1199)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontHinting](../prelude/enum.FontHinting.html "enum bevy::prelude::FontHinting")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#486)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontSize](../prelude/enum.FontSize.html "enum bevy::prelude::FontSize")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1179)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontSmoothing](../prelude/enum.FontSmoothing.html "enum bevy::prelude::FontSmoothing")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontSource](../prelude/enum.FontSource.html "enum bevy::prelude::FontSource")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#704)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontStyle](../prelude/enum.FontStyle.html "enum bevy::prelude::FontStyle")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#913)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontVariationTag](../text/struct.FontVariationTag.html "struct bevy::text::FontVariationTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#960)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontVariations](../text/struct.FontVariations.html "struct bevy::text::FontVariations")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#596)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontWeight](../prelude/struct.FontWeight.html "struct bevy::prelude::FontWeight")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#659)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FontWidth](../prelude/struct.FontWidth.html "struct bevy::prelude::FontWidth")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#73)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ForceTouch](../input/touch/enum.ForceTouch.html "enum bevy::input::touch::ForceTouch")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#62)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ForwardDecal](../pbr/decal/struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#108)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FpsOverlayConfig](../dev_tools/fps_overlay/struct.FpsOverlayConfig.html "struct bevy::dev_tools::fps_overlay::FpsOverlayConfig")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#139)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FrameTimeGraphConfig](../dev_tools/fps_overlay/struct.FrameTimeGraphConfig.html "struct bevy::dev_tools::fps_overlay::FrameTimeGraphConfig")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#247)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Frustum](../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#78)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FrustumGizmoConfigGroup](../prelude/struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Fxaa](../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#371)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Gamepad](../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#664)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadAxis](../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#258)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadAxisChangedEvent](../input/gamepad/struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#572)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadButton](../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#222)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadButtonChangedEvent](../input/gamepad/struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#190)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadButtonStateChangedEvent](../input/gamepad/struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1554)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadConnection](../input/gamepad/enum.GamepadConnection.html "enum bevy::input::gamepad::GamepadConnection")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#151)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadConnectionEvent](../input/gamepad/struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#38)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadEvent](../input/gamepad/enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#710)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadInput](../input/gamepad/enum.GamepadInput.html "enum bevy::input::gamepad::GamepadInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1688)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadRumbleIntensity](../input/gamepad/struct.GamepadRumbleIntensity.html "struct bevy::input::gamepad::GamepadRumbleIntensity")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1778)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadRumbleRequest](../input/gamepad/enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#736)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GamepadSettings](../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#261)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GeneratedEnvironmentMapLight](../prelude/struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GhostNode](../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Gizmo](../prelude/struct.Gizmo.html "struct bevy::prelude::Gizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#206)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GizmoConfig](../prelude/struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#97)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GizmoConfigStore](../prelude/struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#246)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GizmoLineConfig](../prelude/struct.GizmoLineConfig.html "struct bevy::prelude::GizmoLineConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#19)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GizmoLineJoint](../prelude/enum.GizmoLineJoint.html "enum bevy::prelude::GizmoLineJoint")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#37)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GizmoLineStyle](../prelude/enum.GizmoLineStyle.html "enum bevy::prelude::GizmoLineStyle")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#60)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlobalAmbientLight](../prelude/struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlobalRenderDebugOverlay](../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#53)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlobalTransform](../prelude/struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#107)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlobalUiDebugOptions](../prelude/struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#8)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlobalVolume](../prelude/struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2448)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlobalZIndex](../prelude/struct.GlobalZIndex.html "struct bevy::prelude::GlobalZIndex")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlobalsUniform](../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#266)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GltfExtras](../prelude/struct.GltfExtras.html "struct bevy::prelude::GltfExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#334)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GltfMaterialExtras](../gltf/struct.GltfMaterialExtras.html "struct bevy::gltf::GltfMaterialExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#344)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GltfMaterialName](../gltf/struct.GltfMaterialName.html "struct bevy::gltf::GltfMaterialName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#309)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GltfMeshExtras](../gltf/struct.GltfMeshExtras.html "struct bevy::gltf::GltfMeshExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#319)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GltfMeshName](../gltf/struct.GltfMeshName.html "struct bevy::gltf::GltfMeshName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#284)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GltfSceneExtras](../gltf/struct.GltfSceneExtras.html "struct bevy::gltf::GltfSceneExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#294)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GltfSceneName](../gltf/struct.GltfSceneName.html "struct bevy::gltf::GltfSceneName")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#32)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlyphAtlasInfo](../text/struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#51)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GlyphAtlasLocation](../text/struct.GlyphAtlasLocation.html "struct bevy::text::GlyphAtlasLocation")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GpuAtmosphereSettings](../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#457)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Gradient](../prelude/enum.Gradient.html "enum bevy::prelude::Gradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1512)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GridAutoFlow](../prelude/enum.GridAutoFlow.html "enum bevy::prelude::GridAutoFlow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2020)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GridPlacement](../prelude/struct.GridPlacement.html "struct bevy::prelude::GridPlacement")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GridTrack](../prelude/struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1768)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GridTrackRepetition](../prelude/enum.GridTrackRepetition.html "enum bevy::prelude::GridTrackRepetition")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/half_space.rs.html#36)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [HalfSpace](../prelude/struct.HalfSpace.html "struct bevy::prelude::HalfSpace")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#60)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [HashedStr](../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#87)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Hdr](../camera/struct.Hdr.html "struct bevy::camera::Hdr")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#133)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [HitData](../picking/backend/struct.HitData.html "struct bevy::picking::backend::HitData")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#336)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Hovered](../picking/hover/struct.Hovered.html "struct bevy::picking::hover::Hovered")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Hsla](../prelude/struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Hsva](../prelude/struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Hwba](../prelude/struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#48-55)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I8Vec2](../math/struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#57-65)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I8Vec3](../math/struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#67-76)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#78-85)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I16Vec2](../math/struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#87-95)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I16Vec3](../math/struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#97-106)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#108-115)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I64Vec2](../math/struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#117-125)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I64Vec3](../math/struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#127-136)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/irect.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IRect](../prelude/struct.IRect.html "struct bevy::prelude::IRect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#20-27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IVec2](../prelude/struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#28-36)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IVec3](../prelude/struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IVec4](../prelude/struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#436)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IgnoreScroll](../prelude/struct.IgnoreScroll.html "struct bevy::prelude::IgnoreScroll")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#608)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Image](../prelude/struct.Image.html "struct bevy::prelude::Image")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#723)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageAddressMode](../image/enum.ImageAddressMode.html "enum bevy::image::ImageAddressMode")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#776)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageCompareFunction](../image/enum.ImageCompareFunction.html "enum bevy::image::ImageCompareFunction")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#757)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageFilterMode](../image/enum.ImageFilterMode.html "enum bevy::image::ImageFilterMode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageNode](../prelude/struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#192)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageNodeSize](../ui/widget/struct.ImageNodeSize.html "struct bevy::ui::widget::ImageNodeSize")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#983)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageRenderTarget](../camera/struct.ImageRenderTarget.html "struct bevy::camera::ImageRenderTarget")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#673)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageSampler](../image/enum.ImageSampler.html "enum bevy::image::ImageSampler")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#804)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageSamplerBorderColor](../image/enum.ImageSamplerBorderColor.html "enum bevy::image::ImageSamplerBorderColor")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#830)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ImageSamplerDescriptor](../image/struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#247)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Ime](../prelude/enum.Ime.html "enum bevy::prelude::Ime")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/index.rs.html#83)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Indices](../mesh/enum.Indices.html "enum bevy::mesh::Indices")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#89)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InfiniteGrid](../dev_tools/infinite_grid/struct.InfiniteGrid.html "struct bevy::dev_tools::infinite_grid::InfiniteGrid")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#105)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InfiniteGridSettings](../dev_tools/infinite_grid/struct.InfiniteGridSettings.html "struct bevy::dev_tools::infinite_grid::InfiniteGridSettings")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#180)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InfinitePlane3d](../prelude/struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InheritableFont](../feathers/font_styles/struct.InheritableFont.html "struct bevy::feathers::font_styles::InheritableFont")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#106)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InheritableThemeTextColor](../feathers/theme/struct.InheritableThemeTextColor.html "struct bevy::feathers::theme::InheritableThemeTextColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#162)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InheritedVisibility](../prelude/struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#875)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InlineDirection](../prelude/enum.InlineDirection.html "enum bevy::prelude::InlineDirection")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#100)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InputFocus](../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#173)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InputFocusVisible](../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#346)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Insert](../prelude/struct.Insert.html "struct bevy::prelude::Insert")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#50)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InstanceId](../world_serialization/struct.InstanceId.html "struct bevy::world_serialization::InstanceId")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/time.rs.html#3-5)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Instant](../platform/time/struct.Instant.html "struct bevy::platform::time::Instant")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#44)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Interaction](../prelude/enum.Interaction.html "enum bevy::prelude::Interaction")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InteractionDisabled](../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1098)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InternalWindowState](../window/struct.InternalWindowState.html "struct bevy::window::InternalWindowState")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#634)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InterpolationColorSpace](../prelude/enum.InterpolationColorSpace.html "enum bevy::prelude::InterpolationColorSpace")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/interval.rs.html#23)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Interval](../prelude/struct.Interval.html "struct bevy::prelude::Interval")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#329)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IrradianceVolume](../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2978)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IsDefaultUiCamera](../prelude/struct.IsDefaultUiCamera.html "struct bevy::prelude::IsDefaultUiCamera")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/resource.rs.html#121)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [IsResource](../ecs/resource/struct.IsResource.html "struct bevy::ecs::resource::IsResource")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#90)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Isometry2d](../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#368)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Isometry3d](../prelude/struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#53)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [JointAabb](../mesh/skinning/struct.JointAabb.html "struct bevy::mesh::skinning::JointAabb")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#336)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [JointIndex](../mesh/skinning/struct.JointIndex.html "struct bevy::mesh::skinning::JointIndex")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#346)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [JumpAt](../prelude/enum.JumpAt.html "enum bevy::prelude::JumpAt")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#230)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Justify](../prelude/enum.Justify.html "enum bevy::prelude::Justify")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1102)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [JustifyContent](../prelude/enum.JustifyContent.html "enum bevy::prelude::JustifyContent")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#938)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [JustifyItems](../prelude/enum.JustifyItems.html "enum bevy::prelude::JustifyItems")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1018)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [JustifySelf](../prelude/enum.JustifySelf.html "enum bevy::prelude::JustifySelf")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#804)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Key](../input/keyboard/enum.Key.html "enum bevy::input::keyboard::Key")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#262)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [KeyCode](../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#152)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [KeyboardFocusLost](../input/keyboard/struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#103)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [KeyboardInput](../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#17)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Laba](../prelude/struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/label.rs.html#5)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Label](../prelude/struct.Label.html "struct bevy::prelude::Label")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2903)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LayoutConfig](../prelude/struct.LayoutConfig.html "struct bevy::prelude::LayoutConfig")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#17)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Lcha](../prelude/struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#273)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Leave](../prelude/struct.Leave.html "struct bevy::prelude::Leave")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LensDistortion](../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1039)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LetterSpacing](../text/enum.LetterSpacing.html "enum bevy::text::LetterSpacing")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#151)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LightGizmoColor](../prelude/enum.LightGizmoColor.html "enum bevy::prelude::LightGizmoColor")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#166)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LightGizmoConfigGroup](../prelude/struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#71)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LightProbe](../prelude/struct.LightProbe.html "struct bevy::prelude::LightProbe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Lightmap](../pbr/struct.Lightmap.html "struct bevy::pbr::Lightmap")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1234)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Line2d](../prelude/struct.Line2d.html "struct bevy::prelude::Line2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#357)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Line3d](../prelude/struct.Line3d.html "struct bevy::prelude::Line3d")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1112)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LineBreak](../prelude/enum.LineBreak.html "enum bevy::prelude::LineBreak")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LineGizmoEntities](../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1011)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LineHeight](../text/enum.LineHeight.html "enum bevy::text::LineHeight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#227)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LinearGradient](../prelude/struct.LinearGradient.html "struct bevy::prelude::LinearGradient")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LinearRgba](../prelude/struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#39)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ListItem](../ui_widgets/struct.ListItem.html "struct bevy::ui_widgets::ListItem")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#210)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for bevy::picking::pointer::[Location](../picking/pointer/struct.Location.html "struct bevy::picking::pointer::Location")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#158)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MainEntity](../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#142)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MainPassResolutionOverride](../camera/struct.MainPassResolutionOverride.html "struct bevy::camera::MainPassResolutionOverride")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#156)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ManageAccessibilityUpdates](../a11y/struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#976)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ManualTextureViewHandle](../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#343-350)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mat2](../prelude/struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#351-359)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mat3](../prelude/struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mat4](../prelude/struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#360-368)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mat3A](../prelude/struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#276)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MaterialBindGroupIndex](../pbr/struct.MaterialBindGroupIndex.html "struct bevy::pbr::MaterialBindGroupIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#294)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MaterialBindGroupSlot](../pbr/struct.MaterialBindGroupSlot.html "struct bevy::pbr::MaterialBindGroupSlot")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#259)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MaterialBindingId](../pbr/struct.MaterialBindingId.html "struct bevy::pbr::MaterialBindingId")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1569)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MaxTrackSizingFunction](../prelude/enum.MaxTrackSizingFunction.html "enum bevy::prelude::MaxTrackSizingFunction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#61)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MenuAction](../ui_widgets/enum.MenuAction.html "enum bevy::ui_widgets::MenuAction")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#414)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MenuButton](../ui_widgets/struct.MenuButton.html "struct bevy::ui_widgets::MenuButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#79)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MenuEvent](../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#139)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MenuFocusState](../ui_widgets/enum.MenuFocusState.html "enum bevy::ui_widgets::MenuFocusState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#133)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MenuItem](../ui_widgets/struct.MenuItem.html "struct bevy::ui_widgets::MenuItem")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#91)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MenuLayout](../ui_widgets/enum.MenuLayout.html "enum bevy::ui_widgets::MenuLayout")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#123)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MenuPopup](../ui_widgets/struct.MenuPopup.html "struct bevy::ui_widgets::MenuPopup")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#225)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mesh](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mesh2d](../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mesh2dWireframe](../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mesh3d](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Mesh3dWireframe](../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#118)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MeshMorphWeights](../mesh/morph/enum.MeshMorphWeights.html "enum bevy::mesh::morph::MeshMorphWeights")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#33)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MeshPickingCamera](../prelude/struct.MeshPickingCamera.html "struct bevy::prelude::MeshPickingCamera")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#38)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MeshPickingSettings](../prelude/struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#154)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MeshTag](../mesh/struct.MeshTag.html "struct bevy::mesh::MeshTag")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MeshletMesh3d](../pbr/experimental/meshlet/struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1540)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MinTrackSizingFunction](../prelude/enum.MinTrackSizingFunction.html "enum bevy::prelude::MinTrackSizingFunction")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#805)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MipBias](../render/camera/struct.MipBias.html "struct bevy::render::camera::MipBias")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#24)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Monitor](../window/struct.Monitor.html "struct bevy::window::Monitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1147)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MonitorSelection](../prelude/enum.MonitorSelection.html "enum bevy::prelude::MonitorSelection")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MorphAttributes](../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#79)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MorphWeights](../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#73)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MotionBlur](../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#76)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MotionVectorPrepass](../core_pipeline/prepass/struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#64)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MouseButton](../prelude/enum.MouseButton.html "enum bevy::prelude::MouseButton")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#34)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MouseButtonInput](../input/mouse/struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#99)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MouseMotion](../input/mouse/struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#121)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MouseScrollUnit](../input/mouse/enum.MouseScrollUnit.html "enum bevy::input::mouse::MouseScrollUnit")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#160)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MouseWheel](../input/mouse/struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#323)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Move](../prelude/struct.Move.html "struct bevy::prelude::Move")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#231)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Msaa](../prelude/enum.Msaa.html "enum bevy::prelude::Msaa")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#29)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MsaaWriteback](../prelude/enum.MsaaWriteback.html "enum bevy::prelude::MsaaWriteback")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Name](../prelude/struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#758)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NativeKey](../input/keyboard/enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#220)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NativeKeyCode](../input/keyboard/enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#107)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NavAction](../input_focus/tab_navigation/enum.NavAction.html "enum bevy::input_focus::tab_navigation::NavAction")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#159)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NavNeighbor](../input_focus/directional_navigation/enum.NavNeighbor.html "enum bevy::input_focus::directional_navigation::NavNeighbor")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#187)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NavNeighbors](../input_focus/directional_navigation/struct.NavNeighbors.html "struct bevy::input_focus::directional_navigation::NavNeighbors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#550)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NoAutoAabb](../camera/visibility/struct.NoAutoAabb.html "struct bevy::camera::visibility::NoAutoAabb")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#55)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NoBackgroundMotionVectors](../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#316)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NoFrustumCulling](../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#868)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NoWireframe](../pbr/wireframe/struct.NoWireframe.html "struct bevy::pbr::wireframe::NoWireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#418)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NoWireframe2d](../sprite_render/struct.NoWireframe2d.html "struct bevy::sprite_render::NoWireframe2d")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#471)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Node](../prelude/struct.Node.html "struct bevy::prelude::Node")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#156)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NodeImageMode](../prelude/enum.NodeImageMode.html "enum bevy::prelude::NodeImageMode")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#15-22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NonNilUuid](../asset/uuid/struct.NonNilUuid.html "struct bevy::asset::uuid::NonNilUuid")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#68)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NormalPrepass](../core_pipeline/prepass/struct.NormalPrepass.html "struct bevy::core_pipeline::prepass::NormalPrepass")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#940)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NormalizedRenderTarget](../camera/enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#105)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NormalizedWindowRef](../window/struct.NormalizedWindowRef.html "struct bevy::window::NormalizedWindowRef")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#256)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NotShadowCaster](../light/struct.NotShadowCaster.html "struct bevy::light::NotShadowCaster")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#264)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NotShadowReceiver](../light/struct.NotShadowReceiver.html "struct bevy::light::NotShadowReceiver")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#131)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NumberFormat](../feathers/controls/enum.NumberFormat.html "enum bevy::feathers::controls::NumberFormat")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#146)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NumberInputValue](../feathers/controls/enum.NumberInputValue.html "enum bevy::feathers::controls::NumberInputValue")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#501)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ObservedBy](../ecs/observer/struct.ObservedBy.html "struct bevy::ecs::observer::ObservedBy")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OcclusionCulling](../render/occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#298)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OffsetAccess](struct.OffsetAccess.html "struct bevy::reflect::OffsetAccess")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#17)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Oklaba](../prelude/struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Oklcha](../prelude/struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/opaque.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OpaqueRendererMethod](../material/enum.OpaqueRendererMethod.html "enum bevy::material::OpaqueRendererMethod")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OrderIndependentTransparencySettings](../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#578)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OrthographicProjection](../prelude/struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#240)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Out](../prelude/struct.Out.html "struct bevy::prelude::Out")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2456)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OuterColor](../prelude/struct.OuterColor.html "struct bevy::prelude::OuterColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2315)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Outline](../prelude/struct.Outline.html "struct bevy::prelude::Outline")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#190)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Over](../prelude/struct.Over.html "struct bevy::prelude::Over")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Overflow](../prelude/struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1347)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OverflowAxis](../prelude/enum.OverflowAxis.html "enum bevy::prelude::OverflowAxis")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1381)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OverflowClipMargin](../prelude/struct.OverflowClipMargin.html "struct bevy::prelude::OverflowClipMargin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2416)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OverrideClip](../prelude/struct.OverrideClip.html "struct bevy::prelude::OverrideClip")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#47)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [OverrideCursor](../feathers/cursor/struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#84)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PanGesture](../input/gestures/struct.PanGesture.html "struct bevy::input::gestures::PanGesture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#410)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ParallaxCorrection](../light/enum.ParallaxCorrection.html "enum bevy::light::ParallaxCorrection")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/parallax.rs.html#14)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ParallaxMappingMethod](../prelude/enum.ParallaxMappingMethod.html "enum bevy::prelude::ParallaxMappingMethod")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#367)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ParsedPath](struct.ParsedPath.html "struct bevy::reflect::ParsedPath")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/pathtracer/mod.rs.html#63)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Pathtracer](../solari/pathtracer/struct.Pathtracer.html "struct bevy::solari::pathtracer::Pathtracer")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#281)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PerspectiveProjection](../prelude/struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#196)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Pickable](../prelude/struct.Pickable.html "struct bevy::prelude::Pickable")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#224)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PickingInteraction](../picking/hover/enum.PickingInteraction.html "enum bevy::picking::hover::PickingInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#296)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PickingSettings](../picking/struct.PickingSettings.html "struct bevy::picking::PickingSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#25)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PinchGesture](../input/gestures/struct.PinchGesture.html "struct bevy::input::gestures::PinchGesture")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1192)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Plane2d](../prelude/struct.Plane2d.html "struct bevy::prelude::Plane2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#96)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Plane3d](../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PlaneMeshBuilder](../mesh/struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#9)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PlaybackMode](../audio/enum.PlaybackMode.html "enum bevy::audio::PlaybackMode")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#33)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PlaybackSettings](../prelude/struct.PlaybackSettings.html "struct bevy::prelude::PlaybackSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#38)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointLight](../prelude/struct.PointLight.html "struct bevy::prelude::PointLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#177)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointLightShadowMap](../light/struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#159)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointLightTexture](../light/struct.PointLightTexture.html "struct bevy::light::PointLightTexture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#248)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerAction](../picking/pointer/enum.PointerAction.html "enum bevy::picking::pointer::PointerAction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#159)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerButton](../prelude/enum.PointerButton.html "enum bevy::prelude::PointerButton")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#91)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerHits](../picking/backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#31)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerId](../picking/pointer/enum.PointerId.html "enum bevy::picking::pointer::PointerId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#278)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerInput](../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#42)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerInputSettings](../picking/input/struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#71)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerInteraction](../picking/pointer/struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#178)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerLocation](../picking/pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#114)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PointerPress](../picking/pointer/struct.PointerPress.html "struct bevy::picking::pointer::PointerPress")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1894)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Polygon](../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1566)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Polyline2d](../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#701)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Polyline2dMeshBuilder](../mesh/struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#624)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Polyline3d](../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#84)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Popover](../ui_widgets/popover/struct.Popover.html "struct bevy::ui_widgets::popover::Popover")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#52)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PopoverAlign](../ui_widgets/popover/enum.PopoverAlign.html "enum bevy::ui_widgets::popover::PopoverAlign")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#69)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PopoverPlacement](../ui_widgets/popover/struct.PopoverPlacement.html "struct bevy::ui_widgets::popover::PopoverPlacement")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#23)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PopoverSide](../ui_widgets/popover/enum.PopoverSide.html "enum bevy::ui_widgets::popover::PopoverSide")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1453)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PositionType](../prelude/enum.PositionType.html "enum bevy::prelude::PositionType")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#13)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PositionedGlyph](../text/struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#16)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PreeditCursor](../text/struct.PreeditCursor.html "struct bevy::text::PreeditCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1214)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PresentMode](../window/enum.PresentMode.html "enum bevy::window::PresentMode")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#286)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Press](../prelude/struct.Press.html "struct bevy::prelude::Press")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#149)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PressDirection](../picking/pointer/enum.PressDirection.html "enum bevy::picking::pointer::PressDirection")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#44)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Pressed](../ui/struct.Pressed.html "struct bevy::ui::Pressed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#53)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PrimaryMonitor](../window/struct.PrimaryMonitor.html "struct bevy::window::PrimaryMonitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#53)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PrimaryWindow](../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#214)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Projection](../prelude/enum.Projection.html "enum bevy::prelude::Projection")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Quat](../prelude/struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#361)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RadialGradient](../prelude/struct.RadialGradient.html "struct bevy::prelude::RadialGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#558)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RadialGradientShape](../prelude/enum.RadialGradientShape.html "enum bevy::prelude::RadialGradientShape")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#58)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RadioButton](../ui_widgets/struct.RadioButton.html "struct bevy::ui_widgets::RadioButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#40)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RadioGroup](../ui_widgets/struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#118)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RawGamepadAxisChangedEvent](../input/gamepad/struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#86)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RawGamepadButtonChangedEvent](../input/gamepad/struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#65)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RawGamepadEvent](../input/gamepad/enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#17)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Ray2d](../prelude/struct.Ray2d.html "struct bevy::prelude::Ray2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#74)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Ray3d](../prelude/struct.Ray3d.html "struct bevy::prelude::Ray3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#12)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RayCast2d](../math/bounding/struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#12)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RayCast3d](../math/bounding/struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#106)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RayCastBackfaces](../prelude/struct.RayCastBackfaces.html "struct bevy::prelude::RayCastBackfaces")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RayCastVisibility](../prelude/enum.RayCastVisibility.html "enum bevy::prelude::RayCastVisibility")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#245)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RayId](../picking/backend/ray/struct.RayId.html "struct bevy::picking::backend::ray::RayId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/intersections.rs.html#8)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RayMeshHit](../picking/mesh_picking/ray_cast/struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/types.rs.html#19)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RaytracingMesh3d](../solari/scene/struct.RaytracingMesh3d.html "struct bevy::solari::scene::RaytracingMesh3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ReadbackComplete](../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#44)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Real](../prelude/struct.Real.html "struct bevy::prelude::Real")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/rect.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Rect](../prelude/struct.Rect.html "struct bevy::prelude::Rect")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/rect_light.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RectLight](../prelude/struct.RectLight.html "struct bevy::prelude::RectLight")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1801)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Rectangle](../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1041)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RectangleMeshBuilder](../mesh/struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2036)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RegularPolygon](../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#482)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RegularPolygonMeshBuilder](../mesh/struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#78)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RelativeCursorPosition](../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#298)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Release](../prelude/struct.Release.html "struct bevy::prelude::Release")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#376)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Remove](../prelude/struct.Remove.html "struct bevy::prelude::Remove")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#399)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RemovedComponentEntity](../ecs/lifecycle/struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/render_asset.rs.html#29)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderAssetUsages](../asset/struct.RenderAssetUsages.html "struct bevy::asset::RenderAssetUsages")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#319)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderDebugMode](../dev_tools/render_debug/enum.RenderDebugMode.html "enum bevy::dev_tools::render_debug::RenderDebugMode")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderDebugOverlay](../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#262)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderDebugOverlayEvent](../dev_tools/render_debug/enum.RenderDebugOverlayEvent.html "enum bevy::dev_tools::render_debug::RenderDebugOverlayEvent")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#129)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderEntity](../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/render_layers.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderLayers](../camera/visibility/struct.RenderLayers.html "struct bevy::camera::visibility::RenderLayers")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#63)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderShadowMapVisibleEntities](../render/view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#890)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderTarget](../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#196)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderTargetInfo](../camera/struct.RenderTargetInfo.html "struct bevy::camera::RenderTargetInfo")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#87)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RenderVisibleEntitiesClass](../render/view/struct.RenderVisibleEntitiesClass.html "struct bevy::render::view::RenderVisibleEntitiesClass")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#469)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RepeatAnimation](../animation/enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RepeatedGridTrack](../prelude/struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#53)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RequestRedraw](../window/struct.RequestRedraw.html "struct bevy::window::RequestRedraw")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2802)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ResolvedBorderRadius](../prelude/struct.ResolvedBorderRadius.html "struct bevy::prelude::ResolvedBorderRadius")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1055)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Rhombus](../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#878)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RhombusMeshBuilder](../mesh/struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#120)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RootNonCameraView](../core_pipeline/schedule/struct.RootNonCameraView.html "struct bevy::core_pipeline::schedule::RootNonCameraView")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#44)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Rot2](../prelude/struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#47)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RotationGesture](../input/gestures/struct.RotationGesture.html "struct bevy::input::gestures::RotationGesture")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#501)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RunGeometry](../text/struct.RunGeometry.html "struct bevy::text::RunGeometry")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#521)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScalingMode](../camera/enum.ScalingMode.html "enum bevy::camera::ScalingMode")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SceneComponentInfo](../scene/struct.SceneComponentInfo.html "struct bevy::scene::SceneComponentInfo")

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SchemaTypesMetadata](../remote/schemas/struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1478)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScreenEdge](../window/enum.ScreenEdge.html "enum bevy::window::ScreenEdge")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScreenSpaceAmbientOcclusion](../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#135)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScreenSpaceAmbientOcclusionQualityLevel](../pbr/enum.ScreenSpaceAmbientOcclusionQualityLevel.html "enum bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#78)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScreenSpaceReflections](../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScreenSpaceTransmission](../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#110)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScreenSpaceTransmissionQuality](../pbr/enum.ScreenSpaceTransmissionQuality.html "enum bevy::pbr::ScreenSpaceTransmissionQuality")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#78)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Screenshot](../render/view/window/screenshot/struct.Screenshot.html "struct bevy::render::view::window::screenshot::Screenshot")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScreenshotCaptured](../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#455)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Scroll](../prelude/struct.Scroll.html "struct bevy::prelude::Scroll")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollarea.rs.html#16)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScrollArea](../ui_widgets/struct.ScrollArea.html "struct bevy::ui_widgets::ScrollArea")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#417)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScrollPosition](../prelude/struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Scrollbar](../ui_widgets/struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#130)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScrollbarDragState](../ui_widgets/struct.ScrollbarDragState.html "struct bevy::ui_widgets::ScrollbarDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#100)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ScrollbarThumb](../ui_widgets/struct.ScrollbarThumb.html "struct bevy::ui_widgets::ScrollbarThumb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1254)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Segment2d](../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#376)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Segment3d](../prelude/struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/text_input.rs.html#406)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SelectAllOnFocus](../ui_widgets/struct.SelectAllOnFocus.html "struct bevy::ui_widgets::SelectAllOnFocus")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#29)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Sensitivity](../anti_alias/fxaa/enum.Sensitivity.html "enum bevy::anti_alias::fxaa::Sensitivity")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SetChecked](../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SetSliderValue](../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/storage.rs.html#27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShaderBuffer](../render/storage/struct.ShaderBuffer.html "struct bevy::render::storage::ShaderBuffer")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#283)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShadowFilteringMethod](../light/enum.ShadowFilteringMethod.html "enum bevy::light::ShadowFilteringMethod")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#854)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShadowLodOrigin](../camera/struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2868)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShadowStyle](../prelude/struct.ShadowStyle.html "struct bevy::prelude::ShadowStyle")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#61)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShowAabbGizmo](../prelude/struct.ShowAabbGizmo.html "struct bevy::prelude::ShowAabbGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#96)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShowFrustumGizmo](../prelude/struct.ShowFrustumGizmo.html "struct bevy::prelude::ShowFrustumGizmo")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#210)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShowLightGizmo](../prelude/struct.ShowLightGizmo.html "struct bevy::prelude::ShowLightGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#76)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ShowSkinnedMeshBoundsGizmo](../prelude/struct.ShowSkinnedMeshBoundsGizmo.html "struct bevy::prelude::ShowSkinnedMeshBoundsGizmo")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SimplifiedMesh](../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SkinnedMesh](../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#88)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SkinnedMeshBounds](../mesh/skinning/struct.SkinnedMeshBounds.html "struct bevy::mesh::skinning::SkinnedMeshBounds")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#52)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SkinnedMeshBoundsGizmoConfigGroup](../prelude/struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#227)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Skybox](../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#27)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliceScaleMode](../prelude/enum.SliceScaleMode.html "enum bevy::prelude::SliceScaleMode")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#103)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Slider](../ui_widgets/struct.Slider.html "struct bevy::ui_widgets::Slider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#147)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderBaseColor](../feathers/controls/struct.SliderBaseColor.html "struct bevy::feathers::controls::SliderBaseColor")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#245)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderDragState](../ui_widgets/struct.SliderDragState.html "struct bevy::ui_widgets::SliderDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#35)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderOrientation](../ui_widgets/enum.SliderOrientation.html "enum bevy::ui_widgets::SliderOrientation")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#233)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderPrecision](../ui_widgets/struct.SliderPrecision.html "struct bevy::ui_widgets::SliderPrecision")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#127)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderRange](../ui_widgets/struct.SliderRange.html "struct bevy::ui_widgets::SliderRange")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#214)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderStep](../ui_widgets/struct.SliderStep.html "struct bevy::ui_widgets::SliderStep")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#113)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderThumb](../ui_widgets/struct.SliderThumb.html "struct bevy::ui_widgets::SliderThumb")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#120)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderValue](../ui_widgets/struct.SliderValue.html "struct bevy::ui_widgets::SliderValue")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#683)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SliderValueChange](../ui_widgets/enum.SliderValueChange.html "enum bevy::ui_widgets::SliderValueChange")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Smaa](../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#106)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SmaaPreset](../anti_alias/smaa/enum.SmaaPreset.html "enum bevy::anti_alias::smaa::SmaaPreset")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/realtime/mod.rs.html#85)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SolariLighting](../solari/realtime/struct.SolariLighting.html "struct bevy::solari::realtime::SolariLighting")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#170)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpatialListener](../prelude/struct.SpatialListener.html "struct bevy::prelude::SpatialListener")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#203)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpatialScale](../audio/struct.SpatialScale.html "struct bevy::audio::SpatialScale")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#23)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for bevy::prelude::[Sphere](../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#196)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for bevy::camera::primitives::[Sphere](../camera/primitives/struct.Sphere.html "struct bevy::camera::primitives::Sphere")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#23)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SphereKind](../mesh/enum.SphereKind.html "enum bevy::mesh::SphereKind")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#51)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SphereMeshBuilder](../mesh/struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpotLight](../prelude/struct.SpotLight.html "struct bevy::prelude::SpotLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#204)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpotLightTexture](../light/struct.SpotLightTexture.html "struct bevy::light::SpotLightTexture")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Sprite](../prelude/struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#178)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpriteAlphaMode](../sprite/enum.SpriteAlphaMode.html "enum bevy::sprite::SpriteAlphaMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#166)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpriteImageMode](../prelude/enum.SpriteImageMode.html "enum bevy::prelude::SpriteImageMode")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpriteMaterial](../prelude/struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpriteMesh](../prelude/struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#34)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpritePickingCamera](../prelude/struct.SpritePickingCamera.html "struct bevy::prelude::SpritePickingCamera")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#39)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpritePickingMode](../prelude/enum.SpritePickingMode.html "enum bevy::prelude::SpritePickingMode")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#51)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpritePickingSettings](../prelude/struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#214)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SpriteScalingMode](../prelude/enum.SpriteScalingMode.html "enum bevy::prelude::SpriteScalingMode")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#20)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Srgba](../prelude/struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [StandardMaterial](../prelude/struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/systems.rs.html#88)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [StaticTransformOptimizations](../prelude/enum.StaticTransformOptimizations.html "enum bevy::prelude::StaticTransformOptimizations")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/stopwatch.rs.html#31)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Stopwatch](../time/struct.Stopwatch.html "struct bevy::time::Stopwatch")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1132)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Strikethrough](../prelude/struct.Strikethrough.html "struct bevy::prelude::Strikethrough")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1137)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [StrikethroughColor](../prelude/struct.StrikethroughColor.html "struct bevy::prelude::StrikethroughColor")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/string.rs.html#7-16)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [String](../prelude/struct.String.html "struct bevy::prelude::String")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#174)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SubCameraView](../camera/struct.SubCameraView.html "struct bevy::camera::SubCameraView")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#121)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SyncToRenderWorld](../render/sync_world/struct.SyncToRenderWorld.html "struct bevy::render::sync_world::SyncToRenderWorld")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/system_cursor.rs.html#89)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SystemCursorIcon](../window/enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#69)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TabGroup](../input_focus/tab_navigation/struct.TabGroup.html "struct bevy::input_focus::tab_navigation::TabGroup")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#60)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TabIndex](../input_focus/tab_navigation/struct.TabIndex.html "struct bevy::input_focus::tab_navigation::TabIndex")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#111)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TemporalAntiAliasing](../anti_alias/taa/struct.TemporalAntiAliasing.html "struct bevy::anti_alias::taa::TemporalAntiAliasing")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#780)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TemporalJitter](../render/camera/struct.TemporalJitter.html "struct bevy::render::camera::TemporalJitter")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#190)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TemporaryRenderEntity](../render/sync_world/struct.TemporaryRenderEntity.html "struct bevy::render::sync_world::TemporaryRenderEntity")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1433)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Tetrahedron](../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#8)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TetrahedronMeshBuilder](../mesh/struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#97)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Text](../prelude/struct.Text.html "struct bevy::prelude::Text")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#85)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Text2d](../prelude/struct.Text2d.html "struct bevy::prelude::Text2d")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#141)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Text2dShadow](../sprite/struct.Text2dShadow.html "struct bevy::sprite::Text2dShadow")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1088)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextBackgroundColor](../prelude/struct.TextBackgroundColor.html "struct bevy::prelude::TextBackgroundColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/bounds.rs.html#13)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextBounds](../text/struct.TextBounds.html "struct bevy::text::TextBounds")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1064)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextColor](../prelude/struct.TextColor.html "struct bevy::prelude::TextColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#25)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextEdit](../text/enum.TextEdit.html "enum bevy::text::TextEdit")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextEntity](../text/struct.TextEntity.html "struct bevy::text::TextEntity")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextFont](../prelude/struct.TextFont.html "struct bevy::prelude::TextFont")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#130)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextLayout](../prelude/struct.TextLayout.html "struct bevy::prelude::TextLayout")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#461)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextLayoutInfo](../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#32)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextNodeFlags](../ui/struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#32)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextScroll](../ui/widget/struct.TextScroll.html "struct bevy::ui::widget::TextScroll")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#144)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextShadow](../prelude/struct.TextShadow.html "struct bevy::prelude::TextShadow")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextSpan](../prelude/struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#211)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextureAtlas](../prelude/struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#95)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextureAtlasLayout](../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/wgpu_types.rs.html#3-10)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextureFormat](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#13)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TextureSlicer](../prelude/struct.TextureSlicer.html "struct bevy::prelude::TextureSlicer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#90)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThemeBackgroundColor](../feathers/theme/struct.ThemeBackgroundColor.html "struct bevy::feathers::theme::ThemeBackgroundColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#99)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThemeBorderColor](../feathers/theme/struct.ThemeBorderColor.html "struct bevy::feathers::theme::ThemeBorderColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#50)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThemeProps](../feathers/theme/struct.ThemeProps.html "struct bevy::feathers::theme::ThemeProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#118)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThemeTextColor](../feathers/theme/struct.ThemeTextColor.html "struct bevy::feathers::theme::ThemeTextColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#22)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThemeToken](../feathers/theme/struct.ThemeToken.html "struct bevy::feathers::theme::ThemeToken")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#125)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThemedText](../feathers/theme/struct.ThemedText.html "struct bevy::feathers::theme::ThemedText")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#298)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThreadedAnimationGraph](../prelude/struct.ThreadedAnimationGraph.html "struct bevy::prelude::ThreadedAnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#288)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ThreadedAnimationGraphs](../prelude/struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#15)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#94)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TileData](../sprite_render/struct.TileData.html "struct bevy::sprite_render::TileData")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/tile_orientation.rs.html#37)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TileOrientation](../sprite_render/enum.TileOrientation.html "enum bevy::sprite_render::TileOrientation")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TilemapChunk](../sprite_render/struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#46)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TilemapChunkMeshCache](../sprite_render/struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#130)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TilemapChunkTileData](../sprite_render/struct.TilemapChunkTileData.html "struct bevy::sprite_render::TilemapChunkTileData")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#31)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Timer](../prelude/struct.Timer.html "struct bevy::prelude::Timer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#492)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TimerMode](../prelude/enum.TimerMode.html "enum bevy::prelude::TimerMode")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ToggleChecked](../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#115)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Tonemapping](../core_pipeline/tonemapping/enum.Tonemapping.html "enum bevy::core_pipeline::tonemapping::Tonemapping")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1124)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Torus](../prelude/struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#8)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TorusMeshBuilder](../mesh/struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#45)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TouchInput](../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#123)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TouchPhase](../input/touch/enum.TouchPhase.html "enum bevy::input::touch::TouchPhase")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#61)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TrackClick](../ui_widgets/enum.TrackClick.html "enum bevy::ui_widgets::TrackClick")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Transform](../prelude/struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#123)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformGizmoAxis](../prelude/enum.TransformGizmoAxis.html "enum bevy::prelude::TransformGizmoAxis")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#95)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformGizmoCamera](../prelude/struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#85)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformGizmoFocus](../prelude/struct.TransformGizmoFocus.html "struct bevy::prelude::TransformGizmoFocus")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#101)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformGizmoMode](../prelude/enum.TransformGizmoMode.html "enum bevy::prelude::TransformGizmoMode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#136)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformGizmoSettings](../prelude/struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#113)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformGizmoSpace](../prelude/enum.TransformGizmoSpace.html "enum bevy::prelude::TransformGizmoSpace")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#179)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformGizmoState](../prelude/struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#666)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransformTreeChanged](../prelude/struct.TransformTreeChanged.html "struct bevy::prelude::TransformTreeChanged")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#274)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [TransmittedShadowReceiver](../light/struct.TransmittedShadowReceiver.html "struct bevy::light::TransmittedShadowReceiver")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1627)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Triangle2d](../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#964)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Triangle2dMeshBuilder](../mesh/struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1236)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Triangle3d](../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#7)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Triangle3dMeshBuilder](../mesh/struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#166-173)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U8Vec2](../math/struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#174-182)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U8Vec3](../math/struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#183-192)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#194-201)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U16Vec2](../math/struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#202-210)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U16Vec3](../math/struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#211-220)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#222-229)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U64Vec2](../math/struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#230-238)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U64Vec3](../math/struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#239-248)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/urect.rs.html#21)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [URect](../prelude/struct.URect.html "struct bevy::prelude::URect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UVec2](../prelude/struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#146-154)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UVec3](../prelude/struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UVec4](../prelude/struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#159)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiAntiAlias](../prelude/enum.UiAntiAlias.html "enum bevy::prelude::UiAntiAlias")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#39)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiDebugOptions](../prelude/struct.UiDebugOptions.html "struct bevy::prelude::UiDebugOptions")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#199)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiGlobalTransform](../prelude/struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#40)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiPickingCamera](../prelude/struct.UiPickingCamera.html "struct bevy::prelude::UiPickingCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#45)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiPickingSettings](../prelude/struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#993)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiPosition](../prelude/struct.UiPosition.html "struct bevy::prelude::UiPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiRect](../prelude/struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#124)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiScale](../prelude/struct.UiScale.html "struct bevy::prelude::UiScale")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#25)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiStack](../ui/struct.UiStack.html "struct bevy::ui::UiStack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2936)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiTargetCamera](../prelude/struct.UiTargetCamera.html "struct bevy::prelude::UiTargetCamera")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#59)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiTheme](../feathers/theme/struct.UiTheme.html "struct bevy::feathers::theme::UiTheme")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#122)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UiTransform](../prelude/struct.UiTransform.html "struct bevy::prelude::UiTransform")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1154)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Underline](../prelude/struct.Underline.html "struct bevy::prelude::Underline")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1159)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UnderlineColor](../prelude/struct.UnderlineColor.html "struct bevy::prelude::UnderlineColor")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#167)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UntypedAssetId](../asset/enum.UntypedAssetId.html "enum bevy::asset::UntypedAssetId")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#474)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UntypedHandle](../prelude/enum.UntypedHandle.html "enum bevy::prelude::UntypedHandle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UpdateNumberInput](../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/uuid.rs.html#4-13)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Uuid](../asset/uuid/struct.Uuid.html "struct bevy::asset::uuid::Uuid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#2531)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UvChannel](../mesh/enum.UvChannel.html "enum bevy::mesh::UvChannel")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#25)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Val](../prelude/enum.Val.html "enum bevy::prelude::Val")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#15)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Val2](../prelude/struct.Val2.html "struct bevy::prelude::Val2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec2](../prelude/struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec3](../prelude/struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec4](../prelude/struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#267-275)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec3A](../prelude/struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#72)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VideoMode](../window/struct.VideoMode.html "struct bevy::window::VideoMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1175)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VideoModeSelection](../prelude/enum.VideoModeSelection.html "enum bevy::prelude::VideoModeSelection")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/view_frustum.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ViewFrustum](../prelude/struct.ViewFrustum.html "struct bevy::prelude::ViewFrustum")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#224)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ViewVisibility](../prelude/struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#60)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Viewport](../camera/struct.Viewport.html "struct bevy::camera::Viewport")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/viewport.rs.html#36)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ViewportNode](../prelude/struct.ViewportNode.html "struct bevy::prelude::ViewportNode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#28)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vignette](../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#74)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Virtual](../prelude/struct.Virtual.html "struct bevy::prelude::Virtual")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#80)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Visibility](../prelude/enum.Visibility.html "enum bevy::prelude::Visibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#208)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VisibilityClass](../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#78)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VisibilityRange](../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#342)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VisibleEntities](../camera/visibility/struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#408)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VisibleMeshEntities](../camera/visibility/struct.VisibleMeshEntities.html "struct bevy::camera::visibility::VisibleMeshEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1435)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VisualBox](../prelude/enum.VisualBox.html "enum bevy::prelude::VisualBox")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#34)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Volume](../audio/enum.Volume.html "enum bevy::audio::Volume")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#23)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VolumetricFog](../light/struct.VolumetricFog.html "struct bevy::light::VolumetricFog")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#14)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VolumetricLight](../light/struct.VolumetricLight.html "struct bevy::light::VolumetricLight")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#64)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WeightsCurveSample](../prelude/struct.WeightsCurveSample.html "struct bevy::prelude::WeightsCurveSample")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#155)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Window](../prelude/struct.Window.html "struct bevy::prelude::Window")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#357)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowBackendScaleFactorChanged](../window/struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#95)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowCloseRequested](../window/struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#113)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowClosed](../window/struct.WindowClosed.html "struct bevy::window::WindowClosed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#134)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowClosing](../window/struct.WindowClosing.html "struct bevy::window::WindowClosing")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#69)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowCreated](../window/struct.WindowCreated.html "struct bevy::window::WindowCreated")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#154)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowDestroyed](../window/struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#496)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowEvent](../window/enum.WindowEvent.html "enum bevy::window::WindowEvent")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#292)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowFocused](../window/struct.WindowFocused.html "struct bevy::window::WindowFocused")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1382)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowLevel](../window/enum.WindowLevel.html "enum bevy::window::WindowLevel")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1334)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowMode](../window/enum.WindowMode.html "enum bevy::window::WindowMode")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#412)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowMoved](../prelude/struct.WindowMoved.html "struct bevy::prelude::WindowMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#319)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowOccluded](../window/struct.WindowOccluded.html "struct bevy::window::WindowOccluded")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#796)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowPosition](../prelude/enum.WindowPosition.html "enum bevy::prelude::WindowPosition")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#64)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowRef](../window/enum.WindowRef.html "enum bevy::window::WindowRef")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#675)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowResizeConstraints](../prelude/struct.WindowResizeConstraints.html "struct bevy::prelude::WindowResizeConstraints")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#31)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowResized](../window/struct.WindowResized.html "struct bevy::window::WindowResized")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#889)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowResolution](../window/struct.WindowResolution.html "struct bevy::window::WindowResolution")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#338)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowScaleFactorChanged](../window/struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1406)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowTheme](../window/enum.WindowTheme.html "enum bevy::window::WindowTheme")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#434)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WindowThemeChanged](../window/struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged")

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#175)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WinitUserEvent](../winit/enum.WinitUserEvent.html "enum bevy::winit::WinitUserEvent")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#199)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Wireframe](../pbr/wireframe/struct.Wireframe.html "struct bevy::pbr::wireframe::Wireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#163)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Wireframe2d](../sprite_render/struct.Wireframe2d.html "struct bevy::sprite_render::Wireframe2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#403)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Wireframe2dColor](../sprite_render/struct.Wireframe2dColor.html "struct bevy::sprite_render::Wireframe2dColor")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Wireframe2dConfig](../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#434)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Wireframe2dMaterial](../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#843)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WireframeColor](../pbr/wireframe/struct.WireframeColor.html "struct bevy::pbr::wireframe::WireframeColor")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WireframeConfig](../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#852)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WireframeLineWidth](../pbr/wireframe/struct.WireframeLineWidth.html "struct bevy::pbr::wireframe::WireframeLineWidth")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#910)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WireframeMaterial](../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#873)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WireframeTopology](../pbr/wireframe/enum.WireframeTopology.html "enum bevy::pbr::wireframe::WireframeTopology")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WorldAssetRoot](../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WorldInstanceReady](../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#17)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Xyza](../prelude/struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2438)

### impl [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ZIndex](../prelude/struct.ZIndex.html "struct bevy::prelude::ZIndex")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/access.rs.html#16)

### impl<'a> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Access](enum.Access.html "enum bevy::reflect::Access")<'a>

where [Access](enum.Access.html "enum bevy::reflect::Access")<'a>: 'static,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/path.rs.html#54)

### impl<'a> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AssetPath](../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>

where [AssetPath](../asset/struct.AssetPath.html "struct bevy::asset::AssetPath")<'a>: 'static,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#303)

### impl<A> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimatableCurveEvaluator](../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>

where A: [Animatable](../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AnimatableCurveEvaluator](../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), BasicAnimationCurveEvaluator<A>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AnimatableProperty](../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty")<Property = A>>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/event.rs.html#49)

### impl<A> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AssetEvent](../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetEvent](../prelude/enum.AssetEvent.html "enum bevy::prelude::AssetEvent")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [AssetId](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/id.rs.html#21)

### impl<A> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AssetId](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [AssetId](../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#132)

### impl<A> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>

where A: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#142)

### impl<B, E> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ExtendedMaterial](../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, E: [MaterialExtension](../pbr/trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [ExtendedMaterial](../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#96)

### impl<C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Inherited](../app/struct.Inherited.html "struct bevy::app::Inherited")<C>

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Inherited](../app/struct.Inherited.html "struct bevy::app::Inherited")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#70)

### impl<C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Propagate](../app/struct.Propagate.html "struct bevy::app::Propagate")<C>

where C: [Component](../prelude/trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Propagate](../app/struct.Propagate.html "struct bevy::app::Propagate")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#78)

### impl<C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PropagateOver](../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>

where [PropagateOver](../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> C>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#83)

### impl<C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PropagateStop](../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>

where [PropagateStop](../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> C>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#152)

### impl<C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SampleDerivativeWrapper](../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

where [SampleDerivativeWrapper](../prelude/derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#185)

### impl<C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SampleTwoDerivativesWrapper](../prelude/derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>

where [SampleTwoDerivativesWrapper](../prelude/derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#19)

### impl<C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WeightsCurve](../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>

where [WeightsCurve](../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#284)

### impl<Config, Clear> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Config: [GizmoConfigGroup](../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Pointer](../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Pointer](../prelude/struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_map.rs.html#12)

### impl<K, V, S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for bevy::platform::collections::[HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<K, V, S>

where K: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), V: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"), S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#189)

### impl<M> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FocusedInput](../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [FocusedInput](../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#172)

### impl<M> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MaterialNode](../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

where M: [UiMaterial](../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MaterialNode](../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

### impl<M> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MeshMaterial2d](../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

where M: [Material2d](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial2d](../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

### impl<M> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MeshMaterial3d](../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

where M: [Material](../prelude/trait.Material.html "trait bevy::prelude::Material") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial3d](../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/mod.rs.html#117)

### impl<M> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MessageId](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [MessageId](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/messages.rs.html#94)

### impl<M> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Messages](../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>

where M: [Message](../prelude/trait.Message.html "trait bevy::prelude::Message") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Messages](../prelude/struct.Messages.html "struct bevy::prelude::Messages")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), MessageSequence<M>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#286)

### impl<P, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimatableCurve](../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>

where [AnimatableCurve](../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), P: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#434)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicBSpline](../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBSpline](../prelude/struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#54)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicBezier](../prelude/struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBezier](../prelude/struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#272)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicCardinalSpline](../prelude/struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCardinalSpline](../prelude/struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1169)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicCurve](../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCurve](../prelude/struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[CubicSegment](../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#144)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicHermite](../prelude/struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicHermite](../prelude/struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[(P, P)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#611)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicNurbs](../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicNurbs](../prelude/struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#946)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicSegment](../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicSegment](../prelude/struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#837)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LinearSpline](../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [LinearSpline](../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1470)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RationalCurve](../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalCurve](../prelude/struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[RationalSegment](../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1328)

### impl<P> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RationalSegment](../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalSegment](../prelude/struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#482)

### impl<S, T, C, D> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ZipCurve](../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

where [ZipCurve](../prelude/struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#188)

### impl<S, T, C, F> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MapCurve](../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>

where [MapCurve](../prelude/struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#229)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DespawnOnEnter](../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DespawnOnEnter](../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#148)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DespawnOnExit](../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DespawnOnExit](../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#66)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DespawnWhen](../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DespawnWhen](../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#476)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DisableOnEnter](../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DisableOnEnter](../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#395)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DisableOnExit](../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DisableOnExit](../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#313)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [DisableWhen](../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [DisableWhen](../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#723)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EnableOnEnter](../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [EnableOnEnter](../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#642)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EnableOnExit](../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [EnableOnExit](../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#560)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EnableWhen](../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EnableWhen](../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#178)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [NextState](../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [NextState](../prelude/enum.NextState.html "enum bevy::prelude::NextState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#131)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PreviousState](../prelude/struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [PreviousState](../prelude/struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#55)

### impl<S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [State](../prelude/struct.State.html "struct bevy::prelude::State")<S>

where S: [States](../prelude/trait.States.html "trait bevy::prelude::States") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [State](../prelude/struct.State.html "struct bevy::prelude::State")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

### impl<Source> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AudioPlayer](../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>

where [AudioPlayer](../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Source: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Decodable](../prelude/trait.Decodable.html "trait bevy::prelude::Decodable") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<Source>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#524)

### impl<T, C, D> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ChainCurve](../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

where [ChainCurve](../prelude/struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#787)

### impl<T, C, D> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ContinuationCurve](../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

where [ContinuationCurve](../prelude/struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#417)

### impl<T, C, D> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CurveReparamCurve](../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

where [CurveReparamCurve](../prelude/struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#285)

### impl<T, C, F> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ReparamCurve](../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>

where [ReparamCurve](../prelude/struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#677)

### impl<T, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ForeverCurve](../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

where [ForeverCurve](../prelude/struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#451)

### impl<T, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [GraphCurve](../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

where [GraphCurve](../prelude/struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#381)

### impl<T, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [LinearReparamCurve](../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

where [LinearReparamCurve](../prelude/struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#732)

### impl<T, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [PingPongCurve](../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

where [PingPongCurve](../prelude/struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#617)

### impl<T, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [RepeatCurve](../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

where [RepeatCurve](../prelude/struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#575)

### impl<T, C> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ReverseCurve](../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

where [ReverseCurve](../prelude/struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#86)

### impl<T, F> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [FunctionCurve](../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>

where [FunctionCurve](../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/hash.rs.html#7)

### impl<T, H> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Hashed](../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<T, H>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), H: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Hashed](../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<T, H>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#26)

### impl<T, I> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SampleCurve](../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>

where [SampleCurve](../prelude/struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [EvenCore](../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#186)

### impl<T, I> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UnevenSampleCurve](../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>

where [UnevenSampleCurve](../prelude/struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [UnevenCore](../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [PartialReflect](../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#722)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [AnimatableKeyframeCurve](../prelude/struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>

where [AnimatableKeyframeCurve](../prelude/struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/sync.rs.html#3)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Arc](../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#301)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ArcMutexValue](../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>

where T: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ArcMutexValue](../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/axis.rs.html#16)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Axis](../prelude/struct.Axis.html "struct bevy::prelude::Axis")<T>

where [Axis](../prelude/struct.Axis.html "struct bevy::prelude::Axis")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<T, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/button_input.rs.html#124)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ButtonInput](../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ButtonInput](../prelude/struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [HashSet](../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#467)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ChunkedUnevenCore](../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>

where [ChunkedUnevenCore](../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_gradient.rs.html#11)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ColorCurve](../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>

where [ColorCurve](../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EvenCore](../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#46)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ConstantCurve](../prelude/struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

where [ConstantCurve](../prelude/struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#50)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [CubicKeyframeCurve](../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>

where [CubicKeyframeCurve](../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#298)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EasingCurve](../prelude/struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>

where [EasingCurve](../prelude/struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#122)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EvenCore](../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>

where [EvenCore](../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#272)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [HandleTemplate](../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>

where T: [Asset](../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HandleTemplate](../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](../prelude/enum.Handle.html "enum bevy::prelude::Handle")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [ArcMutexValue](../asset/struct.ArcMutexValue.html "struct bevy::asset::ArcMutexValue")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#45)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>

where T: [Internable](../ecs/intern/trait.Internable.html "trait bevy::ecs::intern::Internable") + 'static + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [&'static T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#25)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [InterpolationDatum](../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>

where [InterpolationDatum](../prelude/cores/enum.InterpolationDatum.html "enum bevy::prelude::cores::InterpolationDatum")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/maybe_location.rs.html#20)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [MaybeLocation](../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>

where [MaybeLocation](../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#139)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SampleAutoCurve](../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>

where [SampleAutoCurve](../prelude/struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [EvenCore](../prelude/struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#12)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [SteppedKeyframeCurve](../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>

where [SteppedKeyframeCurve](../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Time](../prelude/struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](../prelude/struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#326)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UnevenCore](../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>

where [UnevenCore](../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#314)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [UnevenSampleAutoCurve](../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>

where [UnevenSampleAutoCurve](../prelude/struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](../prelude/struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [ValueChange](../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>

where [ValueChange](../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/alloc/vec.rs.html#10-17)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<T>

where T: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration"),

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VirtualKeyPressed](../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>

where [VirtualKeyPressed](../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#22)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [VirtualKeyboard](../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [VirtualKeyboard](../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#285)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WideCubicKeyframeCurve](../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>

where [WideCubicKeyframeCurve](../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#174)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WideLinearKeyframeCurve](../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>

where [WideLinearKeyframeCurve](../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#228)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WideSteppedKeyframeCurve](../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>

where [WideSteppedKeyframeCurve](../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](../prelude/cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#602)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WithDerivative](../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

where [WithDerivative](../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, <T as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#617)

### impl<T> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [WithTwoDerivatives](../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>

where [WithTwoDerivatives](../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, <T as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, <<T as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent") as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/bevy_platform/collections/hash_set.rs.html#9)

### impl<V, S> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for bevy::platform::collections::[HashSet](../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<V, S>

where V: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash"), S: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html "trait core::hash::BuildHasher") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#159)

### impl<V, W> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [Sum](../math/struct.Sum.html "struct bevy::math::Sum")<V, W>

where [Sum](../math/struct.Sum.html "struct bevy::math::Sum")<V, W>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, W: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_map.rs.html#19)

### impl<V> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityHashMap](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>

where [EntityHashMap](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") for [EntityIndexMap](../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](../prelude/struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](../prelude/trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,