[bevy](../../index.html)::[dev\_tools](../index.html)

# Module ci\_testing 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/lib.rs.html#14)

Available on **crate feature `bevy_ci_testing`** only.

Utilities for testing in CI environments.

## Structs

[CiTestingConfig](struct.CiTestingConfig.html "struct bevy::dev_tools::ci_testing::CiTestingConfig")

A configuration struct for automated CI testing.

[CiTestingCustomEvent](struct.CiTestingCustomEvent.html "struct bevy::dev_tools::ci_testing::CiTestingCustomEvent")

A custom event that can be configured from a configuration file for CI testing.

[CiTestingEventOnFrame](struct.CiTestingEventOnFrame.html "struct bevy::dev_tools::ci_testing::CiTestingEventOnFrame")

An event to send at a given frame, used for CI testing.

[CiTestingPlugin](struct.CiTestingPlugin.html "struct bevy::dev_tools::ci_testing::CiTestingPlugin")

A plugin that instruments continuous integration testing by automatically executing user-defined actions.

[CiTestingSetup](struct.CiTestingSetup.html "struct bevy::dev_tools::ci_testing::CiTestingSetup")

Setup for a test.

## Enums

[CiTestingEvent](enum.CiTestingEvent.html "enum bevy::dev_tools::ci_testing::CiTestingEvent")

An event to send, used for CI testing.