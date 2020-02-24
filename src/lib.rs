use std::ffi::{CStr};
use std::os::raw::c_char;
use stegano_core::*;

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

/// # Safety
///
/// This copies the memory over and does not rely on the external lifetime of the arguments.
#[no_mangle]
pub unsafe extern "C" fn unveil_image_to_folder(
    image_path: *const c_char,
    output_folder: *const c_char
) {
    let image_path = CStr::from_ptr(image_path).to_string_lossy().into_owned();
    let output_folder = CStr::from_ptr(output_folder).to_string_lossy().into_owned();

    SteganoCore::decoder()
        .use_source_image(&image_path)
        .write_to_folder(&output_folder)
        .unveil();
}