use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSF {
    pub size: size_t,
    pub parent_or_size: *mut libc::c_uint,
    pub flip: *mut libc::c_uchar,
    pub min: *mut libc::c_uint,
}
unsafe extern "C" fn dsf_new_internal(
    mut size: libc::c_int,
    mut flip: bool,
    mut min: bool,
) -> *mut DSF {
    let mut dsf: *mut DSF = 0 as *mut DSF;
    if (0 as libc::c_int) < size
        && size as libc::c_uint
            <= ((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        && !(b"Bad dsf size\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"0 < size && size <= DSF_MAX && \"Bad dsf size\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"DSF *dsf_new_internal(int, _Bool, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3038: {
        if (0 as libc::c_int) < size
            && size as libc::c_uint
                <= ((2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    >> 1 as libc::c_int)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
            && !(b"Bad dsf size\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"0 < size && size <= DSF_MAX && \"Bad dsf size\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                    b"DSF *dsf_new_internal(int, _Bool, _Bool)\0",
                ))
                .as_ptr(),
            );
        }
    };
    dsf = smalloc(::core::mem::size_of::<DSF>() as libc::c_ulong) as *mut DSF;
    (*dsf).size = size as size_t;
    (*dsf).parent_or_size = smalloc(
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    ) as *mut libc::c_uint;
    (*dsf).flip = if flip as libc::c_int != 0 {
        smalloc(
            (size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        ) as *mut libc::c_uchar
    } else {
        0 as *mut libc::c_uchar
    };
    (*dsf).min = if min as libc::c_int != 0 {
        smalloc(
            (size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        ) as *mut libc::c_uint
    } else {
        0 as *mut libc::c_uint
    };
    dsf_reinit(dsf);
    return dsf;
}
#[no_mangle]
pub unsafe extern "C" fn dsf_new(mut size: libc::c_int) -> *mut DSF {
    return dsf_new_internal(size, 0 as libc::c_int != 0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn dsf_new_flip(mut size: libc::c_int) -> *mut DSF {
    return dsf_new_internal(size, 1 as libc::c_int != 0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn dsf_new_min(mut size: libc::c_int) -> *mut DSF {
    return dsf_new_internal(size, 0 as libc::c_int != 0, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn dsf_reinit(mut dsf: *mut DSF) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*dsf).size {
        *((*dsf).parent_or_size).offset(i as isize) = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            & !((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int)
            | 1 as libc::c_int as libc::c_uint;
        i = i.wrapping_add(1);
        i;
    }
    if !((*dsf).min).is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*dsf).size {
            *((*dsf).min).offset(i as isize) = i as libc::c_uint;
            i = i.wrapping_add(1);
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dsf_copy(mut to: *mut DSF, mut from: *mut DSF) {
    if (*to).size == (*from).size
        && !(b"Mismatch in dsf_copy\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"to->size == from->size && \"Mismatch in dsf_copy\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"void dsf_copy(DSF *, DSF *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3354: {
        if (*to).size == (*from).size
            && !(b"Mismatch in dsf_copy\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"to->size == from->size && \"Mismatch in dsf_copy\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"void dsf_copy(DSF *, DSF *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    memcpy(
        (*to).parent_or_size as *mut libc::c_void,
        (*from).parent_or_size as *const libc::c_void,
        ((*to).size).wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
    );
    if !((*to).flip).is_null() {
        if !((*from).flip).is_null()
            && !(b"Copying a non-flip dsf to a flip one\0" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"from->flip && \"Copying a non-flip dsf to a flip one\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                113 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"void dsf_copy(DSF *, DSF *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_3282: {
            if !((*from).flip).is_null()
                && !(b"Copying a non-flip dsf to a flip one\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"from->flip && \"Copying a non-flip dsf to a flip one\"\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                    113 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        b"void dsf_copy(DSF *, DSF *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        memcpy(
            (*to).flip as *mut libc::c_void,
            (*from).flip as *const libc::c_void,
            ((*to).size).wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        );
    }
    if !((*to).min).is_null() {
        if !((*from).min).is_null()
            && !(b"Copying a non-min dsf to a min one\0" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"from->min && \"Copying a non-min dsf to a min one\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"void dsf_copy(DSF *, DSF *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_3205: {
            if !((*from).min).is_null()
                && !(b"Copying a non-min dsf to a min one\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"from->min && \"Copying a non-min dsf to a min one\"\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                    117 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        b"void dsf_copy(DSF *, DSF *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        memcpy(
            (*to).min as *mut libc::c_void,
            (*from).min as *const libc::c_void,
            ((*to).size).wrapping_mul(::core::mem::size_of::<libc::c_uint>() as libc::c_ulong),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn dsf_free(mut dsf: *mut DSF) {
    if !dsf.is_null() {
        sfree((*dsf).parent_or_size as *mut libc::c_void);
        sfree((*dsf).flip as *mut libc::c_void);
        sfree((*dsf).min as *mut libc::c_void);
        sfree(dsf as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn dsf_find_root(mut dsf: *mut DSF, mut n: size_t) -> size_t {
    while *((*dsf).parent_or_size).offset(n as isize)
        & ((2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            & !((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int))
        == 0
    {
        n = *((*dsf).parent_or_size).offset(n as isize) as size_t;
    }
    return n;
}
#[inline]
unsafe extern "C" fn dsf_path_compress(mut dsf: *mut DSF, mut n: size_t, mut root: size_t) {
    while *((*dsf).parent_or_size).offset(n as isize)
        & ((2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            & !((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int))
        == 0
    {
        let mut prev: size_t = n;
        n = *((*dsf).parent_or_size).offset(n as isize) as size_t;
        *((*dsf).parent_or_size).offset(prev as isize) = root as libc::c_uint;
    }
    if n == root {
    } else {
        __assert_fail(
            b"n == root\0" as *const u8 as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"void dsf_path_compress(DSF *, size_t, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3441: {
        if n == root {
        } else {
            __assert_fail(
                b"n == root\0" as *const u8 as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                147 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                    b"void dsf_path_compress(DSF *, size_t, size_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dsf_canonify(mut dsf: *mut DSF, mut n: libc::c_int) -> libc::c_int {
    let mut root: size_t = 0;
    if 0 as libc::c_int <= n
        && (n as size_t) < (*dsf).size
        && !(b"Overrun in dsf_canonify\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"0 <= n && n < dsf->size && \"Overrun in dsf_canonify\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"int dsf_canonify(DSF *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3603: {
        if 0 as libc::c_int <= n
            && (n as size_t) < (*dsf).size
            && !(b"Overrun in dsf_canonify\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"0 <= n && n < dsf->size && \"Overrun in dsf_canonify\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                154 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"int dsf_canonify(DSF *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    root = dsf_find_root(dsf, n as size_t);
    dsf_path_compress(dsf, n as size_t, root);
    return root as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dsf_merge(mut dsf: *mut DSF, mut n1: libc::c_int, mut n2: libc::c_int) {
    let mut r1: size_t = 0;
    let mut r2: size_t = 0;
    let mut s1: size_t = 0;
    let mut s2: size_t = 0;
    let mut root: size_t = 0;
    if 0 as libc::c_int <= n1
        && (n1 as size_t) < (*dsf).size
        && !(b"Overrun in dsf_merge\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"0 <= n1 && n1 < dsf->size && \"Overrun in dsf_merge\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void dsf_merge(DSF *, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4068: {
        if 0 as libc::c_int <= n1
            && (n1 as size_t) < (*dsf).size
            && !(b"Overrun in dsf_merge\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"0 <= n1 && n1 < dsf->size && \"Overrun in dsf_merge\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                165 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"void dsf_merge(DSF *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int <= n2
        && (n2 as size_t) < (*dsf).size
        && !(b"Overrun in dsf_merge\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"0 <= n2 && n2 < dsf->size && \"Overrun in dsf_merge\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void dsf_merge(DSF *, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4007: {
        if 0 as libc::c_int <= n2
            && (n2 as size_t) < (*dsf).size
            && !(b"Overrun in dsf_merge\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"0 <= n2 && n2 < dsf->size && \"Overrun in dsf_merge\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                166 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"void dsf_merge(DSF *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if ((*dsf).flip).is_null()
        && !(b"dsf_merge on a flip dsf\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"!dsf->flip && \"dsf_merge on a flip dsf\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"void dsf_merge(DSF *, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3961: {
        if ((*dsf).flip).is_null()
            && !(b"dsf_merge on a flip dsf\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"!dsf->flip && \"dsf_merge on a flip dsf\"\0" as *const u8 as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                167 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"void dsf_merge(DSF *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    r1 = dsf_find_root(dsf, n1 as size_t);
    r2 = dsf_find_root(dsf, n2 as size_t);
    if r1 == r2 {
        root = r1;
    } else {
        s1 = (*((*dsf).parent_or_size).offset(r1 as isize)
            & (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int) as size_t;
        s2 = (*((*dsf).parent_or_size).offset(r2 as isize)
            & (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int) as size_t;
        if s1 > s2 {
            root = r1;
            *((*dsf).parent_or_size).offset(r2 as isize) = root as libc::c_uint;
        } else {
            root = r2;
            *((*dsf).parent_or_size).offset(r1 as isize) = root as libc::c_uint;
        }
        *((*dsf).parent_or_size).offset(root as isize) = (s1.wrapping_add(s2)
            | ((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                & !((2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    >> 1 as libc::c_int)) as size_t)
            as libc::c_uint;
        if !((*dsf).min).is_null() {
            let mut m1: libc::c_uint = *((*dsf).min).offset(r1 as isize);
            let mut m2: libc::c_uint = *((*dsf).min).offset(r2 as isize);
            *((*dsf).min).offset(root as isize) = if m1 < m2 { m1 } else { m2 };
        }
    }
    dsf_path_compress(dsf, n1 as size_t, root);
    dsf_path_compress(dsf, n2 as size_t, root);
}
#[no_mangle]
pub unsafe extern "C" fn dsf_equivalent(
    mut dsf: *mut DSF,
    mut n1: libc::c_int,
    mut n2: libc::c_int,
) -> bool {
    return dsf_canonify(dsf, n1) == dsf_canonify(dsf, n2);
}
#[no_mangle]
pub unsafe extern "C" fn dsf_size(mut dsf: *mut DSF, mut n: libc::c_int) -> libc::c_int {
    let mut root: size_t = dsf_canonify(dsf, n) as size_t;
    return (*((*dsf).parent_or_size).offset(root as isize)
        & (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            >> 1 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn dsf_find_root_flip(
    mut dsf: *mut DSF,
    mut n: size_t,
    mut flip: *mut libc::c_uint,
) -> size_t {
    *flip = 0 as libc::c_int as libc::c_uint;
    while *((*dsf).parent_or_size).offset(n as isize)
        & ((2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            & !((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int))
        == 0
    {
        *flip ^= *((*dsf).flip).offset(n as isize) as libc::c_uint;
        n = *((*dsf).parent_or_size).offset(n as isize) as size_t;
    }
    return n;
}
#[inline]
unsafe extern "C" fn dsf_path_compress_flip(
    mut dsf: *mut DSF,
    mut n: size_t,
    mut root: size_t,
    mut flip: libc::c_uint,
) {
    while *((*dsf).parent_or_size).offset(n as isize)
        & ((2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint)
            & !((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int))
        == 0
    {
        let mut prev: size_t = n;
        let mut flip_prev: libc::c_uint = flip;
        n = *((*dsf).parent_or_size).offset(n as isize) as size_t;
        flip ^= *((*dsf).flip).offset(prev as isize) as libc::c_uint;
        *((*dsf).flip).offset(prev as isize) = flip_prev as libc::c_uchar;
        *((*dsf).parent_or_size).offset(prev as isize) = root as libc::c_uint;
    }
    if n == root {
    } else {
        __assert_fail(
            b"n == root\0" as *const u8 as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void dsf_path_compress_flip(DSF *, size_t, size_t, unsigned int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4267: {
        if n == root {
        } else {
            __assert_fail(
                b"n == root\0" as *const u8 as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void dsf_path_compress_flip(DSF *, size_t, size_t, unsigned int)\0",
                ))
                .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dsf_canonify_flip(
    mut dsf: *mut DSF,
    mut n: libc::c_int,
    mut inverse: *mut bool,
) -> libc::c_int {
    let mut root: size_t = 0;
    let mut flip: libc::c_uint = 0;
    if 0 as libc::c_int <= n
        && (n as size_t) < (*dsf).size
        && !(b"Overrun in dsf_canonify_flip\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"0 <= n && n < dsf->size && \"Overrun in dsf_canonify_flip\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"int dsf_canonify_flip(DSF *, int, _Bool *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_5078: {
        if 0 as libc::c_int <= n
            && (n as size_t) < (*dsf).size
            && !(b"Overrun in dsf_canonify_flip\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"0 <= n && n < dsf->size && \"Overrun in dsf_canonify_flip\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                241 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"int dsf_canonify_flip(DSF *, int, _Bool *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !((*dsf).flip).is_null()
        && !(b"dsf_canonify_flip on a non-flip dsf\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"dsf->flip && \"dsf_canonify_flip on a non-flip dsf\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                b"int dsf_canonify_flip(DSF *, int, _Bool *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_5034: {
        if !((*dsf).flip).is_null()
            && !(b"dsf_canonify_flip on a non-flip dsf\0" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"dsf->flip && \"dsf_canonify_flip on a non-flip dsf\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                242 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"int dsf_canonify_flip(DSF *, int, _Bool *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    root = dsf_find_root_flip(dsf, n as size_t, &mut flip);
    dsf_path_compress_flip(dsf, n as size_t, root, flip);
    *inverse = flip != 0;
    return root as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dsf_merge_flip(
    mut dsf: *mut DSF,
    mut n1: libc::c_int,
    mut n2: libc::c_int,
    mut inverse: bool,
) {
    let mut r1: size_t = 0;
    let mut r2: size_t = 0;
    let mut s1: size_t = 0;
    let mut s2: size_t = 0;
    let mut root: size_t = 0;
    let mut f1: libc::c_uint = 0;
    let mut f2: libc::c_uint = 0;
    if 0 as libc::c_int <= n1
        && (n1 as size_t) < (*dsf).size
        && !(b"Overrun in dsf_merge_flip\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"0 <= n1 && n1 < dsf->size && \"Overrun in dsf_merge_flip\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4923: {
        if 0 as libc::c_int <= n1
            && (n1 as size_t) < (*dsf).size
            && !(b"Overrun in dsf_merge_flip\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"0 <= n1 && n1 < dsf->size && \"Overrun in dsf_merge_flip\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                255 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int <= n2
        && (n2 as size_t) < (*dsf).size
        && !(b"Overrun in dsf_merge_flip\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"0 <= n2 && n2 < dsf->size && \"Overrun in dsf_merge_flip\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4861: {
        if 0 as libc::c_int <= n2
            && (n2 as size_t) < (*dsf).size
            && !(b"Overrun in dsf_merge_flip\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"0 <= n2 && n2 < dsf->size && \"Overrun in dsf_merge_flip\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                256 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if !((*dsf).flip).is_null()
        && !(b"dsf_merge_flip on a non-flip dsf\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"dsf->flip && \"dsf_merge_flip on a non-flip dsf\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4817: {
        if !((*dsf).flip).is_null()
            && !(b"dsf_merge_flip on a non-flip dsf\0" as *const u8 as *const libc::c_char)
                .is_null()
        {
        } else {
            __assert_fail(
                b"dsf->flip && \"dsf_merge_flip on a non-flip dsf\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                257 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
                ))
                .as_ptr(),
            );
        }
    };
    r1 = dsf_find_root_flip(dsf, n1 as size_t, &mut f1);
    r2 = dsf_find_root_flip(dsf, n2 as size_t, &mut f2);
    if r1 == r2 {
        if f1 ^ f2 ^ inverse as libc::c_uint == 0 as libc::c_int as libc::c_uint
            && !(b"Inconsistency in dsf_merge_flip\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"(f1 ^ f2 ^ inverse) == 0 && \"Inconsistency in dsf_merge_flip\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                265 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
                ))
                .as_ptr(),
            );
        }
        'c_4651: {
            if f1 ^ f2 ^ inverse as libc::c_uint == 0 as libc::c_int as libc::c_uint
                && !(b"Inconsistency in dsf_merge_flip\0" as *const u8 as *const libc::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"(f1 ^ f2 ^ inverse) == 0 && \"Inconsistency in dsf_merge_flip\"\0"
                        as *const u8 as *const libc::c_char,
                    b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                    265 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"void dsf_merge_flip(DSF *, int, int, _Bool)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        root = r1;
    } else {
        s1 = (*((*dsf).parent_or_size).offset(r1 as isize)
            & (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int) as size_t;
        s2 = (*((*dsf).parent_or_size).offset(r2 as isize)
            & (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                >> 1 as libc::c_int) as size_t;
        if s1 > s2 {
            root = r1;
            *((*dsf).parent_or_size).offset(r2 as isize) = root as libc::c_uint;
            *((*dsf).flip).offset(r2 as isize) =
                (f1 ^ f2 ^ inverse as libc::c_uint) as libc::c_uchar;
            f2 ^= *((*dsf).flip).offset(r2 as isize) as libc::c_uint;
        } else {
            root = r2;
            root = r2;
            *((*dsf).parent_or_size).offset(r1 as isize) = root as libc::c_uint;
            *((*dsf).flip).offset(r1 as isize) =
                (f1 ^ f2 ^ inverse as libc::c_uint) as libc::c_uchar;
            f1 ^= *((*dsf).flip).offset(r1 as isize) as libc::c_uint;
        }
        *((*dsf).parent_or_size).offset(root as isize) = (s1.wrapping_add(s2)
            | ((2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                & !((2147483647 as libc::c_int as libc::c_uint)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_add(1 as libc::c_uint)
                    >> 1 as libc::c_int)) as size_t)
            as libc::c_uint;
        if !((*dsf).min).is_null() {
            let mut m1: libc::c_uint = *((*dsf).min).offset(r1 as isize);
            let mut m2: libc::c_uint = *((*dsf).min).offset(r2 as isize);
            *((*dsf).min).offset(root as isize) = if m1 < m2 { m1 } else { m2 };
        }
    }
    dsf_path_compress_flip(dsf, n1 as size_t, root, f1);
    dsf_path_compress_flip(dsf, n2 as size_t, root, f2);
}
#[no_mangle]
pub unsafe extern "C" fn dsf_minimal(mut dsf: *mut DSF, mut n: libc::c_int) -> libc::c_int {
    let mut root: size_t = 0;
    if !((*dsf).min).is_null()
        && !(b"dsf_minimal on a non-min dsf\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"dsf->min && \"dsf_minimal on a non-min dsf\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"int dsf_minimal(DSF *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4170: {
        if !((*dsf).min).is_null()
            && !(b"dsf_minimal on a non-min dsf\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"dsf->min && \"dsf_minimal on a non-min dsf\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/dsf.c\0" as *const u8 as *const libc::c_char,
                301 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    b"int dsf_minimal(DSF *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    root = dsf_canonify(dsf, n) as size_t;
    return *((*dsf).min).offset(root as isize) as libc::c_int;
}
