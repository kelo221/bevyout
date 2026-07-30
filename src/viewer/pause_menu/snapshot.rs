//! Capture a low-res blurred freeze-frame of the 3D view, then suspend the
//! main camera so the pause menu does not keep rasterizing the scene.

use bevy::asset::RenderAssetUsages;
use bevy::camera::Camera;
use bevy::image::{Image, ImageSampler};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured};
use bevy::ui::IsDefaultUiCamera;
use bevy::ui::widget::NodeImageMode;

use super::ui::{PauseMenuBackdrop, PauseMenuRoot};

/// How many Update ticks to wait for GPU readback before showing the menu
/// with the solid fill (so Esc is never a black hole). Real captures often
/// need more than a handful of frames; a short stall was cancelling them.
const CAPTURE_STALL_FRAMES: u32 = 90;

#[derive(Resource, Default)]
pub(super) struct PauseSnapshot {
    pub(super) handle: Option<Handle<Image>>,
    /// Bumped every enter. Late screenshot observers must still match this
    /// generation while the menu is open — they must not be dropped just
    /// because a stall-reveal already unhid the solid fill.
    pub(super) generation: u64,
    /// `Some` while we still expect a capture for `generation`.
    pub(super) pending_generation: Option<u64>,
    /// Frames waited while the menu is still hidden.
    pub(super) hidden_frames: u32,
}

#[derive(Component)]
pub(super) struct PauseUiCamera;

#[derive(Component)]
struct PendingPauseSnapshot {
    generation: u64,
}

/// Marker that the pause menu hid this entity for the freeze-frame capture.
/// We restore on `OnExit(Paused)` to whatever the visibility sync systems would
/// set (`Inherited` here is fine — `sync_ui_visibility` re-applies next tick).
#[derive(Component)]
pub(super) struct HiddenByPauseMenu;

pub(super) fn hide_gameplay_ui<Marker: Component>(
    mut commands: Commands,
    entities: Query<Entity, With<Marker>>,
) {
    for entity in &entities {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert((Visibility::Hidden, HiddenByPauseMenu));
        }
    }
}

