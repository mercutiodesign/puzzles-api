use ::libc;
extern "C" {
    pub type random_state;
    pub type tree234_Tag;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn srealloc(p: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn random_new(seed: *const libc::c_char, len: libc::c_int) -> *mut random_state;
    fn random_upto(state: *mut random_state, limit: libc::c_ulong) -> libc::c_ulong;
    fn random_free(state: *mut random_state);
    fn newtree234(cmp: cmpfn234) -> *mut tree234;
    fn freetree234(t: *mut tree234);
    fn add234(t: *mut tree234, e: *mut libc::c_void) -> *mut libc::c_void;
    fn find234(t: *mut tree234, e: *mut libc::c_void, cmp: cmpfn234) -> *mut libc::c_void;
    fn delpos234(t: *mut tree234, index: libc::c_int) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PenrosePatchParams {
    pub start_vertex: libc::c_uint,
    pub orientation: libc::c_int,
    pub ncoords: size_t,
    pub coords: *mut libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PENROSE_P3: C2RustUnnamed = 1;
pub const PENROSE_P2: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PenroseContext {
    pub rs: *mut random_state,
    pub must_free_rs: bool,
    pub start_vertex: libc::c_uint,
    pub orientation: libc::c_int,
    pub prototype: *mut PenroseCoords,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PenroseCoords {
    pub c: *mut libc::c_char,
    pub nc: size_t,
    pub csize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Point {
    pub coeffs: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PenroseOutputCtx {
    pub xoff: libc::c_int,
    pub yoff: libc::c_int,
    pub xmin: Coord,
    pub xmax: Coord,
    pub ymin: Coord,
    pub ymax: Coord,
    pub external_cb: penrose_tile_callback_fn,
    pub external_cbctx: *mut libc::c_void,
}
pub type penrose_tile_callback_fn =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Coord {
    pub c1: libc::c_int,
    pub cr5: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PenroseTriangle {
    pub vertices: [Point; 3],
    pub pc: *mut PenroseCoords,
    pub next: *mut PenroseTriangle,
    pub reported: bool,
}
pub type tree234 = tree234_Tag;
pub type cmpfn234 =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub new_child: libc::c_char,
    pub new_edge: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub internal: C2RustUnnamed_0,
    pub external: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub parent_edge: libc::c_uchar,
    pub end: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TransitionResult {
    pub type_0: C2RustUnnamed_3,
    pub u: C2RustUnnamed_1,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const FAIL: C2RustUnnamed_3 = 2;
pub const EXTERNAL: C2RustUnnamed_3 = 1;
pub const INTERNAL: C2RustUnnamed_3 = 0;
#[inline]
unsafe extern "C" fn point_add(mut a: Point, mut b: Point) -> Point {
    let mut r: Point = Point { coeffs: [0; 4] };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as size_t {
        r.coeffs[i as usize] = a.coeffs[i as usize] + b.coeffs[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    return r;
}
#[inline]
unsafe extern "C" fn point_sub(mut a: Point, mut b: Point) -> Point {
    let mut r: Point = Point { coeffs: [0; 4] };
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as size_t {
        r.coeffs[i as usize] = a.coeffs[i as usize] - b.coeffs[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    return r;
}
#[inline]
unsafe extern "C" fn point_mul_by_t(mut x: Point) -> Point {
    let mut r: Point = Point { coeffs: [0; 4] };
    r.coeffs[0 as libc::c_int as usize] = -x.coeffs[3 as libc::c_int as usize];
    r.coeffs[1 as libc::c_int as usize] =
        x.coeffs[0 as libc::c_int as usize] + x.coeffs[3 as libc::c_int as usize];
    r.coeffs[2 as libc::c_int as usize] =
        x.coeffs[1 as libc::c_int as usize] - x.coeffs[3 as libc::c_int as usize];
    r.coeffs[3 as libc::c_int as usize] =
        x.coeffs[2 as libc::c_int as usize] + x.coeffs[3 as libc::c_int as usize];
    return r;
}
#[inline]
unsafe extern "C" fn point_mul(mut a: Point, mut b: Point) -> Point {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut r: Point = Point { coeffs: [0; 4] };
    j = 0 as libc::c_int as size_t;
    while j < 4 as libc::c_int as size_t {
        r.coeffs[j as usize] = a.coeffs[j as usize] * b.coeffs[3 as libc::c_int as usize];
        j = j.wrapping_add(1);
        j;
    }
    i = 3 as libc::c_int as size_t;
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as size_t) {
            break;
        }
        r = point_mul_by_t(r);
        j = 0 as libc::c_int as size_t;
        while j < 4 as libc::c_int as size_t {
            r.coeffs[j as usize] += a.coeffs[j as usize] * b.coeffs[i as usize];
            j = j.wrapping_add(1);
            j;
        }
    }
    return r;
}
#[inline]
unsafe extern "C" fn point_rot(mut s: libc::c_int) -> Point {
    let mut r: Point = {
        let mut init = Point {
            coeffs: [
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ],
        };
        init
    };
    let mut tpower: Point = {
        let mut init = Point {
            coeffs: [
                0 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ],
        };
        init
    };
    s = s % 10 as libc::c_int;
    if s < 0 as libc::c_int {
        s += 10 as libc::c_int;
    }
    loop {
        if s & 1 as libc::c_int != 0 {
            r = point_mul(r, tpower);
        }
        s >>= 1 as libc::c_int;
        if s == 0 {
            break;
        }
        tpower = point_mul(tpower, tpower);
    }
    return r;
}
#[inline]
unsafe extern "C" fn point_x(mut p: Point) -> Coord {
    let mut x: Coord = {
        let mut init = Coord {
            c1: 4 as libc::c_int * p.coeffs[0 as libc::c_int as usize]
                + p.coeffs[1 as libc::c_int as usize]
                - p.coeffs[2 as libc::c_int as usize]
                + p.coeffs[3 as libc::c_int as usize],
            cr5: p.coeffs[1 as libc::c_int as usize] + p.coeffs[2 as libc::c_int as usize]
                - p.coeffs[3 as libc::c_int as usize],
        };
        init
    };
    return x;
}
#[inline]
unsafe extern "C" fn point_y(mut p: Point) -> Coord {
    let mut y: Coord = {
        let mut init = Coord {
            c1: 2 as libc::c_int * p.coeffs[1 as libc::c_int as usize]
                + p.coeffs[2 as libc::c_int as usize]
                + p.coeffs[3 as libc::c_int as usize],
            cr5: p.coeffs[2 as libc::c_int as usize] + p.coeffs[3 as libc::c_int as usize],
        };
        init
    };
    return y;
}
#[inline]
unsafe extern "C" fn coord_sign(mut x: Coord) -> libc::c_int {
    if x.c1 == 0 as libc::c_int && x.cr5 == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if x.c1 >= 0 as libc::c_int && x.cr5 >= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if x.c1 <= 0 as libc::c_int && x.cr5 <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if x.c1 * x.c1 > 5 as libc::c_int * x.cr5 * x.cr5 {
        return if x.c1 < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
    } else {
        return if x.cr5 < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
    };
}
#[inline]
unsafe extern "C" fn coord_sub(mut a: Coord, mut b: Coord) -> Coord {
    let mut diff: Coord = Coord { c1: 0, cr5: 0 };
    diff.c1 = a.c1 - b.c1;
    diff.cr5 = a.cr5 - b.cr5;
    return diff;
}
#[inline]
unsafe extern "C" fn coord_cmp(mut a: Coord, mut b: Coord) -> libc::c_int {
    return coord_sign(coord_sub(a, b));
}
#[no_mangle]
pub unsafe extern "C" fn penrose_valid_letter(mut c: libc::c_char, mut which: libc::c_int) -> bool {
    match c as libc::c_int {
        65 | 66 | 85 | 86 => return which == PENROSE_P2 as libc::c_int,
        67 | 68 | 88 | 89 => return which == PENROSE_P3 as libc::c_int,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn internal(
    mut new_child: libc::c_char,
    mut new_edge: libc::c_uint,
) -> TransitionResult {
    let mut tr: TransitionResult = TransitionResult {
        type_0: INTERNAL,
        u: C2RustUnnamed_1 {
            internal: C2RustUnnamed_0 {
                new_child: 0,
                new_edge: 0,
            },
        },
    };
    tr.type_0 = INTERNAL;
    tr.u.internal.new_child = new_child;
    tr.u.internal.new_edge = new_edge as libc::c_uchar;
    return tr;
}
#[inline]
unsafe extern "C" fn external(
    mut parent_edge: libc::c_uint,
    mut end: libc::c_int,
) -> TransitionResult {
    let mut tr: TransitionResult = TransitionResult {
        type_0: INTERNAL,
        u: C2RustUnnamed_1 {
            internal: C2RustUnnamed_0 {
                new_child: 0,
                new_edge: 0,
            },
        },
    };
    tr.type_0 = EXTERNAL;
    tr.u.external.parent_edge = parent_edge as libc::c_uchar;
    tr.u.external.end = end as libc::c_schar;
    return tr;
}
#[inline]
unsafe extern "C" fn fail() -> TransitionResult {
    let mut tr: TransitionResult = TransitionResult {
        type_0: INTERNAL,
        u: C2RustUnnamed_1 {
            internal: C2RustUnnamed_0 {
                new_child: 0,
                new_edge: 0,
            },
        },
    };
    tr.type_0 = FAIL;
    return tr;
}
unsafe extern "C" fn transition(
    mut parent: libc::c_char,
    mut child: libc::c_char,
    mut edge: libc::c_uint,
) -> TransitionResult {
    match parent as libc::c_int {
        65 => {
            let mut current_block_12: u64;
            match child as libc::c_int {
                65 => {
                    match edge {
                        0 => {
                            return external(2 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                        }
                        1 => {
                            return external(0 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                        }
                        2 => {
                            return internal(
                                'B' as i32 as libc::c_char,
                                1 as libc::c_int as libc::c_uint,
                            );
                        }
                        _ => {}
                    }
                    current_block_12 = 17005639753219924481;
                }
                66 => {
                    current_block_12 = 17005639753219924481;
                }
                85 => {
                    current_block_12 = 10999470842310942890;
                }
                _ => {
                    current_block_12 = 3129321334768669954;
                }
            }
            match current_block_12 {
                17005639753219924481 => {
                    match edge {
                        0 => {
                            return internal(
                                'U' as i32 as libc::c_char,
                                1 as libc::c_int as libc::c_uint,
                            );
                        }
                        1 => {
                            return internal(
                                'A' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        2 => {
                            return external(1 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                        }
                        _ => {}
                    }
                    current_block_12 = 10999470842310942890;
                }
                _ => {}
            }
            match current_block_12 {
                10999470842310942890 => match edge {
                    0 => {
                        return external(2 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                    }
                    1 => {
                        return internal(
                            'B' as i32 as libc::c_char,
                            0 as libc::c_int as libc::c_uint,
                        );
                    }
                    2 => {
                        return external(1 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        66 => {
            let mut current_block_26: u64;
            match child as libc::c_int {
                65 => {
                    match edge {
                        0 => {
                            return internal(
                                'V' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        1 => {
                            return external(2 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                        }
                        2 => {
                            return internal(
                                'B' as i32 as libc::c_char,
                                1 as libc::c_int as libc::c_uint,
                            );
                        }
                        _ => {}
                    }
                    current_block_26 = 3296947174477454490;
                }
                66 => {
                    current_block_26 = 3296947174477454490;
                }
                86 => {
                    current_block_26 = 14466346190789488127;
                }
                _ => {
                    current_block_26 = 17182251364036868676;
                }
            }
            match current_block_26 {
                3296947174477454490 => {
                    match edge {
                        0 => {
                            return external(1 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                        }
                        1 => {
                            return internal(
                                'A' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        2 => {
                            return external(0 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                        }
                        _ => {}
                    }
                    current_block_26 = 14466346190789488127;
                }
                _ => {}
            }
            match current_block_26 {
                14466346190789488127 => match edge {
                    0 => {
                        return external(1 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                    }
                    1 => {
                        return external(2 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                    }
                    2 => {
                        return internal(
                            'A' as i32 as libc::c_char,
                            0 as libc::c_int as libc::c_uint,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        85 => {
            let mut current_block_36: u64;
            match child as libc::c_int {
                66 => {
                    match edge {
                        0 => {
                            return internal(
                                'U' as i32 as libc::c_char,
                                1 as libc::c_int as libc::c_uint,
                            );
                        }
                        1 => {
                            return external(2 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                        }
                        2 => {
                            return external(0 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                        }
                        _ => {}
                    }
                    current_block_36 = 8485601539572644793;
                }
                85 => {
                    current_block_36 = 8485601539572644793;
                }
                _ => {
                    current_block_36 = 3779556845191537975;
                }
            }
            match current_block_36 {
                8485601539572644793 => match edge {
                    0 => {
                        return external(1 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                    }
                    1 => {
                        return internal(
                            'B' as i32 as libc::c_char,
                            0 as libc::c_int as libc::c_uint,
                        );
                    }
                    2 => {
                        return external(0 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        86 => {
            let mut current_block_46: u64;
            match child as libc::c_int {
                65 => {
                    match edge {
                        0 => {
                            return internal(
                                'V' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        1 => {
                            return external(0 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                        }
                        2 => {
                            return external(1 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                        }
                        _ => {}
                    }
                    current_block_46 = 16989790556132644855;
                }
                86 => {
                    current_block_46 = 16989790556132644855;
                }
                _ => {
                    current_block_46 = 299272565458146675;
                }
            }
            match current_block_46 {
                16989790556132644855 => match edge {
                    0 => {
                        return external(2 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                    }
                    1 => {
                        return external(0 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                    }
                    2 => {
                        return internal(
                            'A' as i32 as libc::c_char,
                            0 as libc::c_int as libc::c_uint,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        67 => {
            let mut current_block_56: u64;
            match child as libc::c_int {
                67 => {
                    match edge {
                        0 => {
                            return external(1 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                        }
                        1 => {
                            return internal(
                                'Y' as i32 as libc::c_char,
                                1 as libc::c_int as libc::c_uint,
                            );
                        }
                        2 => {
                            return external(0 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                        }
                        _ => {}
                    }
                    current_block_56 = 15760596312215808921;
                }
                89 => {
                    current_block_56 = 15760596312215808921;
                }
                _ => {
                    current_block_56 = 12884427692930221142;
                }
            }
            match current_block_56 {
                15760596312215808921 => match edge {
                    0 => {
                        return external(2 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                    }
                    1 => {
                        return internal(
                            'C' as i32 as libc::c_char,
                            1 as libc::c_int as libc::c_uint,
                        );
                    }
                    2 => {
                        return external(1 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        68 => {
            let mut current_block_66: u64;
            match child as libc::c_int {
                68 => {
                    match edge {
                        0 => {
                            return external(2 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                        }
                        1 => {
                            return external(0 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                        }
                        2 => {
                            return internal(
                                'X' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        _ => {}
                    }
                    current_block_66 = 8953668966427799871;
                }
                88 => {
                    current_block_66 = 8953668966427799871;
                }
                _ => {
                    current_block_66 = 11151674866193327754;
                }
            }
            match current_block_66 {
                8953668966427799871 => match edge {
                    0 => {
                        return external(1 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                    }
                    1 => {
                        return external(2 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                    }
                    2 => {
                        return internal(
                            'D' as i32 as libc::c_char,
                            2 as libc::c_int as libc::c_uint,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        88 => {
            let mut current_block_80: u64;
            match child as libc::c_int {
                67 => {
                    match edge {
                        0 => {
                            return external(2 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                        }
                        1 => {
                            return internal(
                                'Y' as i32 as libc::c_char,
                                1 as libc::c_int as libc::c_uint,
                            );
                        }
                        2 => {
                            return internal(
                                'X' as i32 as libc::c_char,
                                1 as libc::c_int as libc::c_uint,
                            );
                        }
                        _ => {}
                    }
                    current_block_80 = 6875173569732010003;
                }
                88 => {
                    current_block_80 = 6875173569732010003;
                }
                89 => {
                    current_block_80 = 6687671717986857593;
                }
                _ => {
                    current_block_80 = 14876250339250250549;
                }
            }
            match current_block_80 {
                6875173569732010003 => {
                    match edge {
                        0 => {
                            return external(1 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                        }
                        1 => {
                            return internal(
                                'C' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        2 => {
                            return external(0 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                        }
                        _ => {}
                    }
                    current_block_80 = 6687671717986857593;
                }
                _ => {}
            }
            match current_block_80 {
                6687671717986857593 => match edge {
                    0 => {
                        return external(0 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                    }
                    1 => {
                        return internal(
                            'C' as i32 as libc::c_char,
                            1 as libc::c_int as libc::c_uint,
                        );
                    }
                    2 => {
                        return external(2 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        89 => {
            let mut current_block_94: u64;
            match child as libc::c_int {
                68 => {
                    match edge {
                        0 => {
                            return external(1 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                        }
                        1 => {
                            return internal(
                                'Y' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        2 => {
                            return internal(
                                'X' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        _ => {}
                    }
                    current_block_94 = 494466633303020227;
                }
                88 => {
                    current_block_94 = 494466633303020227;
                }
                89 => {
                    current_block_94 = 16137079087587250406;
                }
                _ => {
                    current_block_94 = 1445032439528402095;
                }
            }
            match current_block_94 {
                494466633303020227 => {
                    match edge {
                        0 => {
                            return external(0 as libc::c_int as libc::c_uint, -(1 as libc::c_int));
                        }
                        1 => {
                            return external(1 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                        }
                        2 => {
                            return internal(
                                'D' as i32 as libc::c_char,
                                2 as libc::c_int as libc::c_uint,
                            );
                        }
                        _ => {}
                    }
                    current_block_94 = 16137079087587250406;
                }
                _ => {}
            }
            match current_block_94 {
                16137079087587250406 => match edge {
                    0 => {
                        return external(2 as libc::c_int as libc::c_uint, 0 as libc::c_int);
                    }
                    1 => {
                        return external(0 as libc::c_int as libc::c_uint, 1 as libc::c_int);
                    }
                    2 => {
                        return internal(
                            'D' as i32 as libc::c_char,
                            1 as libc::c_int as libc::c_uint,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
            return fail();
        }
        _ => return fail(),
    };
}
unsafe extern "C" fn transition_in(
    mut parent: libc::c_char,
    mut edge: libc::c_uint,
    mut end: libc::c_int,
) -> TransitionResult {
    match parent as libc::c_int {
        65 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                1 => {
                    return internal('A' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                3 => {
                    return internal('B' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                5 => {
                    return internal('U' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                6 => {
                    return internal('U' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                8 => {
                    return internal('A' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        66 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                1 => {
                    return internal('B' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                3 => {
                    return internal('B' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                5 => {
                    return internal('V' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                6 => {
                    return internal('V' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                8 => {
                    return internal('A' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        85 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                0 => {
                    return internal('B' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                2 => {
                    return internal('U' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                4 => {
                    return internal('U' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                7 => {
                    return internal('B' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        86 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                0 => {
                    return internal('V' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                2 => {
                    return internal('A' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                4 => {
                    return internal('A' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                7 => {
                    return internal('V' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        67 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                1 => {
                    return internal('C' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                3 => {
                    return internal('C' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                5 => {
                    return internal('Y' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                7 => {
                    return internal('Y' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        68 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                1 => {
                    return internal('D' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                4 => {
                    return internal('X' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                6 => {
                    return internal('X' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                8 => {
                    return internal('D' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        88 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                0 => {
                    return internal('Y' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                2 => {
                    return internal('X' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                4 => {
                    return internal('X' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                6 => {
                    return internal('C' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                8 => {
                    return internal('Y' as i32 as libc::c_char, 2 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        89 => {
            match (3 as libc::c_int as libc::c_uint)
                .wrapping_mul(edge)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
                .wrapping_add(end as libc::c_uint)
            {
                2 => {
                    return internal('X' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                0 => {
                    return internal('Y' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                3 => {
                    return internal('X' as i32 as libc::c_char, 1 as libc::c_int as libc::c_uint);
                }
                5 => {
                    return internal('D' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                7 => {
                    return internal('Y' as i32 as libc::c_char, 0 as libc::c_int as libc::c_uint);
                }
                _ => return fail(),
            }
        }
        _ => return fail(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn penrose_coords_new() -> *mut PenroseCoords {
    let mut pc: *mut PenroseCoords =
        smalloc(::core::mem::size_of::<PenroseCoords>() as libc::c_ulong) as *mut PenroseCoords;
    (*pc).csize = 0 as libc::c_int as size_t;
    (*pc).nc = (*pc).csize;
    (*pc).c = 0 as *mut libc::c_char;
    return pc;
}
#[no_mangle]
pub unsafe extern "C" fn penrose_coords_free(mut pc: *mut PenroseCoords) {
    if !pc.is_null() {
        sfree((*pc).c as *mut libc::c_void);
        sfree(pc as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn penrose_coords_make_space(mut pc: *mut PenroseCoords, mut size: size_t) {
    if (*pc).csize < size {
        (*pc).csize = ((*pc).csize * 5 as libc::c_int as size_t / 4 as libc::c_int as size_t)
            .wrapping_add(16 as libc::c_int as size_t);
        if (*pc).csize < size {
            (*pc).csize = size;
        }
        (*pc).c = srealloc(
            (*pc).c as *mut libc::c_void,
            ((*pc).csize).wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn penrose_coords_copy(mut pc_in: *mut PenroseCoords) -> *mut PenroseCoords {
    let mut pc_out: *mut PenroseCoords = penrose_coords_new();
    penrose_coords_make_space(pc_out, (*pc_in).nc);
    memcpy(
        (*pc_out).c as *mut libc::c_void,
        (*pc_in).c as *const libc::c_void,
        ((*pc_in).nc).wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    (*pc_out).nc = (*pc_in).nc;
    return pc_out;
}
unsafe extern "C" fn penrosectx_step_recurse(
    mut ctx: *mut PenroseContext,
    mut pc: *mut PenroseCoords,
    mut depth: size_t,
    mut edge: libc::c_uint,
    mut outedge: *mut libc::c_uint,
) {
    let mut tr: TransitionResult = TransitionResult {
        type_0: INTERNAL,
        u: C2RustUnnamed_1 {
            internal: C2RustUnnamed_0 {
                new_child: 0,
                new_edge: 0,
            },
        },
    };
    penrosectx_extend_coords(ctx, pc, depth.wrapping_add(2 as libc::c_int as size_t));
    tr = transition(
        *((*pc).c).offset(depth.wrapping_add(1 as libc::c_int as size_t) as isize),
        *((*pc).c).offset(depth as isize),
        edge,
    );
    if tr.type_0 as libc::c_uint == EXTERNAL as libc::c_int as libc::c_uint {
        let mut parent_outedge: libc::c_uint = 0;
        penrosectx_step_recurse(
            ctx,
            pc,
            depth.wrapping_add(1 as libc::c_int as size_t),
            tr.u.external.parent_edge as libc::c_uint,
            &mut parent_outedge,
        );
        tr = transition_in(
            *((*pc).c).offset(depth.wrapping_add(1 as libc::c_int as size_t) as isize),
            parent_outedge,
            tr.u.external.end as libc::c_int,
        );
    }
    if tr.type_0 as libc::c_uint == INTERNAL as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"tr.type == INTERNAL\0" as *const u8 as *const libc::c_char,
            b"/puzzles/penrose.c\0" as *const u8 as *const libc::c_char,
            426 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 102],
                &[libc::c_char; 102],
            >(
                b"void penrosectx_step_recurse(PenroseContext *, PenroseCoords *, size_t, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4916: {
        if tr.type_0 as libc::c_uint == INTERNAL as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(
                b"tr.type == INTERNAL\0" as *const u8 as *const libc::c_char,
                b"/puzzles/penrose.c\0" as *const u8 as *const libc::c_char,
                426 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 102],
                    &[libc::c_char; 102],
                >(
                    b"void penrosectx_step_recurse(PenroseContext *, PenroseCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *((*pc).c).offset(depth as isize) = tr.u.internal.new_child;
    *outedge = tr.u.internal.new_edge as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn penrosectx_step(
    mut ctx: *mut PenroseContext,
    mut pc: *mut PenroseCoords,
    mut edge: libc::c_uint,
    mut outedge: *mut libc::c_uint,
) {
    let mut dummy_outedge: libc::c_uint = 0;
    if outedge.is_null() {
        outedge = &mut dummy_outedge;
    }
    penrosectx_step_recurse(ctx, pc, 0 as libc::c_int as size_t, edge, outedge);
}
unsafe extern "C" fn penrose_triangle_post_edge(
    mut c: libc::c_char,
    mut edge: libc::c_uint,
) -> Point {
    static mut acute_post_edge: [Point; 3] = [
        {
            let mut init = Point {
                coeffs: [
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                ],
            };
            init
        },
        {
            let mut init = Point {
                coeffs: [
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                ],
            };
            init
        },
        {
            let mut init = Point {
                coeffs: [
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                ],
            };
            init
        },
    ];
    static mut obtuse_post_edge: [Point; 3] = [
        {
            let mut init = Point {
                coeffs: [
                    0 as libc::c_int,
                    -(1 as libc::c_int),
                    1 as libc::c_int,
                    0 as libc::c_int,
                ],
            };
            init
        },
        {
            let mut init = Point {
                coeffs: [
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                ],
            };
            init
        },
        {
            let mut init = Point {
                coeffs: [
                    -(1 as libc::c_int),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    1 as libc::c_int,
                ],
            };
            init
        },
    ];
    match c as libc::c_int {
        65 | 66 | 67 | 68 => return acute_post_edge[edge as usize],
        _ => return obtuse_post_edge[edge as usize],
    };
}
#[no_mangle]
pub unsafe extern "C" fn penrose_place(
    mut tri: *mut PenroseTriangle,
    mut u: Point,
    mut v: Point,
    mut index_of_u: libc::c_int,
) {
    let mut i: libc::c_uint = 0;
    let mut here: Point = u;
    let mut delta: Point = point_sub(v, u);
    i = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        let mut edge: libc::c_uint = (index_of_u as libc::c_uint)
            .wrapping_add(i)
            .wrapping_rem(3 as libc::c_int as libc::c_uint);
        (*tri).vertices[edge as usize] = here;
        here = point_add(here, delta);
        delta = point_mul(
            delta,
            penrose_triangle_post_edge(*((*(*tri).pc).c).offset(0 as libc::c_int as isize), edge),
        );
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn penrose_free(mut tri: *mut PenroseTriangle) {
    penrose_coords_free((*tri).pc);
    sfree(tri as *mut libc::c_void);
}
unsafe extern "C" fn penrose_relative_probability(mut c: libc::c_char) -> bool {
    match c as libc::c_int {
        65 | 66 | 88 | 89 => return 165580141 as libc::c_int != 0,
        67 | 68 | 85 | 86 => return 102334155 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn penrose_choose_random(
    mut possibilities: *const libc::c_char,
    mut rs: *mut random_state,
) -> libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut value: libc::c_ulong = 0;
    let mut limit: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    p = possibilities;
    while *p != 0 {
        limit = limit.wrapping_add(penrose_relative_probability(*p) as libc::c_ulong);
        p = p.offset(1);
        p;
    }
    value = random_upto(rs, limit);
    p = possibilities;
    while *p != 0 {
        let mut curr: libc::c_ulong = penrose_relative_probability(*p) as libc::c_ulong;
        if value < curr {
            return *p;
        }
        value = value.wrapping_sub(curr);
        p = p.offset(1);
        p;
    }
    if 0 as libc::c_int != 0
        && !(b"Probability overflow!\0" as *const u8 as *const libc::c_char).is_null()
    {
    } else {
        __assert_fail(
            b"false && \"Probability overflow!\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/penrose.c\0" as *const u8 as *const libc::c_char,
            512 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"char penrose_choose_random(const char *, random_state *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_7362: {
        if 0 as libc::c_int != 0
            && !(b"Probability overflow!\0" as *const u8 as *const libc::c_char).is_null()
        {
        } else {
            __assert_fail(
                b"false && \"Probability overflow!\"\0" as *const u8 as *const libc::c_char,
                b"/puzzles/penrose.c\0" as *const u8 as *const libc::c_char,
                512 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                    b"char penrose_choose_random(const char *, random_state *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    return *possibilities.offset(0 as libc::c_int as isize);
}
unsafe extern "C" fn penrose_starting_tiles(mut which: libc::c_int) -> *const libc::c_char {
    return if which == PENROSE_P2 as libc::c_int {
        b"ABUV\0" as *const u8 as *const libc::c_char
    } else {
        b"CDXY\0" as *const u8 as *const libc::c_char
    };
}
unsafe extern "C" fn penrose_valid_parents(mut tile: libc::c_char) -> *const libc::c_char {
    match tile as libc::c_int {
        65 => return b"ABV\0" as *const u8 as *const libc::c_char,
        66 => return b"ABU\0" as *const u8 as *const libc::c_char,
        85 => return b"AU\0" as *const u8 as *const libc::c_char,
        86 => return b"BV\0" as *const u8 as *const libc::c_char,
        67 => return b"CX\0" as *const u8 as *const libc::c_char,
        68 => return b"DY\0" as *const u8 as *const libc::c_char,
        88 => return b"DXY\0" as *const u8 as *const libc::c_char,
        89 => return b"CXY\0" as *const u8 as *const libc::c_char,
        _ => return 0 as *const libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn penrosectx_init_random(
    mut ctx: *mut PenroseContext,
    mut rs: *mut random_state,
    mut which: libc::c_int,
) {
    (*ctx).rs = rs;
    (*ctx).must_free_rs = 0 as libc::c_int != 0;
    (*ctx).prototype = penrose_coords_new();
    penrose_coords_make_space((*ctx).prototype, 1 as libc::c_int as size_t);
    *((*(*ctx).prototype).c).offset(0 as libc::c_int as isize) =
        penrose_choose_random(penrose_starting_tiles(which), rs);
    (*(*ctx).prototype).nc = 1 as libc::c_int as size_t;
    (*ctx).start_vertex = random_upto(rs, 3 as libc::c_int as libc::c_ulong) as libc::c_uint;
    (*ctx).orientation = random_upto(rs, 10 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn penrosectx_init_from_params(
    mut ctx: *mut PenroseContext,
    mut ps: *const PenrosePatchParams,
) {
    let mut i: size_t = 0;
    (*ctx).rs = 0 as *mut random_state;
    (*ctx).must_free_rs = 0 as libc::c_int != 0;
    (*ctx).prototype = penrose_coords_new();
    penrose_coords_make_space((*ctx).prototype, (*ps).ncoords);
    i = 0 as libc::c_int as size_t;
    while i < (*ps).ncoords {
        *((*(*ctx).prototype).c).offset(i as isize) = *((*ps).coords).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    (*(*ctx).prototype).nc = (*ps).ncoords;
    (*ctx).start_vertex = (*ps).start_vertex;
    (*ctx).orientation = (*ps).orientation;
}
#[no_mangle]
pub unsafe extern "C" fn penrosectx_cleanup(mut ctx: *mut PenroseContext) {
    if (*ctx).must_free_rs {
        random_free((*ctx).rs);
    }
    penrose_coords_free((*ctx).prototype);
}
#[no_mangle]
pub unsafe extern "C" fn penrosectx_initial_coords(
    mut ctx: *mut PenroseContext,
) -> *mut PenroseCoords {
    return penrose_coords_copy((*ctx).prototype);
}
#[no_mangle]
pub unsafe extern "C" fn penrosectx_extend_coords(
    mut ctx: *mut PenroseContext,
    mut pc: *mut PenroseCoords,
    mut n: size_t,
) {
    if (*(*ctx).prototype).nc < n {
        penrose_coords_make_space((*ctx).prototype, n);
        while (*(*ctx).prototype).nc < n {
            if ((*ctx).rs).is_null() {
                (*ctx).rs = random_new(
                    b"dummy\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
                (*ctx).must_free_rs = 1 as libc::c_int != 0;
            }
            *((*(*ctx).prototype).c).offset((*(*ctx).prototype).nc as isize) =
                penrose_choose_random(
                    penrose_valid_parents(*((*(*ctx).prototype).c).offset(
                        ((*(*ctx).prototype).nc).wrapping_sub(1 as libc::c_int as size_t) as isize,
                    )),
                    (*ctx).rs,
                );
            (*(*ctx).prototype).nc = ((*(*ctx).prototype).nc).wrapping_add(1);
            (*(*ctx).prototype).nc;
        }
    }
    penrose_coords_make_space(pc, n);
    while (*pc).nc < n {
        *((*pc).c).offset((*pc).nc as isize) = *((*(*ctx).prototype).c).offset((*pc).nc as isize);
        (*pc).nc = ((*pc).nc).wrapping_add(1);
        (*pc).nc;
    }
}
unsafe extern "C" fn penrose_triangle_edge_0_length(mut c: libc::c_char) -> Point {
    static mut one: Point = {
        let mut init = Point {
            coeffs: [
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ],
        };
        init
    };
    static mut phi: Point = {
        let mut init = Point {
            coeffs: [
                1 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
                -(1 as libc::c_int),
            ],
        };
        init
    };
    static mut invphi: Point = {
        let mut init = Point {
            coeffs: [
                0 as libc::c_int,
                0 as libc::c_int,
                1 as libc::c_int,
                -(1 as libc::c_int),
            ],
        };
        init
    };
    match c as libc::c_int {
        65 | 66 => return invphi,
        85 | 86 => return one,
        67 | 68 => return invphi,
        _ => return phi,
    };
}
#[no_mangle]
pub unsafe extern "C" fn penrose_initial(mut ctx: *mut PenroseContext) -> *mut PenroseTriangle {
    let mut type_0: libc::c_char = *((*(*ctx).prototype).c).offset(0 as libc::c_int as isize);
    let mut origin: Point = {
        let mut init = Point {
            coeffs: [
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            ],
        };
        init
    };
    let mut edge0: Point = penrose_triangle_edge_0_length(type_0);
    let mut negoffset: Point = Point { coeffs: [0; 4] };
    let mut i: size_t = 0;
    let mut tri: *mut PenroseTriangle =
        smalloc(::core::mem::size_of::<PenroseTriangle>() as libc::c_ulong) as *mut PenroseTriangle;
    edge0 = point_mul(edge0, point_rot((*ctx).orientation));
    (*tri).pc = penrose_coords_copy((*ctx).prototype);
    penrose_place(tri, origin, edge0, 0 as libc::c_int);
    negoffset = (*tri).vertices[(*ctx).start_vertex as usize];
    i = 0 as libc::c_int as size_t;
    while i < 3 as libc::c_int as size_t {
        (*tri).vertices[i as usize] = point_sub((*tri).vertices[i as usize], negoffset);
        i = i.wrapping_add(1);
        i;
    }
    return tri;
}
#[no_mangle]
pub unsafe extern "C" fn penrose_adjacent(
    mut ctx: *mut PenroseContext,
    mut src_tri: *const PenroseTriangle,
    mut src_edge: libc::c_uint,
    mut dst_edge_out: *mut libc::c_uint,
) -> *mut PenroseTriangle {
    let mut dst_edge: libc::c_uint = 0;
    let mut dst_tri: *mut PenroseTriangle =
        smalloc(::core::mem::size_of::<PenroseTriangle>() as libc::c_ulong) as *mut PenroseTriangle;
    (*dst_tri).pc = penrose_coords_copy((*src_tri).pc);
    penrosectx_step(ctx, (*dst_tri).pc, src_edge, &mut dst_edge);
    penrose_place(
        dst_tri,
        (*src_tri).vertices[src_edge
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(3 as libc::c_int as libc::c_uint) as usize],
        (*src_tri).vertices[src_edge as usize],
        dst_edge as libc::c_int,
    );
    if !dst_edge_out.is_null() {
        *dst_edge_out = dst_edge;
    }
    return dst_tri;
}
unsafe extern "C" fn penrose_cmp(
    mut av: *mut libc::c_void,
    mut bv: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut PenroseTriangle = av as *mut PenroseTriangle;
    let mut b: *mut PenroseTriangle = bv as *mut PenroseTriangle;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 2 as libc::c_int as size_t {
        j = 0 as libc::c_int as size_t;
        while j < 4 as libc::c_int as size_t {
            let mut ac: libc::c_int = (*a).vertices[i as usize].coeffs[j as usize];
            let mut bc: libc::c_int = (*b).vertices[i as usize].coeffs[j as usize];
            if ac < bc {
                return -(1 as libc::c_int);
            }
            if ac > bc {
                return 1 as libc::c_int;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn penrose_sibling_edge_index(mut c: libc::c_char) -> libc::c_uint {
    match c as libc::c_int {
        65 | 85 => return 2 as libc::c_int as libc::c_uint,
        66 | 86 => return 1 as libc::c_int as libc::c_uint,
        _ => return 0 as libc::c_int as libc::c_uint,
    };
}
#[no_mangle]
pub unsafe extern "C" fn penrosectx_generate(
    mut ctx: *mut PenroseContext,
    mut inbounds_0: Option<unsafe extern "C" fn(*mut libc::c_void, *const PenroseTriangle) -> bool>,
    mut inboundsctx: *mut libc::c_void,
    mut tile: Option<unsafe extern "C" fn(*mut libc::c_void, *const Point) -> ()>,
    mut tilectx: *mut libc::c_void,
) {
    let mut placed: *mut tree234 = newtree234(Some(
        penrose_cmp as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    ));
    let mut qhead: *mut PenroseTriangle = 0 as *mut PenroseTriangle;
    let mut qtail: *mut PenroseTriangle = 0 as *mut PenroseTriangle;
    let mut tri: *mut PenroseTriangle = penrose_initial(ctx);
    add234(placed, tri as *mut libc::c_void);
    (*tri).next = 0 as *mut PenroseTriangle;
    (*tri).reported = 0 as libc::c_int != 0;
    if inbounds_0.expect("non-null function pointer")(inboundsctx, tri) {
        qtail = tri;
        qhead = qtail;
    }
    while !qhead.is_null() {
        let mut tri_0: *mut PenroseTriangle = qhead;
        let mut edge: libc::c_uint = 0;
        let mut sibling_edge: libc::c_uint =
            penrose_sibling_edge_index(*((*(*tri_0).pc).c).offset(0 as libc::c_int as isize));
        edge = 0 as libc::c_int as libc::c_uint;
        while edge < 3 as libc::c_int as libc::c_uint {
            let mut new_tri: *mut PenroseTriangle = 0 as *mut PenroseTriangle;
            let mut found_tri: *mut PenroseTriangle = 0 as *mut PenroseTriangle;
            new_tri = penrose_adjacent(ctx, tri_0, edge, 0 as *mut libc::c_uint);
            if !inbounds_0.expect("non-null function pointer")(inboundsctx, new_tri) {
                penrose_free(new_tri);
            } else {
                found_tri =
                    find234(placed, new_tri as *mut libc::c_void, None) as *mut PenroseTriangle;
                if !found_tri.is_null() {
                    if edge == sibling_edge && !(*tri_0).reported && !(*found_tri).reported {
                        let mut new_sibling_edge: libc::c_uint = penrose_sibling_edge_index(
                            *((*(*found_tri).pc).c).offset(0 as libc::c_int as isize),
                        );
                        let mut tilevertices: [Point; 4] = [
                            (*tri_0).vertices[sibling_edge
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                .wrapping_rem(3 as libc::c_int as libc::c_uint)
                                as usize],
                            (*tri_0).vertices[sibling_edge
                                .wrapping_add(2 as libc::c_int as libc::c_uint)
                                .wrapping_rem(3 as libc::c_int as libc::c_uint)
                                as usize],
                            (*found_tri).vertices[new_sibling_edge
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                .wrapping_rem(3 as libc::c_int as libc::c_uint)
                                as usize],
                            (*found_tri).vertices[new_sibling_edge
                                .wrapping_add(2 as libc::c_int as libc::c_uint)
                                .wrapping_rem(3 as libc::c_int as libc::c_uint)
                                as usize],
                        ];
                        tile.expect("non-null function pointer")(
                            tilectx,
                            tilevertices.as_mut_ptr(),
                        );
                        (*tri_0).reported = 1 as libc::c_int != 0;
                        (*found_tri).reported = 1 as libc::c_int != 0;
                    }
                    penrose_free(new_tri);
                } else {
                    add234(placed, new_tri as *mut libc::c_void);
                    (*qtail).next = new_tri;
                    qtail = new_tri;
                    (*new_tri).next = 0 as *mut PenroseTriangle;
                    (*new_tri).reported = 0 as libc::c_int != 0;
                }
            }
            edge = edge.wrapping_add(1);
            edge;
        }
        qhead = (*qhead).next;
    }
    let mut tri_1: *mut PenroseTriangle = 0 as *mut PenroseTriangle;
    loop {
        tri_1 = delpos234(placed, 0 as libc::c_int) as *mut PenroseTriangle;
        if tri_1.is_null() {
            break;
        }
        penrose_free(tri_1);
    }
    freetree234(placed);
}
#[no_mangle]
pub unsafe extern "C" fn penrose_tiling_params_invalid(
    mut params: *const PenrosePatchParams,
    mut which: libc::c_int,
) -> *const libc::c_char {
    let mut i: size_t = 0;
    if (*params).ncoords == 0 as libc::c_int as size_t {
        return b"expected at least one coordinate\0" as *const u8 as *const libc::c_char;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*params).ncoords {
        if !penrose_valid_letter(*((*params).coords).offset(i as isize), which) {
            return b"invalid coordinate letter\0" as *const u8 as *const libc::c_char;
        }
        if i > 0 as libc::c_int as size_t
            && (strchr(
                penrose_valid_parents(
                    *((*params).coords).offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize),
                ),
                *((*params).coords).offset(i as isize) as libc::c_int,
            ))
            .is_null()
        {
            return b"invalid pair of consecutive coordinates\0" as *const u8
                as *const libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn inbounds(
    mut vctx: *mut libc::c_void,
    mut tri: *const PenroseTriangle,
) -> bool {
    let mut octx: *mut PenroseOutputCtx = vctx as *mut PenroseOutputCtx;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < 3 as libc::c_int as size_t {
        let mut x: Coord = point_x((*tri).vertices[i as usize]);
        let mut y: Coord = point_y((*tri).vertices[i as usize]);
        if coord_cmp(x, (*octx).xmin) < 0 as libc::c_int
            || coord_cmp(x, (*octx).xmax) > 0 as libc::c_int
            || coord_cmp(y, (*octx).ymin) < 0 as libc::c_int
            || coord_cmp(y, (*octx).ymax) > 0 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn null_output_tile(mut vctx: *mut libc::c_void, mut vertices: *const Point) {}
unsafe extern "C" fn really_output_tile(mut vctx: *mut libc::c_void, mut vertices: *const Point) {
    let mut octx: *mut PenroseOutputCtx = vctx as *mut PenroseOutputCtx;
    let mut i: size_t = 0;
    let mut coords: [libc::c_int; 16] = [0; 16];
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as size_t {
        let mut x: Coord = point_x(*vertices.offset(i as isize));
        let mut y: Coord = point_y(*vertices.offset(i as isize));
        coords
            [(4 as libc::c_int as size_t * i).wrapping_add(0 as libc::c_int as size_t) as usize] =
            x.c1 + (*octx).xoff;
        coords
            [(4 as libc::c_int as size_t * i).wrapping_add(1 as libc::c_int as size_t) as usize] =
            x.cr5;
        coords
            [(4 as libc::c_int as size_t * i).wrapping_add(2 as libc::c_int as size_t) as usize] =
            y.c1 + (*octx).yoff;
        coords
            [(4 as libc::c_int as size_t * i).wrapping_add(3 as libc::c_int as size_t) as usize] =
            y.cr5;
        i = i.wrapping_add(1);
        i;
    }
    ((*octx).external_cb).expect("non-null function pointer")(
        (*octx).external_cbctx,
        coords.as_mut_ptr(),
    );
}
unsafe extern "C" fn penrose_set_bounds(
    mut octx: *mut PenroseOutputCtx,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    (*octx).xoff = w / 2 as libc::c_int;
    (*octx).yoff = h / 2 as libc::c_int;
    (*octx).xmin.c1 = -(*octx).xoff;
    (*octx).xmax.c1 = -(*octx).xoff + w;
    (*octx).ymin.c1 = (*octx).yoff - h;
    (*octx).ymax.c1 = (*octx).yoff;
    (*octx).xmin.cr5 = 0 as libc::c_int;
    (*octx).xmax.cr5 = 0 as libc::c_int;
    (*octx).ymin.cr5 = 0 as libc::c_int;
    (*octx).ymax.cr5 = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn penrose_tiling_randomise(
    mut params: *mut PenrosePatchParams,
    mut which: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut rs: *mut random_state,
) {
    let mut ctx: [PenroseContext; 1] = [PenroseContext {
        rs: 0 as *mut random_state,
        must_free_rs: false,
        start_vertex: 0,
        orientation: 0,
        prototype: 0 as *mut PenroseCoords,
    }; 1];
    let mut octx: [PenroseOutputCtx; 1] = [PenroseOutputCtx {
        xoff: 0,
        yoff: 0,
        xmin: Coord { c1: 0, cr5: 0 },
        xmax: Coord { c1: 0, cr5: 0 },
        ymin: Coord { c1: 0, cr5: 0 },
        ymax: Coord { c1: 0, cr5: 0 },
        external_cb: None,
        external_cbctx: 0 as *mut libc::c_void,
    }; 1];
    penrose_set_bounds(octx.as_mut_ptr(), w, h);
    penrosectx_init_random(ctx.as_mut_ptr(), rs, which);
    penrosectx_generate(
        ctx.as_mut_ptr(),
        Some(inbounds as unsafe extern "C" fn(*mut libc::c_void, *const PenroseTriangle) -> bool),
        octx.as_mut_ptr() as *mut libc::c_void,
        Some(null_output_tile as unsafe extern "C" fn(*mut libc::c_void, *const Point) -> ()),
        0 as *mut libc::c_void,
    );
    (*params).orientation = (*ctx.as_mut_ptr()).orientation;
    (*params).start_vertex = (*ctx.as_mut_ptr()).start_vertex;
    (*params).ncoords = (*(*ctx.as_mut_ptr()).prototype).nc;
    (*params).coords = smalloc(
        ((*params).ncoords).wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*params).coords as *mut libc::c_void,
        (*(*ctx.as_mut_ptr()).prototype).c as *const libc::c_void,
        (*params).ncoords,
    );
    penrosectx_cleanup(ctx.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn penrose_tiling_generate(
    mut params: *const PenrosePatchParams,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut cb: penrose_tile_callback_fn,
    mut cbctx: *mut libc::c_void,
) {
    let mut ctx: [PenroseContext; 1] = [PenroseContext {
        rs: 0 as *mut random_state,
        must_free_rs: false,
        start_vertex: 0,
        orientation: 0,
        prototype: 0 as *mut PenroseCoords,
    }; 1];
    let mut octx: [PenroseOutputCtx; 1] = [PenroseOutputCtx {
        xoff: 0,
        yoff: 0,
        xmin: Coord { c1: 0, cr5: 0 },
        xmax: Coord { c1: 0, cr5: 0 },
        ymin: Coord { c1: 0, cr5: 0 },
        ymax: Coord { c1: 0, cr5: 0 },
        external_cb: None,
        external_cbctx: 0 as *mut libc::c_void,
    }; 1];
    penrose_set_bounds(octx.as_mut_ptr(), w, h);
    let ref mut fresh1 = (*octx.as_mut_ptr()).external_cb;
    *fresh1 = cb;
    let ref mut fresh2 = (*octx.as_mut_ptr()).external_cbctx;
    *fresh2 = cbctx;
    penrosectx_init_from_params(ctx.as_mut_ptr(), params);
    penrosectx_generate(
        ctx.as_mut_ptr(),
        Some(inbounds as unsafe extern "C" fn(*mut libc::c_void, *const PenroseTriangle) -> bool),
        octx.as_mut_ptr() as *mut libc::c_void,
        Some(really_output_tile as unsafe extern "C" fn(*mut libc::c_void, *const Point) -> ()),
        octx.as_mut_ptr() as *mut libc::c_void,
    );
    penrosectx_cleanup(ctx.as_mut_ptr());
}
