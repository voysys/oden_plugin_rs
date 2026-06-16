use crate::gui_api::{IndentGuard, TreeNodeGuard};
#[cfg(feature = "serialize")]
use crate::impl_settings_ext_api;
use crate::{
    impl_scene_api, impl_scene_api_ext, impl_settings_api, math::Vec4, utils::utf8_from_raw,
};
use std::marker::PhantomData;
use std::{
    ffi::{c_void, CString},
    os::raw::{c_char, c_int},
};

/// # GuiParams
/// The [`OdenPlugin::gui`](crate::OdenPlugin::gui) function is called each frame if the sidebar GUI of the plugin entity is visible.
///
/// GuiParams is a struct that exposes the Oden Plugin API functions associated with the [`OdenPlugin::gui`](crate::OdenPlugin::gui) function.
/// The API is passed as a parameter to `gui(&mut self, api: &GuiParams)` and the API can be accessed as shown in the example below:
///
/// # Examples
/// ```ignore
/// fn gui(&mut self, api: &GuiParams) {
///     api.label("Hello World!");
/// }
/// ```
pub struct GuiParams<'a> {
    #[doc(hidden)]
    pub inner: *mut crate::plugin_h::OdenPluginEntityGuiParams,
    #[doc(hidden)]
    pub phantom: PhantomData<&'a i32>,
}

impl_settings_api!(GuiParams);
#[cfg(feature = "serialize")]
impl_settings_ext_api!(GuiParams);
impl_scene_api!(GuiParams);
impl_scene_api_ext!(GuiParams);

