[bevy](../../index.html)::[reflect](../index.html)::[func](index.html)

# Enum ArgValue 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/arg.rs.html#198)

```rust
pub enum ArgValue<'a> {
    Owned(Box<dyn PartialReflect>),
    Ref(&'a (dyn PartialReflect + 'static)),
    Mut(&'a mut (dyn PartialReflect + 'static)),
}
```

Available on **crate feature `functions`** only.

Represents an argument that can be passed to a [`DynamicFunction`](struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

## Variants

### Owned([Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

An owned argument.

### Ref(&'a (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

An immutable reference argument.

### Mut(&'a mut (dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

A mutable reference argument.

## Methods from [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#459)

#### pub fn [represents](#method.represents)<T>(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Reflect](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

Returns `true` if the underlying value represents a value of type `T`, or `false` otherwise.

Read `is` for more information on underlying values and represented types.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/reflection/dynamic\_types.rs ([line 89](../../../src/dynamic_types/dynamic_types.rs.html#89))

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflect.rs.html#494)

#### pub fn [try\_downcast\_ref](#method.try_downcast_ref)<T>(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Downcasts the value to type `T` by reference.

If the underlying value does not implement [`Reflect`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") or is not of type `T`, returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None").

For remote types, `T` should be the type itself rather than the wrapper type.

##### [Examples found in repository](#scraped-examples-1)[?](../../../scrape-examples-help.html)

examples/reflection/reflection\_types.rs ([line 49](../../../src/reflection_types/reflection_types.rs.html#49))

```rust
49#[reflect(Hash, PartialEq, Clone)]
50pub struct E {
51    x: usize,
52}
53
54/// By default, deriving with Reflect assumes the type is either a "struct" or an "enum".
55///
56/// You can tell reflect to treat your type instead as an "opaque type" by using the `#[reflect(opaque)]`.
57/// It is generally a good idea to implement (and reflect) the `PartialEq` and `Clone` (optionally also `Serialize` and `Deserialize`)
58/// traits on opaque types to ensure that these values behave as expected when nested in other reflected types.
59#[derive(Reflect, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
60#[reflect(opaque)]
61#[reflect(PartialEq
```

Hide additional examples

examples/reflection/reflection.rs ([line 78](../../../src/reflection/reflection.rs.html#78))

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

examples/reflection/function\_reflection.rs ([line 162](../../../src/function_reflection/function_reflection.rs.html#162))

```rust
19fn main() {
20    // There are times when it may be helpful to store a function away for later.
21    // In Rust, we can do this by storing either a function pointer or a function trait object.
22    // For example, say we wanted to store the following function:
23    fn add(left: i32, right: i32) -> i32 {
24        left + right
25    }
26
27    // We could store it as either of the following:
28    let fn_pointer: fn(i32, i32) -> i32 = add;
29    let fn_trait_object: Box<dyn Fn(i32, i32) -> i32> = Box::new(add);
30
31    // And we can call them like so:
32    let result = fn_pointer(2, 2);
33    assert_eq!(result, 4);
34    let result = fn_trait_object(2, 2);
35    assert_eq!(result, 4);
36
37    // However, you'll notice that we have to know the types of the arguments and return value at compile time.
38    // This means there's not really a way to store or call these functions dynamically at runtime.
39    // Luckily, Bevy's reflection crate comes with a set of tools for doing just that!
40    // We do this by first converting our function into the reflection-based `DynamicFunction` type
41    // using the `IntoFunction` trait.
42    let function: DynamicFunction<'static> = dbg!(add.into_function());
43
44    // This time, you'll notice that `DynamicFunction` doesn't take any information about the function's arguments or return value.
45    // This is because `DynamicFunction` checks the types of the arguments and return value at runtime.
46    // Now we can generate a list of arguments:
47    let args: ArgList = dbg!(ArgList::new().with_owned(2_i32).with_owned(2_i32));
48
49    // And finally, we can call the function.
50    // This returns a `Result` indicating whether the function was called successfully.
51    // For now, we'll just unwrap it to get our `Return` value,
52    // which is an enum containing the function's return value.
53    let return_value: Return = dbg!(function.call(args).unwrap());
54
55    // The `Return` value can be pattern matched or unwrapped to get the underlying reflection data.
56    // For the sake of brevity, we'll just unwrap it here and downcast it to the expected type of `i32`.
57    let value: Box<dyn PartialReflect> = return_value.unwrap_owned();
58    assert_eq!(value.try_take::<i32>().unwrap(), 4);
59
60    // The same can also be done for closures that capture references to their environment.
61    // Closures that capture their environment immutably can be converted into a `DynamicFunction`
62    // using the `IntoFunction` trait.
63    let minimum = 5;
64    let clamp = |value: i32| value.max(minimum);
65
66    let function: DynamicFunction = dbg!(clamp.into_function());
67    let args = dbg!(ArgList::new().with_owned(2_i32));
68    let return_value = dbg!(function.call(args).unwrap());
69    let value: Box<dyn PartialReflect> = return_value.unwrap_owned();
70    assert_eq!(value.try_take::<i32>().unwrap(), 5);
71
72    // We can also handle closures that capture their environment mutably
73    // using the `IntoFunctionMut` trait.
74    let mut count = 0;
75    let increment = |amount: i32| count += amount;
76
77    let closure: DynamicFunctionMut = dbg!(increment.into_function_mut());
78    let args = dbg!(ArgList::new().with_owned(5_i32));
79
80    // Because `DynamicFunctionMut` mutably borrows `total`,
81    // it will need to be dropped before `total` can be accessed again.
82    // This can be done manually with `drop(closure)` or by using the `DynamicFunctionMut::call_once` method.
83    dbg!(closure.call_once(args).unwrap());
84    assert_eq!(count, 5);
85
86    // Generic functions can also be converted into a `DynamicFunction`,
87    // however, they will need to be manually monomorphized first.
88    fn stringify<T: ToString>(value: T) -> String {
89        value.to_string()
90    }
91
92    // We have to manually specify the concrete generic type we want to use.
93    let function = stringify::<i32>.into_function();
94
95    let args = ArgList::new().with_owned(123_i32);
96    let return_value = function.call(args).unwrap();
97    let value: Box<dyn PartialReflect> = return_value.unwrap_owned();
98    assert_eq!(value.try_take::<String>().unwrap(), "123");
99
100    // To make things a little easier, we can also "overload" functions.
101    // This makes it so that a single `DynamicFunction` can represent multiple functions,
102    // and the correct one is chosen based on the types of the arguments.
103    // Each function overload must have a unique argument signature.
104    let function = stringify::<i32>
105        .into_function()
106        .with_overload(stringify::<f32>);
107
108    // Now our `function` accepts both `i32` and `f32` arguments.
109    let args = ArgList::new().with_owned(1.23_f32);
110    let return_value = function.call(args).unwrap();
111    let value: Box<dyn PartialReflect> = return_value.unwrap_owned();
112    assert_eq!(value.try_take::<String>().unwrap(), "1.23");
113
114    // Function overloading even allows us to have a variable number of arguments.
115    let function = (|| 0)
116        .into_function()
117        .with_overload(|a: i32| a)
118        .with_overload(|a: i32, b: i32| a + b)
119        .with_overload(|a: i32, b: i32, c: i32| a + b + c);
120
121    let args = ArgList::new()
122        .with_owned(1_i32)
123        .with_owned(2_i32)
124        .with_owned(3_i32);
125    let return_value = function.call(args).unwrap();
126    let value: Box<dyn PartialReflect> = return_value.unwrap_owned();
127    assert_eq!(value.try_take::<i32>().unwrap(), 6);
128
129    // As stated earlier, `IntoFunction` works for many kinds of simple functions.
130    // Functions with non-reflectable arguments or return values may not be able to be converted.
131    // Generic functions are also not supported (unless manually monomorphized like `foo::<i32>.into_function()`).
132    // Additionally, the lifetime of the return value is tied to the lifetime of the first argument.
133    // However, this means that many methods (i.e. functions with a `self` parameter) are also supported:
134    #[derive(Reflect, Default)]
135    struct Data {
136        value: String,
137    }
138
139    impl Data {
140        fn set_value(&mut self, value: String) {
141            self.value = value;
142        }
143
144        // Note that only `&'static str` implements `Reflect`.
145        // To get around this limitation we can use `&String` instead.
146        fn get_value(&self) -> &String {
147            &self.value
148        }
149    }
150
151    let mut data = Data::default();
152
153    let set_value = dbg!(Data::set_value.into_function());
154    let args = dbg!(ArgList::new().with_mut(&mut data)).with_owned(String::from("Hello, world!"));
155    dbg!(set_value.call(args).unwrap());
156    assert_eq!(data.value, "Hello, world!");
157
158    let get_value = dbg!(Data::get_value.into_function());
159    let args = dbg!(ArgList::new().with_ref(&data));
160    let return_value = dbg!(get_value.call(args).unwrap());
161    let value: &dyn PartialReflect = return_value.unwrap_ref();
162    assert_eq!(value.try_downcast_ref::<String>().unwrap(), "Hello, world!");
163
164    // For more complex use cases, you can always create a custom `DynamicFunction` manually.
165    // This is useful for functions that can't be converted via the `IntoFunction` trait.
166    // For example, this function doesn't implement `IntoFunction` due to the fact that
167    // the lifetime of the return value is not tied to the lifetime of the first argument.
168    fn get_or_insert(value: i32, container: &mut Option<i32>) -> &i32 {
169        if container.is_none() {
170            *container = Some(value);
171        }
172
173        container.as_ref().unwrap()
174    }
175
176    let get_or_insert_function = dbg!(DynamicFunction::new(
177        |mut args: ArgList| -> FunctionResult {
178            // The `ArgList` contains the arguments in the order they were pushed.
179            // The `DynamicFunction` will validate that the list contains
180            // exactly the number of arguments we expect.
181            // We can retrieve them out in order (note that this modifies the `ArgList`):
182            let value = args.take::<i32>()?;
183            let container = args.take::<&mut Option<i32>>()?;
184
185            // We could have also done the following to make use of type inference:
186            // let value = args.take_owned()?;
187            // let container = args.take_mut()?;
188
189            Ok(Return::Ref(get_or_insert(value, container)))
190        },
191        // Functions can be either anonymous or named.
192        // It's good practice, though, to try and name your functions whenever possible.
193        // This makes it easier to debug and is also required for function registration.
194        // We can either give it a custom name or use the function's type name as
195        // derived from `std::any::type_name_of_val`.
196        SignatureInfo::named(std::any::type_name_of_val(&get_or_insert))
197            // We can always change the name if needed.
198            // It's a good idea to also ensure that the name is unique,
199            // such as by using its type name or by prefixing it with your crate name.
200            .with_name("my_crate::get_or_insert")
201            // Since our function takes arguments, we should provide that argument information.
202            // This is used to validate arguments when calling the function.
203            // And it aids consumers of the function with their own validation and debugging.
204            // Arguments should be provided in the order they are defined in the function.
205            .with_arg::<i32>("value")
206            .with_arg::<&mut Option<i32>>("container")
207            // We can provide return information as well.
208            .with_return::<&i32>(),
209    ));
210
211    let mut container: Option<i32> = None;
212
213    let args = dbg!(ArgList::new().with_owned(5_i32).with_mut(&mut container));
214    let value = dbg!(get_or_insert_function.call(args).unwrap()).unwrap_ref();
215    assert_eq!(value.try_downcast_ref::<i32>(), Some(&5));
216
217    let args = dbg!(ArgList::new().with_owned(500_i32).with_mut(&mut container));
218    let value = dbg!(get_or_insert_function.call(args).unwrap()).unwrap_ref();
219    assert_eq!(value.try_downcast_ref::<i32>(), Some(&5));
220}
```

examples/reflection/dynamic\_types.rs ([line 73](../../../src/dynamic_types/dynamic_types.rs.html#73))

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

## Trait Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/arg.rs.html#197)

### impl<'a> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/arg.rs.html#197)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/arg.rs.html#207)

### impl<'a> [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/arg.rs.html#208)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target) = dyn [PartialReflect](../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")

The resulting type after dereferencing.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/arg.rs.html#210)

#### fn [deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref)(&self) -> &<[ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a> as [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")\>::[Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target "type core::ops::deref::Deref::Target")

Dereferences the value.

## Auto Trait Implementations

### impl<'a> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

### impl<'a> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

### impl<'a> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

### impl<'a> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

### impl<'a> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

### impl<'a> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

### impl<'a> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ArgValue](enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#212)

### impl<T> [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#214)

#### fn [borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)(&self) -> [&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Immutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html#tymethod.borrow)

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#221)

### impl<T> [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<T> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/borrow.rs.html#222)

#### fn [borrow\_mut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)(&mut self) -> [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

Mutably borrows from an owned value. [Read more](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#244)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized"), [Initialized](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Initialized.html "enum zerocopy::pointer::invariant::Initialized")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#242)

### impl<ST, DT> [CastableFrom](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.CastableFrom.html "trait zerocopy::pointer::invariant::CastableFrom")<ST, [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit"), [Uninit](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Uninit.html "enum zerocopy::pointer::invariant::Uninit")\> for DT

where ST: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), DT: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#58)

### impl<T> [Conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html "trait tap::conv::Conv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#49-52)

#### fn [conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)<T>(self) -> T

where Self: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

Converts `self` into `T` using `Into<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.Conv.html#method.conv)

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#201)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#202)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`, which can then be `downcast` into `Box<dyn ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Converts `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`, which can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#205)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Converts `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#189)

### impl<T> [Downcast](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html "trait downcast_rs::Downcast") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#190)

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Box<dyn Trait>` (where `Trait: Downcast`) to `Box<dyn Any>`. `Box<dyn Any>` can then be further `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#191)

#### fn [into\_any\_rc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any_rc)(self: [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<T>) -> [Rc](https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html "struct alloc::rc::Rc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Convert `Rc<Trait>` (where `Trait: Downcast`) to `Rc<Any>`. `Rc<Any>` can then be further `downcast` into `Rc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#192)

#### fn [as\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&Any`’s vtable from `&Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#193)

#### fn [as\_any\_mut](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.as_any_mut)(&mut self) -> &mut (dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Convert `&mut Trait` (where `Trait: Downcast`) to `&Any`. This is needed since Rust cannot generate `&mut Any`’s vtable from `&mut Trait`’s.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#215)

### impl<T> [DowncastSend](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html "trait downcast_rs::DowncastSend") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#216)

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

Convert `Arc<Trait>` (where `Trait: Downcast`) to `Arc<Any>`. `Arc<Any>` can then be further `downcast` into `Arc<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#699)

### impl<S, T> [Duplex](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.Duplex.html "trait dasp_sample::conv::Duplex")<S> for T

where T: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> + [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<S>,

[Source](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/src/yoke/erased.rs.html#22)

### impl<T> [ErasedDestructor](https://docs.rs/yoke/0.8.3/x86_64-unknown-linux-gnu/yoke/erased/trait.ErasedDestructor.html "trait yoke::erased::ErasedDestructor") for T

where T: 'static,

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#114)

### impl<T> [FmtForward](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html "trait wyz::fmt::FmtForward") for T

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#41-42)

#### fn [fmt\_binary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_binary)(self) -> [FmtBinary](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtBinary.html "struct wyz::fmt::FmtBinary")<Self>

where Self: [Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html "trait core::fmt::Binary"),

Causes `self` to use its `Binary` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#49-50)

#### fn [fmt\_display](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_display)(self) -> [FmtDisplay](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtDisplay.html "struct wyz::fmt::FmtDisplay")<Self>

where Self: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display"),

Causes `self` to use its `Display` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#57-58)

#### fn [fmt\_lower\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_exp)(self) -> [FmtLowerExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerExp.html "struct wyz::fmt::FmtLowerExp")<Self>

where Self: [LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html "trait core::fmt::LowerExp"),

Causes `self` to use its `LowerExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#65-66)

#### fn [fmt\_lower\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_lower_hex)(self) -> [FmtLowerHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtLowerHex.html "struct wyz::fmt::FmtLowerHex")<Self>

where Self: [LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html "trait core::fmt::LowerHex"),

Causes `self` to use its `LowerHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#72-73)

#### fn [fmt\_octal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_octal)(self) -> [FmtOctal](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtOctal.html "struct wyz::fmt::FmtOctal")<Self>

where Self: [Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html "trait core::fmt::Octal"),

Causes `self` to use its `Octal` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#80-81)

#### fn [fmt\_pointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_pointer)(self) -> [FmtPointer](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtPointer.html "struct wyz::fmt::FmtPointer")<Self>

where Self: [Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html "trait core::fmt::Pointer"),

Causes `self` to use its `Pointer` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#88-89)

#### fn [fmt\_upper\_exp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_exp)(self) -> [FmtUpperExp](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperExp.html "struct wyz::fmt::FmtUpperExp")<Self>

where Self: [UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html "trait core::fmt::UpperExp"),

Causes `self` to use its `UpperExp` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#96-97)

#### fn [fmt\_upper\_hex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_upper_hex)(self) -> [FmtUpperHex](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtUpperHex.html "struct wyz::fmt::FmtUpperHex")<Self>

where Self: [UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html "trait core::fmt::UpperHex"),

Causes `self` to use its `UpperHex` implementation when `Debug`\-formatted.

[Source](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/src/wyz/fmt.rs.html#108-109)

#### fn [fmt\_list](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)(self) -> [FmtList](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/struct.FmtList.html "struct wyz::fmt::FmtList")<Self>

where &'a Self: for<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator"),

Formats each item in a sequence. [Read more](https://docs.rs/wyz/0.5.1/x86_64-unknown-linux-gnu/wyz/fmt/trait.FmtForward.html#method.fmt_list)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#787)

### impl<T> [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for T

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#790)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(t: T) -> T

Returns the argument unchanged.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#574)

### impl<S> [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<S> for S

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#576)

#### fn [from\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html#tymethod.from_sample_)(s: S) -> S

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static,

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#77)

### impl<T> [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#80)

#### const [TYPE\_EQ](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedconstant.TYPE_EQ): [TypeEq](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_eq/type_eq_/struct.TypeEq.html "struct typewit::type_eq::type_eq_::TypeEq")<T, <T as [Identity](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html "trait typewit::type_identity::Identity")\>::[Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type "type typewit::type_identity::Identity::Type")\> = TypeEq::NEW

Proof that `Self` is the same type as `Self::Type`, provides methods for casting between `Self` and `Self::Type`.

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_identity.rs.html#78)

#### type [Type](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_identity/trait.Identity.html#associatedtype.Type) = T

The same type as `Self`, used to emulate type equality bounds (`T == U`) with associated type equality constraints (`T: Identity<Type = U>`).

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#19)

### impl<T> [InitializeFromFunction](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html "trait dioxus_signals::global::InitializeFromFunction")<T> for T

[Source](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_signals/global/mod.rs.html#20)

#### fn [initialize\_from\_function](https://docs.rs/dioxus-signals/0.7.9/x86_64-unknown-linux-gnu/dioxus_signals/global/trait.InitializeFromFunction.html#tymethod.initialize_from_function)(f: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T) -> T

Create an instance of this type from an initialization function

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../log/tracing/trait.Instrument.html#method.in_current_span)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#769-771)

### impl<T, U> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U> for T

where U: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#779)

#### fn [into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html#tymethod.into)(self) -> U

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of `[From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T> for U` chooses to do.

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#64)

### impl<T> [IntoEither](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither") for T

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#29)

#### fn [into\_either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)(self, into\_left: [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left` is `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either)

[Source](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/src/either/into_either.rs.html#55-57)

#### fn [into\_either\_with](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)<F>(self, into\_left: F) -> [Either](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either")<Self, Self> [ⓘ](#)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html),

Converts `self` into a [`Left`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Left "variant either::Either::Left") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") if `into_left(&self)` returns `true`. Converts `self` into a [`Right`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html#variant.Right "variant either::Either::Right") variant of [`Either<Self, Self>`](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/enum.Either.html "enum either::Either") otherwise. [Read more](https://docs.rs/either/1.16.0/x86_64-unknown-linux-gnu/either/into_either/trait.IntoEither.html#method.into_either_with)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../trait.Is.html#tymethod.is)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#234)

### impl<T> [Pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html "trait tap::pipe::Pipe") for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#73-76)

#### fn [pipe](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(Self) -> R) -> R

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Pipes by value. This is generally the method you want to use. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#97-99)

#### fn [pipe\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)<'a, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a Self) -> R) -> R

where R: 'a,

Borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#122-127)

#### fn [pipe\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)<'a, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&'a mut Self) -> R) -> R

where R: 'a,

Mutably borrows `self` and passes that borrow into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#145-149)

#### fn [pipe\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)<'a, B, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.borrow()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#169-176)

#### fn [pipe\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)<'a, B, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.borrow_mut()` into the pipe function. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#183-187)

#### fn [pipe\_as\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_ref)<'a, U, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.as_ref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#195-202)

#### fn [pipe\_as\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_as_mut)<'a, U, R>(&'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut U](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<U>, U: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.as_mut()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#209-213)

#### fn [pipe\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref)<'a, T, R>(&'a self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R) -> R

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Borrows `self`, then passes `self.deref()` into the pipe function.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/pipe.rs.html#221-228)

#### fn [pipe\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/pipe/trait.Pipe.html#method.pipe_deref_mut)<'a, T, R>( &'a mut self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> R, ) -> R

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: 'a + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), R: 'a,

Mutably borrows `self`, then passes `self.deref_mut()` into the pipe function.

[Source](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/src/zerocopy/pointer/invariant.rs.html#263)

### impl<T> [Read](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/trait.Read.html "trait zerocopy::pointer::invariant::Read")<[Exclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.Exclusive.html "enum zerocopy::pointer::invariant::Exclusive"), [BecauseExclusive](https://docs.rs/zerocopy/0.8.50/x86_64-unknown-linux-gnu/zerocopy/pointer/invariant/enum.BecauseExclusive.html "enum zerocopy::pointer::invariant::BecauseExclusive")\> for T

where T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#379-381)

### impl<P, T> [Receiver](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html "trait core::ops::deref::Receiver") for P

where P: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/ops/deref.rs.html#383)

#### type [Target](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Receiver.html#associatedtype.Target) = T

🔬This is a nightly-only experimental API. (`arbitrary_self_types`)

The target type on which the method may be called.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

where T: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#328)

### impl<Ret> [SpawnIfAsync](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html "trait dioxus_core::events::SpawnIfAsync")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), Ret> for Ret

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/events.rs.html#329)

#### fn [spawn](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/events/trait.SpawnIfAsync.html#tymethod.spawn)(self) -> Ret

Spawn the value into the dioxus runtime if it is an async block

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#199-201)

### impl<T, O> [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T> for O

where O: [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<T>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#203)

#### fn [super\_from](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html#tymethod.super_from)(input: T) -> O

Convert from a type to another type.

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#183-185)

### impl<T, O, M> [SuperInto](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html "trait dioxus_core::properties::SuperInto")<O, M> for T

where O: [SuperFrom](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperFrom.html "trait dioxus_core::properties::SuperFrom")<T, M>,

[Source](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/src/dioxus_core/properties.rs.html#187)

#### fn [super\_into](https://docs.rs/dioxus-core/0.7.9/x86_64-unknown-linux-gnu/dioxus_core/properties/trait.SuperInto.html#tymethod.super_into)(self) -> O

Convert from a type to another type.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#329)

### impl<T> [Tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html "trait tap::tap::Tap") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#78)

#### fn [tap](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Immutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#116)

#### fn [tap\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Mutable access to a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#129-132)

#### fn [tap\_borrow](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Borrow<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#146-149)

#### fn [tap\_borrow\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `BorrowMut<B>` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#163-166)

#### fn [tap\_ref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `AsRef<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#180-183)

#### fn [tap\_ref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `AsMut<R>` view of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#197-200)

#### fn [tap\_deref](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Immutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#214-217)

#### fn [tap\_deref\_mut](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Mutable access to the `Deref::Target` of a value. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut)

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#227)

#### fn [tap\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&Self)) -> Self

Calls `.tap()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#237)

#### fn [tap\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_mut_dbg)(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(&mut Self)) -> Self

Calls `.tap_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#247-250)

#### fn [tap\_borrow\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#261-264)

#### fn [tap\_borrow\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_borrow_mut_dbg)<B>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut B](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut")<B>, B: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_borrow_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#275-278)

#### fn [tap\_ref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#289-292)

#### fn [tap\_ref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_ref_mut_dbg)<R>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html "trait core::convert::AsMut")<R>, R: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_ref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#303-306)

#### fn [tap\_deref\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref")<Target = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/tap.rs.html#317-320)

#### fn [tap\_deref\_mut\_dbg](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/tap/trait.Tap.html#method.tap_deref_mut_dbg)<T>(self, func: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html))) -> Self

where Self: [DerefMut](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")<Target = T> + [Deref](https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html "trait core::ops::deref::Deref"), T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Calls `.tap_deref_mut()` only in debug builds, and is erased in release builds.

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#687-689)

### impl<T, U> [ToSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html "trait dasp_sample::conv::ToSample")<U> for T

where U: [FromSample](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.FromSample.html "trait dasp_sample::conv::FromSample")<T>,

[Source](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/src/dasp_sample/conv.rs.html#692)

#### fn [to\_sample\_](https://docs.rs/dasp_sample/0.11.0/x86_64-unknown-linux-gnu/dasp_sample/conv/trait.ToSample.html#tymethod.to_sample_)(self) -> U

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#87)

### impl<T> [TryConv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html "trait tap::conv::TryConv") for T

[Source](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/src/tap/conv.rs.html#78-81)

#### fn [try\_conv](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)<T>(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, Self::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error "type core::convert::TryInto::Error")\>

where Self: [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<T>,

Attempts to convert `self` into `T` using `TryInto<T>`. [Read more](https://docs.rs/tap/1.0.1/x86_64-unknown-linux-gnu/tap/conv/trait.TryConv.html#method.try_conv)

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#829-831)

### impl<T, U> [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U> for T

where U: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#833)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error) = [Infallible](https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html "enum core::convert::Infallible")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#836)

#### fn [try\_from](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from)(value: U) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, <T as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<U>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#813-815)

### impl<T, U> [TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html "trait core::convert::TryInto")<U> for T

where U: [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>,

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#817)

#### type [Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#associatedtype.Error) = <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")

The type returned in the event of a conversion error.

[Source](https://doc.rust-lang.org/nightly/src/core/convert/mod.rs.html#820)

#### fn [try\_into](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html#tymethod.try_into)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<U, <U as [TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html "trait core::convert::TryFrom")<T>>::[Error](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error")\>

Performs the conversion.

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#221-223)

### impl<V, T> [VZip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html "trait ppv_lite86::types::VZip")<V> for T

where V: [MultiLane](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.MultiLane.html "trait ppv_lite86::types::MultiLane")<T>,

[Source](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/src/ppv_lite86/types.rs.html#226)

#### fn [vzip](https://docs.rs/ppv-lite86/0.2.21/x86_64-unknown-linux-gnu/ppv_lite86/types/trait.VZip.html#tymethod.vzip)(self) -> V

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#18)

### impl<T> [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#2)

### impl<T> [WasmNotSendSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSendSync.html "trait wgpu_types::send_sync::WasmNotSendSync") for T

where T: [WasmNotSend](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSend.html "trait wgpu_types::send_sync::WasmNotSend") + [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync"),

[Source](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/src/wgpu_types/send_sync.rs.html#51)

### impl<T> [WasmNotSync](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/send_sync/trait.WasmNotSync.html "trait wgpu_types::send_sync::WasmNotSync") for T

where T: [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}