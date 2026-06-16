#![allow(missing_docs)]

use crate::{
    plugin_h::{self, OdenRay},
    CameraCalibration, ODEN_GLOBAL,
};
use std::{error::Error, fmt::Display, str::FromStr};

#[cfg(feature = "glam_conversion")]
use glam;
#[cfg(feature = "nalgebra_conversion")]
use nalgebra;

pub use plugin_h::{
    OdenMatrix4_s as Matrix4, OdenQuat_s as Quat, OdenRay_s as Ray, OdenUuid_s as Uuid,
    OdenVec2_s as Vec2, OdenVec2i_s as Vec2i, OdenVec3_s as Vec3, OdenVec4_s as Vec4,
};

impl Vec2 {
    pub const fn xy(x: f32, y: f32) -> Self {
        Self { d: [x, y] }
    }

    pub const fn x(&self) -> f32 {
        self.d[0]
    }

    pub const fn y(&self) -> f32 {
        self.d[1]
    }
}

#[cfg(feature = "glam_conversion")]
impl From<glam::Vec2> for Vec2 {
    fn from(v: glam::Vec2) -> Self {
        Vec2::xy(v.x, v.y)
    }
}

#[cfg(feature = "glam_conversion")]
impl From<Vec2> for glam::Vec2 {
    fn from(v: Vec2) -> Self {
        glam::Vec2::new(v.x(), v.y())
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<nalgebra::Vector2<f32>> for Vec2 {
    fn from(v: nalgebra::Vector2<f32>) -> Self {
        Vec2::xy(v[0], v[1])
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<Vec2> for nalgebra::Vector2<f32> {
    fn from(v: Vec2) -> Self {
        nalgebra::Vector2::new(v.x(), v.y())
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(v: (f32, f32)) -> Self {
        Vec2::xy(v.0, v.1)
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(v: [f32; 2]) -> Self {
        Vec2::xy(v[0], v[1])
    }
}

impl Vec2i {
    pub const fn xy(x: i32, y: i32) -> Self {
        Self { d: [x, y] }
    }

    pub const fn x(&self) -> i32 {
        self.d[0]
    }

    pub const fn y(&self) -> i32 {
        self.d[1]
    }
}

#[cfg(feature = "glam_conversion")]
impl From<glam::IVec2> for Vec2i {
    fn from(v: glam::IVec2) -> Self {
        Vec2i::xy(v.x, v.y)
    }
}

#[cfg(feature = "glam_conversion")]
impl From<Vec2i> for glam::IVec2 {
    fn from(v: Vec2i) -> Self {
        glam::IVec2::new(v.x(), v.y())
    }
}

impl From<(i32, i32)> for Vec2i {
    fn from(v: (i32, i32)) -> Self {
        Vec2i::xy(v.0, v.1)
    }
}

impl From<[i32; 2]> for Vec2i {
    fn from(v: [i32; 2]) -> Self {
        Vec2i::xy(v[0], v[1])
    }
}

impl Vec3 {
    pub const fn new() -> Self {
        Self { d: [0.0, 0.0, 0.0] }
    }

    pub const fn xyz(x: f32, y: f32, z: f32) -> Self {
        Self { d: [x, y, z] }
    }

    pub const fn x(&self) -> f32 {
        self.d[0]
    }

    pub const fn y(&self) -> f32 {
        self.d[1]
    }

    pub const fn z(&self) -> f32 {
        self.d[2]
    }
}

#[cfg(feature = "glam_conversion")]
impl From<glam::Vec3> for Vec3 {
    fn from(v: glam::Vec3) -> Self {
        Vec3::xyz(v.x, v.y, v.z)
    }
}

#[cfg(feature = "glam_conversion")]
impl From<Vec3> for glam::Vec3 {
    fn from(v: Vec3) -> Self {
        glam::Vec3::new(v.x(), v.y(), v.z())
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<nalgebra::Vector3<f32>> for Vec3 {
    fn from(v: nalgebra::Vector3<f32>) -> Self {
        Vec3::xyz(v[0], v[1], v[2])
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<Vec3> for nalgebra::Vector3<f32> {
    fn from(v: Vec3) -> Self {
        nalgebra::Vector3::new(v.x(), v.y(), v.z())
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(v: (f32, f32, f32)) -> Self {
        Vec3::xyz(v.0, v.1, v.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(v: [f32; 3]) -> Self {
        Vec3::xyz(v[0], v[1], v[2])
    }
}

impl Vec4 {
    pub const fn xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { d: [x, y, z, w] }
    }

    /// When used for color information, the range is ```[0, 1]```
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { d: [r, g, b, a] }
    }

    pub const fn x(&self) -> f32 {
        self.d[0]
    }

    pub const fn y(&self) -> f32 {
        self.d[1]
    }

    pub const fn z(&self) -> f32 {
        self.d[2]
    }

    pub const fn w(&self) -> f32 {
        self.d[3]
    }
}

#[cfg(feature = "glam_conversion")]
impl From<glam::Vec4> for Vec4 {
    fn from(v: glam::Vec4) -> Self {
        Vec4::xyzw(v.x, v.y, v.z, v.w)
    }
}

#[cfg(feature = "glam_conversion")]
impl From<Vec4> for glam::Vec4 {
    fn from(v: Vec4) -> Self {
        glam::Vec4::new(v.x(), v.y(), v.z(), v.w())
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<nalgebra::Vector4<f32>> for Vec4 {
    fn from(v: nalgebra::Vector4<f32>) -> Self {
        Vec4::xyzw(v[0], v[1], v[2], v[3])
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<Vec4> for nalgebra::Vector4<f32> {
    fn from(v: Vec4) -> Self {
        nalgebra::Vector4::new(v.x(), v.y(), v.z(), v.w())
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Vec4::xyzw(v.0, v.1, v.2, v.3)
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(v: [f32; 4]) -> Self {
        Vec4::xyzw(v[0], v[1], v[2], v[3])
    }
}

impl Quat {
    pub const fn wxyz(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { d: [w, x, y, z] }
    }

    pub const fn w(&self) -> f32 {
        self.d[0]
    }

    pub const fn x(&self) -> f32 {
        self.d[1]
    }

    pub const fn y(&self) -> f32 {
        self.d[2]
    }

    pub const fn z(&self) -> f32 {
        self.d[3]
    }
}

#[cfg(feature = "glam_conversion")]
impl From<glam::Quat> for Quat {
    fn from(v: glam::Quat) -> Self {
        Quat::wxyz(v.w, v.x, v.y, v.z)
    }
}

#[cfg(feature = "glam_conversion")]
impl From<Quat> for glam::Quat {
    fn from(v: Quat) -> Self {
        glam::Quat::from_xyzw(v.x(), v.y(), v.z(), v.w())
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<nalgebra::Quaternion<f32>> for Quat {
    fn from(v: nalgebra::Quaternion<f32>) -> Self {
        Quat::wxyz(v.i, v.j, v.k, v.w)
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<Quat> for nalgebra::Quaternion<f32> {
    fn from(v: Quat) -> Self {
        nalgebra::Quaternion::new(v.x(), v.y(), v.z(), v.w())
    }
}

impl Matrix4 {
    pub const fn identity() -> Matrix4 {
        Self {
            d: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

#[cfg(feature = "glam_conversion")]
impl From<glam::Mat4> for Matrix4 {
    fn from(mat: glam::Mat4) -> Self {
        Matrix4 {
            d: mat.to_cols_array_2d(),
        }
    }
}

#[cfg(feature = "glam_conversion")]
impl From<Matrix4> for glam::Mat4 {
    fn from(mat: Matrix4) -> Self {
        glam::Mat4::from_cols_array_2d(&mat.d)
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<nalgebra::Matrix4<f32>> for Matrix4 {
    fn from(m: nalgebra::Matrix4<f32>) -> Self {
        Self {
            d: [
                [m.m11, m.m12, m.m13, m.m14],
                [m.m21, m.m22, m.m23, m.m24],
                [m.m31, m.m32, m.m33, m.m34],
                [m.m41, m.m42, m.m43, m.m44],
            ],
        }
    }
}

#[cfg(feature = "nalgebra_conversion")]
impl From<Matrix4> for nalgebra::Matrix4<f32> {
    fn from(m: Matrix4) -> Self {
        nalgebra::Matrix4::new(
            m.d[0][0], m.d[0][1], m.d[0][2], m.d[0][3], m.d[1][0], m.d[1][1], m.d[1][2], m.d[1][3],
            m.d[2][0], m.d[2][1], m.d[2][2], m.d[2][3], m.d[3][0], m.d[3][1], m.d[3][2], m.d[3][3],
        )
    }
}

impl From<[[f32; 4]; 4]> for Matrix4 {
    fn from(v: [[f32; 4]; 4]) -> Self {
        Self { d: v }
    }
}

impl From<&[f32]> for Matrix4 {
    fn from(v: &[f32]) -> Self {
        Self {
            d: [
                [v[0], v[1], v[2], v[3]],
                [v[4], v[5], v[6], v[7]],
                [v[8], v[9], v[10], v[11]],
                [v[12], v[13], v[14], v[15]],
            ],
        }
    }
}

pub fn project_camera(
    calibration: CameraCalibration,
    image_width: i32,
    image_height: i32,
    pixel_coords: Vec2,
) -> Option<OdenRay> {
    unsafe {
        assert!(!ODEN_GLOBAL.is_null());
        if let Some(project_camera) = (*ODEN_GLOBAL).projectCamera {
            return Some(project_camera(
                calibration,
                image_width,
                image_height,
                pixel_coords,
            ));
        }
    }
    None
}
pub fn closest_point_between_rays(ray1: Ray, ray2: Ray) -> Option<Vec3> {
    unsafe {
        assert!(!ODEN_GLOBAL.is_null());
        if let Some(closest_point_between_rays) = (*ODEN_GLOBAL).closestPointBetweenRays {
            let mut vec = Vec3::default();
            closest_point_between_rays(ray1, ray2, &mut vec as *mut _);
            return Some(vec);
        }
    }
    None
}
pub fn mult_matrix(a: Matrix4, b: Matrix4) -> Option<Matrix4> {
    unsafe {
        assert!(!ODEN_GLOBAL.is_null());
        if let Some(mult_matrix) = (*ODEN_GLOBAL).multMatrix {
            let mut mat = Matrix4::default();
            mult_matrix(a, b, &mut mat as *mut _);
            return Some(mat);
        }
    }
    None
}
pub fn get_rotation_xyz(mat: Matrix4) -> Option<Vec3> {
    unsafe {
        assert!(!ODEN_GLOBAL.is_null());
        if let Some(get_rotation_xyz) = (*ODEN_GLOBAL).getRotationXyz {
            return Some(get_rotation_xyz(mat));
        }
    }
    None
}

fn u8_to_hex(mut i: u8) -> char {
    i &= 0x0F;
    if i <= 9 {
        return (b'0' + i) as char;
    }
    (b'a' + i - 10) as char
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.uuid.iter().enumerate() {
            let v1 = v >> 4;
            let v2 = v & 0x0F;
            write!(f, "{}", u8_to_hex(v1))?;
            write!(f, "{}", u8_to_hex(v2))?;

            if i == 3 || i == 5 || i == 7 || i == 9 {
                write!(f, "-")?;
            }
        }
        Ok(())
    }
}

impl FromStr for Uuid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36
            || s.chars().nth(8) != Some('-')
            || s.chars().nth(13) != Some('-')
            || s.chars().nth(18) != Some('-')
            || s.chars().nth(23) != Some('-')
        {
            return Err("Invalid Uuid Format!".into());
        }

        let mut parts = s.split('-');

        let p1 = u64::from_str_radix(parts.next().ok_or("Missing part 1 of Uuid")?, 16)?;
        let p2 = u16::from_str_radix(parts.next().ok_or("Missing part 2 of Uuid")?, 16)?;
        let p3 = u16::from_str_radix(parts.next().ok_or("Missing part 3 of Uuid")?, 16)?;
        let p4 = u16::from_str_radix(parts.next().ok_or("Missing part 4 of Uuid")?, 16)?;
        let p5 = u64::from_str_radix(parts.next().ok_or("Missing part 5 of Uuid")?, 16)?;

        Ok(Uuid {
            uuid: [
                (p1 >> 24) as u8,
                (p1 >> 16) as u8,
                (p1 >> 8) as u8,
                p1 as u8,
                (p2 >> 8) as u8,
                p2 as u8,
                (p3 >> 8) as u8,
                p3 as u8,
                (p4 >> 8) as u8,
                p4 as u8,
                (p5 >> 40) as u8,
                (p5 >> 32) as u8,
                (p5 >> 24) as u8,
                (p5 >> 16) as u8,
                (p5 >> 8) as u8,
                p5 as u8,
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Uuid;
    use std::str::FromStr;

    #[test]
    fn valid_uuid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let uuid = Uuid::from_str(uuid_str).unwrap();
        let expected_bytes = [
            0x55, 0x0e, 0x84, 0x00, 0xe2, 0x9b, 0x41, 0xd4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44,
            0x00, 0x00,
        ];
        assert_eq!(uuid.uuid, expected_bytes);
    }

    #[test]
    fn invalid_uuid_length() {
        let uuid_str = "550e8400-e29b-41d4-a716-44665544000";
        assert!(Uuid::from_str(uuid_str).is_err());
    }

    #[test]
    fn invalid_uuid_format() {
        let uuid_str = "550e8400e29b41d4a716446655440000";
        assert!(Uuid::from_str(uuid_str).is_err());
    }

    #[test]
    fn invalid_uuid_part() {
        let uuid_str = "550e8400-e29b-41d4-a716-xyz655440000";
        assert!(Uuid::from_str(uuid_str).is_err());
    }

    #[test]
    fn missing_uuid_part() {
        let uuid_str = "550e8400-e29b-41d4--446655440000";
        assert!(Uuid::from_str(uuid_str).is_err());
    }
}
