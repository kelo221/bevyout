[bevy](../../../index.html)::[reflect](../../index.html)::[func](../index.html)::[args](index.html)

# Struct ArgList 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#39)

```rust
pub struct ArgList<'a> { /* private fields */ }
```

Available on **crate feature `functions`** only.

A list of arguments that can be passed to a [`DynamicFunction`](../struct.DynamicFunction.html "struct bevy::reflect::func::DynamicFunction") or [`DynamicFunctionMut`](../struct.DynamicFunctionMut.html "struct bevy::reflect::func::DynamicFunctionMut").

## Example

```rust
let foo = 123;
let bar = 456;
let mut baz = 789;
let args = ArgList::new()
  // Push an owned argument
  .with_owned(foo)
  // Push an owned and boxed argument
  .with_boxed(Box::new(foo))
  // Push a reference argument
  .with_ref(&bar)
  // Push a mutable reference argument
  .with_mut(&mut baz)
  // Push a manually constructed argument
  .with_arg(ArgValue::Ref(&3.14));
```

## Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#48)

### impl<'a> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#50)

#### pub fn [new](#method.new)() -> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

Create a new empty list of arguments.

##### [Examples found in repository](#scraped-examples)[?](../../../../scrape-examples-help.html)

examples/reflection/function\_reflection.rs ([line 47](../../../../src/function_reflection/function_reflection.rs.html#47))

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#61)

#### pub fn [push\_arg](#method.push_arg)(&mut self, arg: [ArgValue](../enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>)

Push an [`ArgValue`](../enum.ArgValue.html "enum bevy::reflect::func::ArgValue") onto the list.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#77)

#### pub fn [push\_ref](#method.push_ref)(&mut self, arg: &'a (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Push an [`ArgValue::Ref`](../enum.ArgValue.html#variant.Ref "variant bevy::reflect::func::ArgValue::Ref") onto the list with the given reference.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#85)

#### pub fn [push\_mut](#method.push_mut)(&mut self, arg: &'a mut (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

Push an [`ArgValue::Mut`](../enum.ArgValue.html#variant.Mut "variant bevy::reflect::func::ArgValue::Mut") onto the list with the given mutable reference.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#93)

#### pub fn [push\_owned](#method.push_owned)(&mut self, arg: impl [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect"))

Push an [`ArgValue::Owned`](../enum.ArgValue.html#variant.Owned "variant bevy::reflect::func::ArgValue::Owned") onto the list with the given owned value.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#101)

#### pub fn [push\_boxed](#method.push_boxed)(&mut self, arg: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>)

Push an [`ArgValue::Owned`](../enum.ArgValue.html#variant.Owned "variant bevy::reflect::func::ArgValue::Owned") onto the list with the given boxed value.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#109)

#### pub fn [with\_arg](#method.with_arg)(self, arg: [ArgValue](../enum.ArgValue.html "enum bevy::reflect::func::ArgValue")<'a>) -> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

Push an [`ArgValue`](../enum.ArgValue.html "enum bevy::reflect::func::ArgValue") onto the list.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#118)

#### pub fn [with\_ref](#method.with_ref)(self, arg: &'a (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)) -> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

Push an [`ArgValue::Ref`](../enum.ArgValue.html#variant.Ref "variant bevy::reflect::func::ArgValue::Ref") onto the list with the given reference.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

##### [Examples found in repository](#scraped-examples-1)[?](../../../../scrape-examples-help.html)

examples/reflection/function\_reflection.rs ([line 159](../../../../src/function_reflection/function_reflection.rs.html#159))

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#126)

#### pub fn [with\_mut](#method.with_mut)( self, arg: &'a mut (dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static), ) -> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

Push an [`ArgValue::Mut`](../enum.ArgValue.html#variant.Mut "variant bevy::reflect::func::ArgValue::Mut") onto the list with the given mutable reference.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

##### [Examples found in repository](#scraped-examples-2)[?](../../../../scrape-examples-help.html)

examples/reflection/function\_reflection.rs ([line 154](../../../../src/function_reflection/function_reflection.rs.html#154))

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#134)

#### pub fn [with\_owned](#method.with_owned)(self, arg: impl [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")) -> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

Push an [`ArgValue::Owned`](../enum.ArgValue.html#variant.Owned "variant bevy::reflect::func::ArgValue::Owned") onto the list with the given owned value.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

##### [Examples found in repository](#scraped-examples-3)[?](../../../../scrape-examples-help.html)

examples/reflection/function\_reflection.rs ([line 47](../../../../src/function_reflection/function_reflection.rs.html#47))

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#142)

#### pub fn [with\_boxed](#method.with_boxed)(self, arg: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [PartialReflect](../../../prelude/trait.PartialReflect.html "trait bevy::prelude::PartialReflect")\>) -> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

Push an [`ArgValue::Owned`](../enum.ArgValue.html#variant.Owned "variant bevy::reflect::func::ArgValue::Owned") onto the list with the given boxed value.

If an argument was previously removed from the beginning of the list, this method will also re-index the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#150)

#### pub fn [take\_arg](#method.take_arg)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'a>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Remove the first argument in the list and return it.

It’s generally preferred to use [`Self::take`](../struct.ArgList.html#method.take "method bevy::reflect::func::ArgList::take") instead of this method as it provides a more ergonomic way to immediately downcast the argument.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#177)

#### pub fn [take](#method.take)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'a>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg"),

Remove the first argument in the list and return `Ok(T::This)`.

If the list is empty or the [`FromArg::from_arg`](trait.FromArg.html#tymethod.from_arg "associated function bevy::reflect::func::args::FromArg::from_arg") call fails, returns an error.

##### Example

```rust
let a = 1u32;
let b = 2u32;
let mut c = 3u32;
let mut args = ArgList::new().with_owned(a).with_ref(&b).with_mut(&mut c);

let a = args.take::<u32>().unwrap();
assert_eq!(a, 1);

let b = args.take::<&u32>().unwrap();
assert_eq!(*b, 2);

let c = args.take::<&mut u32>().unwrap();
assert_eq!(*c, 3);
```

##### [Examples found in repository](#scraped-examples-4)[?](../../../../scrape-examples-help.html)

examples/reflection/function\_reflection.rs ([line 182](../../../../src/function_reflection/function_reflection.rs.html#182))

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

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#196)

#### pub fn [take\_owned](#method.take_owned)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

Remove the first argument in the list and return `Ok(T)` if the argument is [`ArgValue::Owned`](../enum.ArgValue.html#variant.Owned "variant bevy::reflect::func::ArgValue::Owned").

If the list is empty or the argument is not owned, returns an error.

It’s generally preferred to use [`Self::take`](../struct.ArgList.html#method.take "method bevy::reflect::func::ArgList::take") instead of this method.

##### Example

```rust
let value = 123u32;
let mut args = ArgList::new().with_owned(value);
let value = args.take_owned::<u32>().unwrap();
assert_eq!(value, 123);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#215)

#### pub fn [take\_ref](#method.take_ref)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

Remove the first argument in the list and return `Ok(&T)` if the argument is [`ArgValue::Ref`](../enum.ArgValue.html#variant.Ref "variant bevy::reflect::func::ArgValue::Ref").

If the list is empty or the argument is not a reference, returns an error.

It’s generally preferred to use [`Self::take`](../struct.ArgList.html#method.take "method bevy::reflect::func::ArgList::take") instead of this method.

##### Example

```rust
let value = 123u32;
let mut args = ArgList::new().with_ref(&value);
let value = args.take_ref::<u32>().unwrap();
assert_eq!(*value, 123);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#234)

#### pub fn [take\_mut](#method.take_mut)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

Remove the first argument in the list and return `Ok(&mut T)` if the argument is [`ArgValue::Mut`](../enum.ArgValue.html#variant.Mut "variant bevy::reflect::func::ArgValue::Mut").

If the list is empty or the argument is not a mutable reference, returns an error.

It’s generally preferred to use [`Self::take`](../struct.ArgList.html#method.take "method bevy::reflect::func::ArgList::take") instead of this method.

##### Example

```rust
let mut value = 123u32;
let mut args = ArgList::new().with_mut(&mut value);
let value = args.take_mut::<u32>().unwrap();
assert_eq!(*value, 123);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#242)

#### pub fn [pop\_arg](#method.pop_arg)(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'a>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

Remove the last argument in the list and return it.

It’s generally preferred to use [`Self::pop`](../struct.ArgList.html#method.pop "method bevy::reflect::func::ArgList::pop") instead of this method as it provides a more ergonomic way to immediately downcast the argument.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#268)

#### pub fn [pop](#method.pop)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<T as [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg")\>::[This](trait.FromArg.html#associatedtype.This "type bevy::reflect::func::args::FromArg::This")<'a>, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [FromArg](trait.FromArg.html "trait bevy::reflect::func::args::FromArg"),

Remove the last argument in the list and return `Ok(T::This)`.

If the list is empty or the [`FromArg::from_arg`](trait.FromArg.html#tymethod.from_arg "associated function bevy::reflect::func::args::FromArg::from_arg") call fails, returns an error.

##### Example

```rust
let a = 1u32;
let b = 2u32;
let mut c = 3u32;
let mut args = ArgList::new().with_owned(a).with_ref(&b).with_mut(&mut c);

let c = args.pop::<&mut u32>().unwrap();
assert_eq!(*c, 3);

let b = args.pop::<&u32>().unwrap();
assert_eq!(*b, 2);

let a = args.pop::<u32>().unwrap();
assert_eq!(a, 1);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#287)

#### pub fn [pop\_owned](#method.pop_owned)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

Remove the last argument in the list and return `Ok(T)` if the argument is [`ArgValue::Owned`](../enum.ArgValue.html#variant.Owned "variant bevy::reflect::func::ArgValue::Owned").

If the list is empty or the argument is not owned, returns an error.

It’s generally preferred to use [`Self::pop`](../struct.ArgList.html#method.pop "method bevy::reflect::func::ArgList::pop") instead of this method.

##### Example

```rust
let value = 123u32;
let mut args = ArgList::new().with_owned(value);
let value = args.pop_owned::<u32>().unwrap();
assert_eq!(value, 123);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#306)

#### pub fn [pop\_ref](#method.pop_ref)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

Remove the last argument in the list and return `Ok(&T)` if the argument is [`ArgValue::Ref`](../enum.ArgValue.html#variant.Ref "variant bevy::reflect::func::ArgValue::Ref").

If the list is empty or the argument is not a reference, returns an error.

It’s generally preferred to use [`Self::pop`](../struct.ArgList.html#method.pop "method bevy::reflect::func::ArgList::pop") instead of this method.

##### Example

```rust
let value = 123u32;
let mut args = ArgList::new().with_ref(&value);
let value = args.pop_ref::<u32>().unwrap();
assert_eq!(*value, 123);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#325)

#### pub fn [pop\_mut](#method.pop_mut)<T>(&mut self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[&'a mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), [ArgError](../enum.ArgError.html "enum bevy::reflect::func::ArgError")\>

where T: [Reflect](../../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](../../../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),

Remove the last argument in the list and return `Ok(&mut T)` if the argument is [`ArgValue::Mut`](../enum.ArgValue.html#variant.Mut "variant bevy::reflect::func::ArgValue::Mut").

If the list is empty or the argument is not a mutable reference, returns an error.

It’s generally preferred to use [`Self::pop`](../struct.ArgList.html#method.pop "method bevy::reflect::func::ArgList::pop") instead of this method.

##### Example

```rust
let mut value = 123u32;
let mut args = ArgList::new().with_mut(&mut value);
let value = args.pop_mut::<u32>().unwrap();
assert_eq!(*value, 123);
```

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#330)

#### pub fn [iter](#method.iter)(&self) -> [Iter](https://doc.rust-lang.org/nightly/alloc/collections/vec_deque/iter/struct.Iter.html "struct alloc::collections::vec_deque::iter::Iter")<'\_, [Arg](struct.Arg.html "struct bevy::reflect::func::args::Arg")<'a>> [ⓘ](#)

Returns an iterator over the arguments in the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#335)

#### pub fn [len](#method.len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of arguments in the list.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#340)

#### pub fn [is\_empty](#method.is_empty)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns `true` if the list of arguments is empty.

## Trait Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#38)

### impl<'a> [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#38)

#### fn [fmt](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)(&self, f: &mut [Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html "struct core::fmt::Formatter")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html "struct core::fmt::Error")\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#38)

### impl<'a> [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/args/list.rs.html#38)

#### fn [default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)() -> [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/signature.rs.html#186)

### impl [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<&[ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'\_>> for [ArgumentSignature](../signature/struct.ArgumentSignature.html "struct bevy::reflect::func::signature::ArgumentSignature")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/func/signature.rs.html#187)

#### fn [from](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from)(args: &[ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'\_>) -> [ArgumentSignature](../signature/struct.ArgumentSignature.html "struct bevy::reflect::func::signature::ArgumentSignature")

Converts to this type from the input type.

## Auto Trait Implementations

### impl<'a> ![RefUnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

### impl<'a> ![UnwindSafe](https://doc.rust-lang.org/nightly/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

### impl<'a> [Freeze](https://doc.rust-lang.org/nightly/core/marker/trait.Freeze.html "trait core::marker::Freeze") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

### impl<'a> [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

### impl<'a> [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

### impl<'a> [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

### impl<'a> [UnsafeUnpin](https://doc.rust-lang.org/nightly/core/marker/trait.UnsafeUnpin.html "trait core::marker::UnsafeUnpin") for [ArgList](../struct.ArgList.html "struct bevy::reflect::func::ArgList")<'a>

## Blanket Implementations

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#141)

### impl<T> [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") for T

where T: 'static + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://doc.rust-lang.org/nightly/src/core/any.rs.html#142)

#### fn [type\_id](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)(&self) -> [TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId")

Gets the `TypeId` of `self`. [Read more](https://doc.rust-lang.org/nightly/core/any/trait.Any.html#tymethod.type_id)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#696-698)

### impl<T, U> [AsBindGroupShaderType](../../../render/render_resource/trait.AsBindGroupShaderType.html "trait bevy::render::render_resource::AsBindGroupShaderType")<U> for T

where U: [ShaderType](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType"), [&'a T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): for<'a> [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<U>,

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group.rs.html#701)

#### fn [as\_bind\_group\_shader\_type](../../../render/render_resource/trait.AsBindGroupShaderType.html#tymethod.as_bind_group_shader_type)(&self, \_images: &[RenderAssets](../../../render/render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../../../render/texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>) -> U

Return the `T` [`ShaderType`](../../../render/render_resource/trait.ShaderType.html "trait bevy::render::render_resource::ShaderType") for `self`. When used in [`AsBindGroup`](../../../render/render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") derives, it is safe to assume that all images in `self` exist.

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

### impl<T> [ConditionalSend](../../../tasks/trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.Downcast.html#tymethod.into_any)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

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

#### fn [into\_any\_send](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSend.html#tymethod.into_any_send)(self: [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<T>) -> [Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send")\>

Converts `Box<Trait>` (where `Trait: DowncastSend`) to `Box<dyn Any + Send>`, which can then be `downcast` into `Box<ConcreteType>` where `ConcreteType` implements `Trait`.

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#203)

### impl<T> [DowncastSync](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html "trait downcast_rs::DowncastSync") for T

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/src/downcast_rs/lib.rs.html#204)

#### fn [into\_any\_arc](https://docs.rs/downcast-rs/1.2.1/x86_64-unknown-linux-gnu/downcast_rs/trait.DowncastSync.html#tymethod.into_any_arc)(self: [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<T>) -> [Arc](../../../platform/sync/struct.Arc.html "struct bevy::platform::sync::Arc")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\> [ⓘ](#)

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

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4000)

### impl<T> [FromWorld](../../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/mod.rs.html#4003)

#### fn [from\_world](../../../prelude/trait.FromWorld.html#tymethod.from_world)(\_world: &mut [World](../../../prelude/struct.World.html "struct bevy::prelude::World")) -> T

Creates `Self` using [`default()`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default "associated function core::default::Default::default").

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#106-109)

### impl<T, W> [HasTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html "trait typewit::type_witness_traits::HasTypeWitness")<W> for T

where W: [MakeTypeWitness](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.MakeTypeWitness.html "trait typewit::type_witness_traits::MakeTypeWitness")<Arg = T>, T: ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/src/typewit/type_witness_traits.rs.html#111)

#### const [WITNESS](https://docs.rs/typewit/1.15.2/x86_64-unknown-linux-gnu/typewit/type_witness_traits/trait.HasTypeWitness.html#associatedconstant.WITNESS): W = W::MAKE

A constant of the type witness

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#80)

### impl<T> [HitDataExtra](../../../picking/backend/trait.HitDataExtra.html "trait bevy::picking::backend::HitDataExtra") for T

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

### impl<T> [Instrument](../../../log/tracing/trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](../../../log/tracing/trait.Instrument.html#method.instrument)(self, span: [Span](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.instrument)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](../../../log/tracing/trait.Instrument.html#method.in_current_span)(self) -> [Instrumented](../../../log/tracing/instrument/struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../../../log/tracing/struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../../../log/tracing/struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper. [Read more](../../../log/tracing/trait.Instrument.html#method.in_current_span)

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

### impl<T> [IntoResult](../../../ecs/system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#597)

#### fn [into\_result](../../../ecs/system/trait.IntoResult.html#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](../../../ecs/system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#636)

### impl<F, T> [IntoSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html "trait symphonia_core::conv::IntoSample")<T> for F

where T: [FromSample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.FromSample.html "trait symphonia_core::conv::FromSample")<F>,

[Source](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/src/symphonia_core/conv.rs.html#638)

#### fn [into\_sample](https://docs.rs/symphonia-core/0.5.5/x86_64-unknown-linux-gnu/symphonia_core/conv/trait.IntoSample.html#tymethod.into_sample)(self) -> T

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#26)

### impl<A> [Is](../../trait.Is.html "trait bevy::reflect::Is") for A

where A: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/is.rs.html#28)

#### fn [is](../../trait.Is.html#tymethod.is)<T>() -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

where T: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any"),

Checks if the current type “is” another type, using a [`TypeId`](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html "struct core::any::TypeId") equality comparison. This is most useful in the context of generic logic. [Read more](../../trait.Is.html#tymethod.is)

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#31-33)

### impl<T> [NoneValue](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html "trait zvariant::optional::NoneValue") for T

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#35)

#### type [NoneType](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#associatedtype.NoneType) = T

[Source](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/src/zvariant/optional.rs.html#37)

#### fn [null\_value](https://docs.rs/zvariant/5.9.2/x86_64-unknown-linux-gnu/zvariant/optional/trait.NoneValue.html#tymethod.null_value)() -> T

The none-equivalent value.

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

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#347)

### impl<R, P> [ReadPrimitive](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html "trait lebe::io::ReadPrimitive")<R> for P

where R: [Read](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read") + [ReadEndian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadEndian.html "trait lebe::io::ReadEndian")<P>, P: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"),

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#377)

#### fn [read\_from\_little\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_little_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_little_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#382)

#### fn [read\_from\_big\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_big_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_big_endian()`.

[Source](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/src/lebe/lib.rs.html#387)

#### fn [read\_from\_native\_endian](https://docs.rs/lebe/0.5.3/x86_64-unknown-linux-gnu/lebe/io/trait.ReadPrimitive.html#method.read_from_native_endian)(read: [&mut R](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self, [Error](../../../tasks/futures_lite/io/struct.Error.html "struct bevy::tasks::futures_lite::io::Error")\>

Read this value from the supplied reader. Same as `ReadEndian::read_from_native_endian()`.

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#34)

### impl<T> [Same](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same") for T

[Source](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/src/typenum/type_operators.rs.html#35)

#### type [Output](https://docs.rs/typenum/1.20.1/x86_64-unknown-linux-gnu/typenum/type_operators/trait.Same.html#associatedtype.Output) = T

Should always be `Self`

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/meta.rs.html#190)

### impl<T> [Settings](../../../asset/meta/trait.Settings.html "trait bevy::asset::meta::Settings") for T

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

### impl<T> [WithSubscriber](../../../log/tracing/instrument/trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../../log/tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_subscriber)

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)(self) -> [WithDispatch](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../../../log/tracing/dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../../../log/tracing/trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](../../../log/tracing/instrument/struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper. [Read more](../../../log/tracing/instrument/trait.WithSubscriber.html#method.with_current_subscriber)

{"Arc<dyn Any + Send + Sync>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a R</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\\" title=\\"trait std::io::Read\\">Read</a>,</div></div><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> for <a class=\\"struct\\" href=\\"../../../platform/sync/struct.Arc.html\\" title=\\"struct bevy::platform::sync::Arc\\">Arc</a>&lt;W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a> + IoHandle + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,\\n <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a W</a>: for&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\\" title=\\"trait std::io::Write\\">Write</a>,</div></div>","Either<Self, Self>":"<h3>Notable traits for <code><a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>,\\n R: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&lt;Item = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;L as <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a>&gt;::<a class=\\"associatedtype\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" title=\\"type core::iter::traits::iterator::Iterator::Item\\">Item</a>;</div><div class=\\"where\\">impl&lt;L, R&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"enum\\" href=\\"https://docs.rs/either/1.16.0/x86\_64-unknown-linux-gnu/either/enum.Either.html\\" title=\\"enum either::Either\\">Either</a>&lt;L, R&gt;<div class=\\"where\\">where\\n L: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n R: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = &lt;L as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;L as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>","Iter<'\_, Arg<'a>>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/iter/struct.Iter.html\\" title=\\"struct alloc::collections::vec\_deque::iter::Iter\\">Iter</a>&lt;'a, T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, T&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/alloc/collections/vec\_deque/iter/struct.Iter.html\\" title=\\"struct alloc::collections::vec\_deque::iter::Iter\\">Iter</a>&lt;'a, T&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\\">&amp;'a T</a>;</div>","WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"../../../log/tracing/instrument/struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}