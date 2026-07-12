[bevy](../../index.html)::[render](../index.html)

# Module settings 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#60)

## Structs

[Backends](struct.Backends.html "struct bevy::render::settings::Backends")

Represents the backends that wgpu will use.

[InstanceFlags](struct.InstanceFlags.html "struct bevy::render::settings::InstanceFlags")

Instance debugging flags.

[RenderResources](struct.RenderResources.html "struct bevy::render::settings::RenderResources")

[WgpuFeatures](struct.WgpuFeatures.html "struct bevy::render::settings::WgpuFeatures")

Features that are not guaranteed to be supported.

[WgpuLimits](struct.WgpuLimits.html "struct bevy::render::settings::WgpuLimits")

Represents the sets of limits an adapter/device supports.

[WgpuSettings](struct.WgpuSettings.html "struct bevy::render::settings::WgpuSettings")

Provides configuration for renderer initialization. Use [`RenderDevice::features`](../renderer/struct.RenderDevice.html#method.features "method bevy::render::renderer::RenderDevice::features"), [`RenderDevice::limits`](../renderer/struct.RenderDevice.html#method.limits "method bevy::render::renderer::RenderDevice::limits"), and the [`RenderAdapterInfo`](../renderer/struct.RenderAdapterInfo.html "struct bevy::render::renderer::RenderAdapterInfo") resource to get runtime information about the actual adapter, backend, features, and limits. NOTE: [`Backends::DX12`](struct.Backends.html#associatedconstant.DX12 "associated constant bevy::render::settings::Backends::DX12"), [`Backends::METAL`](struct.Backends.html#associatedconstant.METAL "associated constant bevy::render::settings::Backends::METAL"), and [`Backends::VULKAN`](struct.Backends.html#associatedconstant.VULKAN "associated constant bevy::render::settings::Backends::VULKAN") are enabled by default for non-web and the best choice is automatically selected. Web using the `webgl` feature uses [`Backends::GL`](struct.Backends.html#associatedconstant.GL "associated constant bevy::render::settings::Backends::GL"). NOTE: If you want to use [`Backends::GL`](struct.Backends.html#associatedconstant.GL "associated constant bevy::render::settings::Backends::GL") in a native app on `Windows` and/or `macOS`, you must use [`ANGLE`](https://github.com/gfx-rs/wgpu#angle) and enable the `gles` feature. This is because wgpu requires EGL to create a GL context without a window and only ANGLE supports that.

## Enums

[Dx12Compiler](enum.Dx12Compiler.html "enum bevy::render::settings::Dx12Compiler")

Selects which DX12 shader compiler to use.

[Gles3MinorVersion](enum.Gles3MinorVersion.html "enum bevy::render::settings::Gles3MinorVersion")

Selects which OpenGL ES 3 minor version to request.

[MemoryHints](enum.MemoryHints.html "enum bevy::render::settings::MemoryHints")

Hints to the device about the memory allocation strategy.

[PowerPreference](enum.PowerPreference.html "enum bevy::render::settings::PowerPreference")

Power Preference when choosing a physical adapter.

[RenderCreation](enum.RenderCreation.html "enum bevy::render::settings::RenderCreation")

An enum describing how the renderer will initialize resources. This is used when creating the [`RenderPlugin`](../struct.RenderPlugin.html "struct bevy::render::RenderPlugin").

[WgpuSettingsPriority](enum.WgpuSettingsPriority.html "enum bevy::render::settings::WgpuSettingsPriority")

Configures the priority used when automatically configuring the features/limits of `wgpu`.

## Functions

[settings\_priority\_from\_env](fn.settings_priority_from_env.html "fn bevy::render::settings::settings_priority_from_env")

Get a features/limits priority from the environment variable `WGPU_SETTINGS_PRIO`