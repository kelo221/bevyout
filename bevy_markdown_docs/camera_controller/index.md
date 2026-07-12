[bevy](../index.html)

# Crate camera\_controller 

[Source](https://docs.rs/bevy_camera_controller/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera_controller/lib.rs.html#1-43)

A home for first-party camera controllers for Bevy, used for moving the camera around your scene.

This crate serves two key purposes:

1.  It provides functional camera controllers to help users quickly get started.
2.  It holds the camera controllers used by Bevy’s own examples and tooling.

While these camera controllers are customizable, there is a limit to the customization options available. If you find your project requires different behavior, do not hesitate to copy-paste the camera controller code into your own project and modify it as needed.

Each of the provided controllers is gated behind a feature flag, so you don’t have to pay for unused camera controllers. These features are all off by default; to enable them, you need to specify the desired features in your Cargo.toml file.

For example, to enable the `free_camera` camera controller, you would add the following to your Cargo.toml:

```toml
[dependencies]
bevy = { version = "0.X", features = ["free_camera"] }
```

Once the correct feature is enabled, add the camera controller plugin to your Bevy app. If your camera is for debugging and development purposes, consider adding a feature flag (e.g. `dev-mode`) or a run condition on the systems in the plugin, which can check a configuration resource.

For a full overview of the available camera controllers, please check out the modules of this crate. Each camera controller is stored in its own module, and gated behind a feature flag of the same name.

## Modules

[free\_camera](free_camera/index.html "mod bevy::camera_controller::free_camera")`free_camera`

A camera controller that allows the user to move freely around the scene.

[pan\_camera](pan_camera/index.html "mod bevy::camera_controller::pan_camera")`pan_camera`

A controller for 2D cameras that supports panning, zooming, and rotation.