pub(super) fn show_gameplay_ui(
    mut commands: Commands,
    entities: Query<Entity, With<HiddenByPauseMenu>>,
) {
    for entity in &entities {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands
                .insert(Visibility::Inherited)
                .remove::<HiddenByPauseMenu>();
        }
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn begin_snapshot_capture(
    mut commands: Commands,
    mut snapshot: ResMut<PauseSnapshot>,
    mut ui_entities: Query<
        &mut Visibility,
        Or<(
            With<crate::viewer::console::GameUi>,
            With<crate::viewer::console::DiagnosticUi>,
        )>,
    >,
) {
    // Hide reticle, crosshair, and HUD UI elements immediately so they are not
    // baked into the blurred pause background snapshot.
    for mut visibility in &mut ui_entities {
        *visibility = Visibility::Hidden;
    }
    snapshot.generation = snapshot.generation.wrapping_add(1);
    let generation = snapshot.generation;
    snapshot.pending_generation = Some(generation);
    snapshot.hidden_frames = 0;
    // Drop any previous freeze-frame so a re-open never shows a stale cell.
    if let Some(old) = snapshot.handle.take() {
        commands.queue(move |world: &mut World| {
            let _ = world.resource_mut::<Assets<Image>>().remove(old.id());
        });
    }
    info!("pause menu: snapshot capture queued gen={generation}");
    commands
        .spawn((
            Screenshot::primary_window(),
            PendingPauseSnapshot { generation },
        ))
        .observe(on_screenshot_captured);
}

#[allow(clippy::too_many_arguments)]
fn on_screenshot_captured(
    captured: On<ScreenshotCaptured>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut snapshot: ResMut<PauseSnapshot>,
    pending: Query<&PendingPauseSnapshot>,
    mut cameras: Query<&mut Camera, (With<Camera3d>, Without<PauseUiCamera>)>,
    mut backdrops: Query<&mut ImageNode, With<PauseMenuBackdrop>>,
    roots: Query<(Entity, &Visibility), With<PauseMenuRoot>>,
    ui_cameras: Query<Entity, With<PauseUiCamera>>,
) {
    let Ok(pending) = pending.get(captured.entity) else {
        return;
    };
    // Accept the frame if it belongs to the currently open menu generation.
    // Do not require `pending_generation` — a stall-reveal may have cleared
    // that while the GPU readback was still in flight.
    if pending.generation != snapshot.generation {
        info!(
            "pause menu: ignoring stale snapshot gen={} current={}",
            pending.generation, snapshot.generation
        );
        return;
    }
    if roots.is_empty() {
        return;
    }
    snapshot.pending_generation = None;

    let source = &captured.image;
    info!(
        "pause menu: snapshot landed gen={} {}x{} format={:?}",
        pending.generation,
        source.width(),
        source.height(),
        source.texture_descriptor.format
    );

    match process_snapshot_blur(source) {
        Some(blurred) => {
            let handle = images.add(blurred);
            if let Some(old) = snapshot.handle.replace(handle.clone()) {
                images.remove(old.id());
            }
            for mut backdrop in &mut backdrops {
                backdrop.image = handle.clone();
                backdrop.color = Color::WHITE;
                backdrop.image_mode = NodeImageMode::Stretch;
            }
        }
        None => {
            warn!(
                "pause menu: could not downsample screenshot format={:?}; keeping solid fill",
                source.texture_descriptor.format
            );
            // Still reveal so the user is not stuck on a hidden menu.
        }
    }

    // Suspend 3D before unhiding so the live scene never composites under
    // a transparent frame of the overlay.
    suspend_world_camera(&mut cameras, &mut commands, &ui_cameras);
    for (entity, visibility) in &roots {
        if matches!(*visibility, Visibility::Hidden)
            && let Ok(mut entity_commands) = commands.get_entity(entity)
        {
            entity_commands.insert(Visibility::Inherited);
        }
    }
}

/// Fallback: if the capture is slow, unhide the solid-fill menu so Esc still
/// works. A late snapshot for the same generation is still applied on top.
pub(super) fn reveal_if_capture_stalled(
    mut commands: Commands,
    mut snapshot: ResMut<PauseSnapshot>,
    roots: Query<(Entity, &Visibility), With<PauseMenuRoot>>,
    mut cameras: Query<&mut Camera, (With<Camera3d>, Without<PauseUiCamera>)>,
    ui_cameras: Query<Entity, With<PauseUiCamera>>,
) {
    let hidden: Vec<Entity> = roots
        .iter()
        .filter(|(_, visibility)| matches!(*visibility, Visibility::Hidden))
        .map(|(entity, _)| entity)
        .collect();
    if hidden.is_empty() {
        return;
    }
    snapshot.hidden_frames = snapshot.hidden_frames.saturating_add(1);
    if snapshot.hidden_frames < CAPTURE_STALL_FRAMES {
        return;
    }
    warn!(
        "pause menu: snapshot still pending after {} frames; revealing solid fill (late capture still accepted)",
        snapshot.hidden_frames
    );
    // Keep `pending_generation` so diagnostics stay accurate; late captures
    // are accepted by generation match regardless.
    suspend_world_camera(&mut cameras, &mut commands, &ui_cameras);
    for entity in hidden {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.insert(Visibility::Inherited);
        }
    }
}

fn suspend_world_camera(
    cameras: &mut Query<&mut Camera, (With<Camera3d>, Without<PauseUiCamera>)>,
    commands: &mut Commands,
    ui_cameras: &Query<Entity, With<PauseUiCamera>>,
) {
    for mut camera in cameras.iter_mut() {
        camera.is_active = false;
    }
    if ui_cameras.is_empty() {
        ensure_pause_ui_camera(commands);
    }
}

pub(super) fn ensure_pause_ui_camera(commands: &mut Commands) {
    commands.spawn((
        PauseUiCamera,
        Camera2d,
        IsDefaultUiCamera,
        Camera {
            // Above any lingering world camera; solid clear so a disabled
            // Camera3d cannot composite the live scene underneath the menu.
            order: 10,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.02, 0.03, 0.01)),
            ..default()
        },
    ));
}

pub(super) fn restore_world_camera(
    mut commands: Commands,
    mut snapshot: ResMut<PauseSnapshot>,
    mut images: ResMut<Assets<Image>>,
    mut cameras: Query<&mut Camera, (With<Camera3d>, Without<PauseUiCamera>)>,
    ui_cameras: Query<Entity, With<PauseUiCamera>>,
) {
    snapshot.pending_generation = None;
    snapshot.hidden_frames = 0;
    // Bump generation so any in-flight capture from this session is ignored
    // after close (otherwise it could paint a closed menu's backdrop resource).
    snapshot.generation = snapshot.generation.wrapping_add(1);
    if let Some(handle) = snapshot.handle.take() {
        images.remove(handle.id());
    }
    for mut camera in &mut cameras {
        camera.is_active = true;
    }
    for entity in &ui_cameras {
        commands.entity(entity).despawn();
    }
}