#[inherent::inherent]
impl crate::gui_api::GuiApi for GuiParams<'_> {
    /// Displays the text specified in `label`.
    pub fn label(&self, label: &str) {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(label_func) = (*self.inner).label {
                label_func(c"%s".as_ptr() as *const _, label.as_ptr());
            } else {
                panic!("This version of Oden is too old to have the label function");
            }
        }
    }

    /// Displays a single clickable button with a `label`. Returns `true` if clicked this frame.
    ///
    /// # Examples
    /// ```no_run
    /// # use oden_plugin_rs::log;
    /// # fn example(api: &impl oden_plugin_rs::GuiApi) {
    ///     if api.button("hello") {
    ///         log::info!("Clicked button");
    ///     }
    /// # }
    /// ```
    pub fn button(&self, label: &str) -> bool {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(button) = (*self.inner).button {
                button(label.as_ptr())
            } else {
                panic!("This version of Oden is too old to have the button function");
            }
        }
    }

    /// Displays a single non-clickable button with a `label`.
    ///
    /// Can be used to signal that something is not available right now,
    /// but may be available later.
    pub fn inactive_button(&self, label: &str) {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(inactive_button) = (*self.inner).inactiveButton {
                inactive_button(label.as_ptr());
            } else {
                panic!("This version of Oden is too old to have the inactive_button function");
            }
        }
    }

    /// A drop-down selection widget the allows the user to select from a number of choices in (`items`).
    ///
    /// Returns `true` if the user clicked one of the choices this frame. If so, `current_item` is set
    /// to the index of the selected choice in `items`.
    ///
    /// # Examples
    /// ```no_run
    /// fn gui(api: &impl oden_plugin_rs::GuiApi) {
    ///     let mut current_item = 0;
    ///     let items = ["Choice 1", "Choice 2", "Choice 3"];
    ///     if api.combo("Select an option", &mut current_item, &items) {
    ///         // Selection changed
    ///     }
    /// }
    /// ```
    pub fn combo(&self, label: &str, current_item: &mut i32, items: &[&str]) -> bool {
        let label = CString::new(label).unwrap();

        // Slow as we're doing multiple allocations
        let c_strings: Vec<CString> = items.iter().map(|s| CString::new(*s).unwrap()).collect();
        let mut c_items: Vec<*const c_char> = Vec::with_capacity(c_strings.len());

        extern "C" fn items_getter_callback(
            data: *mut c_void,
            idx: c_int,
            out_text: *mut *const c_char,
        ) -> bool {
            unsafe {
                let c_items: &Vec<*const c_char> = &*(data as *const Vec<*const c_char>);
                *out_text = c_items[idx as usize];
            }
            true
        }

        unsafe {
            c_strings.iter().for_each(|s| {
                c_items.push(s.as_ptr());
            });
            if let Some(combo) = (*self.inner).combo {
                combo(
                    label.as_ptr(),
                    current_item as *mut _,
                    Some(items_getter_callback),
                    &c_items as *const _ as *mut c_void,
                    items.len() as i32,
                )
            } else {
                panic!("This version of Oden is too old to have the combo function");
            }
        }
    }

    /// Displays an inactive combo box with a `label`.
    pub fn inactive_combo(&self, label: &str) {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(inactive_combo) = (*self.inner).inactiveCombo {
                inactive_combo(label.as_ptr());
            } else {
                panic!("This version of Oden is too old to have the inactive_combo function");
            }
        }
    }

    /// Convenience wrapper over [`combo`](crate::gui_api::GuiApi::combo) for strum-derived enums
    ///
    /// # Examples
    /// ```no_run
    /// use strum_macros::{Display, EnumIter, VariantNames};
    ///
    /// #[derive(Display, EnumIter, VariantNames, PartialEq)]
    /// enum MyEnum {
    ///     OptionA,
    ///     OptionB,
    ///     OptionC,
    /// }
    ///
    /// fn gui(api: &impl oden_plugin_rs::GuiApi) {
    ///     let mut current_value = MyEnum::OptionA;
    ///     if api.combo_enum("Select an option", &mut current_value) {
    ///         // Selection changed
    ///     }
    /// }
    /// ```
    #[cfg(feature = "strum_ex")]
    pub fn combo_enum<
        T: crate::strum::IntoEnumIterator
            + crate::strum::VariantNames
            + std::fmt::Display
            + PartialEq
            + 'static,
    >(
        &self,
        label: &str,
        current_value: &mut T,
    ) -> bool {
        let current_index = T::iter().position(|v| v == *current_value).unwrap_or(0);
        let mut current_index = current_index as i32;

        let options: Vec<&'static str> = T::VARIANTS.to_vec();

        if self.combo(label, &mut current_index, &options) {
            if let Some(new_value) = T::iter().nth(current_index as usize) {
                *current_value = new_value;
                return true;
            }
        }
        false
    }

    /// A slider with a `label` for a single integer `value` that is modifiable in the range [`min_value`, `max_value`].
    ///
    /// `display_format` specifies how to display `value` using standard C `printf` syntax.
    ///
    /// Returns `true` if `value` was changed this frame.
    pub fn slider_int(
        &self,
        label: &str,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut i32,
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(slider_int) = (*self.inner).sliderInt {
                slider_int(
                    label.as_ptr(),
                    value as *mut _,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the slider_int function");
            }
        }
    }

    /// A slider with a `label` for a single floating point `value` between the known boundary [`min_value`, `max_value`].
    ///
    /// `display_format` specifies how to display `value` using standard C `printf` syntax.
    ///
    /// Returns `true` if `value` was changed this frame.
    pub fn slider_float(
        &self,
        label: &str,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut f32,
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(slider_float) = (*self.inner).sliderFloat {
                slider_float(
                    label.as_ptr(),
                    value as *mut _,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the slider_float function");
            }
        }
    }

    /// The same as [`slider_float`](crate::gui_api::GuiApi::slider_float) but for three floats.
    pub fn slider_float3(
        &self,
        label: &str,
        min_value: f32,
        max_value: f32,
        value: &mut [f32; 3],
    ) -> bool {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(slider_float3) = (*self.inner).sliderFloat3 {
                slider_float3(label.as_ptr(), value.as_mut_ptr(), min_value, max_value)
            } else {
                panic!("This version of Oden is too old to have the slider_float3 function");
            }
        }
    }

    /// Return: true if value was changed
    /// Checkboxes are a good way to visualize boolean choices
    pub fn checkbox(&self, label: &str, value: &mut bool) -> bool {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(checkbox) = (*self.inner).checkbox {
                checkbox(label.as_ptr(), value as *mut _)
            } else {
                panic!("This version of Oden is too old to have the checkbox function");
            }
        }
    }

    /// Displays a tree node.
    ///
    /// Opens or closes the tree node when clicked.
    /// The GUI system in Oden automatically keeps track of the tree node state.
    ///
    /// Returns a `TreeNodeGuard` that automatically pops the tree node when it goes out of scope.
    ///
    /// There are two ways of disambiguating labels, aside from keeping the labels unique:
    /// 1. Adding `##` to the label and then more text will make the label including
    ///     what comes after `##` as the unique label.
    /// 1. Adding `###` to the label and then more text will
    ///    only take into account the text following `###` as the identifier.
    ///    (This means it is possible to have a varying label and still have a unique identifier)
    ///
    /// # Examples
    /// ```no_run
    /// # fn example(api: &impl oden_plugin_rs::GuiApi) {
    ///     if let Some(_guard) = api.tree_node("Tree Node Test") {
    ///         // Things drawn here will be in the tree node
    ///     }
    /// # }
    /// ```
    pub fn tree_node<'a>(&'a self, label: &str) -> Option<TreeNodeGuard<'a>> {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(tree_node) = (*self.inner).treeNode {
                if tree_node(label.as_ptr()) {
                    Some(TreeNodeGuard {
                        inner: self.inner,
                        phantom: PhantomData,
                    })
                } else {
                    None
                }
            } else {
                panic!("This version of Oden is too old to have the tree_node function");
            }
        }
    }

    /// A GUI element for dragging a single integer `value` between the boundary [`min_value`, `max_value`].
    ///
    /// `label` specifies the text label.
    /// `speed` specifies how much `value` should change when the user drags it.
    /// `display_format` specifies how to display `value` using standard C `printf` syntax.
    ///
    /// Returns `true` if `value` was changed this frame.
    ///
    /// Dragging variables is useful when the bounds of the variable are not known.
    /// Dragging can also provide additional precision over sliders.
    pub fn drag_int(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut i32,
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_int) = (*self.inner).dragInt {
                drag_int(
                    label.as_ptr(),
                    value as *mut _,
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_int function");
            }
        }
    }

    /// The same as [`drag_int`](crate::gui_api::GuiApi::drag_int) but for two integers.
    pub fn drag_int2(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 2],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_int2) = (*self.inner).dragInt2 {
                drag_int2(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_int2 function");
            }
        }
    }

    /// The same as [`drag_int`](crate::gui_api::GuiApi::drag_int) but for three integers.
    pub fn drag_int3(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 3],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_int3) = (*self.inner).dragInt3 {
                drag_int3(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_int3 function");
            }
        }
    }

    /// The same as [`drag_int`](crate::gui_api::GuiApi::drag_int) but for four integers.
    pub fn drag_int4(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 4],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_int4) = (*self.inner).dragInt4 {
                drag_int4(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_int4 function");
            }
        }
    }

    /// The same as [`drag_int`](crate::gui_api::GuiApi::drag_int) but for five integers.
    pub fn drag_int5(
        &self,
        label: &str,
        speed: f32,
        min_value: i32,
        max_value: i32,
        display_format: &str,
        value: &mut [i32; 5],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_int5) = (*self.inner).dragInt5 {
                drag_int5(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_int5 function");
            }
        }
    }

    /// A GUI element for dragging a single float `value` between the boundary [`min_value`, `max_value`].
    ///
    /// `label` specifies the text label.
    /// `speed` specifies how much `value` should change when the user drags it.
    /// `display_format` specifies how to display `value` using standard C `printf` syntax.
    /// `power`: In an earlier version of ImGui this used to be a float variable, but this has been replaced
    /// with a flag for setting the behavior to "logarithmic".
    /// To remain backwards compatibility Oden will compare the given power value with 1.0 and set
    /// the logarithmic flag if it is not 1.0.
    ///
    /// Returns `true` if `value` was changed this frame.
    ///
    /// Dragging variables is useful when the bounds of the variable are not known.
    /// Dragging can also provide additional precision over sliders.
    #[allow(clippy::too_many_arguments)]
    pub fn drag_float(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        power: f32,
        value: &mut f32,
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_float) = (*self.inner).dragFloat {
                drag_float(
                    label.as_ptr(),
                    value as *mut _,
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                    power,
                )
            } else {
                panic!("This version of Oden is too old to have the drag_float function");
            }
        }
    }

    /// The same as [`drag_float`](crate::gui_api::GuiApi::drag_float) but for two floats.
    pub fn drag_float2(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 2],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_float2) = (*self.inner).dragFloat2 {
                drag_float2(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_float2 function");
            }
        }
    }

    /// The same as [`drag_float`](crate::gui_api::GuiApi::drag_float) but for three floats.
    pub fn drag_float3(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 3],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_float3) = (*self.inner).dragFloat3 {
                drag_float3(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_float3 function");
            }
        }
    }

    /// The same as [`drag_float`](crate::gui_api::GuiApi::drag_float) but for four floats.
    pub fn drag_float4(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 4],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_float4) = (*self.inner).dragFloat4 {
                drag_float4(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_float4 function");
            }
        }
    }

    /// The same as [`drag_float`](crate::gui_api::GuiApi::drag_float) but for five floats.
    pub fn drag_float5(
        &self,
        label: &str,
        speed: f32,
        min_value: f32,
        max_value: f32,
        display_format: &str,
        value: &mut [f32; 5],
    ) -> bool {
        let label = CString::new(label).unwrap();
        let display_format = CString::new(display_format).unwrap();
        unsafe {
            if let Some(drag_float5) = (*self.inner).dragFloat5 {
                drag_float5(
                    label.as_ptr(),
                    value.as_mut_ptr(),
                    speed,
                    min_value,
                    max_value,
                    display_format.as_ptr(),
                )
            } else {
                panic!("This version of Oden is too old to have the drag_float5 function");
            }
        }
    }

    /// Color Picker for a color without alpha value
    pub fn color_edit3(&self, label: &str, color: &mut [f32; 3]) -> bool {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(color_edit3) = (*self.inner).colorEdit3 {
                color_edit3(label.as_ptr(), color.as_mut_ptr())
            } else {
                panic!("This version of Oden is too old to have the color_edit3 function");
            }
        }
    }

    /// Color Picker for a color with alpha value
    pub fn color_edit4(&self, label: &str, color: &mut [f32; 4]) -> bool {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(color_edit4) = (*self.inner).colorEdit4 {
                color_edit4(label.as_ptr(), color.as_mut_ptr())
            } else {
                panic!("This version of Oden is too old to have the color_edit4 function");
            }
        }
    }

    /// Convenience wrapper over [`drag_float2`](crate::gui_api::GuiApi::drag_float2) for glam::Vec2
    #[cfg(feature = "glam_conversion")]
    pub fn drag_vec2(&self, label: &str, value: &mut glam::Vec2) -> bool {
        self.drag_float2(label, 0.01, 0.0, 0.0, "%.02f", value.as_mut())
    }

    /// Convenience wrapper over [`drag_float3`](crate::gui_api::GuiApi::drag_float3) for glam::Vec3
    #[cfg(feature = "glam_conversion")]
    pub fn drag_vec3(&self, label: &str, value: &mut glam::Vec3) -> bool {
        self.drag_float3(label, 0.01, 0.0, 0.0, "%.02f", value.as_mut())
    }

    /// Convenience wrapper over [`drag_float4`](crate::gui_api::GuiApi::drag_float4) for glam::Vec4
    #[cfg(feature = "glam_conversion")]
    pub fn drag_vec4(&self, label: &str, value: &mut glam::Vec4) -> bool {
        self.drag_float4(label, 0.01, 0.0, 0.0, "%.02f", value.as_mut())
    }

    /// Places the next GUI element on the same line as the last.
    pub fn same_line(&self) {
        unsafe {
            if let Some(same_line) = (*self.inner).sameLine {
                same_line();
            } else {
                panic!("This version of Oden is too old to have the same_line function");
            }
        }
    }

    /// Allows for editing `text` in the sidebar GUI.
    ///
    /// Returns `true` if text was edited.
    pub fn input_text(&self, label: &str, text: &mut String) -> bool {
        let label = CString::new(label).unwrap();

        let mut buffer = vec![0; text.len() + 4096];
        buffer[..text.len()].copy_from_slice(text.as_bytes());

        let res;

        unsafe {
            if let Some(input_text) = (*self.inner).inputText {
                res = input_text(
                    label.as_ptr(),
                    buffer.as_mut_ptr() as *mut _,
                    buffer.len() as u64,
                );
            } else {
                panic!("This version of Oden is too old to have the input_text function");
            }

            if let Ok(s) = utf8_from_raw(&buffer) {
                *text = s;
            }
        }

        res
    }

    /// A text `label` with custom RGBA `color`.
    pub fn label_colored(&self, label: &str, color: Vec4) {
        let label = CString::new(label).unwrap();
        unsafe {
            if let Some(label_colored) = (*self.inner).labelColored {
                label_colored(color, c"%s".as_ptr() as *const _, label.as_ptr());
            } else {
                panic!("This version of Oden is too old to have the label_colored function");
            }
        }
    }

    /// Adds a `text` tooltip over the last GUI element, if it is hovered.
    pub fn tooltip_if_last_item_hovered(&self, text: &str) {
        unsafe {
            if let Some(tooltip_if_last_item_hovered) = (*self.inner).tooltipIfLastItemHovered {
                let text = CString::new(text).unwrap();
                tooltip_if_last_item_hovered(text.as_ptr())
            } else {
                panic!("This version of Oden is too old to have the tooltip_if_last_item_hovered function");
            }
        }
    }

    /// Shows a graph of the given float `data` within the boundary `[min_val, max_val]`.
    pub fn graph(&self, label: &str, data: &mut [f32], min_val: f32, max_val: f32) {
        unsafe {
            if let Some(graph) = (*self.inner).graph {
                let label = CString::new(label).unwrap();
                let data_size = data.len();
                let data = data.as_mut_ptr();

                graph(label.as_ptr(), data, data_size as i32, min_val, max_val);
            } else {
                panic!("This version of Oden is too old to have the graph function");
            }
        }
    }

    /// Indents the GUI by `amount` pixels.
    ///
    /// Indents by the default value if `0.0` is given.
    ///
    /// Returns an `IdentGuard` that automatically deindents when it goes out of scope.
    #[allow(clippy::needless_lifetimes)]
    pub fn indent<'a>(&'a self, amount: f32) -> Option<IndentGuard<'a>> {
        unsafe {
            if let Some(indent) = (*self.inner).indent {
                indent(amount);
                Some(IndentGuard {
                    amount,
                    inner: self.inner,
                    phantom: PhantomData,
                })
            } else {
                panic!("This version of Oden is too old to have the indent function");
            }
        }
    }

    /// A checkbox that is backed with an int in the settings
    pub fn checkbox_settings_backed(&self, label: &str, settings_key: &str) -> bool {
        let mut tmp_val = 0;
        if let Some(v) = self.read_int(settings_key) {
            tmp_val = v;
        }
        let mut tmp_val = tmp_val == 1;
        self.checkbox(label, &mut tmp_val);
        self.write_int(settings_key, i32::from(tmp_val));
        tmp_val
    }

    /// Combo box with all entities and optionally filter on Entity Type
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::GuiParams) {
    /// let mut selected_entity = oden_plugin_rs::math::Uuid::default();
    /// if api.entity_picker("Entity Picker", &mut selected_entity, Some(vec!["2D Video", "Stiched Video"])) {
    ///     // Do something with selected_entity
    /// }
    /// # }
    /// ```
    #[allow(clippy::needless_lifetimes)]
    pub fn entity_picker<'a>(
        &self,
        label: &str,
        current_item: &mut crate::math::Uuid,
        entity_type_filter: Option<Vec<&'a str>>,
    ) -> bool {
        let mut filtered_entities = Vec::new();

        if let Some(entities) = self.all_entity_uuids() {
            for entity in &entities {
                if let Some(type_name) = self.entity_type_name(&entity.to_string()) {
                    let filter = entity_type_filter.as_ref();
                    if filter.is_none() || filter.unwrap().contains(&type_name.as_str()) {
                        let name = self
                            .entity_name(&entity.to_string())
                            .map(|id| format!(": {id}"))
                            .unwrap_or_default();
                        filtered_entities.push((*entity, type_name + &name));
                    }
                }
            }
        }

        let mut selected_entity = filtered_entities
            .iter()
            .position(|e| e.0 == *current_item)
            .map_or(-1, |v| v as i32);

        let res = self.combo(
            label,
            &mut selected_entity,
            &filtered_entities
                .iter()
                .map(|e| e.1.as_str())
                .collect::<Vec<&str>>(),
        );

        *current_item = if !selected_entity.is_negative() {
            filtered_entities[selected_entity as usize].0
        } else {
            crate::math::Uuid::default()
        };

        res
    }

    /// Allows selecting a joystick from a list of connected joysticks.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the combo box.
    /// * `selected_joystick_name` - The currently selected joystick name.
    ///
    /// # Returns
    ///
    /// * `true` if the selection changed, `false` otherwise.
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::GuiParams) {
    /// let mut selected_joystick = String::new();
    /// if api.joystick_picker("Select Joystick", &mut selected_joystick) {
    ///     // do something with the selected joystick name, e.g. save to settings
    /// }
    /// # }
    /// ```
    pub fn joystick_picker(&self, label: &str, selected_joystick_name: &mut String) -> bool {
        let mut joystick_names = Vec::new();
        let mut selection = -1;

        for i in 0..16 {
            if let Some(joystick) = self.joystick_state(i) {
                if joystick.name == *selected_joystick_name {
                    selection = i;
                }
                joystick_names.push(joystick.name);
            } else {
                break;
            }
        }

        let res = self.combo(
            label,
            &mut selection,
            &joystick_names
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<&str>>(),
        );

        if res && !selection.is_negative() {
            *selected_joystick_name = joystick_names.remove(selection as usize);
        }

        res
    }

    /// Allows selecting a joystick from a list of connected joysticks,
    /// with the selection backed by settings.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the combo box.
    /// * `settings_key` - The settings key where the selected joystick name is stored.
    ///
    /// # Returns
    ///
    /// * `true` if the selection changed, `false` otherwise.
    ///
    /// Example
    /// ```no_run
    /// # fn example(api: &oden_plugin_rs::GuiParams) {
    /// api.joystick_picker_settings_backed("Select Joystick", "joystick_name");
    /// # }
    /// ```
    pub fn joystick_picker_settings_backed(&self, label: &str, settings_key: &str) -> bool {
        let mut selected_joystick_name = self.read_string(settings_key).unwrap_or_default();

        let res = self.joystick_picker(label, &mut selected_joystick_name);

        if res {
            self.write_string(settings_key, &selected_joystick_name);
        }

        res
    }
}
