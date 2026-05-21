//! The Draw Api is a trait that has all the functions that are available in an draw context in Oden.

#![allow(missing_docs)]

use crate::math::Matrix4;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait DrawApi {
    fn world_matrix(&self) -> Matrix4;
    fn proj_matrix(&self) -> Matrix4;
    fn viewport(&self) -> (i32, i32, i32, i32);
    fn target_size(&self) -> (i32, i32);
}
