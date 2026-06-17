//! The Init and Update Api is a trait that has all the functions that are available in an both the init and the update context.

#![allow(missing_docs)]

// Descriptor of a font with raw ttf data and the ranges of characters to render.
// No need for explicit null terminations.
pub struct FontDescriptor<'a> {
    pub ttf_data: &'a [u8],
    pub range: &'a [(i32, i32)],
}

#[allow(clippy::needless_lifetimes)]
#[cfg_attr(feature = "mock", mockall::automock)]
pub trait InitAndUpdateApi {
    fn upload_image(&self, id: i32, image_width: i32, image_height: i32, image: &[u8]);
    fn upload_image_ex(
        &self,
        id: i32,
        image_width: i32,
        image_height: i32,
        image: &[u8],
        params: &crate::UploadImageParams,
    );
    fn upload_font<'a>(&self, id: i32, glyph_size: f32, font_descriptors: &[FontDescriptor<'a>]);
    fn upload_shader(&self, id: i32, code: &str) -> bool;
    fn register_gl_tex_id(&self, id: i32, tex_id: u32);
    fn register_iosurface_id(&self, id: i32, iosurface_id: u32);
    fn register_image_callback<F>(&self, stream_id: &str, callback: F)
    where
        F: FnMut(crate::VideoFrame) + Send + 'static;
    fn set_window_title(&self, title: &str);
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_init_and_update_api {
    ($interface:ident) => {
        // For proper docs links because we can't use $interface directly in doc comments
        #[allow(unused_imports)]
        use $interface as init_and_update_api_impl;

        #[inherent::inherent]
        impl $crate::init_and_update_api::InitAndUpdateApi for $interface<'_> {
            /// Uploads an `image` resource with dimensions `image_width` x `image_height`.
            ///
            /// The image MUST be in RGBA color format.
            /// The supplied `id` MUST be unique for every image.
            ///
            /// This only needs to be done once for every image. The image can then be used
            /// efficiently every frame by referring to the `id`.
            pub fn upload_image(&self, id: i32, image_width: i32, image_height: i32, image: &[u8]) {
                unsafe {
                    let image_desc = $crate::ImDrawImage {
                        width: image_width,
                        height: image_height,
                        data: image.as_ptr(),
                        size: image.len() as i32,
                    };
                    if let Some(upload_image) = (*self.inner).uploadImage {
                        upload_image(id, image_desc);
                    } else {
                        panic!("This version of Oden is too old to have the upload_image function");
                    }
                }
            }

            /// Uploads an `image` resource with dimensions `image_width` x `image_height`.
            ///
            /// The supplied `id` MUST be unique for every image.
            /// Using an existing `id` will overwrite the previously uploaded image.
            ///
            /// This only needs to be done once for every image. The image can then be used
            /// efficiently every frame by referring to the `id`.
            ///
            /// Use `params` to specify the texture format
            pub fn upload_image_ex(
                &self,
                id: i32,
                image_width: i32,
                image_height: i32,
                image: &[u8],
                params: &$crate::UploadImageParams,
            ) {
                unsafe {
                    let image_desc = $crate::ImDrawImage {
                        width: image_width,
                        height: image_height,
                        data: image.as_ptr(),
                        size: image.len() as i32,
                    };

                    if let Some(upload_image_ex) = (*self.inner).uploadImageEx {
                        upload_image_ex(id, image_desc, params as *const _);
                    } else {
                        panic!("This version of Oden is too old to have the upload_image_ex function");
                    }
                }
            }

            /// Works similarly to [`upload_image`](init_and_update_api_impl::upload_image) but for TTF font files.
            /// Contact Voysys if support is needed for font functionality.
            pub fn upload_font(
                &self,
                id: i32,
                glyph_size: f32,
                font_descriptors: &[$crate::init_and_update_api::FontDescriptor],
            ) {
                let mut ranges: Vec<Vec<$crate::FontRangeEntry>> = Vec::new();
                for d in font_descriptors {
                    let mut range: Vec<$crate::FontRangeEntry> = Vec::new();
                    for (start, end) in d.range {
                        range.push($crate::FontRangeEntry {
                            start: *start,
                            end: *end,
                        });
                    }
                    // null terminate each range array
                    range.push($crate::FontRangeEntry { start: 0, end: 0 });
                    ranges.push(range);
                }

                let mut ttf_data_c: Vec<*const u8> = Vec::new();
                let mut ttf_sizes_c: Vec<i32> = Vec::new();
                let mut ranges_c: Vec<*const $crate::FontRangeEntry> = Vec::new();
                for d in font_descriptors {
                    ttf_data_c.push(d.ttf_data.as_ptr());
                    ttf_sizes_c.push(d.ttf_data.len() as i32);
                }
                for range in &ranges {
                    ranges_c.push((*range).as_ptr() as *const _);
                }
                // null terminate all arrays
                ttf_data_c.push(std::ptr::null() as *const _);
                ttf_sizes_c.push(0);
                ranges_c.push(std::ptr::null() as *const _);
                unsafe {
                    if let Some(upload_font) = (*self.inner).uploadFont {
                        upload_font(
                            id,
                            glyph_size,
                            ttf_data_c.as_ptr(),
                            ttf_sizes_c.as_ptr(),
                            ranges_c.as_ptr(),
                        );
                    } else {
                        panic!("This version of Oden is too old to have the upload_font function");
                    }
                }
            }

            /// Uploads a `shader` for use with im draw
            ///
            /// The supplied code must be glsl and declare a function with the following prototype:
            /// vec4 frag(Params p) { ... }
            ///
            /// where Params is
            ///
            /// struct Params {
            ///     vec4 tint;
            ///     vec4 vertex_color;
            ///     vec4 texture_color;
            ///     vec4 back_color;
            ///     vec3 pos;
            ///     vec2 uv;
            ///     float time;
            /// };
            pub fn upload_shader(&self, id: i32, code: &str) -> bool {
                let code = std::ffi::CString::new(code.trim_end_matches('\0')).unwrap();

                unsafe {
                    if let Some(upload_shader) = (*self.inner).uploadShader {
                        upload_shader(id, code.as_ptr())
                    } else {
                        panic!("This version of Oden is too old to have the upload_shader function");
                    }
                }
            }

            /// Registers an OpenGL `texture_id` to an Oden Plugin `image_id`.
            ///
            /// The `image_id` needs to be maintained by the plugin and MUST be unique for
            /// each image.
            pub fn register_gl_tex_id(&self, image_id: i32, texture_id: u32) {
                unsafe {
                    if let Some(register_gl_tex_id) = (*self.inner).registerGlTexId {
                        register_gl_tex_id(image_id, texture_id)
                    } else {
                        panic!("This version of Oden is too old to have the register_gl_tex_id function");
                    }
                }
            }

            /// Registers a macOS IOSurface ID to an Oden Plugin `image_id`.
            pub fn register_iosurface_id(&self, image_id: i32, iosurface_id: u32) {
                unsafe {
                    if let Some(register_iosurface_id) = (*self.inner).registerIOSurfaceId {
                        register_iosurface_id(image_id, iosurface_id)
                    } else {
                        panic!("This version of Oden is too old to have the register_iosurface_id function");
                    }
                }
            }

            /// Registers a callback for when Oden receives a non coded image from a camera
            ///
            /// `stream_id` is a name of video source and the video stream id "name_of_the_entity:index_of_the_stream"
            ///
            /// # Examples
            /// ```no_run
            /// # use oden_plugin_rs::log::info;
            /// # fn example(api: &oden_plugin_rs::UpdateParams) {
            /// api.register_image_callback("test:1", |frame| {
            ///     info!("{} {}", frame.size[0], frame.format);
            /// });
            /// # }
            /// ```
            pub fn register_image_callback<F>(&self, stream_id: &str, callback: F)
            where
                F: FnMut($crate::VideoFrame) + Send + 'static,
            {
                if let Some(register_image_callback) =
                    unsafe { (*self.inner).registerImageCallback }
                {
                    let stream_id =
                        std::ffi::CString::new(stream_id.trim_end_matches('\0')).unwrap();
                    let callback_user_data: *mut Box<dyn FnMut($crate::VideoFrame)> =
                        Box::into_raw(Box::new(Box::new(callback)));

                    unsafe {
                        register_image_callback(
                            stream_id.as_ptr(),
                            Some($crate::init_and_update_api::image_callback),
                            callback_user_data as *mut _,
                        )
                    }
                } else {
                    panic!("This version of Oden is too old to have the set_has_crashed function");
                }
            }

            /// Sets the window title (overriding the default "Application Name - loaded_project.vproj"
            ///
            /// The supplied `title` will override the default window title.
            /// Pass an empty string to clear the override and restore the default title.
            pub fn set_window_title(&self, title: &str) {
                let title_cstring = std::ffi::CString::new(title.trim_end_matches('\0')).unwrap();

                unsafe {
                    if let Some(set_window_title) = (*self.inner).setWindowTitle {
                        set_window_title(title_cstring.as_ptr());
                    } else {
                        panic!("This version of Oden is too old to have the set_window_title function");
                    }
                }
            }
        }
    };
}

pub(crate) unsafe extern "C" fn image_callback(
    params: *mut crate::plugin_h::OdenPluginImageCallbackParams,
) {
    let wrapper = (*params).callbackUserData as *mut Box<dyn FnMut(crate::VideoFrame)>;
    (*wrapper)((*params).frame);
}
