//! Global functions that are available on all threads

use crate::plugin_h;
use crate::ODEN_GLOBAL;

/// This function does a deep-copy of the data (copy all the bytes)
/// Before we can call it we need to provide data for the storage of the data
pub fn copy_frame(src: plugin_h::OdenPluginVideoFrame_s, dst: plugin_h::OdenPluginVideoFrame_s) {
    if unsafe { ODEN_GLOBAL.is_null() } {
        return;
    }
    let globals: &plugin_h::OdenPluginGlobalFunctions = unsafe { &*ODEN_GLOBAL };

    if let Some(copy_frame) = globals.copyFrame {
        unsafe { copy_frame(src, dst) };
    }
}
