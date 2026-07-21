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

/// Target width of the freeze-frame. Height keeps the source aspect ratio.
pub(super) const SNAPSHOT_WIDTH: u32 = 160;

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

pub(super) fn begin_snapshot_capture(mut commands: Commands, mut snapshot: ResMut<PauseSnapshot>) {
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

    match downsample_box_blur(source, SNAPSHOT_WIDTH) {
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
        if matches!(*visibility, Visibility::Hidden) {
            if let Ok(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(Visibility::Inherited);
            }
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

/// Average-pool the screenshot down to `target_width`, keeping aspect ratio.
/// Stretching this tiny texture with linear sampling yields a cheap blur.
pub(super) fn downsample_box_blur(source: &Image, target_width: u32) -> Option<Image> {
    let target_width = target_width.max(1);
    let rgba = screenshot_to_rgba8(source)?;
    let src_w = rgba.width.max(1);
    let src_h = rgba.height.max(1);
    let target_height = ((u64::from(src_h) * u64::from(target_width)) / u64::from(src_w))
        .max(1)
        .min(u64::from(u32::MAX)) as u32;

    let mut out = vec![0_u8; (target_width * target_height * 4) as usize];
    for ty in 0..target_height {
        let y0 = (u64::from(ty) * u64::from(src_h)) / u64::from(target_height);
        let y1 = ((u64::from(ty) + 1) * u64::from(src_h)) / u64::from(target_height);
        let y1 = y1.max(y0 + 1);
        for tx in 0..target_width {
            let x0 = (u64::from(tx) * u64::from(src_w)) / u64::from(target_width);
            let x1 = ((u64::from(tx) + 1) * u64::from(src_w)) / u64::from(target_width);
            let x1 = x1.max(x0 + 1);
            let mut sum = [0_u64; 4];
            let mut count = 0_u64;
            for y in y0..y1 {
                for x in x0..x1 {
                    let idx = ((y * u64::from(src_w) + x) * 4) as usize;
                    for channel in 0..4 {
                        sum[channel] += u64::from(rgba.bytes[idx + channel]);
                    }
                    count += 1;
                }
            }
            let count = count.max(1);
            let dst = ((ty * target_width + tx) * 4) as usize;
            for channel in 0..4 {
                out[dst + channel] = (sum[channel] / count) as u8;
            }
        }
    }

    // Warm CRT phosphor lean: push greens up slightly and crush blue.
    for pixel in out.chunks_exact_mut(4) {
        let r = f32::from(pixel[0]);
        let g = f32::from(pixel[1]);
        let b = f32::from(pixel[2]);
        pixel[0] = (r * 0.92 + g * 0.08).clamp(0.0, 255.0) as u8;
        pixel[1] = (g * 1.05 + r * 0.05).clamp(0.0, 255.0) as u8;
        pixel[2] = (b * 0.55).clamp(0.0, 255.0) as u8;
        pixel[3] = 255;
    }

    let mut image = Image::new(
        Extent3d {
            width: target_width,
            height: target_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        out,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    // Linear sampling is the actual "blur" once the tiny texture is stretched.
    image.sampler = ImageSampler::linear();
    Some(image)
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
mod tests {
    use super::*;

    fn solid_image(width: u32, height: u32, format: TextureFormat, color: [u8; 4]) -> Image {
        let data = match format {
            TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
                // Store as BGRA in memory.
                [color[2], color[1], color[0], color[3]]
                    .iter()
                    .cycle()
                    .take((width * height * 4) as usize)
                    .copied()
                    .collect()
            }
            _ => color
                .iter()
                .cycle()
                .take((width * height * 4) as usize)
                .copied()
                .collect(),
        };
        Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            format,
            RenderAssetUsages::MAIN_WORLD,
        )
    }

    #[test]
    fn downsample_preserves_aspect_and_reduces_resolution() {
        let source = solid_image(320, 180, TextureFormat::Rgba8UnormSrgb, [10, 20, 30, 255]);
        let blurred = downsample_box_blur(&source, 80).expect("downsample");
        assert_eq!(blurred.width(), 80);
        assert_eq!(blurred.height(), 45);
    }

    #[test]
    fn downsample_accepts_bgra_window_format() {
        let source = solid_image(64, 32, TextureFormat::Bgra8UnormSrgb, [10, 20, 30, 255]);
        let blurred = downsample_box_blur(&source, 16).expect("bgra downsample");
        assert_eq!(blurred.width(), 16);
        assert_eq!(blurred.height(), 8);
        // First pixel should be near the CRT-tinted source RGB (not swapped).
        let data = blurred.data.as_ref().expect("cpu bytes");
        assert!(data[0] > 5, "red channel present: {:?}", &data[..4]);
        assert!(data[1] > 10, "green channel present: {:?}", &data[..4]);
    }
}
