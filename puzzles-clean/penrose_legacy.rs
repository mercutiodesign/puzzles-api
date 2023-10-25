#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub c: libc::c_int,
    pub d: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct penrose_legacy_state {
    pub start_size: libc::c_int,
    pub max_depth: libc::c_int,
    pub new_tile: tile_callback,
    pub ctx: *mut libc::c_void,
}
pub type tile_callback = Option::<
    unsafe extern "C" fn(
        *mut penrose_legacy_state,
        *mut vector,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type C2RustUnnamed = libc::c_uint;
pub const PENROSE_P3: C2RustUnnamed = 1;
pub const PENROSE_P2: C2RustUnnamed = 0;
#[inline(always)]
unsafe extern "C" fn __tg_pow(
    mut __x: libc::c_double,
    mut __y: libc::c_double,
) -> libc::c_double {
    return pow(__x, __y);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_double) -> libc::c_double {
    return sqrt(__x);
}
unsafe extern "C" fn v_origin() -> vector {
    let mut v: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    v.d = 0 as libc::c_int;
    v.c = v.d;
    v.b = v.c;
    v.a = v.b;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn penrose_legacy_vx(
    mut vs: *mut vector,
    mut i: libc::c_int,
) -> libc::c_double {
    return ((*vs.offset(i as isize)).a + (*vs.offset(i as isize)).d) as libc::c_double
        * 0.5877852f64
        + ((*vs.offset(i as isize)).b + (*vs.offset(i as isize)).c) as libc::c_double
            * 0.9510565f64;
}
#[no_mangle]
pub unsafe extern "C" fn penrose_legacy_vy(
    mut vs: *mut vector,
    mut i: libc::c_int,
) -> libc::c_double {
    return ((*vs.offset(i as isize)).a - (*vs.offset(i as isize)).d) as libc::c_double
        * 0.8090169f64
        + ((*vs.offset(i as isize)).b - (*vs.offset(i as isize)).c) as libc::c_double
            * 0.3090169f64;
}
unsafe extern "C" fn v_trans(mut v: vector, mut trans: vector) -> vector {
    v.a += trans.a;
    v.b += trans.b;
    v.c += trans.c;
    v.d += trans.d;
    return v;
}
unsafe extern "C" fn v_rotate_36(mut v: vector) -> vector {
    let mut vv: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    vv.a = -v.d;
    vv.b = v.d + v.a;
    vv.c = -v.d + v.b;
    vv.d = v.d + v.c;
    return vv;
}
unsafe extern "C" fn v_rotate(mut v: vector, mut ang: libc::c_int) -> vector {
    let mut i: libc::c_int = 0;
    if ang % 36 as libc::c_int == 0 as libc::c_int {} else {
        __assert_fail(
            b"(ang % 36) == 0\0" as *const u8 as *const libc::c_char,
            b"/puzzles/penrose-legacy.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"vector v_rotate(vector, int)\0"))
                .as_ptr(),
        );
    }
    'c_7496: {
        if ang % 36 as libc::c_int == 0 as libc::c_int {} else {
            __assert_fail(
                b"(ang % 36) == 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/penrose-legacy.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"vector v_rotate(vector, int)\0"))
                    .as_ptr(),
            );
        }
    };
    while ang < 0 as libc::c_int {
        ang += 360 as libc::c_int;
    }
    ang = 360 as libc::c_int - ang;
    i = 0 as libc::c_int;
    while i < ang / 36 as libc::c_int {
        v = v_rotate_36(v);
        i += 1;
        i;
    }
    return v;
}
unsafe extern "C" fn v_growphi(mut v: vector) -> vector {
    let mut vv: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    vv.a = v.a + v.b - v.d;
    vv.b = v.c + v.d;
    vv.c = v.a + v.b;
    vv.d = v.c + v.d - v.a;
    return vv;
}
unsafe extern "C" fn v_shrinkphi(mut v: vector) -> vector {
    let mut vv: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    vv.a = v.b - v.d;
    vv.b = v.c + v.d - v.b;
    vv.c = v.a + v.b - v.c;
    vv.d = v.c - v.a;
    return vv;
}
unsafe extern "C" fn xform_coord(
    mut vo: vector,
    mut shrink: libc::c_int,
    mut vtrans: vector,
    mut ang: libc::c_int,
) -> vector {
    if shrink < 0 as libc::c_int {
        vo = v_shrinkphi(vo);
    } else if shrink > 0 as libc::c_int {
        vo = v_growphi(vo);
    }
    vo = v_rotate(vo, ang);
    vo = v_trans(vo, vtrans);
    return vo;
}
unsafe extern "C" fn penrose_p2_large(
    mut state: *mut penrose_legacy_state,
    mut depth: libc::c_int,
    mut flip: libc::c_int,
    mut v_orig: vector,
    mut v_edge: vector,
) -> libc::c_int {
    let mut vv_orig: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    let mut vv_edge: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    if flip > 0 as libc::c_int {
        let mut vs: [vector; 4] = [vector { a: 0, b: 0, c: 0, d: 0 }; 4];
        vs[0 as libc::c_int as usize] = v_orig;
        vs[1 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            -(36 as libc::c_int),
        );
        vs[2 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            0 as libc::c_int,
        );
        vs[3 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            36 as libc::c_int,
        );
        ((*state).new_tile)
            .expect(
                "non-null function pointer",
            )(state, vs.as_mut_ptr(), 4 as libc::c_int, depth);
    }
    if depth >= (*state).max_depth {
        return 0 as libc::c_int;
    }
    vv_orig = v_trans(v_orig, v_rotate(v_edge, -(36 as libc::c_int) * flip));
    vv_edge = v_rotate(v_edge, 108 as libc::c_int * flip);
    penrose_p2_small(state, depth + 1 as libc::c_int, flip, v_orig, v_shrinkphi(v_edge));
    penrose_p2_large(
        state,
        depth + 1 as libc::c_int,
        flip,
        vv_orig,
        v_shrinkphi(vv_edge),
    );
    penrose_p2_large(
        state,
        depth + 1 as libc::c_int,
        -flip,
        vv_orig,
        v_shrinkphi(vv_edge),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn penrose_p2_small(
    mut state: *mut penrose_legacy_state,
    mut depth: libc::c_int,
    mut flip: libc::c_int,
    mut v_orig: vector,
    mut v_edge: vector,
) -> libc::c_int {
    let mut vv_orig: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    if flip > 0 as libc::c_int {
        let mut vs: [vector; 4] = [vector { a: 0, b: 0, c: 0, d: 0 }; 4];
        vs[0 as libc::c_int as usize] = v_orig;
        vs[1 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            -(72 as libc::c_int),
        );
        vs[2 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            -(1 as libc::c_int),
            vs[0 as libc::c_int as usize],
            -(36 as libc::c_int),
        );
        vs[3 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            0 as libc::c_int,
        );
        ((*state).new_tile)
            .expect(
                "non-null function pointer",
            )(state, vs.as_mut_ptr(), 4 as libc::c_int, depth);
    }
    if depth >= (*state).max_depth {
        return 0 as libc::c_int;
    }
    vv_orig = v_trans(v_orig, v_edge);
    penrose_p2_large(
        state,
        depth + 1 as libc::c_int,
        -flip,
        v_orig,
        v_shrinkphi(v_rotate(v_edge, -(36 as libc::c_int) * flip)),
    );
    penrose_p2_small(
        state,
        depth + 1 as libc::c_int,
        flip,
        vv_orig,
        v_shrinkphi(v_rotate(v_edge, -(144 as libc::c_int) * flip)),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn penrose_p3_large(
    mut state: *mut penrose_legacy_state,
    mut depth: libc::c_int,
    mut flip: libc::c_int,
    mut v_orig: vector,
    mut v_edge: vector,
) -> libc::c_int {
    let mut vv_orig: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    if flip > 0 as libc::c_int {
        let mut vs: [vector; 4] = [vector { a: 0, b: 0, c: 0, d: 0 }; 4];
        vs[0 as libc::c_int as usize] = v_orig;
        vs[1 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            -(36 as libc::c_int),
        );
        vs[2 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            1 as libc::c_int,
            vs[0 as libc::c_int as usize],
            0 as libc::c_int,
        );
        vs[3 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            36 as libc::c_int,
        );
        ((*state).new_tile)
            .expect(
                "non-null function pointer",
            )(state, vs.as_mut_ptr(), 4 as libc::c_int, depth);
    }
    if depth >= (*state).max_depth {
        return 0 as libc::c_int;
    }
    vv_orig = v_trans(v_orig, v_edge);
    penrose_p3_large(
        state,
        depth + 1 as libc::c_int,
        -flip,
        vv_orig,
        v_shrinkphi(v_rotate(v_edge, 180 as libc::c_int)),
    );
    penrose_p3_small(
        state,
        depth + 1 as libc::c_int,
        flip,
        vv_orig,
        v_shrinkphi(v_rotate(v_edge, -(108 as libc::c_int) * flip)),
    );
    vv_orig = v_trans(v_orig, v_growphi(v_edge));
    penrose_p3_large(
        state,
        depth + 1 as libc::c_int,
        flip,
        vv_orig,
        v_shrinkphi(v_rotate(v_edge, -(144 as libc::c_int) * flip)),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn penrose_p3_small(
    mut state: *mut penrose_legacy_state,
    mut depth: libc::c_int,
    mut flip: libc::c_int,
    mut v_orig: vector,
    mut v_edge: vector,
) -> libc::c_int {
    let mut vv_orig: vector = vector { a: 0, b: 0, c: 0, d: 0 };
    if flip > 0 as libc::c_int {
        let mut vs: [vector; 4] = [vector { a: 0, b: 0, c: 0, d: 0 }; 4];
        vs[0 as libc::c_int as usize] = v_orig;
        vs[1 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            -(36 as libc::c_int),
        );
        vs[3 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[0 as libc::c_int as usize],
            0 as libc::c_int,
        );
        vs[2 as libc::c_int
            as usize] = xform_coord(
            v_edge,
            0 as libc::c_int,
            vs[3 as libc::c_int as usize],
            -(36 as libc::c_int),
        );
        ((*state).new_tile)
            .expect(
                "non-null function pointer",
            )(state, vs.as_mut_ptr(), 4 as libc::c_int, depth);
    }
    if depth >= (*state).max_depth {
        return 0 as libc::c_int;
    }
    vv_orig = v_trans(v_orig, v_edge);
    penrose_p3_large(
        state,
        depth + 1 as libc::c_int,
        -flip,
        vv_orig,
        v_shrinkphi(v_rotate(v_edge, 180 as libc::c_int)),
    );
    penrose_p3_small(
        state,
        depth + 1 as libc::c_int,
        flip,
        vv_orig,
        v_shrinkphi(v_rotate(v_edge, -(108 as libc::c_int) * flip)),
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn penrose_legacy_side_length(
    mut start_size: libc::c_double,
    mut depth: libc::c_int,
) -> libc::c_double {
    return start_size / __tg_pow(1.6180339887f64, depth as libc::c_double);
}
#[no_mangle]
pub unsafe extern "C" fn penrose_legacy(
    mut state: *mut penrose_legacy_state,
    mut which: libc::c_int,
    mut angle: libc::c_int,
) -> libc::c_int {
    let mut vo: vector = v_origin();
    let mut vb: vector = v_origin();
    vo.c = -(*state).start_size;
    vo.b = vo.c;
    vo = v_shrinkphi(v_shrinkphi(vo));
    vb.b = (*state).start_size;
    vo = v_rotate(vo, angle);
    vb = v_rotate(vb, angle);
    if which == PENROSE_P2 as libc::c_int {
        return penrose_p2_large(state, 0 as libc::c_int, 1 as libc::c_int, vo, vb)
    } else {
        return penrose_p3_small(state, 0 as libc::c_int, 1 as libc::c_int, vo, vb)
    };
}
#[no_mangle]
pub unsafe extern "C" fn penrose_legacy_calculate_size(
    mut which: libc::c_int,
    mut tilesize: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut required_radius: *mut libc::c_double,
    mut start_size: *mut libc::c_int,
    mut depth: *mut libc::c_int,
) {
    let mut rradius: libc::c_double = 0.;
    let mut size: libc::c_double = 0.;
    let mut n: libc::c_int = 0 as libc::c_int;
    if which == PENROSE_P2 as libc::c_int {
        tilesize = tilesize * 3 as libc::c_int / 2 as libc::c_int;
    } else {
        tilesize = tilesize * 5 as libc::c_int / 4 as libc::c_int;
    }
    rradius = tilesize as libc::c_double * 3.11f64
        * __tg_sqrt((w * w + h * h) as libc::c_double);
    size = tilesize as libc::c_double;
    while size * 0.22426f64 < rradius {
        n += 1;
        n;
        size = size * 1.6180339887f64;
    }
    *start_size = size as libc::c_int;
    *depth = n;
    *required_radius = rradius;
}
