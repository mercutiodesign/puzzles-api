use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _combi_ctx {
    pub r: libc::c_int,
    pub n: libc::c_int,
    pub nleft: libc::c_int,
    pub total: libc::c_int,
    pub a: *mut libc::c_int,
}
pub type combi_ctx = _combi_ctx;
unsafe extern "C" fn factx(mut x: libc::c_long, mut y: libc::c_long) -> libc::c_long {
    let mut acc: libc::c_long = 1 as libc::c_int as libc::c_long;
    let mut i: libc::c_long = 0;
    i = y;
    while i <= x {
        acc *= i;
        i += 1;
        i;
    }
    return acc;
}
#[no_mangle]
pub unsafe extern "C" fn reset_combi(mut combi: *mut combi_ctx) {
    let mut i: libc::c_int = 0;
    (*combi).nleft = (*combi).total;
    i = 0 as libc::c_int;
    while i < (*combi).r {
        *((*combi).a).offset(i as isize) = i;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn new_combi(mut r: libc::c_int, mut n: libc::c_int) -> *mut combi_ctx {
    let mut nfr: libc::c_long = 0;
    let mut nrf: libc::c_long = 0;
    let mut combi: *mut combi_ctx = 0 as *mut combi_ctx;
    if r <= n {
    } else {
        __assert_fail(
            b"r <= n\0" as *const u8 as *const libc::c_char,
            b"/puzzles/combi.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"combi_ctx *new_combi(int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3271: {
        if r <= n {
        } else {
            __assert_fail(
                b"r <= n\0" as *const u8 as *const libc::c_char,
                b"/puzzles/combi.c\0" as *const u8 as *const libc::c_char,
                29 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"combi_ctx *new_combi(int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if n >= 1 as libc::c_int {
    } else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const libc::c_char,
            b"/puzzles/combi.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"combi_ctx *new_combi(int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3230: {
        if n >= 1 as libc::c_int {
        } else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const libc::c_char,
                b"/puzzles/combi.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"combi_ctx *new_combi(int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    combi = smalloc(::core::mem::size_of::<combi_ctx>() as libc::c_ulong) as *mut combi_ctx;
    memset(
        combi as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<combi_ctx>() as libc::c_ulong,
    );
    (*combi).r = r;
    (*combi).n = n;
    (*combi).a = smalloc(
        (r as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        (*combi).a as *mut libc::c_void,
        0 as libc::c_int,
        (r as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    nfr = factx(n as libc::c_long, (r + 1 as libc::c_int) as libc::c_long);
    nrf = factx((n - r) as libc::c_long, 1 as libc::c_int as libc::c_long);
    (*combi).total = (nfr / nrf) as libc::c_int;
    reset_combi(combi);
    return combi;
}
#[no_mangle]
pub unsafe extern "C" fn next_combi(mut combi: *mut combi_ctx) -> *mut combi_ctx {
    let mut i: libc::c_int = (*combi).r - 1 as libc::c_int;
    let mut j: libc::c_int = 0;
    if !((*combi).nleft == (*combi).total) {
        if (*combi).nleft <= 0 as libc::c_int {
            return 0 as *mut combi_ctx;
        }
        while *((*combi).a).offset(i as isize) == (*combi).n - (*combi).r + i {
            i -= 1;
            i;
        }
        *((*combi).a).offset(i as isize) += 1 as libc::c_int;
        j = i + 1 as libc::c_int;
        while j < (*combi).r {
            *((*combi).a).offset(j as isize) = *((*combi).a).offset(i as isize) + j - i;
            j += 1;
            j;
        }
    }
    (*combi).nleft -= 1;
    (*combi).nleft;
    return combi;
}
#[no_mangle]
pub unsafe extern "C" fn free_combi(mut combi: *mut combi_ctx) {
    sfree((*combi).a as *mut libc::c_void);
    sfree(combi as *mut libc::c_void);
}
