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

impl Uuid {
    /// Parse a canonical hyphenated UUID string, `None` if malformed.
    pub const fn try_parse(s: &str) -> Option<Self> {
        const HEX: [u8; 256] = {
            let mut t = [0xFFu8; 256];
            let mut i = 0u8;
            while i < 10 {
                t[(b'0' + i) as usize] = i;
                i += 1;
            }
            let mut i = 0u8;
            while i < 6 {
                t[(b'a' + i) as usize] = 10 + i;
                t[(b'A' + i) as usize] = 10 + i;
                i += 1;
            }
            t
        };
        const PAIR_POS: [usize; 16] = [0, 2, 4, 6, 9, 11, 14, 16, 19, 21, 24, 26, 28, 30, 32, 34];

        let b = s.as_bytes();
        if b.len() != 36 || b[8] != b'-' || b[13] != b'-' || b[18] != b'-' || b[23] != b'-' {
            return None;
        }

        let mut uuid = [0u8; 16];
        let mut k = 0;
        while k < 16 {
            let i = PAIR_POS[k];
            let hi = HEX[b[i] as usize];
            let lo = HEX[b[i + 1] as usize];
            if (hi | lo) > 0x0F {
                return None;
            }
            uuid[k] = (hi << 4) | lo;
            k += 1;
        }

        Some(Uuid { uuid })
    }

    /// Parse a canonical hyphenated UUID string. Evaluable in `const` context,
    /// so a malformed literal is rejected at compile time:
    ///
    /// ```
    /// use oden_plugin_rs::math::Uuid;
    /// const BUS: Uuid = Uuid::parse("f6599e51-0ea3-46c1-9907-0e7d0334a807");
    /// ```
    ///
    /// ```compile_fail
    /// use oden_plugin_rs::math::Uuid;
    /// const BAD: Uuid = Uuid::parse("not-a-uuid");
    /// ```
    ///
    /// For strings only known at runtime, use [`std::str::FromStr`], which
    /// returns an error instead of panicking.
    #[track_caller]
    pub const fn parse(s: &str) -> Self {
        match Self::try_parse(s) {
            Some(uuid) => uuid,
            None => panic!("invalid UUID: expected 36 characters in 8-4-4-4-12 hex format"),
        }
    }
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
        Self::try_parse(s).ok_or_else(|| "Invalid Uuid Format!".into())
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

    #[test]
    fn parse_in_const_context_matches_runtime() {
        const UUID: Uuid = Uuid::parse("550e8400-e29b-41d4-a716-446655440000");
        let runtime = Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(UUID.uuid, runtime.uuid);
    }

    #[test]
    fn parse_is_case_insensitive() {
        let lower = Uuid::try_parse("550e8400-e29b-41d4-a716-446655440aff").unwrap();
        let upper = Uuid::try_parse("550E8400-E29B-41D4-A716-446655440AFF").unwrap();
        let mixed = Uuid::try_parse("550e8400-E29B-41d4-A716-446655440aFf").unwrap();
        assert_eq!(lower.uuid, upper.uuid);
        assert_eq!(lower.uuid, mixed.uuid);
    }

    #[test]
    fn parse_display_round_trip() {
        for s in [
            "00000000-0000-0000-0000-000000000000",
            "ffffffff-ffff-ffff-ffff-ffffffffffff",
            "550e8400-e29b-41d4-a716-446655440000",
            "01234567-89ab-cdef-0123-456789abcdef",
        ] {
            assert_eq!(Uuid::try_parse(s).unwrap().to_string(), s);
        }
    }

    #[test]
    fn nil_and_max_values() {
        let nil = Uuid::try_parse("00000000-0000-0000-0000-000000000000").unwrap();
        assert_eq!(nil.uuid, [0u8; 16]);
        let max = Uuid::try_parse("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap();
        assert_eq!(max.uuid, [0xFFu8; 16]);
    }

    #[test]
    fn rejects_bytes_adjacent_to_hex_ranges() {
        for c in ['/', ':', '`', 'g', '@', 'G'] {
            let s = format!("{c}50e8400-e29b-41d4-a716-446655440000");
            assert!(Uuid::try_parse(&s).is_none(), "accepted {c:?}");
        }
    }

    #[test]
    fn accepts_exactly_the_hex_digits_in_every_pair_position() {
        for b in 0u8..=127 {
            let mut bytes = *b"00000000-0000-0000-0000-000000000000";
            bytes[0] = b;
            let s = std::str::from_utf8(&bytes).unwrap();
            assert_eq!(
                Uuid::try_parse(s).is_some(),
                b.is_ascii_hexdigit(),
                "byte {b:#04x}"
            );
        }
    }

    #[test]
    fn rejects_hyphen_anywhere_but_the_four_separators() {
        let canonical = *b"550e8400-e29b-41d4-a716-446655440000";
        for i in 0..36 {
            if canonical[i] == b'-' {
                continue;
            }
            let mut bytes = canonical;
            bytes[i] = b'-';
            let s = std::str::from_utf8(&bytes).unwrap();
            assert!(Uuid::try_parse(s).is_none(), "accepted '-' at index {i}");
        }
    }

    #[test]
    fn rejects_multibyte_utf8_of_matching_byte_length() {
        let s = "é50e8400-e29b-41d4-a716-44665544000";
        assert_eq!(s.len(), 36);
        assert!(Uuid::try_parse(s).is_none());
    }

    #[test]
    fn rejects_unicode_lookalikes_at_exact_byte_length() {
        let cases = [
            "あいうえおかきくけこさし",
            "あe8400-e29b-41d4-a716-446655440000",
            "０e8400-e29b-41d4-a716-446655440000",
            "🦀8400-e29b-41d4-a716-446655440000",
            "550e84\u{2010}e29b-41d4-a716-446655440000",
            "550e8400-e29b-41d4-a716-446655440\u{200b}",
            "e\u{301}e8400-e29b-41d4-a716-446655440000",
            "\u{202e}e8400-e29b-41d4-a716-446655440000",
            "\u{feff}e8400-e29b-41d4-a716-446655440000",
            "550e8400-e29b-41d4-a716-44665544000\0",
        ];
        for s in cases {
            assert_eq!(s.len(), 36, "case {s:?} must be exactly 36 bytes");
            assert!(Uuid::try_parse(s).is_none(), "accepted {s:?}");
        }
    }

    #[test]
    fn rejects_fullwidth_uuid_lookalike_with_36_chars_but_more_bytes() {
        let s = "５５０ｅ８４００－ｅ２９ｂ－４１ｄ４－ａ７１６－４４６６５５４４００００";
        assert_eq!(s.chars().count(), 36);
        assert!(s.len() > 36);
        assert!(Uuid::try_parse(s).is_none());
    }

    #[test]
    fn rejects_sign_characters_accepted_by_from_str_radix() {
        assert!(Uuid::try_parse("+50e8400-e29b-41d4-a716-446655440000").is_none());
        assert!(Uuid::try_parse("550e8400-e29b-41d4-a716-+46655440000").is_none());
    }

    #[test]
    fn rejects_wrong_lengths() {
        assert!(Uuid::try_parse("").is_none());
        assert!(Uuid::try_parse("550e8400-e29b-41d4-a716-44665544000").is_none());
        assert!(Uuid::try_parse("550e8400-e29b-41d4-a716-4466554400000").is_none());
    }

    #[test]
    #[should_panic(expected = "invalid UUID")]
    fn parse_panics_on_malformed_input() {
        let _ = Uuid::parse("not-a-uuid");
    }
}
