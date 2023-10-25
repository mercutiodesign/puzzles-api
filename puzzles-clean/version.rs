#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut ver: [libc::c_char; 19] = unsafe {
    *::core::mem::transmute::<
        &[u8; 19],
        &mut [libc::c_char; 19],
    >(b"Unidentified build\0")
};
