[bevy](../../index.html)::[app](../index.html)::[hotpatch](index.html)

# Function call 

[Source](https://docs.rs/subsecond/0.7.9/x86_64-unknown-linux-gnu/src/subsecond/lib.rs.html#250)

```rust
pub fn call<O>(f: impl FnMut() -> O) -> O
```

Available on **crate feature `hotpatching`** only.

Call a given function with hot-reloading enabled. If the function’s code changes, `call` will use the new version of the function. If code _above_ the function changes, this will emit a panic that forces an unwind to the next [`call`](fn.call.html "fn bevy::app::hotpatch::call") instance.

WASM/rust does not support unwinding, so [`call`](fn.call.html "fn bevy::app::hotpatch::call") will not track dependency graph changes. If you are building a framework for use on WASM, you will need to use `Subsecond::HotFn` directly.

However, if you wrap your calling code in a future, you _can_ simply drop the future which will cause `drop` to execute and get something similar to unwinding. Not great if refcells are open.

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/hotpatching\_systems.rs ([line 88](../../../src/hotpatching_systems/hotpatching_systems.rs.html#88))

```rust
81fn start_thread(receiver: crossbeam_channel::Receiver<()>) {
82    std::thread::spawn(move || {
83        while receiver.recv().is_ok() {
84            let start = bevy::platform::time::Instant::now();
85
86            // You can also make any part outside of a system hot patchable by wrapping it
87            // In this part, only the duration is hot patchable:
88            let duration = bevy::app::hotpatch::call(|| Duration::from_secs(2));
89
90            std::thread::sleep(duration);
91            info!("done after {:?}", start.elapsed());
92        }
93    });
94}
```