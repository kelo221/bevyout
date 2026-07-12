[bevy](../../../index.html)::[dev\_tools](../../index.html)::[schedule\_data](../index.html)

# Module plugin 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/schedule_data/mod.rs.html#4)

Available on **crate feature `schedule_data`** only.

Convenience plugin for automatically performing serialization of schedules on boot.

## Structs

[SerializeSchedulesFilePath](struct.SerializeSchedulesFilePath.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesFilePath")

The file path where schedules will be written to after collected by [`SerializeSchedulesPlugin`](struct.SerializeSchedulesPlugin.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesPlugin").

[SerializeSchedulesPlugin](struct.SerializeSchedulesPlugin.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesPlugin")

A plugin to automatically collect and write all schedule data on boot to a file that can later be parsed.

[SerializeSchedulesSystems](struct.SerializeSchedulesSystems.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesSystems")

A system set for allowing users to configure scheduling properties of systems in [`SerializeSchedulesPlugin`](struct.SerializeSchedulesPlugin.html "struct bevy::dev_tools::schedule_data::plugin::SerializeSchedulesPlugin").