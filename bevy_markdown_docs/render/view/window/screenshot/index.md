[bevy](../../../../index.html)::[render](../../../index.html)::[view](../../index.html)::[window](../index.html)

# Module screenshot 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/mod.rs.html#24)

## Structs

[Captured](struct.Captured.html "struct bevy::render::view::window::screenshot::Captured")

A marker component that indicates that a screenshot has been captured, the image is ready, and the screenshot entity can be despawned.

[CapturedScreenshots](struct.CapturedScreenshots.html "struct bevy::render::view::window::screenshot::CapturedScreenshots")

[Capturing](struct.Capturing.html "struct bevy::render::view::window::screenshot::Capturing")

A marker component that indicates that a screenshot is currently being captured.

[Screenshot](struct.Screenshot.html "struct bevy::render::view::window::screenshot::Screenshot")

A component that signals to the renderer to capture a screenshot this frame.

[ScreenshotCaptured](struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

[ScreenshotPlugin](struct.ScreenshotPlugin.html "struct bevy::render::view::window::screenshot::ScreenshotPlugin")

[ScreenshotToScreenPipeline](struct.ScreenshotToScreenPipeline.html "struct bevy::render::view::window::screenshot::ScreenshotToScreenPipeline")

## Functions

[init\_screenshot\_to\_screen\_pipeline](fn.init_screenshot_to_screen_pipeline.html "fn bevy::render::view::window::screenshot::init_screenshot_to_screen_pipeline")

[save\_to\_disk](fn.save_to_disk.html "fn bevy::render::view::window::screenshot::save_to_disk")

Saves the captured screenshot to disk at the provided path.

[trigger\_screenshots](fn.trigger_screenshots.html "fn bevy::render::view::window::screenshot::trigger_screenshots")