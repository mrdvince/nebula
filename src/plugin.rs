#![allow(renamed_and_removed_lints)]
#![allow(missing_safety_doc)]

use std::os::raw::{ c_char, c_int, c_void };

use crate::sobel::{ apply_sobel, gray_to_rgba, rgba_to_gray };
use crate::types::{ F0rParamInfo, F0rPluginInfo, Rgba };
use crate::types::{
    F0R_COLOR_MODEL_RGBA8888,
    F0R_PLUGIN_TYPE_FILTER,
    PLUGIN_AUTHOR,
    PLUGIN_DESCRIPTION,
    PLUGIN_NAME,
};

#[no_mangle]
pub extern "C" fn f0r_init() -> c_int {
    println!("Initializing Nebula Sobel filter");
    1 // Return 1 for success
}

#[no_mangle]
pub extern "C" fn f0r_deinit() {
    println!("Cleaning up Nebula Sobel filter");
}

#[no_mangle]
pub unsafe extern "C" fn f0r_get_plugin_info(info: *mut F0rPluginInfo) {
    (*info).name = PLUGIN_NAME.as_ptr() as *const c_char;
    (*info).author = PLUGIN_AUTHOR.as_ptr() as *const c_char;
    (*info).plugin_type = F0R_PLUGIN_TYPE_FILTER;
    (*info).color_model = F0R_COLOR_MODEL_RGBA8888;
    (*info).frei0r_version = 1;
    (*info).major_version = 0;
    (*info).minor_version = 1;
    (*info).num_params = 0;
    (*info).explanation = PLUGIN_DESCRIPTION.as_ptr() as *const c_char;
}

#[no_mangle]
pub extern "C" fn f0r_construct(width: u32, height: u32) -> *mut c_void {
    println!("Constructing instance {}x{}", width, height);
    Box::into_raw(Box::new((width, height))) as *mut c_void
}

#[no_mangle]
pub extern "C" fn f0r_destruct(instance: *mut c_void) {
    unsafe {
        println!("Destructing instance");
        let _ = Box::from_raw(instance as *mut (u32, u32));
    }
}

#[no_mangle]
pub unsafe extern "C" fn f0r_update(
    instance: *mut c_void,
    time: f64,
    inframe: *const Rgba,
    outframe: *mut Rgba
) {
    let (width, height) = *(instance as *const (u32, u32));
    f0r_update2(instance, time, inframe, outframe, width, height);
}

#[no_mangle]
pub unsafe extern "C" fn f0r_update2(
    _instance: *mut c_void,
    _time: f64,
    inframe: *const Rgba,
    outframe: *mut Rgba,
    width: u32,
    height: u32
) {
    let width = width as usize;
    let height = height as usize;
    let size = width * height;

    // Convert input frame to grayscale
    let mut gray_pixels = Vec::with_capacity(size);
    for i in 0..size {
        let rgba = *inframe.add(i);
        gray_pixels.push(rgba_to_gray(rgba));
    }

    // Apply Sobel filter
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let edge = apply_sobel(&gray_pixels, width, height, x, y);
            *outframe.add(idx) = gray_to_rgba(edge);
        }
    }
}

// Required parameter functions (even if we don't use parameters)
#[no_mangle]
pub extern "C" fn f0r_get_param_info(_info: *mut F0rParamInfo, _param_index: c_int) {
    // We don't have any parameters yet
}

#[no_mangle]
pub extern "C" fn f0r_get_param_value(
    _instance: *mut c_void,
    _param: *mut c_void,
    _param_index: c_int
) {
    // We don't have any parameters yet
}

#[no_mangle]
pub extern "C" fn f0r_set_param_value(
    _instance: *mut c_void,
    _param: *const c_void,
    _param_index: c_int
) {
    // We don't have any parameters yet
}
