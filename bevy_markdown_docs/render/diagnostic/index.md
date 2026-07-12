[bevy](../../index.html)::[render](../index.html)

# Module diagnostic 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#41)

Infrastructure for recording render diagnostics.

For more info, see [`RenderDiagnosticsPlugin`](struct.RenderDiagnosticsPlugin.html "struct bevy::render::diagnostic::RenderDiagnosticsPlugin").

## Structs

[DiagnosticsRecorder](struct.DiagnosticsRecorder.html "struct bevy::render::diagnostic::DiagnosticsRecorder")

Records diagnostics into [`QuerySet`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/query_set/struct.QuerySet.html "struct wgpu::api::query_set::QuerySet")’s keeping track of the mapping between spans and indices to the corresponding entries in the [`QuerySet`](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/wgpu/api/query_set/struct.QuerySet.html "struct wgpu::api::query_set::QuerySet").

[ErasedRenderAssetDiagnosticPlugin](struct.ErasedRenderAssetDiagnosticPlugin.html "struct bevy::render::diagnostic::ErasedRenderAssetDiagnosticPlugin")

Collects diagnostics for a [`ErasedRenderAsset`](../erased_render_asset/trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset").

[MeshAllocatorDiagnosticPlugin](struct.MeshAllocatorDiagnosticPlugin.html "struct bevy::render::diagnostic::MeshAllocatorDiagnosticPlugin")

[PassSpanGuard](struct.PassSpanGuard.html "struct bevy::render::diagnostic::PassSpanGuard")

Guard returned by [`RecordDiagnostics::pass_span`](trait.RecordDiagnostics.html#method.pass_span "method bevy::render::diagnostic::RecordDiagnostics::pass_span").

[RenderAssetDiagnosticPlugin](struct.RenderAssetDiagnosticPlugin.html "struct bevy::render::diagnostic::RenderAssetDiagnosticPlugin")

[RenderDiagnosticsPlugin](struct.RenderDiagnosticsPlugin.html "struct bevy::render::diagnostic::RenderDiagnosticsPlugin")

Enables collecting render diagnostics, such as CPU/GPU elapsed time per render pass, as well as pipeline statistics (number of primitives, number of shader invocations, etc).

[TimeSpanGuard](struct.TimeSpanGuard.html "struct bevy::render::diagnostic::TimeSpanGuard")

Guard returned by [`RecordDiagnostics::time_span`](trait.RecordDiagnostics.html#method.time_span "method bevy::render::diagnostic::RecordDiagnostics::time_span").

## Traits

[RecordDiagnostics](trait.RecordDiagnostics.html "trait bevy::render::diagnostic::RecordDiagnostics")

Allows recording diagnostic spans.

## Functions

[begin\_diagnostics\_frame](fn.begin_diagnostics_frame.html "fn bevy::render::diagnostic::begin_diagnostics_frame")

Starts the diagnostics recorder for the frame.

[resolve\_encoder](fn.resolve_encoder.html "fn bevy::render::diagnostic::resolve_encoder")

Resolves the encoder used for diagnostic recording