/// Process the full-resolution screenshot with a separable blur pass and warm CRT phosphor grade.
pub(super) fn process_snapshot_blur(source: &Image) -> Option<Image> {
    let mut rgba = screenshot_to_rgba8(source)?;
    let width = rgba.width;
    let height = rgba.height;
    if width == 0 || height == 0 {
        return None;
    }

    // Full-resolution separable blur pass across native 1:1 screenshot texels.
    apply_separable_blur(&mut rgba.bytes, width, height, 16);

    // Sepia phosphor grade: classic warm-brown sepia matrix over the blurred
    // freeze-frame, with a small lift so dark scenes keep a base CRT glow.
    for pixel in rgba.bytes.chunks_exact_mut(4) {
        let r = f32::from(pixel[0]);
        let g = f32::from(pixel[1]);
        let b = f32::from(pixel[2]);
        let sepia_r = 0.393 * r + 0.769 * g + 0.189 * b;
        let sepia_g = 0.349 * r + 0.686 * g + 0.168 * b;
        let sepia_b = 0.272 * r + 0.534 * g + 0.131 * b;
        pixel[0] = (sepia_r * 1.02 + 12.0).clamp(0.0, 255.0) as u8;
        pixel[1] = (sepia_g * 0.98 + 10.0).clamp(0.0, 255.0) as u8;
        pixel[2] = (sepia_b * 0.92 + 6.0).clamp(0.0, 255.0) as u8;
        pixel[3] = 255;
    }

    let mut image = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba.bytes,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::linear();
    Some(image)
}

fn apply_separable_blur(bytes: &mut [u8], width: u32, height: u32, radius: usize) {
    let w = width as usize;
    let h = height as usize;
    if w == 0 || h == 0 || radius == 0 {
        return;
    }

    let mut temp = vec![0_u8; bytes.len()];
    let r = radius as i32;

    // Horizontal pass: sliding window along each row
    for y in 0..h {
        let row_offset = y * w * 4;
        let mut sum = [0_u64; 4];
        let mut count = 0_u64;

        // Initialize sum for x = 0 (window 0 .. r)
        for kx in 0..=(r as usize).min(w - 1) {
            let px = row_offset + kx * 4;
            for c in 0..4 {
                sum[c] += u64::from(bytes[px + c]);
            }
            count += 1;
        }

        for x in 0..w {
            let xi = x as i32;
            let add_x = xi + r;
            if add_x > r && add_x < w as i32 {
                let px = row_offset + (add_x as usize) * 4;
                for c in 0..4 {
                    sum[c] += u64::from(bytes[px + c]);
                }
                count += 1;
            }

            let rem_x = xi - r - 1;
            if rem_x >= 0 {
                let px = row_offset + (rem_x as usize) * 4;
                for c in 0..4 {
                    sum[c] -= u64::from(bytes[px + c]);
                }
                count -= 1;
            }

            let dst = row_offset + x * 4;
            let div = count.max(1);
            for c in 0..4 {
                temp[dst + c] = (sum[c] / div) as u8;
            }
        }
    }

    // Vertical pass: sliding window along each column
    for x in 0..w {
        let mut sum = [0_u64; 4];
        let mut count = 0_u64;

        // Initialize sum for y = 0 (window 0 .. r)
        for ky in 0..=(r as usize).min(h - 1) {
            let px = (ky * w + x) * 4;
            for c in 0..4 {
                sum[c] += u64::from(temp[px + c]);
            }
            count += 1;
        }

        for y in 0..h {
            let yi = y as i32;
            let add_y = yi + r;
            if add_y > r && add_y < h as i32 {
                let px = ((add_y as usize) * w + x) * 4;
                for c in 0..4 {
                    sum[c] += u64::from(temp[px + c]);
                }
                count += 1;
            }

            let rem_y = yi - r - 1;
            if rem_y >= 0 {
                let px = ((rem_y as usize) * w + x) * 4;
                for c in 0..4 {
                    sum[c] -= u64::from(temp[px + c]);
                }
                count -= 1;
            }

            let dst = (y * w + x) * 4;
            let div = count.max(1);
            for c in 0..4 {
                bytes[dst + c] = (sum[c] / div) as u8;
            }
        }
    }
}

