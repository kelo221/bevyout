[bevy](../../../../index.html)::[render](../../../index.html)::[view](../../index.html)::[window](../index.html)::[screenshot](index.html)

# Function save\_to\_disk 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#134)

```rust
pub fn save_to_disk(
    path: impl AsRef<Path>,
) -> impl FnMut(On<'_, '_, ScreenshotCaptured>)
```

Saves the captured screenshot to disk at the provided path.

##### [Examples found in repository](#scraped-examples)[?](../../../../../scrape-examples-help.html)

examples/app/externally\_driven\_headless\_renderer.rs ([line 126](../../../../../src/externally_driven_headless_renderer/externally_driven_headless_renderer.rs.html#126))

```rust
121    fn screenshot(&mut self, target: RenderTarget, i: u32) {
122        self.0
123            .main
124            .world_mut()
125            .spawn(Screenshot::image(target.as_image().unwrap().clone()))
126            .observe(save_to_disk(format!("test_images/screenshot{i}.png")));
127    }
```

Hide additional examples

examples/window/screenshot.rs ([line 27](../../../../../src/screenshot/screenshot.rs.html#27))

```rust
17fn screenshot_on_spacebar(
18    mut commands: Commands,
19    input: Res<ButtonInput<KeyCode>>,
20    mut counter: Local<u32>,
21) {
22    if input.just_pressed(KeyCode::Space) {
23        let path = format!("./screenshot-{}.png", *counter);
24        *counter += 1;
25        commands
26            .spawn(Screenshot::primary_window())
27            .observe(save_to_disk(path));
28    }
29}
```