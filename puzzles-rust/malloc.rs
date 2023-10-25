use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn smalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if size > 9223372036854775807 as libc::c_long as size_t {
        panic!("allocation too large");
    }
    p = malloc(size);
    if p.is_null() {
        panic!("out of memory");
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn sfree(mut p: *mut libc::c_void) {
    if !p.is_null() {
        free(p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn srealloc(mut p: *mut libc::c_void, mut size: size_t) -> *mut libc::c_void {
    let mut q: *mut libc::c_void = 0 as *mut libc::c_void;
    if size > 9223372036854775807 as libc::c_long as size_t {
        panic!("allocation too large");
    }
    if !p.is_null() {
        q = realloc(p, size);
    } else {
        q = malloc(size);
    }
    if q.is_null() {
        panic!("out of memory");
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn dupstr(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut r: *mut libc::c_char =
        smalloc((1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(s))) as *mut libc::c_char;
    strcpy(r, s);
    return r;
}