struct Rgba8Image {
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

/// Convert a window screenshot into tightly-packed RGBA8.
///
/// Window surfaces are often `Bgra8Unorm` / `Bgra8UnormSrgb` (not covered by a
/// plain `try_into_dynamic` path on every Bevy build), and HDR paths may use
/// 16-bit float. Handle the common cases explicitly so the freeze-frame is not
/// silently dropped.
fn screenshot_to_rgba8(source: &Image) -> Option<Rgba8Image> {
    let width = source.width();
    let height = source.height();
    let data = source.data.as_ref()?;
    if width == 0 || height == 0 {
        return None;
    }

    match source.texture_descriptor.format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            if data.len() < (width as usize) * (height as usize) * 4 {
                return None;
            }
            Some(Rgba8Image {
                width,
                height,
                bytes: data[..(width as usize) * (height as usize) * 4].to_vec(),
            })
        }
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            let px = (width as usize) * (height as usize);
            if data.len() < px * 4 {
                return None;
            }
            let mut bytes = Vec::with_capacity(px * 4);
            for chunk in data[..px * 4].chunks_exact(4) {
                bytes.extend_from_slice(&[chunk[2], chunk[1], chunk[0], chunk[3]]);
            }
            Some(Rgba8Image {
                width,
                height,
                bytes,
            })
        }
        TextureFormat::Rgba16Float => {
            let px = (width as usize) * (height as usize);
            if data.len() < px * 8 {
                return None;
            }
            let mut bytes = Vec::with_capacity(px * 4);
            for chunk in data[..px * 8].chunks_exact(8) {
                let r = half_to_f32([chunk[0], chunk[1]]);
                let g = half_to_f32([chunk[2], chunk[3]]);
                let b = half_to_f32([chunk[4], chunk[5]]);
                let a = half_to_f32([chunk[6], chunk[7]]);
                bytes.push(linear_to_u8(r));
                bytes.push(linear_to_u8(g));
                bytes.push(linear_to_u8(b));
                bytes.push(linear_to_u8(a));
            }
            Some(Rgba8Image {
                width,
                height,
                bytes,
            })
        }
        other => {
            // Fall back to Bevy's converter for anything else it knows.
            match source.clone().try_into_dynamic() {
                Ok(dyn_image) => {
                    let rgba = dyn_image.to_rgba8();
                    Some(Rgba8Image {
                        width: rgba.width(),
                        height: rgba.height(),
                        bytes: rgba.into_raw(),
                    })
                }
                Err(error) => {
                    warn!("pause menu: unsupported screenshot format {other:?}: {error}");
                    None
                }
            }
        }
    }
}

fn half_to_f32(bytes: [u8; 2]) -> f32 {
    let bits = u16::from_le_bytes(bytes);
    // IEEE 754 binary16 → f32
    let sign = (bits >> 15) & 1;
    let exp = (bits >> 10) & 0x1f;
    let frac = bits & 0x3ff;
    let value = if exp == 0 {
        if frac == 0 {
            0.0
        } else {
            let mut f = frac as f32 / 1024.0;
            f *= 2f32.powi(-14);
            f
        }
    } else if exp == 31 {
        if frac == 0 { f32::INFINITY } else { f32::NAN }
    } else {
        let mut f = 1.0 + (frac as f32 / 1024.0);
        f *= 2f32.powi(exp as i32 - 15);
        f
    };
    if sign == 0 { value } else { -value }
}

fn linear_to_u8(value: f32) -> u8 {
    // Rough tonemap + sRGB encode so HDR screenshots stay visible.
    let tonemapped = value / (1.0 + value.max(0.0));
    let srgb = if tonemapped <= 0.003_130_8 {
        tonemapped * 12.92
    } else {
        1.055 * tonemapped.powf(1.0 / 2.4) - 0.055
    };
    (srgb.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// 1×4 scanline strip (transparent, transparent, dark, dark) for tiling.
pub(super) fn scanline_texture() -> Image {
    let data = vec![
        0, 0, 0, 0, //
        0, 0, 0, 0, //
        0, 0, 0, 55, //
        0, 0, 0, 55,
    ];
    let mut image = Image::new(
        Extent3d {
            width: 1,
            height: 4,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::nearest();
    image
}

#[cfg(test)]
#[path = "tests/snapshot.rs"]
mod tests;
