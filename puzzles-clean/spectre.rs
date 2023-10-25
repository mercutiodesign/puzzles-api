#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type random_state;
    pub type tree234_Tag;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn srealloc(p: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn random_new(seed: *const libc::c_char, len: libc::c_int) -> *mut random_state;
    fn random_upto(state: *mut random_state, limit: libc::c_ulong) -> libc::c_ulong;
    fn random_free(state: *mut random_state);
    fn newtree234(cmp: cmpfn234) -> *mut tree234;
    fn freetree234(t: *mut tree234);
    fn add234(t: *mut tree234, e: *mut libc::c_void) -> *mut libc::c_void;
    fn find234(
        t: *mut tree234,
        e: *mut libc::c_void,
        cmp: cmpfn234,
    ) -> *mut libc::c_void;
    fn delpos234(t: *mut tree234, index: libc::c_int) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type tree234 = tree234_Tag;
pub type cmpfn234 = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
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
pub struct SpectreContext {
    pub rs: *mut random_state,
    pub must_free_rs: bool,
    pub start_vertices: [Point; 2],
    pub orientation: libc::c_int,
    pub prototype: *mut SpectreCoords,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpectreCoords {
    pub index: libc::c_int,
    pub c: *mut HexCoord,
    pub nc: size_t,
    pub csize: size_t,
    pub hex_colour: libc::c_uchar,
    pub prev_hex_colour: libc::c_uchar,
    pub incoming_hex_edge: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HexCoord {
    pub index: libc::c_int,
    pub type_0: Hex,
}
pub type Hex = libc::c_uint;
pub const HEX_Y: Hex = 8;
pub const HEX_F: Hex = 7;
pub const HEX_S: Hex = 6;
pub const HEX_P: Hex = 5;
pub const HEX_X: Hex = 4;
pub const HEX_L: Hex = 3;
pub const HEX_J: Hex = 2;
pub const HEX_D: Hex = 1;
pub const HEX_G: Hex = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Point {
    pub coeffs: [libc::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpectreCallbackContext {
    pub xoff: libc::c_int,
    pub yoff: libc::c_int,
    pub xmin: Coord,
    pub xmax: Coord,
    pub ymin: Coord,
    pub ymax: Coord,
    pub external_cb: spectre_tile_callback_fn,
    pub external_cbctx: *mut libc::c_void,
}
pub type spectre_tile_callback_fn = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Coord {
    pub c1: libc::c_int,
    pub cr3: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Spectre {
    pub vertices: [Point; 14],
    pub sc: *mut SpectreCoords,
    pub next: *mut Spectre,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapEntry {
    pub internal: bool,
    pub hi: libc::c_uchar,
    pub lo: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapEdge {
    pub startindex: libc::c_uchar,
    pub len: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HexData {
    pub hexmap: *const MapEntry,
    pub hexin: *const MapEntry,
    pub specmap: *const MapEntry,
    pub specin: *const MapEntry,
    pub hexedges: *const MapEdge,
    pub specedges: *const MapEdge,
    pub subhexes: *const Hex,
    pub poss: *const Possibility,
    pub nposs: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Possibility {
    pub hi: libc::c_uchar,
    pub lo: libc::c_uchar,
    pub prob: libc::c_ulong,
}
#[inline]
unsafe extern "C" fn num_subhexes(mut h: Hex) -> libc::c_uint {
    return (if h as libc::c_uint == HEX_G as libc::c_int as libc::c_uint {
        7 as libc::c_int
    } else {
        8 as libc::c_int
    }) as libc::c_uint;
}
#[inline]
unsafe extern "C" fn num_spectres(mut h: Hex) -> libc::c_uint {
    return (if h as libc::c_uint == HEX_G as libc::c_int as libc::c_uint {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    }) as libc::c_uint;
}
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
unsafe extern "C" fn point_mul_by_d(mut x: Point) -> Point {
    let mut r: Point = Point { coeffs: [0; 4] };
    r.coeffs[0 as libc::c_int as usize] = -x.coeffs[3 as libc::c_int as usize];
    r.coeffs[1 as libc::c_int as usize] = x.coeffs[0 as libc::c_int as usize];
    r
        .coeffs[2 as libc::c_int
        as usize] = x.coeffs[1 as libc::c_int as usize]
        + x.coeffs[3 as libc::c_int as usize];
    r.coeffs[3 as libc::c_int as usize] = x.coeffs[2 as libc::c_int as usize];
    return r;
}
#[inline]
unsafe extern "C" fn point_mul(mut a: Point, mut b: Point) -> Point {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut r: Point = Point { coeffs: [0; 4] };
    j = 0 as libc::c_int as size_t;
    while j < 4 as libc::c_int as size_t {
        r
            .coeffs[j
            as usize] = a.coeffs[j as usize] * b.coeffs[3 as libc::c_int as usize];
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
        r = point_mul_by_d(r);
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
    let mut dpower: Point = {
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
    s = s % 12 as libc::c_int;
    if s < 0 as libc::c_int {
        s += 12 as libc::c_int;
    }
    loop {
        if s & 1 as libc::c_int != 0 {
            r = point_mul(r, dpower);
        }
        s >>= 1 as libc::c_int;
        if s == 0 {
            break;
        }
        dpower = point_mul(dpower, dpower);
    }
    return r;
}
#[inline]
unsafe extern "C" fn point_x(mut p: Point) -> Coord {
    let mut x: Coord = {
        let mut init = Coord {
            c1: 2 as libc::c_int * p.coeffs[0 as libc::c_int as usize]
                + p.coeffs[2 as libc::c_int as usize],
            cr3: p.coeffs[1 as libc::c_int as usize],
        };
        init
    };
    return x;
}
#[inline]
unsafe extern "C" fn point_y(mut p: Point) -> Coord {
    let mut y: Coord = {
        let mut init = Coord {
            c1: 2 as libc::c_int * p.coeffs[3 as libc::c_int as usize]
                + p.coeffs[1 as libc::c_int as usize],
            cr3: p.coeffs[2 as libc::c_int as usize],
        };
        init
    };
    return y;
}
#[inline]
unsafe extern "C" fn coord_cmp(mut a: Coord, mut b: Coord) -> libc::c_int {
    return coord_sign(coord_sub(a, b));
}
#[inline]
unsafe extern "C" fn coord_sub(mut a: Coord, mut b: Coord) -> Coord {
    let mut diff: Coord = Coord { c1: 0, cr3: 0 };
    diff.c1 = a.c1 - b.c1;
    diff.cr3 = a.cr3 - b.cr3;
    return diff;
}
#[inline]
unsafe extern "C" fn coord_sign(mut x: Coord) -> libc::c_int {
    if x.c1 == 0 as libc::c_int && x.cr3 == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if x.c1 >= 0 as libc::c_int && x.cr3 >= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if x.c1 <= 0 as libc::c_int && x.cr3 <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if x.c1 * x.c1 > 3 as libc::c_int * x.cr3 * x.cr3 {
        return if x.c1 < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }
    } else {
        return if x.cr3 < 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        }
    };
}
static mut subhexes_G: [Hex; 7] = [HEX_F, HEX_X, HEX_G, HEX_S, HEX_P, HEX_D, HEX_J];
static mut subhexes_D: [Hex; 8] = [
    HEX_F,
    HEX_P,
    HEX_G,
    HEX_S,
    HEX_X,
    HEX_D,
    HEX_F,
    HEX_X,
];
static mut subhexes_J: [Hex; 8] = [
    HEX_F,
    HEX_P,
    HEX_G,
    HEX_S,
    HEX_Y,
    HEX_D,
    HEX_F,
    HEX_P,
];
static mut subhexes_L: [Hex; 8] = [
    HEX_F,
    HEX_P,
    HEX_G,
    HEX_S,
    HEX_Y,
    HEX_D,
    HEX_F,
    HEX_X,
];
static mut subhexes_X: [Hex; 8] = [
    HEX_F,
    HEX_Y,
    HEX_G,
    HEX_S,
    HEX_Y,
    HEX_D,
    HEX_F,
    HEX_P,
];
static mut subhexes_P: [Hex; 8] = [
    HEX_F,
    HEX_Y,
    HEX_G,
    HEX_S,
    HEX_Y,
    HEX_D,
    HEX_F,
    HEX_X,
];
static mut subhexes_S: [Hex; 8] = [
    HEX_L,
    HEX_P,
    HEX_G,
    HEX_S,
    HEX_X,
    HEX_D,
    HEX_F,
    HEX_X,
];
static mut subhexes_F: [Hex; 8] = [
    HEX_F,
    HEX_P,
    HEX_G,
    HEX_S,
    HEX_Y,
    HEX_D,
    HEX_F,
    HEX_Y,
];
static mut subhexes_Y: [Hex; 8] = [
    HEX_F,
    HEX_Y,
    HEX_G,
    HEX_S,
    HEX_Y,
    HEX_D,
    HEX_F,
    HEX_Y,
];
static mut spectre_angles: [libc::c_int; 14] = [
    -(3 as libc::c_int),
    -(2 as libc::c_int),
    3 as libc::c_int,
    -(2 as libc::c_int),
    -(3 as libc::c_int),
    2 as libc::c_int,
    -(3 as libc::c_int),
    2 as libc::c_int,
    -(3 as libc::c_int),
    -(2 as libc::c_int),
    0 as libc::c_int,
    -(2 as libc::c_int),
    3 as libc::c_int,
    -(2 as libc::c_int),
];
static mut hexmap_G: [MapEntry; 42] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_G: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 18 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_G: [MapEntry; 20] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_D: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_D: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 10 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 14 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 17 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_D: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_J: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_J: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 7 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 15 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 17 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_J: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_L: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_L: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 7 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 16 as libc::c_int as libc::c_uchar,
            len: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_L: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_X: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_X: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 15 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 17 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_X: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_P: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_P: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 16 as libc::c_int as libc::c_uchar,
            len: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_P: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_S: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_S: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 10 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 14 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 17 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_S: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_F: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_F: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 7 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 17 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_F: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexmap_Y: [MapEntry; 48] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexedges_Y: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 17 as libc::c_int as libc::c_uchar,
            len: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut hexin_Y: [MapEntry; 22] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 6 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 7 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_G: [MapEntry; 28] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_G: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 6 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 11 as libc::c_int as libc::c_uchar,
            len: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 17 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_G: [MapEntry; 20] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_D: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_D: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 2 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 4 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 7 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 12 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_D: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_J: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_J: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 12 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_J: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_L: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_L: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 10 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_L: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_X: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_X: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 6 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 9 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 12 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_X: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_P: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_P: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 6 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 10 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_P: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_S: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_S: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 6 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 11 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 13 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 16 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_S: [MapEntry; 18] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_F: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_F: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 5 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 10 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 12 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_F: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specmap_Y: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 2 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 1 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 5 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 4 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 0 as libc::c_int != 0,
            hi: 3 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specedges_Y: [MapEdge; 6] = [
    {
        let mut init = MapEdge {
            startindex: 0 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 3 as libc::c_int as libc::c_uchar,
            len: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 6 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 8 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 10 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEdge {
            startindex: 12 as libc::c_int as libc::c_uchar,
            len: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut specin_Y: [MapEntry; 14] = [
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 8 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 13 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 12 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 11 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 10 as libc::c_int as libc::c_uchar,
        };
        init
    },
    {
        let mut init = MapEntry {
            internal: 1 as libc::c_int != 0,
            hi: 0 as libc::c_int as libc::c_uchar,
            lo: 9 as libc::c_int as libc::c_uchar,
        };
        init
    },
];
static mut poss_G: [Possibility; 9] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 2 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_D: [Possibility; 9] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 5 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_J: [Possibility; 1] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_L: [Possibility; 1] = [
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_X: [Possibility; 7] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_P: [Possibility; 8] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_S: [Possibility; 9] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 3 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_F: [Possibility; 16] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 6 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_Y: [Possibility; 11] = [
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 4 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 7 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut poss_spectre: [Possibility; 10] = [
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_G as libc::c_int as libc::c_uchar,
            lo: 1 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_D as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_J as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_L as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 1270167 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_X as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_P as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 7459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_S as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 10000000 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_F as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 17459667 as libc::c_int as libc::c_ulong,
        };
        init
    },
    {
        let mut init = Possibility {
            hi: HEX_Y as libc::c_int as libc::c_uchar,
            lo: 0 as libc::c_int as libc::c_uchar,
            prob: 13810500 as libc::c_int as libc::c_ulong,
        };
        init
    },
];
static mut letters: *const libc::c_char = b"GDJLXPSFY\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn spectre_valid_hex_letter(mut letter: libc::c_char) -> bool {
    return !(strchr(letters, letter as libc::c_int)).is_null();
}
unsafe extern "C" fn hex_from_letter(mut letter: libc::c_char) -> Hex {
    let mut buf: [libc::c_char; 2] = [0; 2];
    buf[0 as libc::c_int as usize] = letter;
    buf[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    return strcspn(letters, buf.as_mut_ptr()) as Hex;
}
unsafe extern "C" fn hex_to_letter(mut letter: libc::c_uchar) -> Hex {
    return *letters.offset(letter as isize) as Hex;
}
static mut hexdata: [HexData; 9] = [HexData {
    hexmap: 0 as *const MapEntry,
    hexin: 0 as *const MapEntry,
    specmap: 0 as *const MapEntry,
    specin: 0 as *const MapEntry,
    hexedges: 0 as *const MapEdge,
    specedges: 0 as *const MapEdge,
    subhexes: 0 as *const Hex,
    poss: 0 as *const Possibility,
    nposs: 0,
}; 9];
unsafe extern "C" fn choose_poss(
    mut rs: *mut random_state,
    mut poss: *const Possibility,
    mut nposs: size_t,
) -> *const Possibility {
    let mut limit: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut value: libc::c_ulong = 0;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < nposs {
        limit = limit.wrapping_add((*poss.offset(i as isize)).prob);
        i = i.wrapping_add(1);
        i;
    }
    value = random_upto(rs, limit);
    i = 0 as libc::c_int as size_t;
    while i.wrapping_add(1 as libc::c_int as size_t) < nposs {
        if value < (*poss.offset(i as isize)).prob {
            return &*poss.offset(i as isize) as *const Possibility;
        }
        value = value.wrapping_sub((*poss.offset(i as isize)).prob);
        i = i.wrapping_add(1);
        i;
    }
    if i == nposs.wrapping_sub(1 as libc::c_int as size_t) {} else {
        __assert_fail(
            b"i == nposs - 1\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"const struct Possibility *choose_poss(random_state *, const struct Possibility *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15261: {
        if i == nposs.wrapping_sub(1 as libc::c_int as size_t) {} else {
            __assert_fail(
                b"i == nposs - 1\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                88 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"const struct Possibility *choose_poss(random_state *, const struct Possibility *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if value < (*poss.offset(i as isize)).prob {} else {
        __assert_fail(
            b"value < poss[i].prob\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"const struct Possibility *choose_poss(random_state *, const struct Possibility *, size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15210: {
        if value < (*poss.offset(i as isize)).prob {} else {
            __assert_fail(
                b"value < poss[i].prob\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                89 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"const struct Possibility *choose_poss(random_state *, const struct Possibility *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return &*poss.offset(i as isize) as *const Possibility;
}
#[no_mangle]
pub unsafe extern "C" fn spectre_coords_new() -> *mut SpectreCoords {
    let mut sc: *mut SpectreCoords = smalloc(
        ::core::mem::size_of::<SpectreCoords>() as libc::c_ulong,
    ) as *mut SpectreCoords;
    (*sc).csize = 0 as libc::c_int as size_t;
    (*sc).nc = (*sc).csize;
    (*sc).c = 0 as *mut HexCoord;
    return sc;
}
#[no_mangle]
pub unsafe extern "C" fn spectre_coords_free(mut sc: *mut SpectreCoords) {
    if !sc.is_null() {
        sfree((*sc).c as *mut libc::c_void);
        sfree(sc as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn spectre_coords_make_space(
    mut sc: *mut SpectreCoords,
    mut size: size_t,
) {
    if (*sc).csize < size {
        (*sc)
            .csize = ((*sc).csize * 5 as libc::c_int as size_t
            / 4 as libc::c_int as size_t)
            .wrapping_add(16 as libc::c_int as size_t);
        if (*sc).csize < size {
            (*sc).csize = size;
        }
        (*sc)
            .c = srealloc(
            (*sc).c as *mut libc::c_void,
            ((*sc).csize)
                .wrapping_mul(::core::mem::size_of::<HexCoord>() as libc::c_ulong),
        ) as *mut HexCoord;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spectre_coords_copy(
    mut sc_in: *mut SpectreCoords,
) -> *mut SpectreCoords {
    let mut sc_out: *mut SpectreCoords = spectre_coords_new();
    spectre_coords_make_space(sc_out, (*sc_in).nc);
    memcpy(
        (*sc_out).c as *mut libc::c_void,
        (*sc_in).c as *const libc::c_void,
        ((*sc_in).nc).wrapping_mul(::core::mem::size_of::<HexCoord>() as libc::c_ulong),
    );
    (*sc_out).nc = (*sc_in).nc;
    (*sc_out).index = (*sc_in).index;
    (*sc_out).hex_colour = (*sc_in).hex_colour;
    (*sc_out).prev_hex_colour = (*sc_in).prev_hex_colour;
    (*sc_out).incoming_hex_edge = (*sc_in).incoming_hex_edge;
    return sc_out;
}
#[no_mangle]
pub unsafe extern "C" fn spectre_place(
    mut spec: *mut Spectre,
    mut u: Point,
    mut v: Point,
    mut index_of_u: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut disp: Point = Point { coeffs: [0; 4] };
    disp = point_sub(v, u);
    i = 0 as libc::c_int as size_t;
    while i < 14 as libc::c_int as size_t {
        (*spec)
            .vertices[(i.wrapping_add(index_of_u as size_t)
            % 14 as libc::c_int as size_t) as usize] = u;
        u = point_add(u, disp);
        disp = point_mul(
            disp,
            point_rot(
                spectre_angles[(i
                    .wrapping_add(1 as libc::c_int as size_t)
                    .wrapping_add(index_of_u as size_t) % 14 as libc::c_int as size_t)
                    as usize],
            ),
        );
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spectre_initial(mut ctx: *mut SpectreContext) -> *mut Spectre {
    let mut spec: *mut Spectre = smalloc(
        ::core::mem::size_of::<Spectre>() as libc::c_ulong,
    ) as *mut Spectre;
    spectre_place(
        spec,
        (*ctx).start_vertices[0 as libc::c_int as usize],
        (*ctx).start_vertices[1 as libc::c_int as usize],
        0 as libc::c_int,
    );
    (*spec).sc = spectre_coords_copy((*ctx).prototype);
    return spec;
}
#[no_mangle]
pub unsafe extern "C" fn spectre_adjacent(
    mut ctx: *mut SpectreContext,
    mut src_spec: *const Spectre,
    mut src_edge: libc::c_uint,
    mut dst_edge_out: *mut libc::c_uint,
) -> *mut Spectre {
    let mut dst_edge: libc::c_uint = 0;
    let mut dst_spec: *mut Spectre = smalloc(
        ::core::mem::size_of::<Spectre>() as libc::c_ulong,
    ) as *mut Spectre;
    (*dst_spec).sc = spectre_coords_copy((*src_spec).sc);
    spectrectx_step(ctx, (*dst_spec).sc, src_edge, &mut dst_edge);
    spectre_place(
        dst_spec,
        (*src_spec)
            .vertices[src_edge
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_rem(14 as libc::c_int as libc::c_uint) as usize],
        (*src_spec).vertices[src_edge as usize],
        dst_edge as libc::c_int,
    );
    if !dst_edge_out.is_null() {
        *dst_edge_out = dst_edge;
    }
    return dst_spec;
}
unsafe extern "C" fn spectre_cmp(
    mut av: *mut libc::c_void,
    mut bv: *mut libc::c_void,
) -> libc::c_int {
    let mut a: *mut Spectre = av as *mut Spectre;
    let mut b: *mut Spectre = bv as *mut Spectre;
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
#[no_mangle]
pub unsafe extern "C" fn spectre_free(mut spec: *mut Spectre) {
    spectre_coords_free((*spec).sc);
    sfree(spec as *mut libc::c_void);
}
unsafe extern "C" fn spectrectx_start_vertices(
    mut ctx: *mut SpectreContext,
    mut orientation: libc::c_int,
) {
    let mut minus_sqrt3: Point = point_add(
        point_rot(5 as libc::c_int),
        point_rot(-(5 as libc::c_int)),
    );
    let mut basicedge: Point = point_mul(
        point_add(point_rot(0 as libc::c_int), point_rot(-(3 as libc::c_int))),
        point_rot(orientation),
    );
    let mut diagonal: Point = point_add(
        basicedge,
        point_mul(basicedge, point_rot(-(3 as libc::c_int))),
    );
    (*ctx).start_vertices[0 as libc::c_int as usize] = point_mul(diagonal, minus_sqrt3);
    (*ctx)
        .start_vertices[1 as libc::c_int
        as usize] = point_add(
        (*ctx).start_vertices[0 as libc::c_int as usize],
        basicedge,
    );
    (*ctx).orientation = orientation;
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_init_random(
    mut ctx: *mut SpectreContext,
    mut rs: *mut random_state,
) {
    let mut poss: *const Possibility = 0 as *const Possibility;
    (*ctx).rs = rs;
    (*ctx).must_free_rs = 0 as libc::c_int != 0;
    (*ctx).prototype = spectre_coords_new();
    spectre_coords_make_space((*ctx).prototype, 1 as libc::c_int as size_t);
    poss = choose_poss(
        rs,
        poss_spectre.as_ptr(),
        (::core::mem::size_of::<[Possibility; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
    );
    (*(*ctx).prototype).index = (*poss).lo as libc::c_int;
    (*((*(*ctx).prototype).c).offset(0 as libc::c_int as isize))
        .type_0 = (*poss).hi as Hex;
    (*((*(*ctx).prototype).c).offset(0 as libc::c_int as isize))
        .index = -(1 as libc::c_int);
    (*(*ctx).prototype).nc = 1 as libc::c_int as size_t;
    spectrectx_start_vertices(
        ctx,
        (random_upto(rs, 6 as libc::c_int as libc::c_ulong))
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add((*(*ctx).prototype).index as libc::c_ulong) as libc::c_int,
    );
    (*(*ctx).prototype).hex_colour = 0 as libc::c_int as libc::c_uchar;
    (*(*ctx).prototype).prev_hex_colour = 0 as libc::c_int as libc::c_uchar;
    (*(*ctx).prototype).incoming_hex_edge = 0 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_init_from_params(
    mut ctx: *mut SpectreContext,
    mut ps: *const SpectrePatchParams,
) {
    let mut i: size_t = 0;
    (*ctx).rs = 0 as *mut random_state;
    (*ctx).must_free_rs = 0 as libc::c_int != 0;
    (*ctx).prototype = spectre_coords_new();
    spectre_coords_make_space((*ctx).prototype, (*ps).ncoords);
    (*(*ctx).prototype)
        .index = *((*ps).coords).offset(0 as libc::c_int as isize) as libc::c_int;
    i = 1 as libc::c_int as size_t;
    while i < (*ps).ncoords {
        (*((*(*ctx).prototype).c)
            .offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize))
            .index = *((*ps).coords).offset(i as isize) as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    (*((*(*ctx).prototype).c)
        .offset(((*ps).ncoords).wrapping_sub(1 as libc::c_int as size_t) as isize))
        .index = -(1 as libc::c_int);
    (*(*ctx).prototype).nc = (*ps).ncoords;
    (*((*(*ctx).prototype).c)
        .offset(((*ps).ncoords).wrapping_sub(1 as libc::c_int as size_t) as isize))
        .type_0 = hex_from_letter((*ps).final_hex);
    i = ((*ps).ncoords).wrapping_sub(1 as libc::c_int as size_t);
    loop {
        let fresh1 = i;
        i = i.wrapping_sub(1);
        if !(fresh1 > 0 as libc::c_int as size_t) {
            break;
        }
        let mut h: *const HexData = &*hexdata
            .as_ptr()
            .offset(
                (*((*(*ctx).prototype).c)
                    .offset(i.wrapping_add(1 as libc::c_int as size_t) as isize))
                    .type_0 as isize,
            ) as *const HexData;
        (*((*(*ctx).prototype).c).offset(i as isize))
            .type_0 = *((*h).subhexes)
            .offset((*((*(*ctx).prototype).c).offset(i as isize)).index as isize);
    }
    spectrectx_start_vertices(ctx, (*ps).orientation);
    (*(*ctx).prototype).hex_colour = 0 as libc::c_int as libc::c_uchar;
    (*(*ctx).prototype).prev_hex_colour = 0 as libc::c_int as libc::c_uchar;
    (*(*ctx).prototype).incoming_hex_edge = 0 as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_cleanup(mut ctx: *mut SpectreContext) {
    if (*ctx).must_free_rs {
        random_free((*ctx).rs);
    }
    spectre_coords_free((*ctx).prototype);
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_initial_coords(
    mut ctx: *mut SpectreContext,
) -> *mut SpectreCoords {
    return spectre_coords_copy((*ctx).prototype);
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_extend_coords(
    mut ctx: *mut SpectreContext,
    mut sc: *mut SpectreCoords,
    mut n: size_t,
) {
    if (*(*ctx).prototype).nc < n {
        spectre_coords_make_space((*ctx).prototype, n);
        while (*(*ctx).prototype).nc < n {
            let mut h: *const HexData = &*hexdata
                .as_ptr()
                .offset(
                    (*((*(*ctx).prototype).c)
                        .offset(
                            ((*(*ctx).prototype).nc)
                                .wrapping_sub(1 as libc::c_int as size_t) as isize,
                        ))
                        .type_0 as isize,
                ) as *const HexData;
            let mut poss: *const Possibility = 0 as *const Possibility;
            if ((*ctx).rs).is_null() {
                (*ctx)
                    .rs = random_new(
                    b"dummy\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
                (*ctx).must_free_rs = 1 as libc::c_int != 0;
            }
            poss = choose_poss((*ctx).rs, (*h).poss, (*h).nposs);
            (*((*(*ctx).prototype).c)
                .offset(
                    ((*(*ctx).prototype).nc).wrapping_sub(1 as libc::c_int as size_t)
                        as isize,
                ))
                .index = (*poss).lo as libc::c_int;
            (*((*(*ctx).prototype).c).offset((*(*ctx).prototype).nc as isize))
                .type_0 = (*poss).hi as Hex;
            (*((*(*ctx).prototype).c).offset((*(*ctx).prototype).nc as isize))
                .index = -(1 as libc::c_int);
            (*(*ctx).prototype).nc = ((*(*ctx).prototype).nc).wrapping_add(1);
            (*(*ctx).prototype).nc;
        }
    }
    spectre_coords_make_space(sc, n);
    while (*sc).nc < n {
        if (*((*sc).c)
            .offset(((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize))
            .index == -(1 as libc::c_int)
        {} else {
            __assert_fail(
                b"sc->c[sc->nc - 1].index == -1\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                342 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void spectrectx_extend_coords(SpectreContext *, SpectreCoords *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_14918: {
            if (*((*sc).c)
                .offset(((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize))
                .index == -(1 as libc::c_int)
            {} else {
                __assert_fail(
                    b"sc->c[sc->nc - 1].index == -1\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                    342 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 73],
                        &[libc::c_char; 73],
                    >(
                        b"void spectrectx_extend_coords(SpectreContext *, SpectreCoords *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (*((*sc).c)
            .offset(((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize))
            .type_0 as libc::c_uint
            == (*((*(*ctx).prototype).c)
                .offset(((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize))
                .type_0 as libc::c_uint
        {} else {
            __assert_fail(
                b"sc->c[sc->nc - 1].type == ctx->prototype->c[sc->nc - 1].type\0"
                    as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                343 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"void spectrectx_extend_coords(SpectreContext *, SpectreCoords *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_14822: {
            if (*((*sc).c)
                .offset(((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize))
                .type_0 as libc::c_uint
                == (*((*(*ctx).prototype).c)
                    .offset(
                        ((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ))
                    .type_0 as libc::c_uint
            {} else {
                __assert_fail(
                    b"sc->c[sc->nc - 1].type == ctx->prototype->c[sc->nc - 1].type\0"
                        as *const u8 as *const libc::c_char,
                    b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                    343 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 73],
                        &[libc::c_char; 73],
                    >(
                        b"void spectrectx_extend_coords(SpectreContext *, SpectreCoords *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        (*((*sc).c).offset(((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize))
            .index = (*((*(*ctx).prototype).c)
            .offset(((*sc).nc).wrapping_sub(1 as libc::c_int as size_t) as isize))
            .index;
        (*((*sc).c).offset((*sc).nc as isize)).index = -(1 as libc::c_int);
        (*((*sc).c).offset((*sc).nc as isize))
            .type_0 = (*((*(*ctx).prototype).c).offset((*sc).nc as isize)).type_0;
        (*sc).nc = ((*sc).nc).wrapping_add(1);
        (*sc).nc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_step_hex(
    mut ctx: *mut SpectreContext,
    mut sc: *mut SpectreCoords,
    mut depth: size_t,
    mut edge: libc::c_uint,
    mut outedge: *mut libc::c_uint,
) {
    let mut h: *const HexData = 0 as *const HexData;
    let mut m: *const MapEntry = 0 as *const MapEntry;
    spectrectx_extend_coords(ctx, sc, depth.wrapping_add(2 as libc::c_int as size_t));
    if 0 as libc::c_int <= (*((*sc).c).offset(depth as isize)).index {} else {
        __assert_fail(
            b"0 <= sc->c[depth].index\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14670: {
        if 0 as libc::c_int <= (*((*sc).c).offset(depth as isize)).index {} else {
            __assert_fail(
                b"0 <= sc->c[depth].index\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                359 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*((*sc).c).offset(depth as isize)).index as libc::c_uint)
        < num_subhexes((*((*sc).c).offset(depth as isize)).type_0)
    {} else {
        __assert_fail(
            b"sc->c[depth].index < num_subhexes(sc->c[depth].type)\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14579: {
        if ((*((*sc).c).offset(depth as isize)).index as libc::c_uint)
            < num_subhexes((*((*sc).c).offset(depth as isize)).type_0)
        {} else {
            __assert_fail(
                b"sc->c[depth].index < num_subhexes(sc->c[depth].type)\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                360 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int as libc::c_uint <= edge {} else {
        __assert_fail(
            b"0 <= edge\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14541: {
        if 0 as libc::c_int as libc::c_uint <= edge {} else {
            __assert_fail(
                b"0 <= edge\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if edge < 6 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"edge < 6\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 98],
                &[libc::c_char; 98],
            >(
                b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_14502: {
        if edge < 6 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"edge < 6\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                362 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    h = &*hexdata
        .as_ptr()
        .offset(
            (*((*sc).c).offset(depth.wrapping_add(1 as libc::c_int as size_t) as isize))
                .type_0 as isize,
        ) as *const HexData;
    m = &*((*h).hexmap)
        .offset(
            ((6 as libc::c_int * (*((*sc).c).offset(depth as isize)).index)
                as libc::c_uint)
                .wrapping_add(edge) as isize,
        ) as *const MapEntry;
    if !(*m).internal {
        let mut recedge: libc::c_uint = 0;
        let mut me: *const MapEdge = 0 as *const MapEdge;
        spectrectx_step_hex(
            ctx,
            sc,
            depth.wrapping_add(1 as libc::c_int as size_t),
            (*m).hi as libc::c_uint,
            &mut recedge,
        );
        if recedge < 6 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"recedge < 6\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                370 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_14400: {
            if recedge < 6 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"recedge < 6\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                    370 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 98],
                        &[libc::c_char; 98],
                    >(
                        b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        h = &*hexdata
            .as_ptr()
            .offset(
                (*((*sc).c)
                    .offset(depth.wrapping_add(1 as libc::c_int as size_t) as isize))
                    .type_0 as isize,
            ) as *const HexData;
        me = &*((*h).hexedges).offset(recedge as isize) as *const MapEdge;
        if ((*m).lo as libc::c_int) < (*me).len as libc::c_int {} else {
            __assert_fail(
                b"m->lo < me->len\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                373 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_14321: {
            if ((*m).lo as libc::c_int) < (*me).len as libc::c_int {} else {
                __assert_fail(
                    b"m->lo < me->len\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                    373 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 98],
                        &[libc::c_char; 98],
                    >(
                        b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        m = &*((*h).hexin)
            .offset(
                ((*me).startindex as libc::c_int + (*me).len as libc::c_int
                    - 1 as libc::c_int - (*m).lo as libc::c_int) as isize,
            ) as *const MapEntry;
        if (*m).internal {} else {
            __assert_fail(
                b"m->internal\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                375 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 98],
                    &[libc::c_char; 98],
                >(
                    b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_14255: {
            if (*m).internal {} else {
                __assert_fail(
                    b"m->internal\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                    375 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 98],
                        &[libc::c_char; 98],
                    >(
                        b"void spectrectx_step_hex(SpectreContext *, SpectreCoords *, size_t, unsigned int, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    (*((*sc).c).offset(depth as isize)).index = (*m).hi as libc::c_int;
    (*((*sc).c).offset(depth as isize))
        .type_0 = *((*h).subhexes)
        .offset((*((*sc).c).offset(depth as isize)).index as isize);
    *outedge = (*m).lo as libc::c_uint;
    if depth == 0 as libc::c_int as size_t {
        let mut new_hex_colour: libc::c_uchar = 0;
        if (edge ^ (*sc).incoming_hex_edge as libc::c_uint)
            & 1 as libc::c_int as libc::c_uint == 0
        {
            new_hex_colour = (*sc).prev_hex_colour;
        } else {
            new_hex_colour = (0 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
                - (*sc).hex_colour as libc::c_int - (*sc).prev_hex_colour as libc::c_int)
                as libc::c_uchar;
        }
        (*sc).prev_hex_colour = (*sc).hex_colour;
        (*sc).hex_colour = new_hex_colour;
        (*sc).incoming_hex_edge = (*m).lo;
    }
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_step(
    mut ctx: *mut SpectreContext,
    mut sc: *mut SpectreCoords,
    mut edge: libc::c_uint,
    mut outedge: *mut libc::c_uint,
) {
    let mut h: *const HexData = 0 as *const HexData;
    let mut m: *const MapEntry = 0 as *const MapEntry;
    if 0 as libc::c_int <= (*sc).index {} else {
        __assert_fail(
            b"0 <= sc->index\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15625: {
        if 0 as libc::c_int <= (*sc).index {} else {
            __assert_fail(
                b"0 <= sc->index\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*sc).index as libc::c_uint)
        < num_spectres((*((*sc).c).offset(0 as libc::c_int as isize)).type_0)
    {} else {
        __assert_fail(
            b"sc->index < num_spectres(sc->c[0].type)\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15548: {
        if ((*sc).index as libc::c_uint)
            < num_spectres((*((*sc).c).offset(0 as libc::c_int as isize)).type_0)
        {} else {
            __assert_fail(
                b"sc->index < num_spectres(sc->c[0].type)\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                413 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if 0 as libc::c_int as libc::c_uint <= edge {} else {
        __assert_fail(
            b"0 <= edge\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15510: {
        if 0 as libc::c_int as libc::c_uint <= edge {} else {
            __assert_fail(
                b"0 <= edge\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                414 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if edge < 14 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"edge < 14\0" as *const u8 as *const libc::c_char,
            b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
            415 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 86],
                &[libc::c_char; 86],
            >(
                b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_15472: {
        if edge < 14 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"edge < 14\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                415 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    h = &*hexdata
        .as_ptr()
        .offset((*((*sc).c).offset(0 as libc::c_int as isize)).type_0 as isize)
        as *const HexData;
    m = &*((*h).specmap)
        .offset(
            ((14 as libc::c_int * (*sc).index) as libc::c_uint).wrapping_add(edge)
                as isize,
        ) as *const MapEntry;
    while !(*m).internal {
        let mut recedge: libc::c_uint = 0;
        let mut me: *const MapEdge = 0 as *const MapEdge;
        spectrectx_step_hex(
            ctx,
            sc,
            0 as libc::c_int as size_t,
            (*m).hi as libc::c_uint,
            &mut recedge,
        );
        if recedge < 6 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"recedge < 6\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                424 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_14039: {
            if recedge < 6 as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"recedge < 6\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                    424 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        h = &*hexdata
            .as_ptr()
            .offset((*((*sc).c).offset(0 as libc::c_int as isize)).type_0 as isize)
            as *const HexData;
        me = &*((*h).specedges).offset(recedge as isize) as *const MapEdge;
        if ((*m).lo as libc::c_int) < (*me).len as libc::c_int {} else {
            __assert_fail(
                b"m->lo < me->len\0" as *const u8 as *const libc::c_char,
                b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                427 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 86],
                    &[libc::c_char; 86],
                >(
                    b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_4910: {
            if ((*m).lo as libc::c_int) < (*me).len as libc::c_int {} else {
                __assert_fail(
                    b"m->lo < me->len\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/spectre.c\0" as *const u8 as *const libc::c_char,
                    427 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 86],
                        &[libc::c_char; 86],
                    >(
                        b"void spectrectx_step(SpectreContext *, SpectreCoords *, unsigned int, unsigned int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        m = &*((*h).specin)
            .offset(
                ((*me).startindex as libc::c_int + (*me).len as libc::c_int
                    - 1 as libc::c_int - (*m).lo as libc::c_int) as isize,
            ) as *const MapEntry;
    }
    (*sc).index = (*m).hi as libc::c_int;
    *outedge = (*m).lo as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn spectrectx_generate(
    mut ctx: *mut SpectreContext,
    mut callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const Spectre) -> bool,
    >,
    mut cbctx: *mut libc::c_void,
) {
    let mut placed: *mut tree234 = newtree234(
        Some(
            spectre_cmp
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut qhead: *mut Spectre = 0 as *mut Spectre;
    let mut qtail: *mut Spectre = 0 as *mut Spectre;
    let mut spec: *mut Spectre = spectre_initial(ctx);
    add234(placed, spec as *mut libc::c_void);
    (*spec).next = 0 as *mut Spectre;
    if callback.expect("non-null function pointer")(cbctx, spec) {
        qtail = spec;
        qhead = qtail;
    }
    while !qhead.is_null() {
        let mut edge: libc::c_uint = 0;
        let mut spec_0: *mut Spectre = qhead;
        edge = 0 as libc::c_int as libc::c_uint;
        while edge < 14 as libc::c_int as libc::c_uint {
            let mut new_spec: *mut Spectre = 0 as *mut Spectre;
            new_spec = spectre_adjacent(ctx, spec_0, edge, 0 as *mut libc::c_uint);
            if !(find234(placed, new_spec as *mut libc::c_void, None)).is_null() {
                spectre_free(new_spec);
            } else if !callback.expect("non-null function pointer")(cbctx, new_spec) {
                spectre_free(new_spec);
            } else {
                add234(placed, new_spec as *mut libc::c_void);
                (*qtail).next = new_spec;
                qtail = new_spec;
                (*new_spec).next = 0 as *mut Spectre;
            }
            edge = edge.wrapping_add(1);
            edge;
        }
        qhead = (*qhead).next;
    }
    let mut spec_1: *mut Spectre = 0 as *mut Spectre;
    loop {
        spec_1 = delpos234(placed, 0 as libc::c_int) as *mut Spectre;
        if spec_1.is_null() {
            break;
        }
        spectre_free(spec_1);
    }
    freetree234(placed);
}
#[no_mangle]
pub unsafe extern "C" fn spectre_tiling_params_invalid(
    mut params: *const SpectrePatchParams,
) -> *const libc::c_char {
    let mut i: size_t = 0;
    let mut h: Hex = HEX_G;
    if (*params).ncoords == 0 as libc::c_int as size_t {
        return b"expected at least one numeric coordinate\0" as *const u8
            as *const libc::c_char;
    }
    if !spectre_valid_hex_letter((*params).final_hex) {
        return b"invalid final hexagon type\0" as *const u8 as *const libc::c_char;
    }
    h = hex_from_letter((*params).final_hex);
    i = (*params).ncoords;
    loop {
        let fresh2 = i;
        i = i.wrapping_sub(1);
        if !(fresh2 > 0 as libc::c_int as size_t) {
            break;
        }
        let mut limit: libc::c_uint = if i == 0 as libc::c_int as size_t {
            num_spectres(h)
        } else {
            num_subhexes(h)
        };
        if *((*params).coords).offset(i as isize) as libc::c_uint >= limit {
            return b"coordinate out of range\0" as *const u8 as *const libc::c_char;
        }
        if i > 0 as libc::c_int as size_t {
            h = *(hexdata[h as usize].subhexes)
                .offset(*((*params).coords).offset(i as isize) as isize);
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn spectre_internal_callback(
    mut vctx: *mut libc::c_void,
    mut spec: *const Spectre,
) -> bool {
    let mut ctx: *mut SpectreCallbackContext = vctx as *mut SpectreCallbackContext;
    let mut i: size_t = 0;
    let mut output_coords: [libc::c_int; 56] = [0; 56];
    i = 0 as libc::c_int as size_t;
    while i < 14 as libc::c_int as size_t {
        let mut p: Point = (*spec).vertices[i as usize];
        let mut x: Coord = point_x(p);
        let mut y: Coord = point_y(p);
        if coord_cmp(x, (*ctx).xmin) < 0 as libc::c_int
            || coord_cmp(x, (*ctx).xmax) > 0 as libc::c_int
            || coord_cmp(y, (*ctx).ymin) < 0 as libc::c_int
            || coord_cmp(y, (*ctx).ymax) > 0 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        output_coords[(4 as libc::c_int as size_t * i)
            .wrapping_add(0 as libc::c_int as size_t) as usize] = (*ctx).xoff + x.c1;
        output_coords[(4 as libc::c_int as size_t * i)
            .wrapping_add(1 as libc::c_int as size_t) as usize] = x.cr3;
        output_coords[(4 as libc::c_int as size_t * i)
            .wrapping_add(2 as libc::c_int as size_t) as usize] = (*ctx).yoff - y.c1;
        output_coords[(4 as libc::c_int as size_t * i)
            .wrapping_add(3 as libc::c_int as size_t) as usize] = -y.cr3;
        i = i.wrapping_add(1);
        i;
    }
    if ((*ctx).external_cb).is_some() {
        ((*ctx).external_cb)
            .expect(
                "non-null function pointer",
            )((*ctx).external_cbctx, output_coords.as_mut_ptr());
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn spectre_set_bounds(
    mut cbctx: *mut SpectreCallbackContext,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    (*cbctx).xoff = w / 2 as libc::c_int;
    (*cbctx).yoff = h / 2 as libc::c_int;
    (*cbctx).xmin.c1 = -(*cbctx).xoff;
    (*cbctx).xmax.c1 = -(*cbctx).xoff + w;
    (*cbctx).ymin.c1 = (*cbctx).yoff - h;
    (*cbctx).ymax.c1 = (*cbctx).yoff;
    (*cbctx).xmin.cr3 = 0 as libc::c_int;
    (*cbctx).xmax.cr3 = 0 as libc::c_int;
    (*cbctx).ymin.cr3 = 0 as libc::c_int;
    (*cbctx).ymax.cr3 = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spectre_tiling_randomise(
    mut ps: *mut SpectrePatchParams,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut rs: *mut random_state,
) {
    let mut ctx: [SpectreContext; 1] = [SpectreContext {
        rs: 0 as *mut random_state,
        must_free_rs: false,
        start_vertices: [Point { coeffs: [0; 4] }; 2],
        orientation: 0,
        prototype: 0 as *mut SpectreCoords,
    }; 1];
    let mut cbctx: [SpectreCallbackContext; 1] = [SpectreCallbackContext {
        xoff: 0,
        yoff: 0,
        xmin: Coord { c1: 0, cr3: 0 },
        xmax: Coord { c1: 0, cr3: 0 },
        ymin: Coord { c1: 0, cr3: 0 },
        ymax: Coord { c1: 0, cr3: 0 },
        external_cb: None,
        external_cbctx: 0 as *mut libc::c_void,
    }; 1];
    let mut i: size_t = 0;
    spectre_set_bounds(cbctx.as_mut_ptr(), w, h);
    let ref mut fresh3 = (*cbctx.as_mut_ptr()).external_cb;
    *fresh3 = None;
    let ref mut fresh4 = (*cbctx.as_mut_ptr()).external_cbctx;
    *fresh4 = 0 as *mut libc::c_void;
    spectrectx_init_random(ctx.as_mut_ptr(), rs);
    spectrectx_generate(
        ctx.as_mut_ptr(),
        Some(
            spectre_internal_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const Spectre) -> bool,
        ),
        cbctx.as_mut_ptr() as *mut libc::c_void,
    );
    (*ps).orientation = (*ctx.as_mut_ptr()).orientation;
    (*ps).ncoords = (*(*ctx.as_mut_ptr()).prototype).nc;
    (*ps)
        .coords = smalloc(
        ((*ps).ncoords)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    *((*ps).coords)
        .offset(
            0 as libc::c_int as isize,
        ) = (*(*ctx.as_mut_ptr()).prototype).index as libc::c_uchar;
    i = 1 as libc::c_int as size_t;
    while i < (*ps).ncoords {
        *((*ps).coords)
            .offset(
                i as isize,
            ) = (*((*(*ctx.as_mut_ptr()).prototype).c)
            .offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize))
            .index as libc::c_uchar;
        i = i.wrapping_add(1);
        i;
    }
    (*ps)
        .final_hex = hex_to_letter(
        (*((*(*ctx.as_mut_ptr()).prototype).c)
            .offset(((*ps).ncoords).wrapping_sub(1 as libc::c_int as size_t) as isize))
            .type_0 as libc::c_uchar,
    ) as libc::c_char;
    spectrectx_cleanup(ctx.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn spectre_tiling_generate(
    mut params: *const SpectrePatchParams,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut external_cb: spectre_tile_callback_fn,
    mut external_cbctx: *mut libc::c_void,
) {
    let mut ctx: [SpectreContext; 1] = [SpectreContext {
        rs: 0 as *mut random_state,
        must_free_rs: false,
        start_vertices: [Point { coeffs: [0; 4] }; 2],
        orientation: 0,
        prototype: 0 as *mut SpectreCoords,
    }; 1];
    let mut cbctx: [SpectreCallbackContext; 1] = [SpectreCallbackContext {
        xoff: 0,
        yoff: 0,
        xmin: Coord { c1: 0, cr3: 0 },
        xmax: Coord { c1: 0, cr3: 0 },
        ymin: Coord { c1: 0, cr3: 0 },
        ymax: Coord { c1: 0, cr3: 0 },
        external_cb: None,
        external_cbctx: 0 as *mut libc::c_void,
    }; 1];
    spectre_set_bounds(cbctx.as_mut_ptr(), w, h);
    let ref mut fresh5 = (*cbctx.as_mut_ptr()).external_cb;
    *fresh5 = external_cb;
    let ref mut fresh6 = (*cbctx.as_mut_ptr()).external_cbctx;
    *fresh6 = external_cbctx;
    spectrectx_init_from_params(ctx.as_mut_ptr(), params);
    spectrectx_generate(
        ctx.as_mut_ptr(),
        Some(
            spectre_internal_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const Spectre) -> bool,
        ),
        cbctx.as_mut_ptr() as *mut libc::c_void,
    );
    spectrectx_cleanup(ctx.as_mut_ptr());
}
unsafe extern "C" fn run_static_initializers() {
    hexdata = [
        {
            let mut init = HexData {
                hexmap: hexmap_G.as_ptr(),
                hexin: hexin_G.as_ptr(),
                specmap: specmap_G.as_ptr(),
                specin: specin_G.as_ptr(),
                hexedges: hexedges_G.as_ptr(),
                specedges: specedges_G.as_ptr(),
                subhexes: subhexes_G.as_ptr(),
                poss: poss_G.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 9]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_D.as_ptr(),
                hexin: hexin_D.as_ptr(),
                specmap: specmap_D.as_ptr(),
                specin: specin_D.as_ptr(),
                hexedges: hexedges_D.as_ptr(),
                specedges: specedges_D.as_ptr(),
                subhexes: subhexes_D.as_ptr(),
                poss: poss_D.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 9]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_J.as_ptr(),
                hexin: hexin_J.as_ptr(),
                specmap: specmap_J.as_ptr(),
                specin: specin_J.as_ptr(),
                hexedges: hexedges_J.as_ptr(),
                specedges: specedges_J.as_ptr(),
                subhexes: subhexes_J.as_ptr(),
                poss: poss_J.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 1]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_L.as_ptr(),
                hexin: hexin_L.as_ptr(),
                specmap: specmap_L.as_ptr(),
                specin: specin_L.as_ptr(),
                hexedges: hexedges_L.as_ptr(),
                specedges: specedges_L.as_ptr(),
                subhexes: subhexes_L.as_ptr(),
                poss: poss_L.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 1]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_X.as_ptr(),
                hexin: hexin_X.as_ptr(),
                specmap: specmap_X.as_ptr(),
                specin: specin_X.as_ptr(),
                hexedges: hexedges_X.as_ptr(),
                specedges: specedges_X.as_ptr(),
                subhexes: subhexes_X.as_ptr(),
                poss: poss_X.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 7]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_P.as_ptr(),
                hexin: hexin_P.as_ptr(),
                specmap: specmap_P.as_ptr(),
                specin: specin_P.as_ptr(),
                hexedges: hexedges_P.as_ptr(),
                specedges: specedges_P.as_ptr(),
                subhexes: subhexes_P.as_ptr(),
                poss: poss_P.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 8]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_S.as_ptr(),
                hexin: hexin_S.as_ptr(),
                specmap: specmap_S.as_ptr(),
                specin: specin_S.as_ptr(),
                hexedges: hexedges_S.as_ptr(),
                specedges: specedges_S.as_ptr(),
                subhexes: subhexes_S.as_ptr(),
                poss: poss_S.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 9]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_F.as_ptr(),
                hexin: hexin_F.as_ptr(),
                specmap: specmap_F.as_ptr(),
                specin: specin_F.as_ptr(),
                hexedges: hexedges_F.as_ptr(),
                specedges: specedges_F.as_ptr(),
                subhexes: subhexes_F.as_ptr(),
                poss: poss_F.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 16]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
        {
            let mut init = HexData {
                hexmap: hexmap_Y.as_ptr(),
                hexin: hexin_Y.as_ptr(),
                specmap: specmap_Y.as_ptr(),
                specin: specin_Y.as_ptr(),
                hexedges: hexedges_Y.as_ptr(),
                specedges: specedges_Y.as_ptr(),
                subhexes: subhexes_Y.as_ptr(),
                poss: poss_Y.as_ptr(),
                nposs: (::core::mem::size_of::<[Possibility; 11]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<Possibility>() as libc::c_ulong),
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
