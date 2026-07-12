[bevy](../../../index.html)::[dev\_tools](../../index.html)::[schedule\_data](../index.html)

# Module serde 

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/schedule_data/mod.rs.html#5)

Available on **crate feature `schedule_data`** only.

Utilities for serializing schedule data for an [`App`](../../../prelude/struct.App.html "struct bevy::prelude::App").

These are mostly around providing types implementing [`Serialize`](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/ser/trait.Serialize.html "trait serde_core::ser::Serialize")/[`Deserialize`](https://docs.rs/serde_core/1.0.228/x86_64-unknown-linux-gnu/serde_core/de/trait.Deserialize.html "trait serde_core::de::Deserialize") that represent schedule data. In addition, there are tools for extracting this data from the [`World`](../../../prelude/struct.World.html "struct bevy::prelude::World").

## Structs

[AppData](struct.AppData.html "struct bevy::dev_tools::schedule_data::serde::AppData")

The data for the entire app’s schedule.

[ComponentData](struct.ComponentData.html "struct bevy::dev_tools::schedule_data::serde::ComponentData")

Data about a component type.

[ConditionData](struct.ConditionData.html "struct bevy::dev_tools::schedule_data::serde::ConditionData")

Data about a run condition for a system.

[ScheduleData](struct.ScheduleData.html "struct bevy::dev_tools::schedule_data::serde::ScheduleData")

Data about a particular schedule.

[SystemConflict](struct.SystemConflict.html "struct bevy::dev_tools::schedule_data::serde::SystemConflict")

Data about an access conflict between two systems.

[SystemData](struct.SystemData.html "struct bevy::dev_tools::schedule_data::serde::SystemData")

Data about a particular system.

[SystemSetData](struct.SystemSetData.html "struct bevy::dev_tools::schedule_data::serde::SystemSetData")

Data about a particular system set.

[SystemSetIndex](struct.SystemSetIndex.html "struct bevy::dev_tools::schedule_data::serde::SystemSetIndex")

A newtype for the index of a system set.

## Enums

[AccessConflict](enum.AccessConflict.html "enum bevy::dev_tools::schedule_data::serde::AccessConflict")

Data for describing the kind of access conflict.

[ExtractAppDataError](enum.ExtractAppDataError.html "enum bevy::dev_tools::schedule_data::serde::ExtractAppDataError")

An error occurring while attempting to extract schedule data from an app.

[ScheduleIndex](enum.ScheduleIndex.html "enum bevy::dev_tools::schedule_data::serde::ScheduleIndex")

An index of an element in a schedule.