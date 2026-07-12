[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias PollType 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/queue.rs.html#52)

```rust
pub type PollType = PollType<SubmissionIndex>;
```

Passed to [`Device::poll`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/device/struct.Device.html#method.poll "method wgpu::api::device::Device::poll") to control how and if it should block.

## Aliased Type

```rust
pub enum PollType {
    Wait {
        submission_index: Option<SubmissionIndex>,
        timeout: Option<Duration>,
    },
    Poll,
}
```

## Variants

### Wait

On wgpu-core based backends, block until the given submission has completed execution, and any callbacks have been invoked.

On WebGPU, this has no effect. Callbacks are invoked from the window event loop.

#### Fields

`submission_index: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[SubmissionIndex](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/queue/struct.SubmissionIndex.html "struct wgpu::api::queue::SubmissionIndex")>`

Submission index to wait for.

If not specified, will wait for the most recent submission at the time of the poll. By the time the method returns, more submissions may have taken place.

`timeout: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html "struct core::time::Duration")>`

Max time to wait for the submission to complete.

If not specified, will wait indefinitely (or until an error is detected). If waiting for the GPU device takes this long or longer, the poll will return \[`PollError::Timeout`\].

### Poll

Check the device for a single time without blocking.