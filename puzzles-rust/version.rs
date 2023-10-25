use ::libc;
#[no_mangle]
pub static mut ver: [libc::c_char; 19] = unsafe {
    *::core::mem::transmute::<&[u8; 19], &mut [libc::c_char; 19]>(b"Unidentified build\0")
};
