#[cfg(feature = "glam_conversion")]
use crate::{
    math::{Vec2, Vec2i, Vec3},
    plugin_h::{OdenCameraCalibration, OdenCameraCropData},
};

pub(crate) fn utf8_from_raw(buf: &[u8]) -> Result<String, std::string::FromUtf8Error> {
    // Filter buf from any '\0' before creating string as it seems to cause panics
    // at some print/log calls.
    if let Some(p) = buf.iter().position(|&v| v == 0) {
        String::from_utf8(buf[..p].to_vec())
    } else {
        Ok(String::new())
    }
}

// For performance reasons we don't want to use `utf8_from_raw` for large strings
// We use this for settings strings and scene strings as they should always be valid utf-8
pub(crate) fn utf8_from_raw_to_first_null_terminator(
    buf: &[u8],
) -> Result<String, std::string::FromUtf8Error> {
    // Find the position of the first null terminator
    let end_pos = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());

    let valid_bytes = &buf[..end_pos];

    String::from_utf8(valid_bytes.to_vec())
}

#[cfg(feature = "glam_conversion")]
#[derive(Debug, Default)]
pub struct ReprojectionResult {
    pub uv_coords: Vec2,
    pub pixel_coords: Vec2,
}

/// Converts a point in space to camera coordinate for a video in a stitched video.
/// This is useful if the user want be able to press a pixel coordinate on screen and some
/// operation should be done on that pixel in the camera image.
/// Example:
/// We have a stitched video named "My Stitched video entity" with 3 videos(Stream 1, 2 and 3).
/// When the user presses somewhere in the stitched video we will display which pixels in the original video that the mouse was over.
/// ```no_run
/// # fn example(api: &oden_plugin_rs::UpdateParams) {
///     if api.mouse_pressed_since_last_swap(){
///         let mouse_ray = api.mouse_ray();
///         for i in 1..4 {
///             if let (Some(calibration), Some(video_size), Some(crop)) = (
///                 api.camera_calibration("My Stitched Video Entity", i),
///                 api.entity_video_size("My Stitched Video Entity", i),
///                 api.camera_crop("My Stitched Video Entity", i),
///             ) {
///                 if let Some(result) = oden_plugin_rs::utils::reproject_point_to_camera_coords(
///                     mouse_ray.direction,
///                     calibration,
///                     crop,
///                     video_size,
///                 ) {
///                     oden_plugin_rs::log::info!("User pressed video {} camera pixel {} {}", i, result.pixel_coords.x(), result.pixel_coords.y());
///                 }
///             }
///         }
///     }
/// # }
/// ```
#[cfg(feature = "glam_conversion")]
pub fn reproject_point_to_camera_coords(
    point_to_reproject: Vec3,
    calibration: OdenCameraCalibration,
    crop: OdenCameraCropData,
    resolution: Vec2i,
) -> Option<ReprojectionResult> {
    let mut point: glam::Vec3 = point_to_reproject.into();
    let width = resolution.x();
    let height = resolution.y();

    let camera_rotation = glam::Quat::from_array(calibration.rotation);
    let global_position = glam::Vec3::from_array(calibration.globalPosition);
    let global_rotation = glam::Quat::from_array(calibration.globalRotation);

    {
        point = point - global_rotation.mul_vec3(global_position) + global_position;
        point = ((global_rotation * camera_rotation).inverse()).mul_vec3(point);

        // point is now in camera local coords

        let aspect = width as f32 / height as f32;

        let mut p_norm = point.length();
        p_norm = p_norm.max(0.00000000001);

        let theta = (-point.z / p_norm).acos();

        let r = calibration.k[0] * theta
            + calibration.k[1] * theta * theta
            + calibration.k[2] * theta.powi(3)
            + calibration.k[3] * theta.powi(4)
            + calibration.k[4] * theta.powi(5);

        let image_unit = glam::Vec2::new(point.x, point.y).normalize();

        let mut pos = r * image_unit;

        let to_lower_left_crop_edge =
            glam::Vec2::new(2.0 * aspect * crop.left - aspect, 2.0 * crop.down - 1.0);
        let after_crop_center_to_top_right_corner = glam::Vec2::new(
            aspect * (1.0 - crop.left - crop.right),
            1.0 - crop.down - crop.up,
        );
        let full_crop_distance = after_crop_center_to_top_right_corner.length();
        let center_of_crop = to_lower_left_crop_edge + after_crop_center_to_top_right_corner;
        let crop_distance = (pos - center_of_crop).length();
        let circular_crop_diff =
            (full_crop_distance - crop_distance) / full_crop_distance - crop.circular;

        pos += glam::Vec2::new(calibration.offset[0], calibration.offset[1]);

        pos.x /= aspect;

        let uv = pos * 0.5 + glam::Vec2::new(0.5, 0.5);

        let theta_diff = calibration.maxTheta - theta;

        let uv_coords = glam::Vec2::new(uv.x * aspect, 1.0 - uv.y);
        let pixel_coords = glam::Vec2::new(uv.x * width as f32, (1.0 - uv.y) * height as f32);

        if theta_diff < 0.0
            || uv.x < 0.0 + crop.left
            || uv.x > 1.0 - crop.right
            || uv.y < 0.0 + crop.down
            || uv.y > 1.0 - crop.up
            || circular_crop_diff < 0.0
        {
            return None;
        }

        Some(ReprojectionResult {
            uv_coords: uv_coords.into(),
            pixel_coords: pixel_coords.into(),
        })
    }
}
