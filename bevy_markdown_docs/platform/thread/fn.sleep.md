[bevy](../../index.html)::[platform](../index.html)::[thread](index.html)

# Function sleep 

1.4.0 · [Source](https://doc.rust-lang.org/nightly/src/std/thread/functions.rs.html#292)

```rust
pub fn sleep(dur: Duration)
```

Puts the current thread to sleep for at least the specified amount of time.

The thread may sleep longer than the duration specified due to scheduling specifics or platform-dependent functionality. It will never sleep less.

This function is blocking, and should not be used in `async` functions.

## Platform-specific behavior

On Unix platforms, the underlying syscall may be interrupted by a spurious wakeup or signal handler. To ensure the sleep occurs for at least the specified duration, this function may invoke that system call multiple times. Platforms which do not support nanosecond precision for sleeping will have `dur` rounded up to the nearest granularity of time they can sleep for.

Currently, specifying a zero duration on Unix platforms returns immediately without invoking the underlying [`nanosleep`](https://linux.die.net/man/2/nanosleep) syscall, whereas on Windows platforms the underlying [`Sleep`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-sleep) syscall is always invoked. If the intention is to yield the current time-slice you may want to use [`yield_now`](https://doc.rust-lang.org/nightly/std/thread/functions/fn.yield_now.html "fn std::thread::functions::yield_now") instead.

## Examples

```rust
use std::{thread, time};

let ten_millis = time::Duration::from_millis(10);
let now = time::Instant::now();

thread::sleep(ten_millis);

assert!(now.elapsed() >= ten_millis);
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/ecs/hotpatching\_systems.rs ([line 90](../../../src/hotpatching_systems/hotpatching_systems.rs.html#90))

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