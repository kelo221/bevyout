[bevy](../../index.html)::[pbr](../index.html)

# Module contact\_shadows 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lib.rs.html#29)

Contact shadows implemented via screenspace raymarching.

## Structs

[ContactShadows](struct.ContactShadows.html "struct bevy::pbr::contact_shadows::ContactShadows")

Add this component to a camera to enable contact shadows.

[ContactShadowsBuffer](struct.ContactShadowsBuffer.html "struct bevy::pbr::contact_shadows::ContactShadowsBuffer")

A GPU buffer that stores the contact shadow settings for each view.

[ContactShadowsPlugin](struct.ContactShadowsPlugin.html "struct bevy::pbr::contact_shadows::ContactShadowsPlugin")

Enables contact shadows for a camera.

[ContactShadowsUniform](struct.ContactShadowsUniform.html "struct bevy::pbr::contact_shadows::ContactShadowsUniform")

A version of [`ContactShadows`](../struct.ContactShadows.html "struct bevy::pbr::ContactShadows") for upload to the GPU.

[ViewContactShadowsUniformOffset](struct.ViewContactShadowsUniformOffset.html "struct bevy::pbr::contact_shadows::ViewContactShadowsUniformOffset")

A component that stores the offset within the [`ContactShadowsBuffer`](../struct.ContactShadowsBuffer.html "struct bevy::pbr::ContactShadowsBuffer") for each view.