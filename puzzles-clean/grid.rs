#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type random_state;
    pub type DSF;
    pub type tree234_Tag;
    pub type vector;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn srealloc(p: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn dupstr(s: *const libc::c_char) -> *mut libc::c_char;
    fn n_times_root_k(n: libc::c_int, k: libc::c_int) -> libc::c_int;
    fn dsf_new(size: libc::c_int) -> *mut DSF;
    fn dsf_free(dsf: *mut DSF);
    fn dsf_canonify(dsf: *mut DSF, n: libc::c_int) -> libc::c_int;
    fn dsf_size(dsf: *mut DSF, n: libc::c_int) -> libc::c_int;
    fn dsf_merge(dsf: *mut DSF, n1: libc::c_int, n2: libc::c_int);
    fn newtree234(cmp: cmpfn234) -> *mut tree234;
    fn freetree234(t: *mut tree234);
    fn add234(t: *mut tree234, e: *mut libc::c_void) -> *mut libc::c_void;
    fn find234(
        t: *mut tree234,
        e: *mut libc::c_void,
        cmp: cmpfn234,
    ) -> *mut libc::c_void;
    fn del234(t: *mut tree234, e: *mut libc::c_void) -> *mut libc::c_void;
    fn penrose_legacy_vx(vs: *mut vector, i: libc::c_int) -> libc::c_double;
    fn penrose_legacy_vy(vs: *mut vector, i: libc::c_int) -> libc::c_double;
    fn penrose_legacy(
        state: *mut penrose_legacy_state,
        which: libc::c_int,
        angle: libc::c_int,
    ) -> libc::c_int;
    fn penrose_legacy_calculate_size(
        which: libc::c_int,
        tilesize: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        required_radius: *mut libc::c_double,
        start_size: *mut libc::c_int,
        depth: *mut libc::c_int,
    );
    fn penrose_valid_letter(c: libc::c_char, which: libc::c_int) -> bool;
    fn penrose_tiling_randomise(
        params: *mut PenrosePatchParams,
        which: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        rs: *mut random_state,
    );
    fn penrose_tiling_params_invalid(
        params: *const PenrosePatchParams,
        which: libc::c_int,
    ) -> *const libc::c_char;
    fn penrose_tiling_generate(
        params: *const PenrosePatchParams,
        w: libc::c_int,
        h: libc::c_int,
        cb: penrose_tile_callback_fn,
        cbctx: *mut libc::c_void,
    );
    fn hat_tiling_randomise(
        params: *mut HatPatchParams,
        w: libc::c_int,
        h: libc::c_int,
        rs: *mut random_state,
    );
    fn hat_tiling_params_invalid(params: *const HatPatchParams) -> *const libc::c_char;
    fn hat_tiling_generate(
        params: *const HatPatchParams,
        w: libc::c_int,
        h: libc::c_int,
        cb: hat_tile_callback_fn,
        cbctx: *mut libc::c_void,
    );
    fn spectre_tiling_randomise(
        params: *mut SpectrePatchParams,
        w: libc::c_int,
        h: libc::c_int,
        rs: *mut random_state,
    );
    fn spectre_tiling_params_invalid(
        params: *const SpectrePatchParams,
    ) -> *const libc::c_char;
    fn spectre_tiling_generate(
        params: *const SpectrePatchParams,
        w: libc::c_int,
        h: libc::c_int,
        cb: spectre_tile_callback_fn,
        cbctx: *mut libc::c_void,
    );
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type tree234 = tree234_Tag;
pub type cmpfn234 = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_face {
    pub index: libc::c_int,
    pub order: libc::c_int,
    pub edges: *mut *mut grid_edge,
    pub dots: *mut *mut grid_dot,
    pub has_incentre: bool,
    pub ix: libc::c_int,
    pub iy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_dot {
    pub index: libc::c_int,
    pub order: libc::c_int,
    pub edges: *mut *mut grid_edge,
    pub faces: *mut *mut grid_face,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_edge {
    pub dot1: *mut grid_dot,
    pub dot2: *mut grid_dot,
    pub face1: *mut grid_face,
    pub face2: *mut grid_face,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid {
    pub num_faces: libc::c_int,
    pub size_faces: libc::c_int,
    pub faces: *mut *mut grid_face,
    pub num_edges: libc::c_int,
    pub size_edges: libc::c_int,
    pub edges: *mut *mut grid_edge,
    pub num_dots: libc::c_int,
    pub size_dots: libc::c_int,
    pub dots: *mut *mut grid_dot,
    pub lowest_x: libc::c_int,
    pub lowest_y: libc::c_int,
    pub highest_x: libc::c_int,
    pub highest_y: libc::c_int,
    pub tilesize: libc::c_int,
    pub refcount: libc::c_int,
}
pub type grid_type = libc::c_uint;
pub const GRID_TYPE_MAX: grid_type = 18;
pub const GRID_SPECTRES: grid_type = 17;
pub const GRID_HATS: grid_type = 16;
pub const GRID_PENROSE_P3: grid_type = 15;
pub const GRID_PENROSE_P2: grid_type = 14;
pub const GRID_COMPASSDODECAGONAL: grid_type = 13;
pub const GRID_GREATGREATDODECAGONAL: grid_type = 12;
pub const GRID_GREATDODECAGONAL: grid_type = 11;
pub const GRID_DODECAGONAL: grid_type = 10;
pub const GRID_FLORET: grid_type = 9;
pub const GRID_KITE: grid_type = 8;
pub const GRID_OCTAGONAL: grid_type = 7;
pub const GRID_KAGOME: grid_type = 6;
pub const GRID_GREATHEXAGONAL: grid_type = 5;
pub const GRID_CAIRO: grid_type = 4;
pub const GRID_SNUBSQUARE: grid_type = 3;
pub const GRID_TRIANGULAR: grid_type = 2;
pub const GRID_HONEYCOMB: grid_type = 1;
pub const GRID_SQUARE: grid_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpectrePatchParams {
    pub orientation: libc::c_int,
    pub ncoords: size_t,
    pub coords: *mut libc::c_uchar,
    pub final_hex: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HatPatchParams {
    pub ncoords: size_t,
    pub coords: *mut libc::c_uchar,
    pub final_metatile: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PenrosePatchParams {
    pub start_vertex: libc::c_uint,
    pub orientation: libc::c_int,
    pub ncoords: size_t,
    pub coords: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct size {
    pub w: libc::c_int,
    pub h: libc::c_int,
}
pub const PENROSE_P3: C2RustUnnamed_0 = 1;
pub const PENROSE_P2: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct penrose_legacy_set_faces_ctx {
    pub xmin: libc::c_int,
    pub xmax: libc::c_int,
    pub ymin: libc::c_int,
    pub ymax: libc::c_int,
    pub g: *mut grid,
    pub points: *mut tree234,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spectrecontext {
    pub g: *mut grid,
    pub points: *mut tree234,
}
pub type spectre_tile_callback_fn = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hatcontext {
    pub g: *mut grid,
    pub points: *mut tree234,
}
pub type hat_tile_callback_fn = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct penrosecontext {
    pub g: *mut grid,
    pub points: *mut tree234,
    pub xunit: libc::c_int,
    pub yunit: libc::c_int,
}
pub type penrose_tile_callback_fn = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_int) -> (),
>;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_double) -> libc::c_double {
    return sqrt(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_fabs(mut __x: libc::c_double) -> libc::c_double {
    return fabs(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_ceil(mut __x: libc::c_double) -> libc::c_double {
    return ceil(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_floor(mut __x: libc::c_double) -> libc::c_double {
    return floor(__x);
}
#[no_mangle]
pub unsafe extern "C" fn grid_free(mut g: *mut grid) {
    if (*g).refcount != 0 {} else {
        __assert_fail(
            b"g->refcount\0" as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void grid_free(grid *)\0"))
                .as_ptr(),
        );
    }
    'c_10850: {
        if (*g).refcount != 0 {} else {
            __assert_fail(
                b"g->refcount\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void grid_free(grid *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*g).refcount -= 1;
    (*g).refcount;
    if (*g).refcount == 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*g).num_faces {
            sfree((**((*g).faces).offset(i as isize)).dots as *mut libc::c_void);
            sfree((**((*g).faces).offset(i as isize)).edges as *mut libc::c_void);
            sfree(*((*g).faces).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < (*g).num_dots {
            sfree((**((*g).dots).offset(i as isize)).faces as *mut libc::c_void);
            sfree((**((*g).dots).offset(i as isize)).edges as *mut libc::c_void);
            sfree(*((*g).dots).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < (*g).num_edges {
            sfree(*((*g).edges).offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        sfree((*g).faces as *mut libc::c_void);
        sfree((*g).edges as *mut libc::c_void);
        sfree((*g).dots as *mut libc::c_void);
        sfree(g as *mut libc::c_void);
    }
}
unsafe extern "C" fn grid_empty() -> *mut grid {
    let mut g: *mut grid = smalloc(::core::mem::size_of::<grid>() as libc::c_ulong)
        as *mut grid;
    (*g).faces = 0 as *mut *mut grid_face;
    (*g).edges = 0 as *mut *mut grid_edge;
    (*g).dots = 0 as *mut *mut grid_dot;
    (*g).num_dots = 0 as libc::c_int;
    (*g).num_edges = (*g).num_dots;
    (*g).num_faces = (*g).num_edges;
    (*g).size_dots = 0 as libc::c_int;
    (*g).size_edges = (*g).size_dots;
    (*g).size_faces = (*g).size_edges;
    (*g).refcount = 1 as libc::c_int;
    (*g).highest_y = 0 as libc::c_int;
    (*g).highest_x = (*g).highest_y;
    (*g).lowest_y = (*g).highest_x;
    (*g).lowest_x = (*g).lowest_y;
    return g;
}
unsafe extern "C" fn point_line_distance(
    mut px: libc::c_long,
    mut py: libc::c_long,
    mut ax: libc::c_long,
    mut ay: libc::c_long,
    mut bx: libc::c_long,
    mut by: libc::c_long,
) -> libc::c_double {
    let mut det: libc::c_long = ax * by - bx * ay + bx * py - px * by + px * ay
        - ax * py;
    let mut len: libc::c_double = 0.;
    det = if det > -det { det } else { -det };
    len = __tg_sqrt(((ax - bx) * (ax - bx) + (ay - by) * (ay - by)) as libc::c_double);
    return det as libc::c_double / len;
}
#[no_mangle]
pub unsafe extern "C" fn grid_nearest_edge(
    mut g: *mut grid,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> *mut grid_edge {
    let mut best_edge: *mut grid_edge = 0 as *mut grid_edge;
    let mut best_distance: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut i: libc::c_int = 0;
    best_edge = 0 as *mut grid_edge;
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut e: *mut grid_edge = *((*g).edges).offset(i as isize);
        let mut e2: libc::c_long = 0;
        let mut a2: libc::c_long = 0;
        let mut b2: libc::c_long = 0;
        let mut dist: libc::c_double = 0.;
        e2 = ((*(*e).dot1).x as libc::c_long - (*(*e).dot2).x as libc::c_long)
            * ((*(*e).dot1).x as libc::c_long - (*(*e).dot2).x as libc::c_long)
            + ((*(*e).dot1).y as libc::c_long - (*(*e).dot2).y as libc::c_long)
                * ((*(*e).dot1).y as libc::c_long - (*(*e).dot2).y as libc::c_long);
        a2 = ((*(*e).dot1).x as libc::c_long - x as libc::c_long)
            * ((*(*e).dot1).x as libc::c_long - x as libc::c_long)
            + ((*(*e).dot1).y as libc::c_long - y as libc::c_long)
                * ((*(*e).dot1).y as libc::c_long - y as libc::c_long);
        b2 = ((*(*e).dot2).x as libc::c_long - x as libc::c_long)
            * ((*(*e).dot2).x as libc::c_long - x as libc::c_long)
            + ((*(*e).dot2).y as libc::c_long - y as libc::c_long)
                * ((*(*e).dot2).y as libc::c_long - y as libc::c_long);
        if !(a2 >= e2 + b2) {
            if !(b2 >= e2 + a2) {
                dist = point_line_distance(
                    x as libc::c_long,
                    y as libc::c_long,
                    (*(*e).dot1).x as libc::c_long,
                    (*(*e).dot1).y as libc::c_long,
                    (*(*e).dot2).x as libc::c_long,
                    (*(*e).dot2).y as libc::c_long,
                );
                if !(4 as libc::c_int as libc::c_double * (dist * dist)
                    > e2 as libc::c_double)
                {
                    if best_edge.is_null() || dist < best_distance {
                        best_edge = e;
                        best_distance = dist;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    return best_edge;
}
unsafe extern "C" fn grid_debug_basic(mut g: *mut grid) {}
unsafe extern "C" fn grid_debug_derived(mut g: *mut grid) {}
unsafe extern "C" fn grid_edge_bydots_cmpfn(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut grid_edge = v1 as *mut grid_edge;
    let mut b: *mut grid_edge = v2 as *mut grid_edge;
    let mut da: *mut grid_dot = 0 as *mut grid_dot;
    let mut db: *mut grid_dot = 0 as *mut grid_dot;
    da = if (*a).dot1 < (*a).dot2 { (*a).dot1 } else { (*a).dot2 };
    db = if (*b).dot1 < (*b).dot2 { (*b).dot1 } else { (*b).dot2 };
    if (*da).index < (*db).index {
        return -(1 as libc::c_int);
    }
    if (*da).index > (*db).index {
        return 1 as libc::c_int;
    }
    da = if (*a).dot1 < (*a).dot2 { (*a).dot2 } else { (*a).dot1 };
    db = if (*b).dot1 < (*b).dot2 { (*b).dot2 } else { (*b).dot1 };
    if (*da).index < (*db).index {
        return -(1 as libc::c_int);
    }
    if (*da).index > (*db).index {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn grid_trim_vigorously(mut g: *mut grid) {
    let mut dotpairs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut faces: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dots: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dsf: *mut DSF = 0 as *mut DSF;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut newfaces: libc::c_int = 0;
    let mut newdots: libc::c_int = 0;
    dotpairs = smalloc(
        (((*g).num_dots * (*g).num_dots) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        j = 0 as libc::c_int;
        while j < (*g).num_dots {
            *dotpairs.offset((i * (*g).num_dots + j) as isize) = -(1 as libc::c_int);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut dot0: libc::c_int = (**((*f).dots)
            .offset(((*f).order - 1 as libc::c_int) as isize))
            .index;
        j = 0 as libc::c_int;
        while j < (*f).order {
            let mut dot1: libc::c_int = (**((*f).dots).offset(j as isize)).index;
            *dotpairs.offset((dot0 * (*g).num_dots + dot1) as isize) = i;
            dot0 = dot1;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dots = smalloc(
        ((*g).num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        *dots.offset(i as isize) = 1 as libc::c_int;
        j = 0 as libc::c_int;
        while j < (*g).num_dots {
            if (*dotpairs.offset((i * (*g).num_dots + j) as isize) >= 0 as libc::c_int)
                as libc::c_int
                ^ (*dotpairs.offset((j * (*g).num_dots + i) as isize)
                    >= 0 as libc::c_int) as libc::c_int != 0
            {
                *dots.offset(i as isize) = 0 as libc::c_int;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    dsf = dsf_new((*g).num_dots);
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        j = 0 as libc::c_int;
        while j < i {
            if *dots.offset(i as isize) != 0 && *dots.offset(j as isize) != 0
                && *dotpairs.offset((i * (*g).num_dots + j) as isize) >= 0 as libc::c_int
                && *dotpairs.offset((j * (*g).num_dots + i) as isize) >= 0 as libc::c_int
            {
                dsf_merge(dsf, i, j);
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    size = 0 as libc::c_int;
    j = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut newsize: libc::c_int = 0;
        if *dots.offset(i as isize) != 0 && dsf_canonify(dsf, i) == i
            && {
                newsize = dsf_size(dsf, i);
                newsize > size
            }
        {
            j = i;
            size = newsize;
        }
        i += 1;
        i;
    }
    faces = smalloc(
        ((*g).num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        *faces.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        *dots.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f_0: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut keep: bool = 0 as libc::c_int != 0;
        k = 0 as libc::c_int;
        while k < (*f_0).order {
            if dsf_canonify(dsf, (**((*f_0).dots).offset(k as isize)).index) == j {
                keep = 1 as libc::c_int != 0;
            }
            k += 1;
            k;
        }
        if keep {
            *faces.offset(i as isize) = 1 as libc::c_int;
            k = 0 as libc::c_int;
            while k < (*f_0).order {
                *dots
                    .offset(
                        (**((*f_0).dots).offset(k as isize)).index as isize,
                    ) = 1 as libc::c_int;
                k += 1;
                k;
            }
        }
        i += 1;
        i;
    }
    newfaces = 0 as libc::c_int;
    i = newfaces;
    while i < (*g).num_faces {
        let mut f_1: *mut grid_face = *((*g).faces).offset(i as isize);
        if *faces.offset(i as isize) != 0 {
            let fresh0 = newfaces;
            newfaces = newfaces + 1;
            (*f_1).index = fresh0;
            let ref mut fresh1 = *((*g).faces).offset((*f_1).index as isize);
            *fresh1 = f_1;
        } else {
            sfree((*f_1).dots as *mut libc::c_void);
            sfree(f_1 as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    (*g).num_faces = newfaces;
    newdots = 0 as libc::c_int;
    i = newdots;
    while i < (*g).num_dots {
        let mut d: *mut grid_dot = *((*g).dots).offset(i as isize);
        if *dots.offset(i as isize) != 0 {
            let fresh2 = newdots;
            newdots = newdots + 1;
            (*d).index = fresh2;
            let ref mut fresh3 = *((*g).dots).offset((*d).index as isize);
            *fresh3 = d;
        } else {
            sfree((*d).edges as *mut libc::c_void);
            sfree((*d).faces as *mut libc::c_void);
            sfree(d as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    (*g).num_dots = newdots;
    sfree(dotpairs as *mut libc::c_void);
    dsf_free(dsf);
    sfree(dots as *mut libc::c_void);
    sfree(faces as *mut libc::c_void);
}
unsafe extern "C" fn grid_make_consistent(mut g: *mut grid) {
    let mut i: libc::c_int = 0;
    let mut incomplete_edges: *mut tree234 = 0 as *mut tree234;
    grid_debug_basic(g);
    incomplete_edges = newtree234(
        Some(
            grid_edge_bydots_cmpfn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < (*f).order {
            let mut e: grid_edge = grid_edge {
                dot1: 0 as *mut grid_dot,
                dot2: 0 as *mut grid_dot,
                face1: 0 as *mut grid_face,
                face2: 0 as *mut grid_face,
                index: 0,
            };
            let mut edge_found: *mut grid_edge = 0 as *mut grid_edge;
            let mut j2: libc::c_int = j + 1 as libc::c_int;
            if j2 == (*f).order {
                j2 = 0 as libc::c_int;
            }
            e.dot1 = *((*f).dots).offset(j as isize);
            e.dot2 = *((*f).dots).offset(j2 as isize);
            edge_found = del234(
                incomplete_edges,
                &mut e as *mut grid_edge as *mut libc::c_void,
            ) as *mut grid_edge;
            if !edge_found.is_null() {
                (*edge_found).face2 = f;
            } else {
                let mut new_edge: *mut grid_edge = smalloc(
                    ::core::mem::size_of::<grid_edge>() as libc::c_ulong,
                ) as *mut grid_edge;
                (*new_edge).dot1 = e.dot1;
                (*new_edge).dot2 = e.dot2;
                (*new_edge).face1 = f;
                (*new_edge).face2 = 0 as *mut grid_face;
                add234(incomplete_edges, new_edge as *mut libc::c_void);
                if (*g).num_edges >= (*g).size_edges {
                    let mut increment: libc::c_int = (*g).num_edges / 4 as libc::c_int
                        + 128 as libc::c_int;
                    (*g)
                        .size_edges = if increment
                        < 2147483647 as libc::c_int - (*g).num_edges
                    {
                        (*g).num_edges + increment
                    } else {
                        2147483647 as libc::c_int
                    };
                    (*g)
                        .edges = srealloc(
                        (*g).edges as *mut libc::c_void,
                        ((*g).size_edges as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<*mut grid_edge>() as libc::c_ulong,
                            ),
                    ) as *mut *mut grid_edge;
                }
                if (*g).num_edges < 2147483647 as libc::c_int {} else {
                    __assert_fail(
                        b"g->num_edges < INT_MAX\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        568 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"void grid_make_consistent(grid *)\0"))
                            .as_ptr(),
                    );
                }
                'c_12574: {
                    if (*g).num_edges < 2147483647 as libc::c_int {} else {
                        __assert_fail(
                            b"g->num_edges < INT_MAX\0" as *const u8
                                as *const libc::c_char,
                            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                            568 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void grid_make_consistent(grid *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                let fresh4 = (*g).num_edges;
                (*g).num_edges = (*g).num_edges + 1;
                (*new_edge).index = fresh4;
                let ref mut fresh5 = *((*g).edges).offset((*new_edge).index as isize);
                *fresh5 = new_edge;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    freetree234(incomplete_edges);
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f_0: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut j_0: libc::c_int = 0;
        (*f_0)
            .edges = smalloc(
            ((*f_0).order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut grid_edge>() as libc::c_ulong),
        ) as *mut *mut grid_edge;
        j_0 = 0 as libc::c_int;
        while j_0 < (*f_0).order {
            let ref mut fresh6 = *((*f_0).edges).offset(j_0 as isize);
            *fresh6 = 0 as *mut grid_edge;
            j_0 += 1;
            j_0;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut e_0: *mut grid_edge = *((*g).edges).offset(i as isize);
        let mut j_1: libc::c_int = 0;
        j_1 = 0 as libc::c_int;
        while j_1 < 2 as libc::c_int {
            let mut f_1: *mut grid_face = if j_1 != 0 {
                (*e_0).face2
            } else {
                (*e_0).face1
            };
            let mut k: libc::c_int = 0;
            let mut k2: libc::c_int = 0;
            if !f_1.is_null() {
                k = 0 as libc::c_int;
                while k < (*f_1).order {
                    if *((*f_1).dots).offset(k as isize) == (*e_0).dot1 {
                        break;
                    }
                    k += 1;
                    k;
                }
                if k != (*f_1).order {} else {
                    __assert_fail(
                        b"k != f->order\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        606 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"void grid_make_consistent(grid *)\0"))
                            .as_ptr(),
                    );
                }
                'c_12340: {
                    if k != (*f_1).order {} else {
                        __assert_fail(
                            b"k != f->order\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                            606 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void grid_make_consistent(grid *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                k2 = k + 1 as libc::c_int;
                if k2 == (*f_1).order {
                    k2 = 0 as libc::c_int;
                }
                if *((*f_1).dots).offset(k2 as isize) == (*e_0).dot2 {
                    if (*((*f_1).edges).offset(k as isize)).is_null() {} else {
                        __assert_fail(
                            b"f->edges[k] == NULL\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                            631 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void grid_make_consistent(grid *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_12257: {
                        if (*((*f_1).edges).offset(k as isize)).is_null() {} else {
                            __assert_fail(
                                b"f->edges[k] == NULL\0" as *const u8
                                    as *const libc::c_char,
                                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                                631 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 34],
                                    &[libc::c_char; 34],
                                >(b"void grid_make_consistent(grid *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    let ref mut fresh7 = *((*f_1).edges).offset(k as isize);
                    *fresh7 = e_0;
                } else {
                    k2 = k - 1 as libc::c_int;
                    if k2 == -(1 as libc::c_int) {
                        k2 = (*f_1).order - 1 as libc::c_int;
                    }
                    if *((*f_1).dots).offset(k2 as isize) == (*e_0).dot2 {
                        if (*((*f_1).edges).offset(k2 as isize)).is_null() {} else {
                            __assert_fail(
                                b"f->edges[k2] == NULL\0" as *const u8
                                    as *const libc::c_char,
                                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                                641 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 34],
                                    &[libc::c_char; 34],
                                >(b"void grid_make_consistent(grid *)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_12154: {
                            if (*((*f_1).edges).offset(k2 as isize)).is_null() {} else {
                                __assert_fail(
                                    b"f->edges[k2] == NULL\0" as *const u8
                                        as *const libc::c_char,
                                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                                    641 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 34],
                                        &[libc::c_char; 34],
                                    >(b"void grid_make_consistent(grid *)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        let ref mut fresh8 = *((*f_1).edges).offset(k2 as isize);
                        *fresh8 = e_0;
                    } else {
                        if (b"Grid broken: bad edge-face relationship\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                        {} else {
                            __assert_fail(
                                b"!\"Grid broken: bad edge-face relationship\"\0"
                                    as *const u8 as *const libc::c_char,
                                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                                645 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 34],
                                    &[libc::c_char; 34],
                                >(b"void grid_make_consistent(grid *)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_12080: {
                            if (b"Grid broken: bad edge-face relationship\0" as *const u8
                                as *const libc::c_char)
                                .is_null()
                            {} else {
                                __assert_fail(
                                    b"!\"Grid broken: bad edge-face relationship\"\0"
                                        as *const u8 as *const libc::c_char,
                                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                                    645 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 34],
                                        &[libc::c_char; 34],
                                    >(b"void grid_make_consistent(grid *)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                    }
                }
            }
            j_1 += 1;
            j_1;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        (**((*g).dots).offset(i as isize)).order = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut e_1: *mut grid_edge = *((*g).edges).offset(i as isize);
        (*(*e_1).dot1).order += 1;
        (*(*e_1).dot1).order;
        (*(*e_1).dot2).order += 1;
        (*(*e_1).dot2).order;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut d: *mut grid_dot = *((*g).dots).offset(i as isize);
        let mut j_2: libc::c_int = 0;
        if (*d).order >= 2 as libc::c_int {} else {
            __assert_fail(
                b"d->order >= 2\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                668 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void grid_make_consistent(grid *)\0"))
                    .as_ptr(),
            );
        }
        'c_11941: {
            if (*d).order >= 2 as libc::c_int {} else {
                __assert_fail(
                    b"d->order >= 2\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    668 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"void grid_make_consistent(grid *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*d)
            .edges = smalloc(
            ((*d).order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut grid_edge>() as libc::c_ulong),
        ) as *mut *mut grid_edge;
        (*d)
            .faces = smalloc(
            ((*d).order as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut grid_face>() as libc::c_ulong),
        ) as *mut *mut grid_face;
        j_2 = 0 as libc::c_int;
        while j_2 < (*d).order {
            let ref mut fresh9 = *((*d).edges).offset(j_2 as isize);
            *fresh9 = 0 as *mut grid_edge;
            let ref mut fresh10 = *((*d).faces).offset(j_2 as isize);
            *fresh10 = 0 as *mut grid_face;
            j_2 += 1;
            j_2;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f_2: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut j_3: libc::c_int = 0;
        j_3 = 0 as libc::c_int;
        while j_3 < (*f_2).order {
            let mut d_0: *mut grid_dot = *((*f_2).dots).offset(j_3 as isize);
            let ref mut fresh11 = *((*d_0).faces).offset(0 as libc::c_int as isize);
            *fresh11 = f_2;
            j_3 += 1;
            j_3;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut d_1: *mut grid_dot = *((*g).dots).offset(i as isize);
        let mut current_face1: libc::c_int = 0 as libc::c_int;
        let mut current_face2: libc::c_int = 0 as libc::c_int;
        loop {
            let mut f_3: *mut grid_face = *((*d_1).faces).offset(current_face1 as isize);
            let mut e_2: *mut grid_edge = 0 as *mut grid_edge;
            let mut j_4: libc::c_int = 0;
            if !f_3.is_null() {} else {
                __assert_fail(
                    b"f != NULL\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    720 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"void grid_make_consistent(grid *)\0"))
                        .as_ptr(),
                );
            }
            'c_11734: {
                if !f_3.is_null() {} else {
                    __assert_fail(
                        b"f != NULL\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        720 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"void grid_make_consistent(grid *)\0"))
                            .as_ptr(),
                    );
                }
            };
            j_4 = 0 as libc::c_int;
            while j_4 < (*f_3).order {
                if *((*f_3).dots).offset(j_4 as isize) == d_1 {
                    break;
                }
                j_4 += 1;
                j_4;
            }
            if j_4 != (*f_3).order {} else {
                __assert_fail(
                    b"j != f->order\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    726 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"void grid_make_consistent(grid *)\0"))
                        .as_ptr(),
                );
            }
            'c_11665: {
                if j_4 != (*f_3).order {} else {
                    __assert_fail(
                        b"j != f->order\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        726 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"void grid_make_consistent(grid *)\0"))
                            .as_ptr(),
                    );
                }
            };
            j_4 -= 1;
            j_4;
            if j_4 == -(1 as libc::c_int) {
                j_4 = (*f_3).order - 1 as libc::c_int;
            }
            e_2 = *((*f_3).edges).offset(j_4 as isize);
            let ref mut fresh12 = *((*d_1).edges).offset(current_face1 as isize);
            *fresh12 = e_2;
            current_face1 += 1;
            current_face1;
            if current_face1 == (*d_1).order {
                break;
            }
            let ref mut fresh13 = *((*d_1).faces).offset(current_face1 as isize);
            *fresh13 = if (*e_2).face1 == f_3 { (*e_2).face2 } else { (*e_2).face1 };
            if (*((*d_1).faces).offset(current_face1 as isize)).is_null() {
                break;
            }
        }
        if !(current_face1 == (*d_1).order) {
            loop {
                let mut f_4: *mut grid_face = *((*d_1).faces)
                    .offset(current_face2 as isize);
                let mut e_3: *mut grid_edge = 0 as *mut grid_edge;
                let mut j_5: libc::c_int = 0;
                if !f_4.is_null() {} else {
                    __assert_fail(
                        b"f != NULL\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        757 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"void grid_make_consistent(grid *)\0"))
                            .as_ptr(),
                    );
                }
                'c_11500: {
                    if !f_4.is_null() {} else {
                        __assert_fail(
                            b"f != NULL\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                            757 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void grid_make_consistent(grid *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                j_5 = 0 as libc::c_int;
                while j_5 < (*f_4).order {
                    if *((*f_4).dots).offset(j_5 as isize) == d_1 {
                        break;
                    }
                    j_5 += 1;
                    j_5;
                }
                if j_5 != (*f_4).order {} else {
                    __assert_fail(
                        b"j != f->order\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        763 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"void grid_make_consistent(grid *)\0"))
                            .as_ptr(),
                    );
                }
                'c_11430: {
                    if j_5 != (*f_4).order {} else {
                        __assert_fail(
                            b"j != f->order\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                            763 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void grid_make_consistent(grid *)\0"))
                                .as_ptr(),
                        );
                    }
                };
                e_3 = *((*f_4).edges).offset(j_5 as isize);
                current_face2 -= 1;
                current_face2;
                if current_face2 == -(1 as libc::c_int) {
                    current_face2 = (*d_1).order - 1 as libc::c_int;
                }
                let ref mut fresh14 = *((*d_1).edges).offset(current_face2 as isize);
                *fresh14 = e_3;
                if current_face2 == current_face1 {
                    break;
                }
                let ref mut fresh15 = *((*d_1).faces).offset(current_face2 as isize);
                *fresh15 = if (*e_3).face1 == f_4 { (*e_3).face2 } else { (*e_3).face1 };
                if !(*((*d_1).faces).offset(current_face2 as isize)).is_null() {} else {
                    __assert_fail(
                        b"d->faces[current_face2]\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        780 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"void grid_make_consistent(grid *)\0"))
                            .as_ptr(),
                    );
                }
                'c_11294: {
                    if !(*((*d_1).faces).offset(current_face2 as isize)).is_null()
                    {} else {
                        __assert_fail(
                            b"d->faces[current_face2]\0" as *const u8
                                as *const libc::c_char,
                            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                            780 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 34],
                                &[libc::c_char; 34],
                            >(b"void grid_make_consistent(grid *)\0"))
                                .as_ptr(),
                        );
                    }
                };
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut d_2: *mut grid_dot = *((*g).dots).offset(i as isize);
        if i == 0 as libc::c_int {
            (*g).highest_x = (*d_2).x;
            (*g).lowest_x = (*g).highest_x;
            (*g).highest_y = (*d_2).y;
            (*g).lowest_y = (*g).highest_y;
        } else {
            (*g)
                .lowest_x = if (*g).lowest_x < (*d_2).x {
                (*g).lowest_x
            } else {
                (*d_2).x
            };
            (*g)
                .highest_x = if (*g).highest_x > (*d_2).x {
                (*g).highest_x
            } else {
                (*d_2).x
            };
            (*g)
                .lowest_y = if (*g).lowest_y < (*d_2).y {
                (*g).lowest_y
            } else {
                (*d_2).y
            };
            (*g)
                .highest_y = if (*g).highest_y > (*d_2).y {
                (*g).highest_y
            } else {
                (*d_2).y
            };
        }
        i += 1;
        i;
    }
    grid_debug_derived(g);
}
unsafe extern "C" fn grid_point_cmp_fn(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
) -> libc::c_int {
    let mut p1: *mut grid_dot = v1 as *mut grid_dot;
    let mut p2: *mut grid_dot = v2 as *mut grid_dot;
    if (*p1).y != (*p2).y { return (*p2).y - (*p1).y } else { return (*p2).x - (*p1).x };
}
unsafe extern "C" fn grid_face_add_new(mut g: *mut grid, mut face_size: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut new_face: *mut grid_face = smalloc(
        ::core::mem::size_of::<grid_face>() as libc::c_ulong,
    ) as *mut grid_face;
    if (*g).num_faces < 2147483647 as libc::c_int {} else {
        __assert_fail(
            b"g->num_faces < INT_MAX\0" as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            823 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void grid_face_add_new(grid *, int)\0"))
                .as_ptr(),
        );
    }
    'c_14780: {
        if (*g).num_faces < 2147483647 as libc::c_int {} else {
            __assert_fail(
                b"g->num_faces < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                823 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void grid_face_add_new(grid *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*g).num_faces >= (*g).size_faces {
        let mut increment: libc::c_int = (*g).num_faces / 4 as libc::c_int
            + 128 as libc::c_int;
        (*g)
            .size_faces = if increment < 2147483647 as libc::c_int - (*g).num_faces {
            (*g).num_faces + increment
        } else {
            2147483647 as libc::c_int
        };
        (*g)
            .faces = srealloc(
            (*g).faces as *mut libc::c_void,
            ((*g).size_faces as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut grid_face>() as libc::c_ulong),
        ) as *mut *mut grid_face;
    }
    let fresh16 = (*g).num_faces;
    (*g).num_faces = (*g).num_faces + 1;
    (*new_face).index = fresh16;
    let ref mut fresh17 = *((*g).faces).offset((*new_face).index as isize);
    *fresh17 = new_face;
    (*new_face).order = face_size;
    (*new_face)
        .dots = smalloc(
        (face_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut grid_dot>() as libc::c_ulong),
    ) as *mut *mut grid_dot;
    i = 0 as libc::c_int;
    while i < face_size {
        let ref mut fresh18 = *((*new_face).dots).offset(i as isize);
        *fresh18 = 0 as *mut grid_dot;
        i += 1;
        i;
    }
    (*new_face).edges = 0 as *mut *mut grid_edge;
    (*new_face).has_incentre = 0 as libc::c_int != 0;
}
unsafe extern "C" fn grid_dot_add_new(
    mut g: *mut grid,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> *mut grid_dot {
    let mut new_dot: *mut grid_dot = smalloc(
        ::core::mem::size_of::<grid_dot>() as libc::c_ulong,
    ) as *mut grid_dot;
    if (*g).num_dots >= (*g).size_dots {
        let mut increment: libc::c_int = (*g).num_dots / 4 as libc::c_int
            + 128 as libc::c_int;
        (*g)
            .size_dots = if increment < 2147483647 as libc::c_int - (*g).num_dots {
            (*g).num_dots + increment
        } else {
            2147483647 as libc::c_int
        };
        (*g)
            .dots = srealloc(
            (*g).dots as *mut libc::c_void,
            ((*g).size_dots as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut grid_dot>() as libc::c_ulong),
        ) as *mut *mut grid_dot;
    }
    if (*g).num_dots < 2147483647 as libc::c_int {} else {
        __assert_fail(
            b"g->num_dots < INT_MAX\0" as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            849 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"grid_dot *grid_dot_add_new(grid *, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_14395: {
        if (*g).num_dots < 2147483647 as libc::c_int {} else {
            __assert_fail(
                b"g->num_dots < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                849 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"grid_dot *grid_dot_add_new(grid *, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    let fresh19 = (*g).num_dots;
    (*g).num_dots = (*g).num_dots + 1;
    (*new_dot).index = fresh19;
    let ref mut fresh20 = *((*g).dots).offset((*new_dot).index as isize);
    *fresh20 = new_dot;
    (*new_dot).order = 0 as libc::c_int;
    (*new_dot).edges = 0 as *mut *mut grid_edge;
    (*new_dot).faces = 0 as *mut *mut grid_face;
    (*new_dot).x = x;
    (*new_dot).y = y;
    return new_dot;
}
unsafe extern "C" fn grid_get_dot(
    mut g: *mut grid,
    mut dot_list: *mut tree234,
    mut x: libc::c_int,
    mut y: libc::c_int,
) -> *mut grid_dot {
    let mut test: grid_dot = grid_dot {
        index: 0,
        order: 0,
        edges: 0 as *mut *mut grid_edge,
        faces: 0 as *mut *mut grid_face,
        x: 0,
        y: 0,
    };
    let mut ret: *mut grid_dot = 0 as *mut grid_dot;
    test.order = 0 as libc::c_int;
    test.edges = 0 as *mut *mut grid_edge;
    test.faces = 0 as *mut *mut grid_face;
    test.x = x;
    test.y = y;
    ret = find234(dot_list, &mut test as *mut grid_dot as *mut libc::c_void, None)
        as *mut grid_dot;
    if !ret.is_null() {
        return ret;
    }
    ret = grid_dot_add_new(g, x, y);
    add234(dot_list, ret as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn grid_face_set_dot(
    mut g: *mut grid,
    mut d: *mut grid_dot,
    mut position: libc::c_int,
) {
    let mut last_face: *mut grid_face = *((*g).faces)
        .offset(((*g).num_faces - 1 as libc::c_int) as isize);
    let ref mut fresh21 = *((*last_face).dots).offset(position as isize);
    *fresh21 = d;
}
unsafe extern "C" fn solve_2x2_matrix(
    mut mx: *mut libc::c_double,
    mut vin: *mut libc::c_double,
    mut vout: *mut libc::c_double,
) -> bool {
    let mut inv: [libc::c_double; 4] = [0.; 4];
    let mut det: libc::c_double = 0.;
    det = *mx.offset(0 as libc::c_int as isize) * *mx.offset(3 as libc::c_int as isize)
        - *mx.offset(1 as libc::c_int as isize) * *mx.offset(2 as libc::c_int as isize);
    if det == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int != 0;
    }
    inv[0 as libc::c_int as usize] = *mx.offset(3 as libc::c_int as isize) / det;
    inv[1 as libc::c_int as usize] = -*mx.offset(1 as libc::c_int as isize) / det;
    inv[2 as libc::c_int as usize] = -*mx.offset(2 as libc::c_int as isize) / det;
    inv[3 as libc::c_int as usize] = *mx.offset(0 as libc::c_int as isize) / det;
    *vout
        .offset(
            0 as libc::c_int as isize,
        ) = inv[0 as libc::c_int as usize] * *vin.offset(0 as libc::c_int as isize)
        + inv[1 as libc::c_int as usize] * *vin.offset(1 as libc::c_int as isize);
    *vout
        .offset(
            1 as libc::c_int as isize,
        ) = inv[2 as libc::c_int as usize] * *vin.offset(0 as libc::c_int as isize)
        + inv[3 as libc::c_int as usize] * *vin.offset(1 as libc::c_int as isize);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn solve_3x3_matrix(
    mut mx: *mut libc::c_double,
    mut vin: *mut libc::c_double,
    mut vout: *mut libc::c_double,
) -> bool {
    let mut inv: [libc::c_double; 9] = [0.; 9];
    let mut det: libc::c_double = 0.;
    det = *mx.offset(0 as libc::c_int as isize) * *mx.offset(4 as libc::c_int as isize)
        * *mx.offset(8 as libc::c_int as isize)
        + *mx.offset(1 as libc::c_int as isize) * *mx.offset(5 as libc::c_int as isize)
            * *mx.offset(6 as libc::c_int as isize)
        + *mx.offset(2 as libc::c_int as isize) * *mx.offset(3 as libc::c_int as isize)
            * *mx.offset(7 as libc::c_int as isize)
        - *mx.offset(0 as libc::c_int as isize) * *mx.offset(5 as libc::c_int as isize)
            * *mx.offset(7 as libc::c_int as isize)
        - *mx.offset(1 as libc::c_int as isize) * *mx.offset(3 as libc::c_int as isize)
            * *mx.offset(8 as libc::c_int as isize)
        - *mx.offset(2 as libc::c_int as isize) * *mx.offset(4 as libc::c_int as isize)
            * *mx.offset(6 as libc::c_int as isize);
    if det == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int != 0;
    }
    inv[0 as libc::c_int
        as usize] = (*mx.offset(4 as libc::c_int as isize)
        * *mx.offset(8 as libc::c_int as isize)
        - *mx.offset(5 as libc::c_int as isize) * *mx.offset(7 as libc::c_int as isize))
        / det;
    inv[1 as libc::c_int
        as usize] = (*mx.offset(2 as libc::c_int as isize)
        * *mx.offset(7 as libc::c_int as isize)
        - *mx.offset(1 as libc::c_int as isize) * *mx.offset(8 as libc::c_int as isize))
        / det;
    inv[2 as libc::c_int
        as usize] = (*mx.offset(1 as libc::c_int as isize)
        * *mx.offset(5 as libc::c_int as isize)
        - *mx.offset(2 as libc::c_int as isize) * *mx.offset(4 as libc::c_int as isize))
        / det;
    inv[3 as libc::c_int
        as usize] = (*mx.offset(5 as libc::c_int as isize)
        * *mx.offset(6 as libc::c_int as isize)
        - *mx.offset(3 as libc::c_int as isize) * *mx.offset(8 as libc::c_int as isize))
        / det;
    inv[4 as libc::c_int
        as usize] = (*mx.offset(0 as libc::c_int as isize)
        * *mx.offset(8 as libc::c_int as isize)
        - *mx.offset(2 as libc::c_int as isize) * *mx.offset(6 as libc::c_int as isize))
        / det;
    inv[5 as libc::c_int
        as usize] = (*mx.offset(2 as libc::c_int as isize)
        * *mx.offset(3 as libc::c_int as isize)
        - *mx.offset(0 as libc::c_int as isize) * *mx.offset(5 as libc::c_int as isize))
        / det;
    inv[6 as libc::c_int
        as usize] = (*mx.offset(3 as libc::c_int as isize)
        * *mx.offset(7 as libc::c_int as isize)
        - *mx.offset(4 as libc::c_int as isize) * *mx.offset(6 as libc::c_int as isize))
        / det;
    inv[7 as libc::c_int
        as usize] = (*mx.offset(1 as libc::c_int as isize)
        * *mx.offset(6 as libc::c_int as isize)
        - *mx.offset(0 as libc::c_int as isize) * *mx.offset(7 as libc::c_int as isize))
        / det;
    inv[8 as libc::c_int
        as usize] = (*mx.offset(0 as libc::c_int as isize)
        * *mx.offset(4 as libc::c_int as isize)
        - *mx.offset(1 as libc::c_int as isize) * *mx.offset(3 as libc::c_int as isize))
        / det;
    *vout
        .offset(
            0 as libc::c_int as isize,
        ) = inv[0 as libc::c_int as usize] * *vin.offset(0 as libc::c_int as isize)
        + inv[1 as libc::c_int as usize] * *vin.offset(1 as libc::c_int as isize)
        + inv[2 as libc::c_int as usize] * *vin.offset(2 as libc::c_int as isize);
    *vout
        .offset(
            1 as libc::c_int as isize,
        ) = inv[3 as libc::c_int as usize] * *vin.offset(0 as libc::c_int as isize)
        + inv[4 as libc::c_int as usize] * *vin.offset(1 as libc::c_int as isize)
        + inv[5 as libc::c_int as usize] * *vin.offset(2 as libc::c_int as isize);
    *vout
        .offset(
            2 as libc::c_int as isize,
        ) = inv[6 as libc::c_int as usize] * *vin.offset(0 as libc::c_int as isize)
        + inv[7 as libc::c_int as usize] * *vin.offset(1 as libc::c_int as isize)
        + inv[8 as libc::c_int as usize] * *vin.offset(2 as libc::c_int as isize);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn grid_find_incentre(mut f: *mut grid_face) {
    let mut xbest: libc::c_double = 0.;
    let mut ybest: libc::c_double = 0.;
    let mut bestdist: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut edgedot1: [*mut grid_dot; 3] = [0 as *mut grid_dot; 3];
    let mut edgedot2: [*mut grid_dot; 3] = [0 as *mut grid_dot; 3];
    let mut dots: [*mut grid_dot; 3] = [0 as *mut grid_dot; 3];
    let mut nedges: libc::c_int = 0;
    let mut ndots: libc::c_int = 0;
    if (*f).has_incentre {
        return;
    }
    ndots = 0 as libc::c_int;
    nedges = ndots;
    bestdist = 0 as libc::c_int as libc::c_double;
    ybest = 0 as libc::c_int as libc::c_double;
    xbest = ybest;
    i = 0 as libc::c_int;
    while (i + 2 as libc::c_int) < 2 as libc::c_int * (*f).order {
        if i < (*f).order {
            edgedot1[nedges as usize] = *((*f).dots).offset(i as isize);
            let fresh22 = nedges;
            nedges = nedges + 1;
            edgedot2[fresh22
                as usize] = *((*f).dots)
                .offset(((i + 1 as libc::c_int) % (*f).order) as isize);
        } else {
            let fresh23 = ndots;
            ndots = ndots + 1;
            dots[fresh23 as usize] = *((*f).dots).offset((i - (*f).order) as isize);
        }
        j = i + 1 as libc::c_int;
        while (j + 1 as libc::c_int) < 2 as libc::c_int * (*f).order {
            if j < (*f).order {
                edgedot1[nedges as usize] = *((*f).dots).offset(j as isize);
                let fresh24 = nedges;
                nedges = nedges + 1;
                edgedot2[fresh24
                    as usize] = *((*f).dots)
                    .offset(((j + 1 as libc::c_int) % (*f).order) as isize);
            } else {
                let fresh25 = ndots;
                ndots = ndots + 1;
                dots[fresh25 as usize] = *((*f).dots).offset((j - (*f).order) as isize);
            }
            k = j + 1 as libc::c_int;
            while k < 2 as libc::c_int * (*f).order {
                let mut cx: [libc::c_double; 2] = [0.; 2];
                let mut cy: [libc::c_double; 2] = [0.; 2];
                let mut cn: libc::c_int = 0 as libc::c_int;
                if k < (*f).order {
                    edgedot1[nedges as usize] = *((*f).dots).offset(k as isize);
                    let fresh26 = nedges;
                    nedges = nedges + 1;
                    edgedot2[fresh26
                        as usize] = *((*f).dots)
                        .offset(((k + 1 as libc::c_int) % (*f).order) as isize);
                } else {
                    let fresh27 = ndots;
                    ndots = ndots + 1;
                    dots[fresh27
                        as usize] = *((*f).dots).offset((k - (*f).order) as isize);
                }
                if nedges == 3 as libc::c_int {
                    let mut matrix: [libc::c_double; 9] = [0.; 9];
                    let mut vector: [libc::c_double; 3] = [0.; 3];
                    let mut vector2: [libc::c_double; 3] = [0.; 3];
                    let mut m_0: libc::c_int = 0;
                    m_0 = 0 as libc::c_int;
                    while m_0 < 3 as libc::c_int {
                        let mut x1: libc::c_int = (*edgedot1[m_0 as usize]).x;
                        let mut x2: libc::c_int = (*edgedot2[m_0 as usize]).x;
                        let mut y1: libc::c_int = (*edgedot1[m_0 as usize]).y;
                        let mut y2: libc::c_int = (*edgedot2[m_0 as usize]).y;
                        let mut dx: libc::c_int = x2 - x1;
                        let mut dy: libc::c_int = y2 - y1;
                        matrix[(3 as libc::c_int * m_0 + 0 as libc::c_int)
                            as usize] = dy as libc::c_double;
                        matrix[(3 as libc::c_int * m_0 + 1 as libc::c_int)
                            as usize] = -dx as libc::c_double;
                        matrix[(3 as libc::c_int * m_0 + 2 as libc::c_int)
                            as usize] = -__tg_sqrt(
                            dx as libc::c_double * dx as libc::c_double
                                + dy as libc::c_double * dy as libc::c_double,
                        );
                        vector[m_0
                            as usize] = x1 as libc::c_double * dy as libc::c_double
                            - y1 as libc::c_double * dx as libc::c_double;
                        m_0 += 1;
                        m_0;
                    }
                    if solve_3x3_matrix(
                        matrix.as_mut_ptr(),
                        vector.as_mut_ptr(),
                        vector2.as_mut_ptr(),
                    ) {
                        cx[cn as usize] = vector2[0 as libc::c_int as usize];
                        cy[cn as usize] = vector2[1 as libc::c_int as usize];
                        cn += 1;
                        cn;
                    }
                } else if nedges == 2 as libc::c_int {
                    let mut eqs: [[libc::c_double; 4]; 2] = [[0.; 4]; 2];
                    let mut eq: [libc::c_double; 3] = [0.; 3];
                    let mut xt: [libc::c_double; 2] = [0.; 2];
                    let mut yt: [libc::c_double; 2] = [0.; 2];
                    let mut rt: [libc::c_double; 2] = [0.; 2];
                    let mut q: [libc::c_double; 3] = [0.; 3];
                    let mut disc: libc::c_double = 0.;
                    m = 0 as libc::c_int;
                    while m < 2 as libc::c_int {
                        let mut x1_0: libc::c_int = (*edgedot1[m as usize]).x;
                        let mut x2_0: libc::c_int = (*edgedot2[m as usize]).x;
                        let mut y1_0: libc::c_int = (*edgedot1[m as usize]).y;
                        let mut y2_0: libc::c_int = (*edgedot2[m as usize]).y;
                        let mut dx_0: libc::c_int = x2_0 - x1_0;
                        let mut dy_0: libc::c_int = y2_0 - y1_0;
                        eqs[m
                            as usize][0 as libc::c_int
                            as usize] = dy_0 as libc::c_double;
                        eqs[m
                            as usize][1 as libc::c_int
                            as usize] = -dx_0 as libc::c_double;
                        eqs[m
                            as usize][2 as libc::c_int
                            as usize] = -__tg_sqrt(
                            (dx_0 * dx_0 + dy_0 * dy_0) as libc::c_double,
                        );
                        eqs[m
                            as usize][3 as libc::c_int
                            as usize] = (x1_0 * dy_0 - y1_0 * dx_0) as libc::c_double;
                        m += 1;
                        m;
                    }
                    eq[0 as libc::c_int
                        as usize] = eqs[0 as libc::c_int
                        as usize][0 as libc::c_int as usize]
                        * eqs[1 as libc::c_int as usize][2 as libc::c_int as usize]
                        - eqs[1 as libc::c_int as usize][0 as libc::c_int as usize]
                            * eqs[0 as libc::c_int as usize][2 as libc::c_int as usize];
                    eq[1 as libc::c_int
                        as usize] = eqs[0 as libc::c_int
                        as usize][1 as libc::c_int as usize]
                        * eqs[1 as libc::c_int as usize][2 as libc::c_int as usize]
                        - eqs[1 as libc::c_int as usize][1 as libc::c_int as usize]
                            * eqs[0 as libc::c_int as usize][2 as libc::c_int as usize];
                    eq[2 as libc::c_int
                        as usize] = eqs[0 as libc::c_int
                        as usize][3 as libc::c_int as usize]
                        * eqs[1 as libc::c_int as usize][2 as libc::c_int as usize]
                        - eqs[1 as libc::c_int as usize][3 as libc::c_int as usize]
                            * eqs[0 as libc::c_int as usize][2 as libc::c_int as usize];
                    if __tg_fabs(eq[0 as libc::c_int as usize])
                        < __tg_fabs(eq[1 as libc::c_int as usize])
                    {
                        xt[0 as libc::c_int
                            as usize] = 1 as libc::c_int as libc::c_double;
                        xt[1 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_double;
                        yt[0 as libc::c_int
                            as usize] = -eq[0 as libc::c_int as usize]
                            / eq[1 as libc::c_int as usize];
                        yt[1 as libc::c_int
                            as usize] = eq[2 as libc::c_int as usize]
                            / eq[1 as libc::c_int as usize];
                    } else {
                        yt[0 as libc::c_int
                            as usize] = 1 as libc::c_int as libc::c_double;
                        yt[1 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_double;
                        xt[0 as libc::c_int
                            as usize] = -eq[1 as libc::c_int as usize]
                            / eq[0 as libc::c_int as usize];
                        xt[1 as libc::c_int
                            as usize] = eq[2 as libc::c_int as usize]
                            / eq[0 as libc::c_int as usize];
                    }
                    rt[0 as libc::c_int
                        as usize] = -(eqs[0 as libc::c_int
                        as usize][0 as libc::c_int as usize]
                        * xt[0 as libc::c_int as usize]
                        + eqs[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            * yt[0 as libc::c_int as usize])
                        / eqs[0 as libc::c_int as usize][2 as libc::c_int as usize];
                    rt[1 as libc::c_int
                        as usize] = (eqs[0 as libc::c_int
                        as usize][3 as libc::c_int as usize]
                        - eqs[0 as libc::c_int as usize][0 as libc::c_int as usize]
                            * xt[1 as libc::c_int as usize]
                        - eqs[0 as libc::c_int as usize][1 as libc::c_int as usize]
                            * yt[1 as libc::c_int as usize])
                        / eqs[0 as libc::c_int as usize][2 as libc::c_int as usize];
                    q[0 as libc::c_int
                        as usize] = -rt[0 as libc::c_int as usize]
                        * rt[0 as libc::c_int as usize];
                    q[1 as libc::c_int
                        as usize] = -(2 as libc::c_int) as libc::c_double
                        * rt[0 as libc::c_int as usize] * rt[1 as libc::c_int as usize];
                    q[2 as libc::c_int
                        as usize] = -rt[1 as libc::c_int as usize]
                        * rt[1 as libc::c_int as usize];
                    q[0 as libc::c_int as usize]
                        += xt[0 as libc::c_int as usize] * xt[0 as libc::c_int as usize];
                    q[1 as libc::c_int as usize]
                        += 2 as libc::c_int as libc::c_double
                            * xt[0 as libc::c_int as usize]
                            * (xt[1 as libc::c_int as usize]
                                - (*dots[0 as libc::c_int as usize]).x as libc::c_double);
                    q[2 as libc::c_int as usize]
                        += (xt[1 as libc::c_int as usize]
                            - (*dots[0 as libc::c_int as usize]).x as libc::c_double)
                            * (xt[1 as libc::c_int as usize]
                                - (*dots[0 as libc::c_int as usize]).x as libc::c_double);
                    q[0 as libc::c_int as usize]
                        += yt[0 as libc::c_int as usize] * yt[0 as libc::c_int as usize];
                    q[1 as libc::c_int as usize]
                        += 2 as libc::c_int as libc::c_double
                            * yt[0 as libc::c_int as usize]
                            * (yt[1 as libc::c_int as usize]
                                - (*dots[0 as libc::c_int as usize]).y as libc::c_double);
                    q[2 as libc::c_int as usize]
                        += (yt[1 as libc::c_int as usize]
                            - (*dots[0 as libc::c_int as usize]).y as libc::c_double)
                            * (yt[1 as libc::c_int as usize]
                                - (*dots[0 as libc::c_int as usize]).y as libc::c_double);
                    disc = q[1 as libc::c_int as usize] * q[1 as libc::c_int as usize]
                        - 4 as libc::c_int as libc::c_double
                            * q[0 as libc::c_int as usize]
                            * q[2 as libc::c_int as usize];
                    if disc >= 0 as libc::c_int as libc::c_double {
                        let mut t: libc::c_double = 0.;
                        disc = __tg_sqrt(disc);
                        t = (-q[1 as libc::c_int as usize] + disc)
                            / (2 as libc::c_int as libc::c_double
                                * q[0 as libc::c_int as usize]);
                        cx[cn
                            as usize] = xt[0 as libc::c_int as usize] * t
                            + xt[1 as libc::c_int as usize];
                        cy[cn
                            as usize] = yt[0 as libc::c_int as usize] * t
                            + yt[1 as libc::c_int as usize];
                        cn += 1;
                        cn;
                        t = (-q[1 as libc::c_int as usize] - disc)
                            / (2 as libc::c_int as libc::c_double
                                * q[0 as libc::c_int as usize]);
                        cx[cn
                            as usize] = xt[0 as libc::c_int as usize] * t
                            + xt[1 as libc::c_int as usize];
                        cy[cn
                            as usize] = yt[0 as libc::c_int as usize] * t
                            + yt[1 as libc::c_int as usize];
                        cn += 1;
                        cn;
                    }
                } else if nedges == 1 as libc::c_int {
                    let mut xt_0: [libc::c_double; 2] = [0.; 2];
                    let mut yt_0: [libc::c_double; 2] = [0.; 2];
                    let mut rt_0: [libc::c_double; 2] = [0.; 2];
                    let mut q_0: [libc::c_double; 3] = [0.; 3];
                    let mut disc_0: libc::c_double = 0.;
                    let mut halfsep: libc::c_double = 0.;
                    let mut x1_1: libc::c_int = (*dots[0 as libc::c_int as usize]).x;
                    let mut x2_1: libc::c_int = (*dots[1 as libc::c_int as usize]).x;
                    let mut y1_1: libc::c_int = (*dots[0 as libc::c_int as usize]).y;
                    let mut y2_1: libc::c_int = (*dots[1 as libc::c_int as usize]).y;
                    let mut dx_1: libc::c_int = x2_1 - x1_1;
                    let mut dy_1: libc::c_int = y2_1 - y1_1;
                    let mut d: libc::c_double = __tg_sqrt(
                        dx_1 as libc::c_double * dx_1 as libc::c_double
                            + dy_1 as libc::c_double * dy_1 as libc::c_double,
                    );
                    xt_0[1 as libc::c_int
                        as usize] = (x1_1 + x2_1) as libc::c_double / 2.0f64;
                    yt_0[1 as libc::c_int
                        as usize] = (y1_1 + y2_1) as libc::c_double / 2.0f64;
                    xt_0[0 as libc::c_int as usize] = -dy_1 as libc::c_double / d;
                    yt_0[0 as libc::c_int as usize] = dx_1 as libc::c_double / d;
                    halfsep = 0.5f64 * d;
                    let mut x1_2: libc::c_int = (*edgedot1[0 as libc::c_int as usize]).x;
                    let mut x2_2: libc::c_int = (*edgedot2[0 as libc::c_int as usize]).x;
                    let mut y1_2: libc::c_int = (*edgedot1[0 as libc::c_int as usize]).y;
                    let mut y2_2: libc::c_int = (*edgedot2[0 as libc::c_int as usize]).y;
                    let mut dx_2: libc::c_int = x2_2 - x1_2;
                    let mut dy_2: libc::c_int = y2_2 - y1_2;
                    let mut d_0: libc::c_double = __tg_sqrt(
                        dx_2 as libc::c_double * dx_2 as libc::c_double
                            + dy_2 as libc::c_double * dy_2 as libc::c_double,
                    );
                    rt_0[0 as libc::c_int
                        as usize] = (xt_0[0 as libc::c_int as usize]
                        * dy_2 as libc::c_double
                        - yt_0[0 as libc::c_int as usize] * dx_2 as libc::c_double)
                        / d_0;
                    rt_0[1 as libc::c_int
                        as usize] = ((xt_0[1 as libc::c_int as usize]
                        - x1_2 as libc::c_double) * dy_2 as libc::c_double
                        - (yt_0[1 as libc::c_int as usize] - y1_2 as libc::c_double)
                            * dx_2 as libc::c_double) / d_0;
                    q_0[0 as libc::c_int
                        as usize] = rt_0[0 as libc::c_int as usize]
                        * rt_0[0 as libc::c_int as usize];
                    q_0[1 as libc::c_int
                        as usize] = 2 as libc::c_int as libc::c_double
                        * rt_0[0 as libc::c_int as usize]
                        * rt_0[1 as libc::c_int as usize];
                    q_0[2 as libc::c_int
                        as usize] = rt_0[1 as libc::c_int as usize]
                        * rt_0[1 as libc::c_int as usize];
                    q_0[0 as libc::c_int as usize] -= 1 as libc::c_int as libc::c_double;
                    q_0[2 as libc::c_int as usize] -= halfsep * halfsep;
                    disc_0 = q_0[1 as libc::c_int as usize]
                        * q_0[1 as libc::c_int as usize]
                        - 4 as libc::c_int as libc::c_double
                            * q_0[0 as libc::c_int as usize]
                            * q_0[2 as libc::c_int as usize];
                    if disc_0 >= 0 as libc::c_int as libc::c_double {
                        let mut t_0: libc::c_double = 0.;
                        disc_0 = __tg_sqrt(disc_0);
                        t_0 = (-q_0[1 as libc::c_int as usize] + disc_0)
                            / (2 as libc::c_int as libc::c_double
                                * q_0[0 as libc::c_int as usize]);
                        cx[cn
                            as usize] = xt_0[0 as libc::c_int as usize] * t_0
                            + xt_0[1 as libc::c_int as usize];
                        cy[cn
                            as usize] = yt_0[0 as libc::c_int as usize] * t_0
                            + yt_0[1 as libc::c_int as usize];
                        cn += 1;
                        cn;
                        t_0 = (-q_0[1 as libc::c_int as usize] - disc_0)
                            / (2 as libc::c_int as libc::c_double
                                * q_0[0 as libc::c_int as usize]);
                        cx[cn
                            as usize] = xt_0[0 as libc::c_int as usize] * t_0
                            + xt_0[1 as libc::c_int as usize];
                        cy[cn
                            as usize] = yt_0[0 as libc::c_int as usize] * t_0
                            + yt_0[1 as libc::c_int as usize];
                        cn += 1;
                        cn;
                    }
                } else if nedges == 0 as libc::c_int {
                    let mut matrix_0: [libc::c_double; 4] = [0.; 4];
                    let mut vector_0: [libc::c_double; 2] = [0.; 2];
                    let mut vector2_0: [libc::c_double; 2] = [0.; 2];
                    let mut m_1: libc::c_int = 0;
                    m_1 = 0 as libc::c_int;
                    while m_1 < 2 as libc::c_int {
                        let mut x1_3: libc::c_int = (*dots[m_1 as usize]).x;
                        let mut x2_3: libc::c_int = (*dots[(m_1 + 1 as libc::c_int)
                            as usize])
                            .x;
                        let mut y1_3: libc::c_int = (*dots[m_1 as usize]).y;
                        let mut y2_3: libc::c_int = (*dots[(m_1 + 1 as libc::c_int)
                            as usize])
                            .y;
                        let mut dx_3: libc::c_int = x2_3 - x1_3;
                        let mut dy_3: libc::c_int = y2_3 - y1_3;
                        matrix_0[(2 as libc::c_int * m_1 + 0 as libc::c_int)
                            as usize] = (2 as libc::c_int * dx_3) as libc::c_double;
                        matrix_0[(2 as libc::c_int * m_1 + 1 as libc::c_int)
                            as usize] = (2 as libc::c_int * dy_3) as libc::c_double;
                        vector_0[m_1
                            as usize] = dx_3 as libc::c_double * dx_3 as libc::c_double
                            + dy_3 as libc::c_double * dy_3 as libc::c_double
                            + 2.0f64 * x1_3 as libc::c_double * dx_3 as libc::c_double
                            + 2.0f64 * y1_3 as libc::c_double * dy_3 as libc::c_double;
                        m_1 += 1;
                        m_1;
                    }
                    if solve_2x2_matrix(
                        matrix_0.as_mut_ptr(),
                        vector_0.as_mut_ptr(),
                        vector2_0.as_mut_ptr(),
                    ) {
                        cx[cn as usize] = vector2_0[0 as libc::c_int as usize];
                        cy[cn as usize] = vector2_0[1 as libc::c_int as usize];
                        cn += 1;
                        cn;
                    }
                }
                m = 0 as libc::c_int;
                while m < cn {
                    let mut x: libc::c_double = cx[m as usize];
                    let mut y: libc::c_double = cy[m as usize];
                    let mut e: libc::c_int = 0;
                    let mut in_0: bool = 0 as libc::c_int != 0;
                    e = 0 as libc::c_int;
                    while e < (*f).order {
                        let mut xs: libc::c_int = (*(**((*f).edges).offset(e as isize))
                            .dot1)
                            .x;
                        let mut xe: libc::c_int = (*(**((*f).edges).offset(e as isize))
                            .dot2)
                            .x;
                        let mut ys: libc::c_int = (*(**((*f).edges).offset(e as isize))
                            .dot1)
                            .y;
                        let mut ye: libc::c_int = (*(**((*f).edges).offset(e as isize))
                            .dot2)
                            .y;
                        if y >= ys as libc::c_double && y < ye as libc::c_double
                            || y >= ye as libc::c_double && y < ys as libc::c_double
                        {
                            let mut num: libc::c_int = xe - xs;
                            let mut denom: libc::c_int = ye - ys;
                            if denom < 0 as libc::c_int {
                                num = -num;
                                denom = -denom;
                            }
                            if (x - xs as libc::c_double) * denom as libc::c_double
                                >= (y - ys as libc::c_double) * num as libc::c_double
                            {
                                in_0 = !in_0;
                            }
                        }
                        e += 1;
                        e;
                    }
                    if in_0 {
                        let mut mindist: libc::c_double = ::core::f64::INFINITY;
                        let mut e_0: libc::c_int = 0;
                        let mut d_1: libc::c_int = 0;
                        d_1 = 0 as libc::c_int;
                        while d_1 < (*f).order {
                            let mut xp: libc::c_int = (**((*f).dots)
                                .offset(d_1 as isize))
                                .x;
                            let mut yp: libc::c_int = (**((*f).dots)
                                .offset(d_1 as isize))
                                .y;
                            let mut dx_4: libc::c_double = x - xp as libc::c_double;
                            let mut dy_4: libc::c_double = y - yp as libc::c_double;
                            let mut dist: libc::c_double = dx_4 * dx_4 + dy_4 * dy_4;
                            if mindist > dist {
                                mindist = dist;
                            }
                            d_1 += 1;
                            d_1;
                        }
                        e_0 = 0 as libc::c_int;
                        while e_0 < (*f).order {
                            let mut xs_0: libc::c_int = (*(**((*f).edges)
                                .offset(e_0 as isize))
                                .dot1)
                                .x;
                            let mut xe_0: libc::c_int = (*(**((*f).edges)
                                .offset(e_0 as isize))
                                .dot2)
                                .x;
                            let mut ys_0: libc::c_int = (*(**((*f).edges)
                                .offset(e_0 as isize))
                                .dot1)
                                .y;
                            let mut ye_0: libc::c_int = (*(**((*f).edges)
                                .offset(e_0 as isize))
                                .dot2)
                                .y;
                            let mut edx: libc::c_int = xe_0 - xs_0;
                            let mut edy: libc::c_int = ye_0 - ys_0;
                            let mut pdx: libc::c_double = x - xs_0 as libc::c_double;
                            let mut pdy: libc::c_double = y - ys_0 as libc::c_double;
                            let mut pde: libc::c_double = pdx * edx as libc::c_double
                                + pdy * edy as libc::c_double;
                            let mut ede: libc::c_long = edx as libc::c_long
                                * edx as libc::c_long
                                + edy as libc::c_long * edy as libc::c_long;
                            if (0 as libc::c_int as libc::c_double) < pde
                                && pde < ede as libc::c_double
                            {
                                let mut pdre: libc::c_double = pdx * edy as libc::c_double
                                    - pdy * edx as libc::c_double;
                                let mut sqlen: libc::c_double = pdre * pdre
                                    / ede as libc::c_double;
                                if mindist > sqlen {
                                    mindist = sqlen;
                                }
                            }
                            e_0 += 1;
                            e_0;
                        }
                        if bestdist < mindist {
                            bestdist = mindist;
                            xbest = x;
                            ybest = y;
                        }
                    }
                    m += 1;
                    m;
                }
                if k < (*f).order {
                    nedges -= 1;
                    nedges;
                } else {
                    ndots -= 1;
                    ndots;
                }
                k += 1;
                k;
            }
            if j < (*f).order {
                nedges -= 1;
                nedges;
            } else {
                ndots -= 1;
                ndots;
            }
            j += 1;
            j;
        }
        if i < (*f).order {
            nedges -= 1;
            nedges;
        } else {
            ndots -= 1;
            ndots;
        }
        i += 1;
        i;
    }
    if bestdist > 0 as libc::c_int as libc::c_double {} else {
        __assert_fail(
            b"bestdist > 0\0" as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            1395 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void grid_find_incentre(grid_face *)\0"))
                .as_ptr(),
        );
    }
    'c_31761: {
        if bestdist > 0 as libc::c_int as libc::c_double {} else {
            __assert_fail(
                b"bestdist > 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                1395 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void grid_find_incentre(grid_face *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*f).has_incentre = 1 as libc::c_int != 0;
    (*f).ix = (xbest + 0.5f64) as libc::c_int;
    (*f).iy = (ybest + 0.5f64) as libc::c_int;
}
unsafe extern "C" fn grid_validate_params_square(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    if width > 2147483647 as libc::c_int / 20 as libc::c_int
        || height > 2147483647 as libc::c_int / 20 as libc::c_int
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_square(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 20 as libc::c_int;
    *tilesize = a;
    *xextent = width * a;
    *yextent = height * a;
}
unsafe extern "C" fn grid_new_square(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 20 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = a;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = a * x;
            let mut py: libc::c_int = a * y;
            grid_face_add_new(g, 4 as libc::c_int);
            d = grid_get_dot(g, points, px, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px, py + a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_honeycomb(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int - 4 as libc::c_int * a) / (3 as libc::c_int * a)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - 3 as libc::c_int * b) / (2 as libc::c_int * b)
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 2 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_honeycomb(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 45 as libc::c_int;
    *xextent = 3 as libc::c_int * a * (width - 1 as libc::c_int) + 4 as libc::c_int * a;
    *yextent = 2 as libc::c_int * b * (height - 1 as libc::c_int) + 3 as libc::c_int * b;
}
unsafe extern "C" fn grid_new_honeycomb(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 45 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut cx: libc::c_int = 3 as libc::c_int * a * x;
            let mut cy: libc::c_int = 2 as libc::c_int * b * y;
            if x % 2 as libc::c_int != 0 {
                cy += b;
            }
            grid_face_add_new(g, 6 as libc::c_int);
            d = grid_get_dot(g, points, cx - a, cy - b);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, cx + a, cy - b);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, cx + 2 as libc::c_int * a, cy);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, cx + a, cy + b);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, cx - a, cy + b);
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, cx - 2 as libc::c_int * a, cy);
            grid_face_set_dot(g, d, 5 as libc::c_int);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_triangular(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut vec_x: libc::c_int = 15 as libc::c_int;
    let mut vec_y: libc::c_int = 26 as libc::c_int;
    if width > 2147483647 as libc::c_int / (2 as libc::c_int * vec_x) - 1 as libc::c_int
        || height > 2147483647 as libc::c_int / vec_y
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 4 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_triangular(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut vec_x: libc::c_int = 15 as libc::c_int;
    let mut vec_y: libc::c_int = 26 as libc::c_int;
    *tilesize = 18 as libc::c_int;
    *xextent = (width + 1 as libc::c_int) * 2 as libc::c_int * vec_x;
    *yextent = height * vec_y;
}
unsafe extern "C" fn grid_validate_desc_triangular(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *const libc::c_char {
    if desc.is_null() || strcmp(desc, b"0\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as *const libc::c_char;
    }
    return b"Unrecognised grid description.\0" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn grid_new_triangular(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut version: libc::c_int = if desc.is_null() {
        -(1 as libc::c_int)
    } else {
        atoi(desc)
    };
    let mut vec_x: libc::c_int = 15 as libc::c_int;
    let mut vec_y: libc::c_int = 26 as libc::c_int;
    let mut index: libc::c_int = 0;
    let mut w: libc::c_int = width + 1 as libc::c_int;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 18 as libc::c_int;
    if version == -(1 as libc::c_int) {
        (*g).size_faces = width * height * 2 as libc::c_int;
        (*g).num_faces = (*g).size_faces;
        (*g).size_dots = (width + 1 as libc::c_int) * (height + 1 as libc::c_int);
        (*g).num_dots = (*g).size_dots;
        (*g)
            .faces = smalloc(
            ((*g).size_faces as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut grid_face>() as libc::c_ulong),
        ) as *mut *mut grid_face;
        (*g)
            .dots = smalloc(
            ((*g).size_dots as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut grid_dot>() as libc::c_ulong),
        ) as *mut *mut grid_dot;
        index = 0 as libc::c_int;
        y = 0 as libc::c_int;
        while y <= height {
            x = 0 as libc::c_int;
            while x <= width {
                let mut d: *mut grid_dot = smalloc(
                    ::core::mem::size_of::<grid_dot>() as libc::c_ulong,
                ) as *mut grid_dot;
                (*d).index = index;
                let ref mut fresh28 = *((*g).dots).offset((*d).index as isize);
                *fresh28 = d;
                (*d).order = 0 as libc::c_int;
                (*d).edges = 0 as *mut *mut grid_edge;
                (*d).faces = 0 as *mut *mut grid_face;
                (*d)
                    .x = x * 2 as libc::c_int * vec_x
                    + (if y % 2 as libc::c_int != 0 { vec_x } else { 0 as libc::c_int });
                (*d).y = y * vec_y;
                index += 1;
                index;
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        index = 0 as libc::c_int;
        y = 0 as libc::c_int;
        while y < height {
            x = 0 as libc::c_int;
            while x < width {
                let mut f1: *mut grid_face = smalloc(
                    ::core::mem::size_of::<grid_face>() as libc::c_ulong,
                ) as *mut grid_face;
                let mut f2: *mut grid_face = smalloc(
                    ::core::mem::size_of::<grid_face>() as libc::c_ulong,
                ) as *mut grid_face;
                (*f1).index = index;
                (*f2).index = index + 1 as libc::c_int;
                let ref mut fresh29 = *((*g).faces).offset((*f1).index as isize);
                *fresh29 = f1;
                let ref mut fresh30 = *((*g).faces).offset((*f2).index as isize);
                *fresh30 = f2;
                (*f1).edges = 0 as *mut *mut grid_edge;
                (*f1).order = 3 as libc::c_int;
                (*f1)
                    .dots = smalloc(
                    ((*f1).order as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut grid_dot>() as libc::c_ulong,
                        ),
                ) as *mut *mut grid_dot;
                (*f1).has_incentre = 0 as libc::c_int != 0;
                (*f2).edges = 0 as *mut *mut grid_edge;
                (*f2).order = 3 as libc::c_int;
                (*f2)
                    .dots = smalloc(
                    ((*f2).order as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<*mut grid_dot>() as libc::c_ulong,
                        ),
                ) as *mut *mut grid_dot;
                (*f2).has_incentre = 0 as libc::c_int != 0;
                if y % 2 as libc::c_int != 0 {
                    let ref mut fresh31 = *((*f1).dots)
                        .offset(0 as libc::c_int as isize);
                    *fresh31 = *((*g).dots).offset((y * w + x) as isize);
                    let ref mut fresh32 = *((*f1).dots)
                        .offset(1 as libc::c_int as isize);
                    *fresh32 = *((*g).dots)
                        .offset(
                            ((y + 1 as libc::c_int) * w + x + 1 as libc::c_int) as isize,
                        );
                    let ref mut fresh33 = *((*f1).dots)
                        .offset(2 as libc::c_int as isize);
                    *fresh33 = *((*g).dots)
                        .offset(((y + 1 as libc::c_int) * w + x) as isize);
                    let ref mut fresh34 = *((*f2).dots)
                        .offset(0 as libc::c_int as isize);
                    *fresh34 = *((*g).dots).offset((y * w + x) as isize);
                    let ref mut fresh35 = *((*f2).dots)
                        .offset(1 as libc::c_int as isize);
                    *fresh35 = *((*g).dots)
                        .offset((y * w + x + 1 as libc::c_int) as isize);
                    let ref mut fresh36 = *((*f2).dots)
                        .offset(2 as libc::c_int as isize);
                    *fresh36 = *((*g).dots)
                        .offset(
                            ((y + 1 as libc::c_int) * w + x + 1 as libc::c_int) as isize,
                        );
                } else {
                    let ref mut fresh37 = *((*f1).dots)
                        .offset(0 as libc::c_int as isize);
                    *fresh37 = *((*g).dots).offset((y * w + x) as isize);
                    let ref mut fresh38 = *((*f1).dots)
                        .offset(1 as libc::c_int as isize);
                    *fresh38 = *((*g).dots)
                        .offset((y * w + x + 1 as libc::c_int) as isize);
                    let ref mut fresh39 = *((*f1).dots)
                        .offset(2 as libc::c_int as isize);
                    *fresh39 = *((*g).dots)
                        .offset(((y + 1 as libc::c_int) * w + x) as isize);
                    let ref mut fresh40 = *((*f2).dots)
                        .offset(0 as libc::c_int as isize);
                    *fresh40 = *((*g).dots)
                        .offset((y * w + x + 1 as libc::c_int) as isize);
                    let ref mut fresh41 = *((*f2).dots)
                        .offset(1 as libc::c_int as isize);
                    *fresh41 = *((*g).dots)
                        .offset(
                            ((y + 1 as libc::c_int) * w + x + 1 as libc::c_int) as isize,
                        );
                    let ref mut fresh42 = *((*f2).dots)
                        .offset(2 as libc::c_int as isize);
                    *fresh42 = *((*g).dots)
                        .offset(((y + 1 as libc::c_int) * w + x) as isize);
                }
                index += 2 as libc::c_int;
                x += 1;
                x;
            }
            y += 1;
            y;
        }
    } else {
        let mut points: *mut tree234 = newtree234(
            Some(
                grid_point_cmp_fn
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        y = 0 as libc::c_int;
        while y < height {
            let mut y0: libc::c_int = 0;
            let mut y1: libc::c_int = 0;
            let mut n1: libc::c_int = 0;
            let mut n2: libc::c_int = 0;
            y1 = y * vec_y;
            y0 = y1;
            if y % 2 as libc::c_int != 0 {
                y1 += vec_y;
                n1 = 2 as libc::c_int;
                n2 = 1 as libc::c_int;
            } else {
                y0 += vec_y;
                n1 = 1 as libc::c_int;
                n2 = 2 as libc::c_int;
            }
            x = 0 as libc::c_int;
            while x <= width {
                let mut x0: libc::c_int = 2 as libc::c_int * x * vec_x;
                let mut x1: libc::c_int = x0 + vec_x;
                let mut x2: libc::c_int = x1 + vec_x;
                if !(height % 2 as libc::c_int == 1 as libc::c_int
                    && y == height - 1 as libc::c_int
                    && (x == 0 as libc::c_int || x == width))
                {
                    grid_face_add_new(g, 3 as libc::c_int);
                    grid_face_set_dot(
                        g,
                        grid_get_dot(g, points, x0, y0),
                        0 as libc::c_int,
                    );
                    grid_face_set_dot(g, grid_get_dot(g, points, x1, y1), n1);
                    grid_face_set_dot(g, grid_get_dot(g, points, x2, y0), n2);
                }
                x += 1;
                x;
            }
            x = 0 as libc::c_int;
            while x < width {
                let mut x0_0: libc::c_int = (2 as libc::c_int * x + 1 as libc::c_int)
                    * vec_x;
                let mut x1_0: libc::c_int = x0_0 + vec_x;
                let mut x2_0: libc::c_int = x1_0 + vec_x;
                grid_face_add_new(g, 3 as libc::c_int);
                grid_face_set_dot(
                    g,
                    grid_get_dot(g, points, x0_0, y1),
                    0 as libc::c_int,
                );
                grid_face_set_dot(g, grid_get_dot(g, points, x1_0, y0), n2);
                grid_face_set_dot(g, grid_get_dot(g, points, x2_0, y1), n1);
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        freetree234(points);
    }
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_snubsquare(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width - 1 as libc::c_int > (2147483647 as libc::c_int - (a + b)) / (a + b)
        || height > (2147483647 as libc::c_int - (a + b)) / (a + b)
        || width > 2147483647 as libc::c_int / 3 as libc::c_int / height
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 2 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_snubsquare(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 18 as libc::c_int;
    *xextent = (a + b) * (width - 1 as libc::c_int) + a + b;
    *yextent = (a + b) * (height - 1 as libc::c_int) + a + b;
}
unsafe extern "C" fn grid_new_snubsquare(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 18 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = (a + b) * x;
            let mut py: libc::c_int = (a + b) * y;
            grid_face_add_new(g, 4 as libc::c_int);
            if (x + y) % 2 as libc::c_int != 0 {
                d = grid_get_dot(g, points, px + a, py);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a + b, py + a);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + b, py + a + b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px, py + b);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            } else {
                d = grid_get_dot(g, points, px + b, py);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a + b, py + b);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + a + b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px, py + a);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if x > 0 as libc::c_int {
                grid_face_add_new(g, 3 as libc::c_int);
                if (x + y) % 2 as libc::c_int != 0 {
                    d = grid_get_dot(g, points, px + a, py);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px, py + b);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px - a, py);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                } else {
                    d = grid_get_dot(g, points, px, py + a);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px + a, py + a + b);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px - a, py + a + b);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                }
            }
            if y > 0 as libc::c_int {
                grid_face_add_new(g, 3 as libc::c_int);
                if (x + y) % 2 as libc::c_int != 0 {
                    d = grid_get_dot(g, points, px + a, py);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px + a + b, py - a);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px + a + b, py + a);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                } else {
                    d = grid_get_dot(g, points, px, py - a);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px + b, py);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px, py + a);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_cairo(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut b: libc::c_int = 31 as libc::c_int;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int - 2 as libc::c_int * b) / (2 as libc::c_int * b)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - 2 as libc::c_int * b) / (2 as libc::c_int * b)
        || width > 2147483647 as libc::c_int / 2 as libc::c_int / height
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 3 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_cairo(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut b: libc::c_int = 31 as libc::c_int;
    *tilesize = 40 as libc::c_int;
    *xextent = 2 as libc::c_int * b * (width - 1 as libc::c_int) + 2 as libc::c_int * b;
    *yextent = 2 as libc::c_int * b * (height - 1 as libc::c_int) + 2 as libc::c_int * b;
}
unsafe extern "C" fn grid_new_cairo(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 14 as libc::c_int;
    let mut b: libc::c_int = 31 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 40 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = 2 as libc::c_int * b * x;
            let mut py: libc::c_int = 2 as libc::c_int * b * y;
            if y > 0 as libc::c_int {
                grid_face_add_new(g, 5 as libc::c_int);
                if (x + y) % 2 as libc::c_int != 0 {
                    d = grid_get_dot(g, points, px + a, py - b);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px + 2 as libc::c_int * b - a, py - b);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px + 2 as libc::c_int * b, py);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(g, points, px + b, py + a);
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(g, points, px, py);
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                } else {
                    d = grid_get_dot(g, points, px, py);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px + b, py - a);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px + 2 as libc::c_int * b, py);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(g, points, px + 2 as libc::c_int * b - a, py + b);
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(g, points, px + a, py + b);
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                }
            }
            if x > 0 as libc::c_int {
                grid_face_add_new(g, 5 as libc::c_int);
                if (x + y) % 2 as libc::c_int != 0 {
                    d = grid_get_dot(g, points, px, py);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px + b, py + a);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px + b, py + 2 as libc::c_int * b - a);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(g, points, px, py + 2 as libc::c_int * b);
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(g, points, px - a, py + b);
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                } else {
                    d = grid_get_dot(g, points, px, py);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px + a, py + b);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px, py + 2 as libc::c_int * b);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(g, points, px - b, py + 2 as libc::c_int * b - a);
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(g, points, px - b, py + a);
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_greathexagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int - 4 as libc::c_int * a) / (3 as libc::c_int * a + b)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - (3 as libc::c_int * b + a))
                / (2 as libc::c_int * a + 2 as libc::c_int * b)
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 6 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_greathexagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 18 as libc::c_int;
    *xextent = (3 as libc::c_int * a + b) * (width - 1 as libc::c_int)
        + 4 as libc::c_int * a;
    *yextent = (2 as libc::c_int * a + 2 as libc::c_int * b)
        * (height - 1 as libc::c_int) + 3 as libc::c_int * b + a;
}
unsafe extern "C" fn grid_new_greathexagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 18 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = (3 as libc::c_int * a + b) * x;
            let mut py: libc::c_int = (2 as libc::c_int * a + 2 as libc::c_int * b) * y;
            if x % 2 as libc::c_int != 0 {
                py += a + b;
            }
            grid_face_add_new(g, 6 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py - b);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py - b);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + 2 as libc::c_int * a, py);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + b);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py + b);
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, px - 2 as libc::c_int * a, py);
            grid_face_set_dot(g, d, 5 as libc::c_int);
            if y < height - 1 as libc::c_int {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + b);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + b);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + 2 as libc::c_int * a + b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + 2 as libc::c_int * a + b);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if x < width - 1 as libc::c_int
                && (x % 2 as libc::c_int == 0 as libc::c_int
                    || y < height - 1 as libc::c_int)
            {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px + 2 as libc::c_int * a, py);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + 2 as libc::c_int * a + b, py + a);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + a + b, py + a + b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + b);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if x > 0 as libc::c_int
                && (x % 2 as libc::c_int == 0 as libc::c_int
                    || y < height - 1 as libc::c_int)
            {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px - 2 as libc::c_int * a, py);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + b);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px - a - b, py + a + b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px - 2 as libc::c_int * a - b, py + a);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if x < width - 1 as libc::c_int && y < height - 1 as libc::c_int {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + b);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a + b, py + a + b);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + 2 as libc::c_int * a + b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            if x > 0 as libc::c_int && y < height - 1 as libc::c_int {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + b);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + 2 as libc::c_int * a + b);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px - a - b, py + a + b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_kagome(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int - 6 as libc::c_int * a) / (4 as libc::c_int * a)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - 2 as libc::c_int * b) / (2 as libc::c_int * b)
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 6 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_kagome(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 18 as libc::c_int;
    *xextent = 4 as libc::c_int * a * (width - 1 as libc::c_int) + 6 as libc::c_int * a;
    *yextent = 2 as libc::c_int * b * (height - 1 as libc::c_int) + 2 as libc::c_int * b;
}
unsafe extern "C" fn grid_new_kagome(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 18 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = 4 as libc::c_int * a * x;
            let mut py: libc::c_int = 2 as libc::c_int * b * y;
            if y % 2 as libc::c_int != 0 {
                px += 2 as libc::c_int * a;
            }
            grid_face_add_new(g, 6 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py - b);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + 2 as libc::c_int * a, py);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + b);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py + b);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, px - 2 as libc::c_int * a, py);
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py - b);
            grid_face_set_dot(g, d, 5 as libc::c_int);
            if x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0 && y != 0 {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + 3 as libc::c_int * a, py - b);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + 2 as libc::c_int * a, py);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            if x < width - 1 as libc::c_int
                || y % 2 as libc::c_int == 0 && y < height - 1 as libc::c_int
            {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + 3 as libc::c_int * a, py + b);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + b);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + 2 as libc::c_int * a, py);
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            if x == 0 && y % 2 as libc::c_int != 0 {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py - b);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px - 2 as libc::c_int * a, py);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px - 3 as libc::c_int * a, py - b);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                if y < height - 1 as libc::c_int {
                    grid_face_add_new(g, 3 as libc::c_int);
                    d = grid_get_dot(g, points, px - a, py + b);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(g, points, px - 3 as libc::c_int * a, py + b);
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(g, points, px - 2 as libc::c_int * a, py);
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_octagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 29 as libc::c_int;
    let mut b: libc::c_int = 41 as libc::c_int;
    if width > 2147483647 as libc::c_int / (2 as libc::c_int * a + b)
        || height > 2147483647 as libc::c_int / (2 as libc::c_int * a + b)
        || height + 1 as libc::c_int
            > 2147483647 as libc::c_int / 4 as libc::c_int / (width + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_octagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 29 as libc::c_int;
    let mut b: libc::c_int = 41 as libc::c_int;
    *tilesize = 40 as libc::c_int;
    *xextent = (2 as libc::c_int * a + b) * width;
    *yextent = (2 as libc::c_int * a + b) * height;
}
unsafe extern "C" fn grid_new_octagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 29 as libc::c_int;
    let mut b: libc::c_int = 41 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 40 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = (2 as libc::c_int * a + b) * x;
            let mut py: libc::c_int = (2 as libc::c_int * a + b) * y;
            grid_face_add_new(g, 8 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + a + b, py);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + 2 as libc::c_int * a + b, py + a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + 2 as libc::c_int * a + b, py + a + b);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, px + a + b, py + 2 as libc::c_int * a + b);
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + 2 as libc::c_int * a + b);
            grid_face_set_dot(g, d, 5 as libc::c_int);
            d = grid_get_dot(g, points, px, py + a + b);
            grid_face_set_dot(g, d, 6 as libc::c_int);
            d = grid_get_dot(g, points, px, py + a);
            grid_face_set_dot(g, d, 7 as libc::c_int);
            if x > 0 as libc::c_int && y > 0 as libc::c_int {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px, py - a);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px, py + a);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_kites(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width
        > (2147483647 as libc::c_int - 2 as libc::c_int * b) / (4 as libc::c_int * b)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - 8 as libc::c_int * a) / (6 as libc::c_int * a)
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 6 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_kites(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 40 as libc::c_int;
    *xextent = 4 as libc::c_int * b * width + 2 as libc::c_int * b;
    *yextent = 6 as libc::c_int * a * (height - 1 as libc::c_int) + 8 as libc::c_int * a;
}
unsafe extern "C" fn grid_new_kites(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 40 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = 4 as libc::c_int * b * x;
            let mut py: libc::c_int = 6 as libc::c_int * a * y;
            if y % 2 as libc::c_int != 0 {
                px += 2 as libc::c_int * b;
            }
            grid_face_add_new(g, 4 as libc::c_int);
            d = grid_get_dot(g, points, px, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + 2 as libc::c_int * b, py);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(
                g,
                points,
                px + 2 as libc::c_int * b,
                py + 2 as libc::c_int * a,
            );
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + b, py + 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            grid_face_add_new(g, 4 as libc::c_int);
            d = grid_get_dot(g, points, px, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + b, py + 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px, py + 4 as libc::c_int * a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px - b, py + 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            grid_face_add_new(g, 4 as libc::c_int);
            d = grid_get_dot(g, points, px, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px - b, py + 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(
                g,
                points,
                px - 2 as libc::c_int * b,
                py + 2 as libc::c_int * a,
            );
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px - 2 as libc::c_int * b, py);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            grid_face_add_new(g, 4 as libc::c_int);
            d = grid_get_dot(g, points, px, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px - 2 as libc::c_int * b, py);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(
                g,
                points,
                px - 2 as libc::c_int * b,
                py - 2 as libc::c_int * a,
            );
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px - b, py - 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            grid_face_add_new(g, 4 as libc::c_int);
            d = grid_get_dot(g, points, px, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px - b, py - 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px, py - 4 as libc::c_int * a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + b, py - 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            grid_face_add_new(g, 4 as libc::c_int);
            d = grid_get_dot(g, points, px, py);
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + b, py - 3 as libc::c_int * a);
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(
                g,
                points,
                px + 2 as libc::c_int * b,
                py - 2 as libc::c_int * a,
            );
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + 2 as libc::c_int * b, py);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_floret(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut px: libc::c_int = 75 as libc::c_int;
    let mut py: libc::c_int = -(26 as libc::c_int);
    let mut qx: libc::c_int = 4 as libc::c_int * px / 5 as libc::c_int;
    let mut qy: libc::c_int = -py * 2 as libc::c_int;
    let mut ry: libc::c_int = qy - py;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int - (4 as libc::c_int * qx + 2 as libc::c_int * px))
            / ((6 as libc::c_int * px + 3 as libc::c_int * qx) / 2 as libc::c_int)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int
                - (4 as libc::c_int * qy + 2 as libc::c_int * ry))
                / (5 as libc::c_int * qy - 4 as libc::c_int * py)
        || width + 1 as libc::c_int
            > 2147483647 as libc::c_int / 9 as libc::c_int / (height + 1 as libc::c_int)
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_floret(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut px: libc::c_int = 75 as libc::c_int;
    let mut py: libc::c_int = -(26 as libc::c_int);
    let mut qx: libc::c_int = 4 as libc::c_int * px / 5 as libc::c_int;
    let mut qy: libc::c_int = -py * 2 as libc::c_int;
    let mut ry: libc::c_int = qy - py;
    *tilesize = 150 as libc::c_int;
    *xextent = (6 as libc::c_int * px + 3 as libc::c_int * qx) / 2 as libc::c_int
        * (width - 1 as libc::c_int) + 4 as libc::c_int * qx + 2 as libc::c_int * px;
    *yextent = (5 as libc::c_int * qy - 4 as libc::c_int * py)
        * (height - 1 as libc::c_int) + 4 as libc::c_int * qy + 2 as libc::c_int * ry;
    if height == 1 as libc::c_int {
        *yextent += (5 as libc::c_int * qy - 4 as libc::c_int * py) / 2 as libc::c_int;
    }
}
unsafe extern "C" fn grid_new_floret(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut px: libc::c_int = 75 as libc::c_int;
    let mut py: libc::c_int = -(26 as libc::c_int);
    let mut qx: libc::c_int = 4 as libc::c_int * px / 5 as libc::c_int;
    let mut qy: libc::c_int = -py * 2 as libc::c_int;
    let mut rx: libc::c_int = qx - px;
    let mut ry: libc::c_int = qy - py;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 150 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        let mut current_block_70: u64;
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut cx: libc::c_int = (6 as libc::c_int * px + 3 as libc::c_int * qx)
                / 2 as libc::c_int * x;
            let mut cy: libc::c_int = (4 as libc::c_int * py - 5 as libc::c_int * qy)
                * y;
            if x % 2 as libc::c_int != 0 {
                cy -= (4 as libc::c_int * py - 5 as libc::c_int * qy) / 2 as libc::c_int;
                current_block_70 = 11650488183268122163;
            } else if y != 0 && y == height - 1 as libc::c_int {
                current_block_70 = 11875828834189669668;
            } else {
                current_block_70 = 11650488183268122163;
            }
            match current_block_70 {
                11650488183268122163 => {
                    grid_face_add_new(g, 5 as libc::c_int);
                    d = grid_get_dot(g, points, cx, cy);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * rx,
                        cy + 2 as libc::c_int * ry,
                    );
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * rx + qx,
                        cy + 2 as libc::c_int * ry + qy,
                    );
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * qx + rx,
                        cy + 2 as libc::c_int * qy + ry,
                    );
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * qx,
                        cy + 2 as libc::c_int * qy,
                    );
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                    grid_face_add_new(g, 5 as libc::c_int);
                    d = grid_get_dot(g, points, cx, cy);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * qx,
                        cy + 2 as libc::c_int * qy,
                    );
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * qx + px,
                        cy + 2 as libc::c_int * qy + py,
                    );
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * px + qx,
                        cy + 2 as libc::c_int * py + qy,
                    );
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * px,
                        cy + 2 as libc::c_int * py,
                    );
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                    grid_face_add_new(g, 5 as libc::c_int);
                    d = grid_get_dot(g, points, cx, cy);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * px,
                        cy + 2 as libc::c_int * py,
                    );
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * px - rx,
                        cy + 2 as libc::c_int * py - ry,
                    );
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * rx + px,
                        cy - 2 as libc::c_int * ry + py,
                    );
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * rx,
                        cy - 2 as libc::c_int * ry,
                    );
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                    grid_face_add_new(g, 5 as libc::c_int);
                    d = grid_get_dot(g, points, cx, cy);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * rx,
                        cy - 2 as libc::c_int * ry,
                    );
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * rx - qx,
                        cy - 2 as libc::c_int * ry - qy,
                    );
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * qx - rx,
                        cy - 2 as libc::c_int * qy - ry,
                    );
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * qx,
                        cy - 2 as libc::c_int * qy,
                    );
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                    grid_face_add_new(g, 5 as libc::c_int);
                    d = grid_get_dot(g, points, cx, cy);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * qx,
                        cy - 2 as libc::c_int * qy,
                    );
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * qx - px,
                        cy - 2 as libc::c_int * qy - py,
                    );
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * px - qx,
                        cy - 2 as libc::c_int * py - qy,
                    );
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * px,
                        cy - 2 as libc::c_int * py,
                    );
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                    grid_face_add_new(g, 5 as libc::c_int);
                    d = grid_get_dot(g, points, cx, cy);
                    grid_face_set_dot(g, d, 0 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * px,
                        cy - 2 as libc::c_int * py,
                    );
                    grid_face_set_dot(g, d, 1 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx - 2 as libc::c_int * px + rx,
                        cy - 2 as libc::c_int * py + ry,
                    );
                    grid_face_set_dot(g, d, 2 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * rx - px,
                        cy + 2 as libc::c_int * ry - py,
                    );
                    grid_face_set_dot(g, d, 3 as libc::c_int);
                    d = grid_get_dot(
                        g,
                        points,
                        cx + 2 as libc::c_int * rx,
                        cy + 2 as libc::c_int * ry,
                    );
                    grid_face_set_dot(g, d, 4 as libc::c_int);
                }
                _ => {}
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_dodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int - 3 as libc::c_int * (2 as libc::c_int * a + b))
            / (4 as libc::c_int * a + 2 as libc::c_int * b)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - 2 as libc::c_int * (2 as libc::c_int * a + b))
                / (3 as libc::c_int * a + 2 as libc::c_int * b)
        || width > 2147483647 as libc::c_int / 14 as libc::c_int / height
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_dodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 26 as libc::c_int;
    *xextent = (4 as libc::c_int * a + 2 as libc::c_int * b) * (width - 1 as libc::c_int)
        + 3 as libc::c_int * (2 as libc::c_int * a + b);
    *yextent = (3 as libc::c_int * a + 2 as libc::c_int * b)
        * (height - 1 as libc::c_int) + 2 as libc::c_int * (2 as libc::c_int * a + b);
}
unsafe extern "C" fn grid_new_dodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 26 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = (4 as libc::c_int * a + 2 as libc::c_int * b) * x;
            let mut py: libc::c_int = (3 as libc::c_int * a + 2 as libc::c_int * b) * y;
            if y % 2 as libc::c_int != 0 {
                px += 2 as libc::c_int * a + b;
            }
            grid_face_add_new(g, 12 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py - (a + b));
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py + (a + b));
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 5 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 6 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py + (a + b));
            grid_face_set_dot(g, d, 7 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 8 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 9 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py - (a + b));
            grid_face_set_dot(g, d, 10 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 11 as libc::c_int);
            if y < height - 1 as libc::c_int
                && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0)
                && (x > 0 as libc::c_int || y % 2 as libc::c_int != 0)
            {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px,
                    py + (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            if y != 0 && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0)
                && (x > 0 as libc::c_int || y % 2 as libc::c_int != 0)
            {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px,
                    py - (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_greatdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int
            - (2 as libc::c_int * (2 as libc::c_int * a + b) + 3 as libc::c_int * a + b))
            / (6 as libc::c_int * a + 2 as libc::c_int * b)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - 2 as libc::c_int * (2 as libc::c_int * a + b))
                / (3 as libc::c_int * a + 3 as libc::c_int * b)
        || width > 2147483647 as libc::c_int / 200 as libc::c_int / height
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_greatdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 26 as libc::c_int;
    *xextent = (6 as libc::c_int * a + 2 as libc::c_int * b) * (width - 1 as libc::c_int)
        + 2 as libc::c_int * (2 as libc::c_int * a + b) + 3 as libc::c_int * a + b;
    *yextent = (3 as libc::c_int * a + 3 as libc::c_int * b)
        * (height - 1 as libc::c_int) + 2 as libc::c_int * (2 as libc::c_int * a + b);
}
unsafe extern "C" fn grid_new_greatdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 26 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = (6 as libc::c_int * a + 2 as libc::c_int * b) * x;
            let mut py: libc::c_int = (3 as libc::c_int * a + 3 as libc::c_int * b) * y;
            if y % 2 as libc::c_int != 0 {
                px += 3 as libc::c_int * a + b;
            }
            grid_face_add_new(g, 12 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py - (a + b));
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py + (a + b));
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 5 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 6 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py + (a + b));
            grid_face_set_dot(g, d, 7 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 8 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 9 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py - (a + b));
            grid_face_set_dot(g, d, 10 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 11 as libc::c_int);
            if y < height - 1 as libc::c_int
                && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0)
                && (x > 0 as libc::c_int || y % 2 as libc::c_int != 0)
            {
                grid_face_add_new(g, 6 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + 2 as libc::c_int * a,
                    py + (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + a,
                    py + (2 as libc::c_int * a + 3 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - a,
                    py + (2 as libc::c_int * a + 3 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 3 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - 2 as libc::c_int * a,
                    py + (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 4 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 5 as libc::c_int);
            }
            if y != 0 && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0)
                && (x > 0 as libc::c_int || y % 2 as libc::c_int != 0)
            {
                grid_face_add_new(g, 6 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - 2 as libc::c_int * a,
                    py - (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - a,
                    py - (2 as libc::c_int * a + 3 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + a,
                    py - (2 as libc::c_int * a + 3 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 3 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + 2 as libc::c_int * a,
                    py - (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 4 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 5 as libc::c_int);
            }
            if x < width - 1 as libc::c_int {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px + 2 as libc::c_int * a + b, py - a);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + 4 as libc::c_int * a + b, py - a);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + 4 as libc::c_int * a + b, py + a);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + 2 as libc::c_int * a + b, py + a);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if y != 0 && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0) {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + 2 as libc::c_int * a,
                    py - (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + b),
                    py - (a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py - (a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if y != 0 && (x != 0 || y % 2 as libc::c_int != 0) {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px - (a + b), py - (a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - (2 as libc::c_int * a + b),
                    py - (a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - 2 as libc::c_int * a,
                    py - (2 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_greatgreatdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width - 1 as libc::c_int
        > (2147483647 as libc::c_int
            - (2 as libc::c_int * (2 as libc::c_int * a + b) + 2 as libc::c_int * a
                + 2 as libc::c_int * b)) / (4 as libc::c_int * a + 4 as libc::c_int * b)
        || height - 1 as libc::c_int
            > (2147483647 as libc::c_int - 2 as libc::c_int * (2 as libc::c_int * a + b))
                / (6 as libc::c_int * a + 2 as libc::c_int * b)
        || width > 2147483647 as libc::c_int / 300 as libc::c_int / height
    {
        return b"Grid size must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_greatgreatdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 26 as libc::c_int;
    *xextent = (4 as libc::c_int * a + 4 as libc::c_int * b) * (width - 1 as libc::c_int)
        + 2 as libc::c_int * (2 as libc::c_int * a + b) + 2 as libc::c_int * a
        + 2 as libc::c_int * b;
    *yextent = (6 as libc::c_int * a + 2 as libc::c_int * b)
        * (height - 1 as libc::c_int) + 2 as libc::c_int * (2 as libc::c_int * a + b);
}
unsafe extern "C" fn grid_new_greatgreatdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 26 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = (4 as libc::c_int * a + 4 as libc::c_int * b) * x;
            let mut py: libc::c_int = (6 as libc::c_int * a + 2 as libc::c_int * b) * y;
            if y % 2 as libc::c_int != 0 {
                px += 2 as libc::c_int * a + 2 as libc::c_int * b;
            }
            grid_face_add_new(g, 12 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py - (a + b));
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py + (a + b));
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 5 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 6 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py + (a + b));
            grid_face_set_dot(g, d, 7 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 8 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 9 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py - (a + b));
            grid_face_set_dot(g, d, 10 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 11 as libc::c_int);
            if y != 0 && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0) {
                grid_face_add_new(g, 6 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py - (4 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py - (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py - (a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - (4 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 4 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + b),
                    py - (5 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 5 as libc::c_int);
            }
            if x < width - 1 as libc::c_int {
                grid_face_add_new(g, 6 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 3 as libc::c_int * b),
                    py - a,
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 3 as libc::c_int * b),
                    py + a,
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py + 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py + a);
                grid_face_set_dot(g, d, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py - a);
                grid_face_set_dot(g, d, 4 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py - 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 5 as libc::c_int);
            }
            if y < height - 1 as libc::c_int
                && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0)
            {
                grid_face_add_new(g, 6 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py + (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py + (4 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + b),
                    py + (5 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + (4 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 4 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py + (a + b));
                grid_face_set_dot(g, d, 5 as libc::c_int);
            }
            if y != 0 && x < width - 1 as libc::c_int {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py - (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py - 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py - a);
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py - (a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if y < height - 1 as libc::c_int && x < width - 1 as libc::c_int {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py + 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py + (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py + (a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py + a);
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if y < height - 1 as libc::c_int
                && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0)
                && (x > 0 as libc::c_int || y % 2 as libc::c_int != 0)
            {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + (4 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + (4 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if x != 0 && y < height - 1 as libc::c_int {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py + a);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px - (a + b), py + (a + b));
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - (a + 2 as libc::c_int * b),
                    py + (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py + 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if x != 0 && y != 0 {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px - (a + b), py - (a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py - a);
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py - 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px - (a + 2 as libc::c_int * b),
                    py - (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if y != 0 && (x < width - 1 as libc::c_int || y % 2 as libc::c_int == 0)
                && (x > 0 as libc::c_int || y % 2 as libc::c_int != 0)
            {
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - (4 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px - a, py - (4 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            if y != 0 && x < width - 1 as libc::c_int {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + 2 as libc::c_int * b),
                    py - (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py - 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py - (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            if y < height - 1 as libc::c_int && x < width - 1 as libc::c_int {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + 2 as libc::c_int * b),
                    py + (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + 2 as libc::c_int * b),
                    py + (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + 2 as libc::c_int * b),
                    py + 2 as libc::c_int * a,
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_compassdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    if width > 2147483647 as libc::c_int / (4 as libc::c_int * a + 2 as libc::c_int * b)
        || height
            > 2147483647 as libc::c_int / (4 as libc::c_int * a + 2 as libc::c_int * b)
        || width > 2147483647 as libc::c_int / 18 as libc::c_int / height
    {
        return b"Grid must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_compassdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    *tilesize = 26 as libc::c_int;
    *xextent = (4 as libc::c_int * a + 2 as libc::c_int * b) * width;
    *yextent = (4 as libc::c_int * a + 2 as libc::c_int * b) * height;
}
unsafe extern "C" fn grid_new_compassdodecagonal(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut a: libc::c_int = 15 as libc::c_int;
    let mut b: libc::c_int = 26 as libc::c_int;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = grid_empty();
    (*g).tilesize = 26 as libc::c_int;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    y = 0 as libc::c_int;
    while y < height {
        x = 0 as libc::c_int;
        while x < width {
            let mut d: *mut grid_dot = 0 as *mut grid_dot;
            let mut px: libc::c_int = (4 as libc::c_int * a + 2 as libc::c_int * b) * x;
            let mut py: libc::c_int = (4 as libc::c_int * a + 2 as libc::c_int * b) * y;
            grid_face_add_new(g, 12 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 0 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py - (a + b));
            grid_face_set_dot(g, d, 1 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 2 as libc::c_int);
            d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 3 as libc::c_int);
            d = grid_get_dot(g, points, px + (a + b), py + (a + b));
            grid_face_set_dot(g, d, 4 as libc::c_int);
            d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 5 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py + (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 6 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py + (a + b));
            grid_face_set_dot(g, d, 7 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py + a);
            grid_face_set_dot(g, d, 8 as libc::c_int);
            d = grid_get_dot(g, points, px - (2 as libc::c_int * a + b), py - a);
            grid_face_set_dot(g, d, 9 as libc::c_int);
            d = grid_get_dot(g, points, px - (a + b), py - (a + b));
            grid_face_set_dot(g, d, 10 as libc::c_int);
            d = grid_get_dot(g, points, px - a, py - (2 as libc::c_int * a + b));
            grid_face_set_dot(g, d, 11 as libc::c_int);
            if x < width - 1 as libc::c_int && y < height - 1 as libc::c_int {
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + (2 as libc::c_int * a + b), py + a);
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + b),
                    py + (a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py + (a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + 2 as libc::c_int * b),
                    py + (2 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + b),
                    py + (3 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + b),
                    py + (a + b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + b),
                    py + (3 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (2 as libc::c_int * a + b),
                    py + (3 as libc::c_int * a + 2 as libc::c_int * b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + b),
                    py + (3 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                grid_face_add_new(g, 3 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py + (a + b));
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + b),
                    py + (3 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(g, points, px + a, py + (2 as libc::c_int * a + b));
                grid_face_set_dot(g, d, 2 as libc::c_int);
                grid_face_add_new(g, 4 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + b),
                    py + (a + b),
                );
                grid_face_set_dot(g, d, 0 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (3 as libc::c_int * a + b),
                    py + (3 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 1 as libc::c_int);
                d = grid_get_dot(
                    g,
                    points,
                    px + (a + b),
                    py + (3 as libc::c_int * a + b),
                );
                grid_face_set_dot(g, d, 2 as libc::c_int);
                d = grid_get_dot(g, points, px + (a + b), py + (a + b));
                grid_face_set_dot(g, d, 3 as libc::c_int);
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    freetree234(points);
    grid_make_consistent(g);
    return g;
}
unsafe extern "C" fn grid_validate_params_penrose(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut l: libc::c_int = 100 as libc::c_int;
    if width > 2147483647 as libc::c_int / l || height > 2147483647 as libc::c_int / l
        || width
            > 2147483647 as libc::c_int
                / (3 as libc::c_int * 3 as libc::c_int * 4 as libc::c_int * height)
    {
        return b"Grid must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_penrose(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    let mut l: libc::c_int = 100 as libc::c_int;
    *tilesize = l;
    *xextent = l * width;
    *yextent = l * height;
}
unsafe extern "C" fn round_int_nearest_away(mut r: libc::c_double) -> libc::c_double {
    return if r > 0.0f64 { __tg_floor(r + 0.5f64) } else { __tg_ceil(r - 0.5f64) };
}
unsafe extern "C" fn penrose_legacy_set_faces(
    mut state: *mut penrose_legacy_state,
    mut vs: *mut vector,
    mut n: libc::c_int,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut sf_ctx: *mut penrose_legacy_set_faces_ctx = (*state).ctx
        as *mut penrose_legacy_set_faces_ctx;
    let mut i: libc::c_int = 0;
    let mut xs: [libc::c_int; 4] = [0; 4];
    let mut ys: [libc::c_int; 4] = [0; 4];
    if depth < (*state).max_depth {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        let mut tx: libc::c_double = penrose_legacy_vx(vs, i);
        let mut ty: libc::c_double = penrose_legacy_vy(vs, i);
        xs[i as usize] = round_int_nearest_away(tx) as libc::c_int;
        ys[i as usize] = round_int_nearest_away(ty) as libc::c_int;
        if xs[i as usize] < (*sf_ctx).xmin || xs[i as usize] > (*sf_ctx).xmax {
            return 0 as libc::c_int;
        }
        if ys[i as usize] < (*sf_ctx).ymin || ys[i as usize] > (*sf_ctx).ymax {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    grid_face_add_new((*sf_ctx).g, n);
    i = 0 as libc::c_int;
    while i < n {
        let mut d: *mut grid_dot = grid_get_dot(
            (*sf_ctx).g,
            (*sf_ctx).points,
            xs[i as usize],
            ys[i as usize],
        );
        grid_face_set_dot((*sf_ctx).g, d, i);
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn grid_validate_desc_penrose_legacy(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *const libc::c_char {
    let mut tilesize: libc::c_int = 100 as libc::c_int;
    let mut startsz: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    let mut xoff: libc::c_int = 0;
    let mut yoff: libc::c_int = 0;
    let mut aoff: libc::c_int = 0;
    let mut inner_radius: libc::c_int = 0;
    let mut outer_radius: libc::c_double = 0.;
    let mut which: libc::c_int = if type_0 as libc::c_uint
        == GRID_PENROSE_P2 as libc::c_int as libc::c_uint
    {
        PENROSE_P2 as libc::c_int
    } else {
        PENROSE_P3 as libc::c_int
    };
    let mut g: *mut grid = 0 as *mut grid;
    if desc.is_null() {
        return b"Missing grid description string.\0" as *const u8 as *const libc::c_char;
    }
    penrose_legacy_calculate_size(
        which,
        tilesize,
        width,
        height,
        &mut outer_radius,
        &mut startsz,
        &mut depth,
    );
    inner_radius = (outer_radius
        - __tg_sqrt((width * width + height * height) as libc::c_double)) as libc::c_int;
    if sscanf(
        desc,
        b"G%d,%d,%d\0" as *const u8 as *const libc::c_char,
        &mut xoff as *mut libc::c_int,
        &mut yoff as *mut libc::c_int,
        &mut aoff as *mut libc::c_int,
    ) != 3 as libc::c_int
    {
        return b"Invalid format grid description string.\0" as *const u8
            as *const libc::c_char;
    }
    if __tg_sqrt((xoff * xoff + yoff * yoff) as libc::c_double)
        > inner_radius as libc::c_double
    {
        return b"Patch offset out of bounds.\0" as *const u8 as *const libc::c_char;
    }
    if aoff % 36 as libc::c_int != 0 as libc::c_int || aoff < 0 as libc::c_int
        || aoff >= 360 as libc::c_int
    {
        return b"Angle offset out of bounds.\0" as *const u8 as *const libc::c_char;
    }
    g = grid_new_penrose_legacy(width, height, which, desc);
    if g.is_null() {
        return b"Patch coordinates do not identify a usable grid fragment\0" as *const u8
            as *const libc::c_char;
    }
    grid_free(g);
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_new_penrose_legacy(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut which: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut tilesize: libc::c_int = 100 as libc::c_int;
    let mut xsz: libc::c_int = 0;
    let mut ysz: libc::c_int = 0;
    let mut xoff: libc::c_int = 0;
    let mut yoff: libc::c_int = 0;
    let mut aoff: libc::c_int = 0;
    let mut rradius: libc::c_double = 0.;
    let mut points: *mut tree234 = 0 as *mut tree234;
    let mut g: *mut grid = 0 as *mut grid;
    let mut ps: penrose_legacy_state = penrose_legacy_state {
        start_size: 0,
        max_depth: 0,
        new_tile: None,
        ctx: 0 as *mut libc::c_void,
    };
    let mut sf_ctx: penrose_legacy_set_faces_ctx = penrose_legacy_set_faces_ctx {
        xmin: 0,
        xmax: 0,
        ymin: 0,
        ymax: 0,
        g: 0 as *mut grid,
        points: 0 as *mut tree234,
    };
    penrose_legacy_calculate_size(
        which,
        tilesize,
        width,
        height,
        &mut rradius,
        &mut ps.start_size,
        &mut ps.max_depth,
    );
    ps
        .new_tile = Some(
        penrose_legacy_set_faces
            as unsafe extern "C" fn(
                *mut penrose_legacy_state,
                *mut vector,
                libc::c_int,
                libc::c_int,
            ) -> libc::c_int,
    );
    ps.ctx = &mut sf_ctx as *mut penrose_legacy_set_faces_ctx as *mut libc::c_void;
    g = grid_empty();
    (*g).tilesize = tilesize;
    points = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    memset(
        &mut sf_ctx as *mut penrose_legacy_set_faces_ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<penrose_legacy_set_faces_ctx>() as libc::c_ulong,
    );
    sf_ctx.g = g;
    sf_ctx.points = points;
    if !desc.is_null() {
        if sscanf(
            desc,
            b"G%d,%d,%d\0" as *const u8 as *const libc::c_char,
            &mut xoff as *mut libc::c_int,
            &mut yoff as *mut libc::c_int,
            &mut aoff as *mut libc::c_int,
        ) != 3 as libc::c_int
        {
            if (b"Invalid grid description.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {} else {
                __assert_fail(
                    b"!\"Invalid grid description.\"\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    3187 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 59],
                        &[libc::c_char; 59],
                    >(b"grid *grid_new_penrose_legacy(int, int, int, const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_13963: {
                if (b"Invalid grid description.\0" as *const u8 as *const libc::c_char)
                    .is_null()
                {} else {
                    __assert_fail(
                        b"!\"Invalid grid description.\"\0" as *const u8
                            as *const libc::c_char,
                        b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                        3187 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 59],
                            &[libc::c_char; 59],
                        >(
                            b"grid *grid_new_penrose_legacy(int, int, int, const char *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
    } else {
        aoff = 0 as libc::c_int;
        yoff = aoff;
        xoff = yoff;
    }
    xsz = width * tilesize;
    ysz = height * tilesize;
    sf_ctx.xmin = xoff - xsz / 2 as libc::c_int;
    sf_ctx.xmax = xoff + xsz / 2 as libc::c_int;
    sf_ctx.ymin = yoff - ysz / 2 as libc::c_int;
    sf_ctx.ymax = yoff + ysz / 2 as libc::c_int;
    penrose_legacy(&mut ps, which, aoff);
    freetree234(points);
    if (*g).num_faces == 0 as libc::c_int || (*g).num_dots == 0 as libc::c_int {
        grid_free(g);
        return 0 as *mut grid;
    }
    grid_trim_vigorously(g);
    if (*g).num_faces == 0 as libc::c_int || (*g).num_dots == 0 as libc::c_int {
        grid_free(g);
        return 0 as *mut grid;
    }
    grid_make_consistent(g);
    (*g).lowest_x
        -= (sf_ctx.xmax - sf_ctx.xmin - ((*g).highest_x - (*g).lowest_x))
            / 2 as libc::c_int;
    (*g).highest_x = (*g).lowest_x + (sf_ctx.xmax - sf_ctx.xmin);
    (*g).lowest_y
        -= (sf_ctx.ymax - sf_ctx.ymin - ((*g).highest_y - (*g).lowest_y))
            / 2 as libc::c_int;
    (*g).highest_y = (*g).lowest_y + (sf_ctx.ymax - sf_ctx.ymin);
    return g;
}
unsafe extern "C" fn api_size_penrose(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut which: libc::c_int,
) -> size {
    let mut xunit: libc::c_int = if which == PENROSE_P2 as libc::c_int {
        37 as libc::c_int
    } else {
        30 as libc::c_int
    };
    let mut yunit: libc::c_int = if which == PENROSE_P2 as libc::c_int {
        44 as libc::c_int
    } else {
        35 as libc::c_int
    };
    let mut size: size = {
        let mut init = size {
            w: width * 100 as libc::c_int / yunit,
            h: height * 100 as libc::c_int / xunit,
        };
        init
    };
    return size;
}
unsafe extern "C" fn grid_new_desc_penrose(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut rs: *mut random_state,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut params: PenrosePatchParams = PenrosePatchParams {
        start_vertex: 0,
        orientation: 0,
        ncoords: 0,
        coords: 0 as *mut libc::c_char,
    };
    let mut which: libc::c_int = if type_0 as libc::c_uint
        == GRID_PENROSE_P2 as libc::c_int as libc::c_uint
    {
        PENROSE_P2 as libc::c_int
    } else {
        PENROSE_P3 as libc::c_int
    };
    let mut size: size = api_size_penrose(width, height, which);
    penrose_tiling_randomise(&mut params, which, size.h, size.w, rs);
    buf = smalloc(
        (params.ncoords)
            .wrapping_add(3 as libc::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = ('0' as i32 + params.orientation) as libc::c_char;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = ('0' as i32 as libc::c_uint).wrapping_add(params.start_vertex)
        as libc::c_char;
    memcpy(
        buf.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        params.coords as *const libc::c_void,
        params.ncoords,
    );
    *buf
        .offset(
            (2 as libc::c_int as size_t).wrapping_add(params.ncoords) as isize,
        ) = '\0' as i32 as libc::c_char;
    sfree(params.coords as *mut libc::c_void);
    return buf;
}
unsafe extern "C" fn grid_desc_to_penrose_params(
    mut desc: *const libc::c_char,
    mut which: libc::c_int,
    mut params: *mut PenrosePatchParams,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    if *desc == 0 {
        return b"empty grid description\0" as *const u8 as *const libc::c_char;
    }
    (*params).ncoords = (strlen(desc)).wrapping_sub(2 as libc::c_int as libc::c_ulong);
    (*params)
        .coords = smalloc(
        ((*params).ncoords)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut c: libc::c_char = *desc.offset(0 as libc::c_int as isize);
    if *(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        (*params).orientation = c as libc::c_int - '0' as i32;
    } else {
        return b"expected digit at start of grid description\0" as *const u8
            as *const libc::c_char
    }
    c = *desc.offset(1 as libc::c_int as isize);
    if c as libc::c_int >= '0' as i32 && (c as libc::c_int) < '3' as i32 {
        (*params).start_vertex = (c as libc::c_int - '0' as i32) as libc::c_uint;
    } else {
        return b"expected digit as second char of grid description\0" as *const u8
            as *const libc::c_char
    }
    i = 0 as libc::c_int;
    while (i as size_t) < (*params).ncoords {
        let mut c_0: libc::c_char = *desc.offset((i + 2 as libc::c_int) as isize);
        if !penrose_valid_letter(c_0, which) {
            return b"expected tile letter in grid description\0" as *const u8
                as *const libc::c_char;
        }
        *((*params).coords).offset(i as isize) = c_0;
        i += 1;
        i;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_validate_desc_penrose(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *const libc::c_char {
    let mut params: PenrosePatchParams = PenrosePatchParams {
        start_vertex: 0,
        orientation: 0,
        ncoords: 0,
        coords: 0 as *mut libc::c_char,
    };
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut which: libc::c_int = if type_0 as libc::c_uint
        == GRID_PENROSE_P2 as libc::c_int as libc::c_uint
    {
        PENROSE_P2 as libc::c_int
    } else {
        PENROSE_P3 as libc::c_int
    };
    if desc.is_null() {
        return b"Missing grid description string.\0" as *const u8 as *const libc::c_char;
    }
    if *desc as libc::c_int == 'G' as i32 {
        return grid_validate_desc_penrose_legacy(type_0, width, height, desc);
    }
    error = grid_desc_to_penrose_params(desc, which, &mut params);
    if error.is_null() {
        error = penrose_tiling_params_invalid(&mut params, which);
    }
    sfree(params.coords as *mut libc::c_void);
    return error;
}
unsafe extern "C" fn grid_penrose_callback(
    mut vctx: *mut libc::c_void,
    mut coords: *const libc::c_int,
) {
    let mut ctx: *mut penrosecontext = vctx as *mut penrosecontext;
    let mut i: size_t = 0;
    grid_face_add_new((*ctx).g, 4 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < 4 as libc::c_int as size_t {
        let mut d: *mut grid_dot = grid_get_dot(
            (*ctx).g,
            (*ctx).points,
            *coords
                .offset(
                    (4 as libc::c_int as size_t * i)
                        .wrapping_add(2 as libc::c_int as size_t) as isize,
                ) * (*ctx).yunit
                + n_times_root_k(
                    *coords
                        .offset(
                            (4 as libc::c_int as size_t * i)
                                .wrapping_add(3 as libc::c_int as size_t) as isize,
                        ) * (*ctx).yunit,
                    5 as libc::c_int,
                ),
            *coords
                .offset(
                    (4 as libc::c_int as size_t * i)
                        .wrapping_add(0 as libc::c_int as size_t) as isize,
                ) * (*ctx).xunit
                + n_times_root_k(
                    *coords
                        .offset(
                            (4 as libc::c_int as size_t * i)
                                .wrapping_add(1 as libc::c_int as size_t) as isize,
                        ) * (*ctx).xunit,
                    5 as libc::c_int,
                ),
        );
        grid_face_set_dot((*ctx).g, d, i as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn grid_new_penrose(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut which: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut params: PenrosePatchParams = PenrosePatchParams {
        start_vertex: 0,
        orientation: 0,
        ncoords: 0,
        coords: 0 as *mut libc::c_char,
    };
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut ctx: [penrosecontext; 1] = [penrosecontext {
        g: 0 as *mut grid,
        points: 0 as *mut tree234,
        xunit: 0,
        yunit: 0,
    }; 1];
    let mut size: size = size { w: 0, h: 0 };
    if *desc as libc::c_int == 'G' as i32 {
        return grid_new_penrose_legacy(width, height, which, desc);
    }
    error = grid_desc_to_penrose_params(desc, which, &mut params);
    if error.is_null()
        && !(b"grid_validate_desc_penrose should have failed\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"error == NULL && \"grid_validate_desc_penrose should have failed\"\0"
                as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            3416 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"grid *grid_new_penrose(int, int, int, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_16295: {
        if error.is_null()
            && !(b"grid_validate_desc_penrose should have failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"error == NULL && \"grid_validate_desc_penrose should have failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3416 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"grid *grid_new_penrose(int, int, int, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let ref mut fresh43 = (*ctx.as_mut_ptr()).g;
    *fresh43 = grid_empty();
    (*(*ctx.as_mut_ptr()).g).tilesize = 100 as libc::c_int;
    let ref mut fresh44 = (*ctx.as_mut_ptr()).points;
    *fresh44 = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    (*ctx.as_mut_ptr())
        .xunit = if which == PENROSE_P2 as libc::c_int {
        37 as libc::c_int
    } else {
        30 as libc::c_int
    };
    (*ctx.as_mut_ptr())
        .yunit = if which == PENROSE_P2 as libc::c_int {
        44 as libc::c_int
    } else {
        35 as libc::c_int
    };
    size = api_size_penrose(width, height, which);
    penrose_tiling_generate(
        &mut params,
        size.h,
        size.w,
        Some(
            grid_penrose_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_int) -> (),
        ),
        ctx.as_mut_ptr() as *mut libc::c_void,
    );
    freetree234((*ctx.as_mut_ptr()).points);
    sfree(params.coords as *mut libc::c_void);
    grid_trim_vigorously((*ctx.as_mut_ptr()).g);
    grid_make_consistent((*ctx.as_mut_ptr()).g);
    let mut w: libc::c_int = width * 100 as libc::c_int;
    let mut h: libc::c_int = height * 100 as libc::c_int;
    (*(*ctx.as_mut_ptr()).g).lowest_x
        -= (w - ((*(*ctx.as_mut_ptr()).g).highest_x - (*(*ctx.as_mut_ptr()).g).lowest_x))
            / 2 as libc::c_int;
    (*(*ctx.as_mut_ptr()).g).lowest_y
        -= (h - ((*(*ctx.as_mut_ptr()).g).highest_y - (*(*ctx.as_mut_ptr()).g).lowest_y))
            / 2 as libc::c_int;
    (*(*ctx.as_mut_ptr()).g).highest_x = (*(*ctx.as_mut_ptr()).g).lowest_x + w;
    (*(*ctx.as_mut_ptr()).g).highest_y = (*(*ctx.as_mut_ptr()).g).lowest_y + h;
    return (*ctx.as_mut_ptr()).g;
}
unsafe extern "C" fn grid_validate_params_penrose_p2_kite(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    return grid_validate_params_penrose(width, height);
}
unsafe extern "C" fn grid_validate_params_penrose_p3_thick(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    return grid_validate_params_penrose(width, height);
}
unsafe extern "C" fn grid_size_penrose_p2_kite(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    grid_size_penrose(width, height, tilesize, xextent, yextent);
}
unsafe extern "C" fn grid_size_penrose_p3_thick(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    grid_size_penrose(width, height, tilesize, xextent, yextent);
}
unsafe extern "C" fn grid_new_penrose_p2_kite(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    return grid_new_penrose(width, height, PENROSE_P2 as libc::c_int, desc);
}
unsafe extern "C" fn grid_new_penrose_p3_thick(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    return grid_new_penrose(width, height, PENROSE_P3 as libc::c_int, desc);
}
unsafe extern "C" fn grid_validate_params_hats(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut l: libc::c_int = 32 as libc::c_int;
    if width > 2147483647 as libc::c_int / l || height > 2147483647 as libc::c_int / l
        || width > 2147483647 as libc::c_int / (6 as libc::c_int * height)
    {
        return b"Grid must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_hats(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    *tilesize = 32 as libc::c_int;
    *xextent = width * 14 as libc::c_int * 4 as libc::c_int;
    *yextent = height * 8 as libc::c_int * 6 as libc::c_int;
}
unsafe extern "C" fn grid_new_desc_hats(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut rs: *mut random_state,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufmax: size_t = 0;
    let mut i: size_t = 0;
    let mut hp: HatPatchParams = HatPatchParams {
        ncoords: 0,
        coords: 0 as *mut libc::c_uchar,
        final_metatile: 0,
    };
    hat_tiling_randomise(&mut hp, width, height, rs);
    bufmax = (3 as libc::c_int as size_t * hp.ncoords)
        .wrapping_add(2 as libc::c_int as size_t);
    buf = smalloc(
        bufmax.wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    p = buf;
    i = 0 as libc::c_int as size_t;
    while i < hp.ncoords {
        if (*(hp.coords).offset(i as isize) as libc::c_int) < 100 as libc::c_int
        {} else {
            __assert_fail(
                b"hp.coords[i] < 100\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3521 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"char *grid_new_desc_hats(grid_type, int, int, random_state *)\0"))
                    .as_ptr(),
            );
        }
        'c_9291: {
            if (*(hp.coords).offset(i as isize) as libc::c_int) < 100 as libc::c_int
            {} else {
                __assert_fail(
                    b"hp.coords[i] < 100\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    3521 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"char *grid_new_desc_hats(grid_type, int, int, random_state *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if p.offset_from(buf) as libc::c_long as size_t
            <= bufmax.wrapping_sub(4 as libc::c_int as size_t)
        {} else {
            __assert_fail(
                b"p - buf <= bufmax-4\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3522 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"char *grid_new_desc_hats(grid_type, int, int, random_state *)\0"))
                    .as_ptr(),
            );
        }
        'c_9239: {
            if p.offset_from(buf) as libc::c_long as size_t
                <= bufmax.wrapping_sub(4 as libc::c_int as size_t)
            {} else {
                __assert_fail(
                    b"p - buf <= bufmax-4\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    3522 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 62],
                        &[libc::c_char; 62],
                    >(
                        b"char *grid_new_desc_hats(grid_type, int, int, random_state *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        p = p
            .offset(
                sprintf(
                    p,
                    b"%d,\0" as *const u8 as *const libc::c_char,
                    *(hp.coords).offset(i as isize) as libc::c_int,
                ) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    if p.offset_from(buf) as libc::c_long as size_t
        <= bufmax.wrapping_sub(2 as libc::c_int as size_t)
    {} else {
        __assert_fail(
            b"p - buf <= bufmax-2\0" as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            3525 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[libc::c_char; 62],
            >(b"char *grid_new_desc_hats(grid_type, int, int, random_state *)\0"))
                .as_ptr(),
        );
    }
    'c_9157: {
        if p.offset_from(buf) as libc::c_long as size_t
            <= bufmax.wrapping_sub(2 as libc::c_int as size_t)
        {} else {
            __assert_fail(
                b"p - buf <= bufmax-2\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3525 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[libc::c_char; 62],
                >(b"char *grid_new_desc_hats(grid_type, int, int, random_state *)\0"))
                    .as_ptr(),
            );
        }
    };
    *p.offset(0 as libc::c_int as isize) = hp.final_metatile;
    *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    sfree(hp.coords as *mut libc::c_void);
    return buf;
}
unsafe extern "C" fn grid_desc_to_hat_params(
    mut desc: *const libc::c_char,
    mut hp: *mut HatPatchParams,
) -> *const libc::c_char {
    let mut maxcoords: size_t = 0;
    let mut p: *const libc::c_char = desc;
    maxcoords = (strlen(desc))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    (*hp)
        .coords = smalloc(
        maxcoords.wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    (*hp).ncoords = 0 as libc::c_int as size_t;
    while *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut p_orig: *const libc::c_char = p;
        let mut n: libc::c_int = atoi(p);
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int != ',' as i32 {
            return b"expected ',' in grid description\0" as *const u8
                as *const libc::c_char;
        }
        if p.offset_from(p_orig) as libc::c_long > 2 as libc::c_int as libc::c_long
            || n > 0xff as libc::c_int
        {
            return b"too-large coordinate in grid description\0" as *const u8
                as *const libc::c_char;
        }
        p = p.offset(1);
        p;
        if (*hp).ncoords < maxcoords {} else {
            __assert_fail(
                b"hp->ncoords < maxcoords\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3558 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"const char *grid_desc_to_hat_params(const char *, struct HatPatchParams *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_10163: {
            if (*hp).ncoords < maxcoords {} else {
                __assert_fail(
                    b"hp->ncoords < maxcoords\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    3558 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"const char *grid_desc_to_hat_params(const char *, struct HatPatchParams *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let fresh45 = (*hp).ncoords;
        (*hp).ncoords = ((*hp).ncoords).wrapping_add(1);
        *((*hp).coords).offset(fresh45 as isize) = n as libc::c_uchar;
    }
    if *p as libc::c_int == 'H' as i32 || *p as libc::c_int == 'T' as i32
        || *p as libc::c_int == 'P' as i32 || *p as libc::c_int == 'F' as i32
    {
        (*hp).final_metatile = *p;
    } else {
        return b"invalid character in grid description\0" as *const u8
            as *const libc::c_char
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_validate_desc_hats(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *const libc::c_char {
    let mut hp: HatPatchParams = HatPatchParams {
        ncoords: 0,
        coords: 0 as *mut libc::c_uchar,
        final_metatile: 0,
    };
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    if desc.is_null() {
        return b"Missing grid description string.\0" as *const u8 as *const libc::c_char;
    }
    error = grid_desc_to_hat_params(desc, &mut hp);
    if error.is_null() {
        error = hat_tiling_params_invalid(&mut hp);
    }
    sfree(hp.coords as *mut libc::c_void);
    return error;
}
unsafe extern "C" fn grid_hats_callback(
    mut vctx: *mut libc::c_void,
    mut nvertices: size_t,
    mut coords: *mut libc::c_int,
) {
    let mut ctx: *mut hatcontext = vctx as *mut hatcontext;
    let mut i: size_t = 0;
    grid_face_add_new((*ctx).g, nvertices as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < nvertices {
        let mut d: *mut grid_dot = grid_get_dot(
            (*ctx).g,
            (*ctx).points,
            *coords.offset((2 as libc::c_int as size_t * i) as isize)
                * 14 as libc::c_int,
            *coords
                .offset(
                    (2 as libc::c_int as size_t * i)
                        .wrapping_add(1 as libc::c_int as size_t) as isize,
                ) * 8 as libc::c_int,
        );
        grid_face_set_dot((*ctx).g, d, i as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn grid_new_hats(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut hp: HatPatchParams = HatPatchParams {
        ncoords: 0,
        coords: 0 as *mut libc::c_uchar,
        final_metatile: 0,
    };
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    error = grid_desc_to_hat_params(desc, &mut hp);
    if error.is_null()
        && !(b"grid_validate_desc_hats should have failed\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"error == NULL && \"grid_validate_desc_hats should have failed\"\0"
                as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            3613 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"grid *grid_new_hats(int, int, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_15825: {
        if error.is_null()
            && !(b"grid_validate_desc_hats should have failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"error == NULL && \"grid_validate_desc_hats should have failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3613 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"grid *grid_new_hats(int, int, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut ctx: [hatcontext; 1] = [hatcontext {
        g: 0 as *mut grid,
        points: 0 as *mut tree234,
    }; 1];
    let ref mut fresh46 = (*ctx.as_mut_ptr()).g;
    *fresh46 = grid_empty();
    (*(*ctx.as_mut_ptr()).g).tilesize = 32 as libc::c_int;
    let ref mut fresh47 = (*ctx.as_mut_ptr()).points;
    *fresh47 = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    hat_tiling_generate(
        &mut hp,
        width,
        height,
        Some(
            grid_hats_callback
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    *mut libc::c_int,
                ) -> (),
        ),
        ctx.as_mut_ptr() as *mut libc::c_void,
    );
    freetree234((*ctx.as_mut_ptr()).points);
    sfree(hp.coords as *mut libc::c_void);
    grid_trim_vigorously((*ctx.as_mut_ptr()).g);
    grid_make_consistent((*ctx.as_mut_ptr()).g);
    return (*ctx.as_mut_ptr()).g;
}
unsafe extern "C" fn grid_validate_params_spectres(
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    let mut l: libc::c_int = 8 as libc::c_int * 7 as libc::c_int;
    if width > 2147483647 as libc::c_int / l || height > 2147483647 as libc::c_int / l
        || width
            > 2147483647 as libc::c_int / 7 as libc::c_int / 7 as libc::c_int / height
    {
        return b"Grid must not be unreasonably large\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_size_spectres(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    *tilesize = 32 as libc::c_int;
    *xextent = width * 8 as libc::c_int * 7 as libc::c_int;
    *yextent = height * 8 as libc::c_int * 7 as libc::c_int;
}
unsafe extern "C" fn grid_new_desc_spectres(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut rs: *mut random_state,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut sp: SpectrePatchParams = SpectrePatchParams {
        orientation: 0,
        ncoords: 0,
        coords: 0 as *mut libc::c_uchar,
        final_hex: 0,
    };
    spectre_tiling_randomise(
        &mut sp,
        width * 7 as libc::c_int,
        height * 7 as libc::c_int,
        rs,
    );
    buf = smalloc(
        (sp.ncoords)
            .wrapping_add(3 as libc::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    *buf
        .offset(
            0 as libc::c_int as isize,
        ) = (if sp.orientation < 10 as libc::c_int {
        '0' as i32 + sp.orientation
    } else {
        'A' as i32 + sp.orientation - 10 as libc::c_int
    }) as libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < sp.ncoords {
        if (*(sp.coords).offset(i as isize) as libc::c_int) < 10 as libc::c_int {} else {
            __assert_fail(
                b"sp.coords[i] < 10\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3671 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"char *grid_new_desc_spectres(grid_type, int, int, random_state *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8959: {
            if (*(sp.coords).offset(i as isize) as libc::c_int) < 10 as libc::c_int
            {} else {
                __assert_fail(
                    b"sp.coords[i] < 10\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    3671 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"char *grid_new_desc_spectres(grid_type, int, int, random_state *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *buf
            .offset(
                i.wrapping_add(1 as libc::c_int as size_t) as isize,
            ) = ('0' as i32 + *(sp.coords).offset(i as isize) as libc::c_int)
            as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    *buf
        .offset(
            (sp.ncoords).wrapping_add(1 as libc::c_int as size_t) as isize,
        ) = sp.final_hex;
    *buf
        .offset(
            (sp.ncoords).wrapping_add(2 as libc::c_int as size_t) as isize,
        ) = '\0' as i32 as libc::c_char;
    sfree(sp.coords as *mut libc::c_void);
    return buf;
}
unsafe extern "C" fn grid_desc_to_spectre_params(
    mut desc: *const libc::c_char,
    mut sp: *mut SpectrePatchParams,
) -> *const libc::c_char {
    let mut i: size_t = 0;
    if *desc == 0 {
        return b"empty grid description\0" as *const u8 as *const libc::c_char;
    }
    (*sp).ncoords = (strlen(desc)).wrapping_sub(2 as libc::c_int as libc::c_ulong);
    (*sp)
        .coords = smalloc(
        ((*sp).ncoords)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    let mut c: libc::c_char = *desc.offset(0 as libc::c_int as isize);
    if *(*__ctype_b_loc()).offset(c as libc::c_uchar as libc::c_int as isize)
        as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        (*sp).orientation = c as libc::c_int - '0' as i32;
    } else if c as libc::c_int == 'A' as i32 || c as libc::c_int == 'B' as i32 {
        (*sp).orientation = 10 as libc::c_int + c as libc::c_int - 'A' as i32;
    } else {
        return b"expected digit or A,B at start of grid description\0" as *const u8
            as *const libc::c_char
    }
    i = 0 as libc::c_int as size_t;
    while i < (*sp).ncoords {
        let mut c_0: libc::c_char = *desc
            .offset(i.wrapping_add(1 as libc::c_int as size_t) as isize);
        if *(*__ctype_b_loc()).offset(c_0 as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return b"expected digit in grid description\0" as *const u8
                as *const libc::c_char;
        }
        *((*sp).coords)
            .offset(i as isize) = (c_0 as libc::c_int - '0' as i32) as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    (*sp)
        .final_hex = *desc
        .offset(((*sp).ncoords).wrapping_add(1 as libc::c_int as size_t) as isize);
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn grid_validate_desc_spectres(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *const libc::c_char {
    let mut sp: SpectrePatchParams = SpectrePatchParams {
        orientation: 0,
        ncoords: 0,
        coords: 0 as *mut libc::c_uchar,
        final_hex: 0,
    };
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    if desc.is_null() {
        return b"Missing grid description string.\0" as *const u8 as *const libc::c_char;
    }
    error = grid_desc_to_spectre_params(desc, &mut sp);
    if error.is_null() {
        error = spectre_tiling_params_invalid(&mut sp);
    }
    sfree(sp.coords as *mut libc::c_void);
    return error;
}
unsafe extern "C" fn grid_spectres_callback(
    mut vctx: *mut libc::c_void,
    mut coords: *const libc::c_int,
) {
    let mut ctx: *mut spectrecontext = vctx as *mut spectrecontext;
    let mut i: size_t = 0;
    grid_face_add_new((*ctx).g, 14 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < 14 as libc::c_int as size_t {
        let mut d: *mut grid_dot = grid_get_dot(
            (*ctx).g,
            (*ctx).points,
            *coords
                .offset(
                    (4 as libc::c_int as size_t * i)
                        .wrapping_add(0 as libc::c_int as size_t) as isize,
                ) * 8 as libc::c_int
                + n_times_root_k(
                    *coords
                        .offset(
                            (4 as libc::c_int as size_t * i)
                                .wrapping_add(1 as libc::c_int as size_t) as isize,
                        ) * 8 as libc::c_int,
                    3 as libc::c_int,
                ),
            *coords
                .offset(
                    (4 as libc::c_int as size_t * i)
                        .wrapping_add(2 as libc::c_int as size_t) as isize,
                ) * 8 as libc::c_int
                + n_times_root_k(
                    *coords
                        .offset(
                            (4 as libc::c_int as size_t * i)
                                .wrapping_add(3 as libc::c_int as size_t) as isize,
                        ) * 8 as libc::c_int,
                    3 as libc::c_int,
                ),
        );
        grid_face_set_dot((*ctx).g, d, i as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn grid_new_spectres(
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut sp: SpectrePatchParams = SpectrePatchParams {
        orientation: 0,
        ncoords: 0,
        coords: 0 as *mut libc::c_uchar,
        final_hex: 0,
    };
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut width2: libc::c_int = width * 7 as libc::c_int;
    let mut height2: libc::c_int = height * 7 as libc::c_int;
    error = grid_desc_to_spectre_params(desc, &mut sp);
    if error.is_null()
        && !(b"grid_validate_desc_spectres should have failed\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"error == NULL && \"grid_validate_desc_spectres should have failed\"\0"
                as *const u8 as *const libc::c_char,
            b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
            3763 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"grid *grid_new_spectres(int, int, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_15557: {
        if error.is_null()
            && !(b"grid_validate_desc_spectres should have failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"error == NULL && \"grid_validate_desc_spectres should have failed\"\0"
                    as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3763 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"grid *grid_new_spectres(int, int, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut ctx: [spectrecontext; 1] = [spectrecontext {
        g: 0 as *mut grid,
        points: 0 as *mut tree234,
    }; 1];
    let ref mut fresh48 = (*ctx.as_mut_ptr()).g;
    *fresh48 = grid_empty();
    (*(*ctx.as_mut_ptr()).g).tilesize = 32 as libc::c_int;
    let ref mut fresh49 = (*ctx.as_mut_ptr()).points;
    *fresh49 = newtree234(
        Some(
            grid_point_cmp_fn
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    spectre_tiling_generate(
        &mut sp,
        width2,
        height2,
        Some(
            grid_spectres_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_int) -> (),
        ),
        ctx.as_mut_ptr() as *mut libc::c_void,
    );
    freetree234((*ctx.as_mut_ptr()).points);
    sfree(sp.coords as *mut libc::c_void);
    grid_trim_vigorously((*ctx.as_mut_ptr()).g);
    grid_make_consistent((*ctx.as_mut_ptr()).g);
    let mut w: libc::c_int = width2 * 8 as libc::c_int;
    let mut h: libc::c_int = height2 * 8 as libc::c_int;
    (*(*ctx.as_mut_ptr()).g).lowest_x
        -= (w - ((*(*ctx.as_mut_ptr()).g).highest_x - (*(*ctx.as_mut_ptr()).g).lowest_x))
            / 2 as libc::c_int;
    (*(*ctx.as_mut_ptr()).g).lowest_y
        -= (h - ((*(*ctx.as_mut_ptr()).g).highest_y - (*(*ctx.as_mut_ptr()).g).lowest_y))
            / 2 as libc::c_int;
    (*(*ctx.as_mut_ptr()).g).highest_x = (*(*ctx.as_mut_ptr()).g).lowest_x + w;
    (*(*ctx.as_mut_ptr()).g).highest_y = (*(*ctx.as_mut_ptr()).g).lowest_y + h;
    return (*ctx.as_mut_ptr()).g;
}
static mut grid_validate_paramses: [Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
>; 18] = [
    Some(
        grid_validate_params_square
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_honeycomb
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_triangular
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_snubsquare
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_cairo
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_greathexagonal
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_kagome
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_octagonal
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_kites
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_floret
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_dodecagonal
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_greatdodecagonal
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_greatgreatdodecagonal
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_compassdodecagonal
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_penrose_p2_kite
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_penrose_p3_thick
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_hats
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
    Some(
        grid_validate_params_spectres
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *const libc::c_char,
    ),
];
static mut grid_news: [Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_char) -> *mut grid,
>; 18] = [
    Some(
        grid_new_square
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_honeycomb
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_triangular
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_snubsquare
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_cairo
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_greathexagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_kagome
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_octagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_kites
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_floret
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_dodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_greatdodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_greatgreatdodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_compassdodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_penrose_p2_kite
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_penrose_p3_thick
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_hats
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
    Some(
        grid_new_spectres
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *const libc::c_char,
            ) -> *mut grid,
    ),
];
static mut grid_sizes: [Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
        *mut libc::c_int,
        *mut libc::c_int,
    ) -> (),
>; 18] = [
    Some(
        grid_size_square
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_honeycomb
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_triangular
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_snubsquare
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_cairo
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_greathexagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_kagome
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_octagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_kites
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_floret
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_dodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_greatdodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_greatgreatdodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_compassdodecagonal
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_penrose_p2_kite
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_penrose_p3_thick
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_hats
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
    Some(
        grid_size_spectres
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_int,
            ) -> (),
    ),
];
#[no_mangle]
pub unsafe extern "C" fn grid_validate_params(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> *const libc::c_char {
    if width <= 0 as libc::c_int || height <= 0 as libc::c_int {
        return b"Width and height must both be positive\0" as *const u8
            as *const libc::c_char;
    }
    return (grid_validate_paramses[type_0 as usize])
        .expect("non-null function pointer")(width, height);
}
#[no_mangle]
pub unsafe extern "C" fn grid_new_desc(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut rs: *mut random_state,
) -> *mut libc::c_char {
    if type_0 as libc::c_uint == GRID_PENROSE_P2 as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == GRID_PENROSE_P3 as libc::c_int as libc::c_uint
    {
        return grid_new_desc_penrose(type_0, width, height, rs)
    } else if type_0 as libc::c_uint == GRID_HATS as libc::c_int as libc::c_uint {
        return grid_new_desc_hats(type_0, width, height, rs)
    } else if type_0 as libc::c_uint == GRID_SPECTRES as libc::c_int as libc::c_uint {
        return grid_new_desc_spectres(type_0, width, height, rs)
    } else if type_0 as libc::c_uint == GRID_TRIANGULAR as libc::c_int as libc::c_uint {
        return dupstr(b"0\0" as *const u8 as *const libc::c_char)
    } else {
        return 0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_validate_desc(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *const libc::c_char {
    if type_0 as libc::c_uint == GRID_PENROSE_P2 as libc::c_int as libc::c_uint
        || type_0 as libc::c_uint == GRID_PENROSE_P3 as libc::c_int as libc::c_uint
    {
        return grid_validate_desc_penrose(type_0, width, height, desc)
    } else if type_0 as libc::c_uint == GRID_HATS as libc::c_int as libc::c_uint {
        return grid_validate_desc_hats(type_0, width, height, desc)
    } else if type_0 as libc::c_uint == GRID_SPECTRES as libc::c_int as libc::c_uint {
        return grid_validate_desc_spectres(type_0, width, height, desc)
    } else if type_0 as libc::c_uint == GRID_TRIANGULAR as libc::c_int as libc::c_uint {
        return grid_validate_desc_triangular(type_0, width, height, desc)
    } else {
        if !desc.is_null() {
            return b"Grid description strings not used with this grid type\0"
                as *const u8 as *const libc::c_char;
        }
        return 0 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_new(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut desc: *const libc::c_char,
) -> *mut grid {
    let mut err: *const libc::c_char = grid_validate_desc(type_0, width, height, desc);
    if !err.is_null() {
        if (b"Invalid grid description.\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"!\"Invalid grid description.\"\0" as *const u8 as *const libc::c_char,
                b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                3853 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"grid *grid_new(grid_type, int, int, const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_30038: {
            if (b"Invalid grid description.\0" as *const u8 as *const libc::c_char)
                .is_null()
            {} else {
                __assert_fail(
                    b"!\"Invalid grid description.\"\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/grid.c\0" as *const u8 as *const libc::c_char,
                    3853 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"grid *grid_new(grid_type, int, int, const char *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    return (grid_news[type_0 as usize])
        .expect("non-null function pointer")(width, height, desc);
}
#[no_mangle]
pub unsafe extern "C" fn grid_compute_size(
    mut type_0: grid_type,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut tilesize: *mut libc::c_int,
    mut xextent: *mut libc::c_int,
    mut yextent: *mut libc::c_int,
) {
    (grid_sizes[type_0 as usize])
        .expect("non-null function pointer")(width, height, tilesize, xextent, yextent);
}
