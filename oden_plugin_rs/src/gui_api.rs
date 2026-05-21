//! The Gui Api is a trait that has all the functions that are available in an gui context in Oden.

#![allow(missing_docs)]

use crate::math::{Uuid, Vec4};
use std::marker::PhantomData;

#[must_use = "Indentation is applied while the guard is in scope"]
pub struct IndentGuard<'a> {
    pub(crate) amount: f32,
    #[doc(hidden)]
    pub(crate) inner: *mut crate::plugin_h::OdenPluginEntityGuiParams,
    #[doc(hidden)]
    pub(crate) phantom: PhantomData<&'a i32>,
}

impl Default for IndentGuard<'_> {
    fn default() -> Self {
        Self {
            amount: Default::default(),
            inner: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }
}

impl Drop for IndentGuard<'_> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                if let Some(unindent) = (*self.inner).unindent {
                    unindent(self.amount)
                }
            }
        }
    }
}

#[must_use = "Tree node is applied while the guard is in scope"]
pub struct TreeNodeGuard<'a> {
    #[doc(hidden)]
    pub(crate) inner: *mut crate::plugin_h::OdenPluginEntityGuiParams,
    #[doc(hidden)]
    pub(crate) phantom: PhantomData<&'a i32>,
}

impl Default for TreeNodeGuard<'_> {
    fn default() -> Self {
        Self {
            inner: std::ptr::null_mut(),
            phantom: PhantomData,
        }
    }
}

impl Drop for TreeNodeGuard<'_> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                if let Some(tree_pop) = (*self.inner).treePop {
                    tree_pop()
                }
            }
        }
    }
}

#[allow(clippy::needless_lifetimes)]
#[allow(clippy::too_many_arguments)]
#[cfg_attr(feature = "mock", mockall::automock)]
pub trait GuiApi {
    fn label(&self, label: &str);
    fn button(&self, label: &str) -> bool;
    fn inactive_button(&self, label: &str);
    fn combo<'a>(&self, label: &str, current_item: &mut i32, items: &[&'a str]) -> bool;
    fn inactive_combo(&self, label: &str);
    fn slider_int(
        &self,
        label: &str,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut i32,
    ) -> bool;
    fn slider_float(
        &self,
        label: &str,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut f32,
    ) -> bool;
    fn slider_float3(
        &self,
        label: &str,
        min_value: f32,
        max_value: f32,
        value: &mut [f32; 3],
    ) -> bool;
    fn checkbox(&self, label: &str, value: &mut bool) -> bool;
    fn tree_node<'a>(&'a self, label: &str) -> Option<TreeNodeGuard<'a>>;
    fn drag_int(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut i32,
    ) -> bool;
    fn drag_int2(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 2],
    ) -> bool;
    fn drag_int3(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 3],
    ) -> bool;
    fn drag_int4(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 4],
    ) -> bool;
    fn drag_int5(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 5],
    ) -> bool;
    fn drag_float(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        power: f32,
        value: &mut f32,
    ) -> bool;
    fn drag_float2(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 2],
    ) -> bool;
    fn drag_float3(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 3],
    ) -> bool;
    fn drag_float4(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 4],
    ) -> bool;
    fn drag_float5(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 5],
    ) -> bool;
    #[cfg(feature = "glam_conversion")]
    fn drag_vec2(&self, label: &str, value: &mut glam::Vec2) -> bool;
    #[cfg(feature = "glam_conversion")]
    fn drag_vec3(&self, label: &str, value: &mut glam::Vec3) -> bool;
    #[cfg(feature = "glam_conversion")]
    fn drag_vec4(&self, label: &str, value: &mut glam::Vec4) -> bool;
    fn same_line(&self);
    fn input_text(&self, label: &str, string: &mut String) -> bool;
    fn label_colored(&self, label: &str, color: Vec4);
    fn tooltip_if_last_item_hovered(&self, text: &str);
    fn graph(&self, label: &str, data: &mut [f32], min_val: f32, max_val: f32);
    fn indent<'a>(&'a self, amount: f32) -> Option<IndentGuard<'a>>;
    fn checkbox_settings_backed(&self, label: &str, value: &str) -> bool;
    fn color_edit3(&self, label: &str, color: &mut [f32; 3]) -> bool;
    fn color_edit4(&self, label: &str, color: &mut [f32; 4]) -> bool;
    fn entity_picker<'a>(
        &self,
        label: &str,
        current_item: &mut Uuid,
        filter: Option<Vec<&'a str>>,
    ) -> bool;
    fn joystick_picker(&self, label: &str, joystick_name: &mut String) -> bool;
    fn joystick_picker_settings_backed(&self, label: &str, settings_key: &str) -> bool;
    #[cfg(feature = "strum_ex")]
    fn combo_enum<
        T: strum::IntoEnumIterator + strum::VariantNames + std::fmt::Display + PartialEq + 'static,
    >(
        &self,
        label: &str,
        current_value: &mut T,
    ) -> bool;
}
