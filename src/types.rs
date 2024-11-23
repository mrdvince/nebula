// Common types and constants
use std::os::raw::{c_char, c_int};

pub type Rgba = u32;

// Plugin constants
pub const PLUGIN_NAME: &[u8] = b"Nebula Sobel\0";
pub const PLUGIN_AUTHOR: &[u8] = b"Vince\0";
pub const PLUGIN_DESCRIPTION: &[u8] = b"Edge detection using Sobel operator\0";

// Plugin type: F0R_PLUGIN_TYPE_FILTER = 0
pub const F0R_PLUGIN_TYPE_FILTER: c_int = 0;
// Color model: F0R_COLOR_MODEL_RGBA8888 = 1
pub const F0R_COLOR_MODEL_RGBA8888: c_int = 1;

// Frei0r plugin info structure
#[repr(C)]
pub struct F0rPluginInfo {
    pub name: *const c_char,
    pub author: *const c_char,
    pub plugin_type: c_int,
    pub color_model: c_int,
    pub frei0r_version: c_int,
    pub major_version: c_int,
    pub minor_version: c_int,
    pub num_params: c_int,
    pub explanation: *const c_char,
}

#[repr(C)]
pub struct F0rParamInfo {
    pub name: *const c_char,
    pub param_type: c_int,
    pub explanation: *const c_char,
}