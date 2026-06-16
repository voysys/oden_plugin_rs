//! The Im Draw Api is a trait that has all the functions that are used for drawing simple shapes in Oden.

#![allow(missing_docs)]

use crate::{
    math::{Matrix4, Vec2, Vec3, Vec4},
    ImDrawTextAlignment,
};

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait ImDrawApi {
    fn im_draw_set_use_z_buffer(&self, enable: bool);
    fn im_draw_set_write_z_buffer(&self, enable: bool);
    fn im_draw_push_matrix(&self, mat: Matrix4);
    fn im_draw_pop_matrix(&self);
    fn im_draw_push_view_space(&self);
    fn im_draw_pop_view_space(&self);
    fn im_draw_set_view_space_z(&self, z: i32);
    fn im_draw_set_tint(&self, tint: Vec4);
    fn im_draw_push_shader(&self, shader_id: i32);
    fn im_draw_pop_shader(&self);
    fn im_draw_latch_backbuffer(&self, blur_passes: i32);
    fn im_draw_add_line(&self, from: Vec3, to: Vec3, normal: Vec3, width: f32, color: Vec4);
    #[allow(clippy::too_many_arguments)]
    fn im_draw_add_circle_segment(
        &self,
        pos: Vec3,
        radius: f32,
        from_angle_rad: f32,
        to_angle_rad: f32,
        width: f32,
        normal: Vec3,
        color: Vec4,
        circle_segments: i32,
    );
    fn im_draw_add_circle(
        &self,
        pos: Vec3,
        radius: f32,
        width: f32,
        normal: Vec3,
        color: Vec4,
        circle_segments: i32,
    );
    fn im_draw_add_square(&self, pos: Vec3, size: Vec2, width: f32, color: Vec4);
    fn im_draw_add_rectangle(&self, pos: Vec3, size: Vec2, color: Vec4);
    fn im_draw_add_rectangle_rounded(&self, pos: Vec3, size: Vec2, rounding: f32, color: Vec4);
    fn im_draw_add_image(&self, pos: Vec3, size: Vec2, image_id: i32);
    fn im_draw_add_video(&self, pos: Vec3, size: Vec2, entity: &str, stream_id: i32);
    fn im_draw_add_video_with_uvs(
        &self,
        pos: Vec3,
        size: Vec2,
        entity: &str,
        stream_id: i32,
        op_left_uv: Vec2,
        bottom_right_uv: Vec2,
    );
    #[allow(clippy::too_many_arguments)]
    fn im_draw_add_text(
        &self,
        text: &str,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        color: Vec4,
        font_id: i32,
    );
    #[allow(clippy::too_many_arguments)]
    fn im_draw_add_text_normal(
        &self,
        text: &str,
        pos: Vec3,
        normal: Vec3,
        size: f32,
        color: Vec4,
        font_id: i32,
    );
    #[allow(clippy::too_many_arguments)]
    fn im_draw_add_text_aligned(
        &self,
        text: &str,
        pos: Vec3,
        bounding_box: Vec2,
        text_height: f32,
        up: Vec3,
        right: Vec3,
        alignment: ImDrawTextAlignment,
        color: Vec4,
        font_id: i32,
    );
    #[allow(clippy::too_many_arguments)]
    fn im_draw_add_text_aligned_normal(
        &self,
        text: &str,
        pos: Vec3,
        bounding_box: Vec2,
        text_height: f32,
        normal: Vec3,
        alignment: ImDrawTextAlignment,
        color: Vec4,
        font_id: i32,
    );
    fn im_draw_add_torus(
        &self,
        pos: Vec3,
        normal: Vec3,
        circle_radius: f32,
        tube_radius: f32,
        color: Vec4,
    );
    fn im_draw_add_lines(&self, point: &[Vec3], normal: Vec3, width: f32, color: Vec4);
    fn im_draw_add_rectangle_border(&self, pos: Vec3, size: Vec2, thickness: f32, color: Vec4);
    fn im_draw_add_rectangle_border_rounded(
        &self,
        pos: Vec3,
        size: Vec2,
        thickness: f32,
        inner_corner_radius: f32,
        inner_corner_segments: i32,
        color: Vec4,
    );
    fn im_draw_add_cuboid(&self, pos: Vec3, size: Vec3, color: Vec4);
    fn im_draw_add_triangles(&self, vertex_positions: &mut [Vec3], vertex_colors: &mut [Vec4]);
    fn im_draw_add_rectangle_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        color: Vec4,
    );
    fn im_draw_add_rectangle_rounded_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        rounding: f32,
        color: Vec4,
    );
    fn im_draw_add_rectangle_border_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        thickness: f32,
        color: Vec4,
    );
    #[allow(clippy::too_many_arguments)]
    fn im_draw_add_rectangle_border_rounded_aligned(
        &self,
        pos: Vec3,
        up: Vec3,
        right: Vec3,
        size: Vec2,
        thickness: f32,
        inner_corner_radius: f32,
        inner_corner_segments: i32,
        color: Vec4,
    );
    fn im_draw_add_triangles_with_uvs(
        &self,
        vertex_positions: &[Vec3],
        vertex_uvs: &[Vec2],
        vertex_colors: &[Vec4],
    );
    fn im_draw_calc_text_size(
        &self,
        text: &str,
        text_height: f32,
        max_width: f32,
        font_id: i32,
    ) -> Vec2;
}
