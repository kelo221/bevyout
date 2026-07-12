[bevy](../../index.html)::[post\_process](../index.html)::[dof](index.html)

# Function calculate\_focal\_length 

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#710)

```rust
pub fn calculate_focal_length(sensor_height: f32, fov: f32) -> f32
```

Given the sensor height and the FOV, returns the focal length.

See [https://photo.stackexchange.com/a/97218](https://photo.stackexchange.com/a/97218).

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/3d/depth\_of\_field.rs ([line 253](../../../src/depth_of_field/depth_of_field.rs.html#253))

```rust
233    fn help_text(&self) -> String {
234        let Some(mode) = self.mode else {
235            return "Mode: Off (Press Space to change)".to_owned();
236        };
237
238        // We leave these as their defaults, so we don't need to store them in
239        // the app settings and can just fetch them from the default camera
240        // parameters.
241        let sensor_height = PhysicalCameraParameters::default().sensor_height;
242        let fov = PerspectiveProjection::default().fov;
243
244        format!(
245            "Focal distance: {:.2} m (Press Up/Down to change)
246Aperture F-stops: f/{:.3} (Press Left/Right to change)
247Sensor height: {:.2}mm
248Focal length: {:.2}mm
249Mode: {} (Press Space to change)",
250            self.focal_distance,
251            self.aperture_f_stops,
252            sensor_height * 1000.0,
253            dof::calculate_focal_length(sensor_height, fov) * 1000.0,
254            match mode {
255                DepthOfFieldMode::Bokeh => "Bokeh",
256                DepthOfFieldMode::Gaussian => "Gaussian",
257            }
258        )
259    }
```