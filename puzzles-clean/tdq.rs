#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tdq {
    pub n: libc::c_int,
    pub queue: *mut libc::c_int,
    pub ip: libc::c_int,
    pub op: libc::c_int,
    pub flags: *mut bool,
}
#[no_mangle]
pub unsafe extern "C" fn tdq_new(mut n: libc::c_int) -> *mut tdq {
    let mut i: libc::c_int = 0;
    let mut tdq: *mut tdq = smalloc(::core::mem::size_of::<tdq>() as libc::c_ulong)
        as *mut tdq;
    (*tdq)
        .queue = smalloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*tdq)
        .flags = smalloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    i = 0 as libc::c_int;
    while i < n {
        *((*tdq).queue).offset(i as isize) = 0 as libc::c_int;
        *((*tdq).flags).offset(i as isize) = 0 as libc::c_int != 0;
        i += 1;
        i;
    }
    (*tdq).n = n;
    (*tdq).op = 0 as libc::c_int;
    (*tdq).ip = (*tdq).op;
    return tdq;
}
#[no_mangle]
pub unsafe extern "C" fn tdq_free(mut tdq: *mut tdq) {
    sfree((*tdq).queue as *mut libc::c_void);
    sfree((*tdq).flags as *mut libc::c_void);
    sfree(tdq as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn tdq_add(mut tdq: *mut tdq, mut k: libc::c_int) {
    if (k as libc::c_uint) < (*tdq).n as libc::c_uint {} else {
        __assert_fail(
            b"(unsigned)k < (unsigned)tdq->n\0" as *const u8 as *const libc::c_char,
            b"/puzzles/tdq.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void tdq_add(tdq *, int)\0"))
                .as_ptr(),
        );
    }
    'c_2791: {
        if (k as libc::c_uint) < (*tdq).n as libc::c_uint {} else {
            __assert_fail(
                b"(unsigned)k < (unsigned)tdq->n\0" as *const u8 as *const libc::c_char,
                b"/puzzles/tdq.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void tdq_add(tdq *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !*((*tdq).flags).offset(k as isize) {
        *((*tdq).queue).offset((*tdq).ip as isize) = k;
        *((*tdq).flags).offset(k as isize) = 1 as libc::c_int != 0;
        (*tdq).ip += 1;
        if (*tdq).ip == (*tdq).n {
            (*tdq).ip = 0 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tdq_remove(mut tdq: *mut tdq) -> libc::c_int {
    let mut ret: libc::c_int = *((*tdq).queue).offset((*tdq).op as isize);
    if !*((*tdq).flags).offset(ret as isize) {
        return -(1 as libc::c_int);
    }
    *((*tdq).flags).offset(ret as isize) = 0 as libc::c_int != 0;
    (*tdq).op += 1;
    if (*tdq).op == (*tdq).n {
        (*tdq).op = 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tdq_fill(mut tdq: *mut tdq) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*tdq).n {
        tdq_add(tdq, i);
        i += 1;
        i;
    }
}
