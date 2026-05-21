//! The Graphics Api is a trait that has all the functions that are used to work with Oden's OpenGL context.

#![allow(missing_docs)]

use crate::GlLoader;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait InitAndGraphicsApi {
    fn gl_loader(&self) -> GlLoader;
}

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait GraphicsApi {
    fn camera_gl_tex_id(&self, entity: &str, stream: i32) -> Option<u32>;
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_init_and_graphics_api {
    ($interface:ident) => {
        #[inherent::inherent]
        impl $crate::graphics_api::InitAndGraphicsApi for $interface<'_> {
            /// Returns the OpenGL function loader that Oden uses.
            pub fn gl_loader(&self) -> $crate::GlLoader {
                unsafe {
                    if let Some(get_gl_loader) = (*self.inner).getGlLoader {
                        get_gl_loader()
                    } else {
                        panic!("This version of Oden is too old to have the gl_loader function");
                    }
                }
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! impl_graphics_api {
    ($interface:ident) => {
        #[inherent::inherent]
        impl $crate::graphics_api::GraphicsApi for $interface<'_> {
            /// Returns the OpenGL texture ID for video `stream` of `entity`.
            ///
            /// The OpenGL texture handle is only guaranteed to be valid this frame.
            /// It is strongly encouraged to request a new handle every frame.
            pub fn camera_gl_tex_id(&self, entity: &str, stream: i32) -> Option<u32> {
                unsafe {
                    if let Some(get_camera_gl_tex_id) = (*self.inner).getCameraGlTexId {
                        let entity = std::ffi::CString::new(entity.trim_end_matches('\0')).unwrap();
                        let mut tex_id_out = 0;
                        if get_camera_gl_tex_id(
                            entity.as_ptr(),
                            stream,
                            &mut tex_id_out as *mut u32,
                        ) {
                            Some(tex_id_out)
                        } else {
                            None
                        }
                    } else {
                        panic!(
                            "This version of Oden is too old to have the camera_gl_tex_id function"
                        );
                    }
                }
            }
        }
    };
}
