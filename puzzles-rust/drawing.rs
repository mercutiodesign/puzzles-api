use ::libc;
extern "C" {
    pub type midend;
    pub type blitter;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn midend_rewrite_statusbar(me: *mut midend, text: *const libc::c_char) -> *mut libc::c_char;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn srealloc(p: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn dupstr(s: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawing {
    pub api: *const drawing_api,
    pub handle: *mut libc::c_void,
    pub colours: *mut print_colour,
    pub ncolours: libc::c_int,
    pub coloursize: libc::c_int,
    pub scale: libc::c_float,
    pub me: *mut midend,
    pub laststatus: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_colour {
    pub hatch: libc::c_int,
    pub hatch_when: libc::c_int,
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub grey: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawing_api {
    pub draw_text: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_char,
        ) -> (),
    >,
    pub draw_rect: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_line: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_polygon: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_circle: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_update: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub clip: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub unclip: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub start_draw: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub end_draw: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub status_bar: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> ()>,
    pub blitter_new:
        Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> *mut blitter>,
    pub blitter_free: Option<unsafe extern "C" fn(*mut libc::c_void, *mut blitter) -> ()>,
    pub blitter_save: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut blitter, libc::c_int, libc::c_int) -> (),
    >,
    pub blitter_load: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut blitter, libc::c_int, libc::c_int) -> (),
    >,
    pub begin_doc: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub begin_page: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub begin_puzzle: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_float,
            libc::c_float,
            libc::c_float,
            libc::c_float,
            libc::c_int,
            libc::c_int,
            libc::c_float,
        ) -> (),
    >,
    pub end_puzzle: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub end_page: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub end_doc: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub line_width: Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_float) -> ()>,
    pub line_dotted: Option<unsafe extern "C" fn(*mut libc::c_void, bool) -> ()>,
    pub text_fallback: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub draw_thick_line: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_float,
            libc::c_float,
            libc::c_float,
            libc::c_float,
            libc::c_float,
            libc::c_int,
        ) -> (),
    >,
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[no_mangle]
pub unsafe extern "C" fn drawing_new(
    mut api: *const drawing_api,
    mut me: *mut midend,
    mut handle: *mut libc::c_void,
) -> *mut drawing {
    let mut dr: *mut drawing =
        smalloc(::core::mem::size_of::<drawing>() as libc::c_ulong) as *mut drawing;
    (*dr).api = api;
    (*dr).handle = handle;
    (*dr).colours = 0 as *mut print_colour;
    (*dr).coloursize = 0 as libc::c_int;
    (*dr).ncolours = (*dr).coloursize;
    (*dr).scale = 1.0f32;
    (*dr).me = me;
    (*dr).laststatus = 0 as *mut libc::c_char;
    return dr;
}
#[no_mangle]
pub unsafe extern "C" fn drawing_free(mut dr: *mut drawing) {
    sfree((*dr).laststatus as *mut libc::c_void);
    sfree((*dr).colours as *mut libc::c_void);
    sfree(dr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn draw_text(
    mut dr: *mut drawing,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut fonttype: libc::c_int,
    mut fontsize: libc::c_int,
    mut align: libc::c_int,
    mut colour: libc::c_int,
    mut text: *const libc::c_char,
) {
    ((*(*dr).api).draw_text).expect("non-null function pointer")(
        (*dr).handle,
        x,
        y,
        fonttype,
        fontsize,
        align,
        colour,
        text,
    );
}
#[no_mangle]
pub unsafe extern "C" fn draw_rect(
    mut dr: *mut drawing,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut colour: libc::c_int,
) {
    ((*(*dr).api).draw_rect).expect("non-null function pointer")((*dr).handle, x, y, w, h, colour);
}
#[no_mangle]
pub unsafe extern "C" fn draw_line(
    mut dr: *mut drawing,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut colour: libc::c_int,
) {
    ((*(*dr).api).draw_line).expect("non-null function pointer")(
        (*dr).handle,
        x1,
        y1,
        x2,
        y2,
        colour,
    );
}
#[no_mangle]
pub unsafe extern "C" fn draw_thick_line(
    mut dr: *mut drawing,
    mut thickness: libc::c_float,
    mut x1: libc::c_float,
    mut y1: libc::c_float,
    mut x2: libc::c_float,
    mut y2: libc::c_float,
    mut colour: libc::c_int,
) {
    if thickness < 1.0f32 {
        thickness = 1.0f32;
    }
    if ((*(*dr).api).draw_thick_line).is_some() {
        ((*(*dr).api).draw_thick_line).expect("non-null function pointer")(
            (*dr).handle,
            thickness,
            x1,
            y1,
            x2,
            y2,
            colour,
        );
    } else {
        let mut len: libc::c_float = __tg_sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
        let mut tvhatx: libc::c_float =
            (x2 - x1) / len * (thickness / 2 as libc::c_int as libc::c_float - 0.2f32);
        let mut tvhaty: libc::c_float =
            (y2 - y1) / len * (thickness / 2 as libc::c_int as libc::c_float - 0.2f32);
        let mut p: [libc::c_int; 8] = [0; 8];
        p[0 as libc::c_int as usize] = (x1 - tvhaty) as libc::c_int;
        p[1 as libc::c_int as usize] = (y1 + tvhatx) as libc::c_int;
        p[2 as libc::c_int as usize] = (x2 - tvhaty) as libc::c_int;
        p[3 as libc::c_int as usize] = (y2 + tvhatx) as libc::c_int;
        p[4 as libc::c_int as usize] = (x2 + tvhaty) as libc::c_int;
        p[5 as libc::c_int as usize] = (y2 - tvhatx) as libc::c_int;
        p[6 as libc::c_int as usize] = (x1 + tvhaty) as libc::c_int;
        p[7 as libc::c_int as usize] = (y1 - tvhatx) as libc::c_int;
        ((*(*dr).api).draw_polygon).expect("non-null function pointer")(
            (*dr).handle,
            p.as_mut_ptr(),
            4 as libc::c_int,
            colour,
            colour,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn draw_polygon(
    mut dr: *mut drawing,
    mut coords: *const libc::c_int,
    mut npoints: libc::c_int,
    mut fillcolour: libc::c_int,
    mut outlinecolour: libc::c_int,
) {
    ((*(*dr).api).draw_polygon).expect("non-null function pointer")(
        (*dr).handle,
        coords,
        npoints,
        fillcolour,
        outlinecolour,
    );
}
#[no_mangle]
pub unsafe extern "C" fn draw_circle(
    mut dr: *mut drawing,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut radius: libc::c_int,
    mut fillcolour: libc::c_int,
    mut outlinecolour: libc::c_int,
) {
    ((*(*dr).api).draw_circle).expect("non-null function pointer")(
        (*dr).handle,
        cx,
        cy,
        radius,
        fillcolour,
        outlinecolour,
    );
}
#[no_mangle]
pub unsafe extern "C" fn draw_update(
    mut dr: *mut drawing,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    if ((*(*dr).api).draw_update).is_some() {
        ((*(*dr).api).draw_update).expect("non-null function pointer")((*dr).handle, x, y, w, h);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clip(
    mut dr: *mut drawing,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    ((*(*dr).api).clip).expect("non-null function pointer")((*dr).handle, x, y, w, h);
}
#[no_mangle]
pub unsafe extern "C" fn unclip(mut dr: *mut drawing) {
    ((*(*dr).api).unclip).expect("non-null function pointer")((*dr).handle);
}
#[no_mangle]
pub unsafe extern "C" fn start_draw(mut dr: *mut drawing) {
    ((*(*dr).api).start_draw).expect("non-null function pointer")((*dr).handle);
}
#[no_mangle]
pub unsafe extern "C" fn end_draw(mut dr: *mut drawing) {
    ((*(*dr).api).end_draw).expect("non-null function pointer")((*dr).handle);
}
#[no_mangle]
pub unsafe extern "C" fn text_fallback(
    mut dr: *mut drawing,
    mut strings: *const *const libc::c_char,
    mut nstrings: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    if !dr.is_null() && ((*(*dr).api).text_fallback).is_some() {
        return ((*(*dr).api).text_fallback).expect("non-null function pointer")(
            (*dr).handle,
            strings,
            nstrings,
        );
    }
    i = 0 as libc::c_int;
    while i < nstrings {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        p = *strings.offset(i as isize);
        while *p != 0 {
            if *p as libc::c_int & 0x80 as libc::c_int != 0 {
                break;
            }
            p = p.offset(1);
        }
        if *p == 0 {
            return dupstr(*strings.offset(i as isize));
        }
        i += 1;
    }
    if (b"Should never get here\0" as *const u8 as *const libc::c_char).is_null() {
    } else {
        __assert_fail(
            b"!\"Should never get here\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/drawing.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"char *text_fallback(drawing *, const char *const *, int)\0",
            ))
            .as_ptr(),
        );
    }

    if (b"Should never get here\0" as *const u8 as *const libc::c_char).is_null() {
    } else {
        __assert_fail(
            b"!\"Should never get here\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/drawing.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 57], &[libc::c_char; 57]>(
                b"char *text_fallback(drawing *, const char *const *, int)\0",
            ))
            .as_ptr(),
        );
    }

    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn status_bar(mut dr: *mut drawing, mut text: *const libc::c_char) {
    let mut rewritten: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*(*dr).api).status_bar).is_none() {
        return;
    }
    if !((*dr).me).is_null() {
    } else {
        __assert_fail(
            b"dr->me\0" as *const u8 as *const libc::c_char,
            b"/puzzles/drawing.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void status_bar(drawing *, const char *)\0",
            ))
            .as_ptr(),
        );
    }

    if !((*dr).me).is_null() {
    } else {
        __assert_fail(
            b"dr->me\0" as *const u8 as *const libc::c_char,
            b"/puzzles/drawing.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"void status_bar(drawing *, const char *)\0",
            ))
            .as_ptr(),
        );
    }

    rewritten = midend_rewrite_statusbar((*dr).me, text);
    if ((*dr).laststatus).is_null() || strcmp(rewritten, (*dr).laststatus) != 0 {
        ((*(*dr).api).status_bar).expect("non-null function pointer")((*dr).handle, rewritten);
        sfree((*dr).laststatus as *mut libc::c_void);
        (*dr).laststatus = rewritten;
    } else {
        sfree(rewritten as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn blitter_new(
    mut dr: *mut drawing,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> *mut blitter {
    return ((*(*dr).api).blitter_new).expect("non-null function pointer")((*dr).handle, w, h);
}
#[no_mangle]
pub unsafe extern "C" fn blitter_free(mut dr: *mut drawing, mut bl: *mut blitter) {
    ((*(*dr).api).blitter_free).expect("non-null function pointer")((*dr).handle, bl);
}
#[no_mangle]
pub unsafe extern "C" fn blitter_save(
    mut dr: *mut drawing,
    mut bl: *mut blitter,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    ((*(*dr).api).blitter_save).expect("non-null function pointer")((*dr).handle, bl, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn blitter_load(
    mut dr: *mut drawing,
    mut bl: *mut blitter,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    ((*(*dr).api).blitter_load).expect("non-null function pointer")((*dr).handle, bl, x, y);
}
#[no_mangle]
pub unsafe extern "C" fn print_begin_doc(mut dr: *mut drawing, mut pages: libc::c_int) {
    ((*(*dr).api).begin_doc).expect("non-null function pointer")((*dr).handle, pages);
}
#[no_mangle]
pub unsafe extern "C" fn print_begin_page(mut dr: *mut drawing, mut number: libc::c_int) {
    ((*(*dr).api).begin_page).expect("non-null function pointer")((*dr).handle, number);
}
#[no_mangle]
pub unsafe extern "C" fn print_begin_puzzle(
    mut dr: *mut drawing,
    mut xm: libc::c_float,
    mut xc: libc::c_float,
    mut ym: libc::c_float,
    mut yc: libc::c_float,
    mut pw: libc::c_int,
    mut ph: libc::c_int,
    mut wmm: libc::c_float,
    mut scale: libc::c_float,
) {
    (*dr).scale = scale;
    (*dr).ncolours = 0 as libc::c_int;
    ((*(*dr).api).begin_puzzle).expect("non-null function pointer")(
        (*dr).handle,
        xm,
        xc,
        ym,
        yc,
        pw,
        ph,
        wmm,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_end_puzzle(mut dr: *mut drawing) {
    ((*(*dr).api).end_puzzle).expect("non-null function pointer")((*dr).handle);
    (*dr).scale = 1.0f32;
}
#[no_mangle]
pub unsafe extern "C" fn print_end_page(mut dr: *mut drawing, mut number: libc::c_int) {
    ((*(*dr).api).end_page).expect("non-null function pointer")((*dr).handle, number);
}
#[no_mangle]
pub unsafe extern "C" fn print_end_doc(mut dr: *mut drawing) {
    ((*(*dr).api).end_doc).expect("non-null function pointer")((*dr).handle);
}
#[no_mangle]
pub unsafe extern "C" fn print_get_colour(
    mut dr: *mut drawing,
    mut colour: libc::c_int,
    mut printing_in_colour: bool,
    mut hatch: *mut libc::c_int,
    mut r: *mut libc::c_float,
    mut g: *mut libc::c_float,
    mut b: *mut libc::c_float,
) {
    if colour >= 0 as libc::c_int && colour < (*dr).ncolours {
    } else {
        __assert_fail(
            b"colour >= 0 && colour < dr->ncolours\0" as *const u8 as *const libc::c_char,
            b"/puzzles/drawing.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 79], &[libc::c_char; 79]>(
                b"void print_get_colour(drawing *, int, _Bool, int *, float *, float *, float *)\0",
            ))
            .as_ptr(),
        );
    }

    if colour >= 0 as libc::c_int && colour < (*dr).ncolours {
    } else {
        __assert_fail(
            b"colour >= 0 && colour < dr->ncolours\0" as *const u8 as *const libc::c_char,
            b"/puzzles/drawing.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 79], &[libc::c_char; 79]>(
                b"void print_get_colour(drawing *, int, _Bool, int *, float *, float *, float *)\0",
            ))
            .as_ptr(),
        );
    }

    if (*((*dr).colours).offset(colour as isize)).hatch_when == 2 as libc::c_int
        || (*((*dr).colours).offset(colour as isize)).hatch_when == 1 as libc::c_int
            && !printing_in_colour
    {
        *hatch = (*((*dr).colours).offset(colour as isize)).hatch;
    } else {
        *hatch = -(1 as libc::c_int);
        if printing_in_colour {
            *r = (*((*dr).colours).offset(colour as isize)).r;
            *g = (*((*dr).colours).offset(colour as isize)).g;
            *b = (*((*dr).colours).offset(colour as isize)).b;
        } else {
            *b = (*((*dr).colours).offset(colour as isize)).grey;
            *g = *b;
            *r = *g;
        }
    };
}
unsafe extern "C" fn print_generic_colour(
    mut dr: *mut drawing,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut grey: libc::c_float,
    mut hatch: libc::c_int,
    mut hatch_when: libc::c_int,
) -> libc::c_int {
    if (*dr).ncolours >= (*dr).coloursize {
        (*dr).coloursize = (*dr).ncolours + 16 as libc::c_int;
        (*dr).colours = srealloc(
            (*dr).colours as *mut libc::c_void,
            ((*dr).coloursize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<print_colour>() as libc::c_ulong),
        ) as *mut print_colour;
    }
    (*((*dr).colours).offset((*dr).ncolours as isize)).hatch = hatch;
    (*((*dr).colours).offset((*dr).ncolours as isize)).hatch_when = hatch_when;
    (*((*dr).colours).offset((*dr).ncolours as isize)).r = r;
    (*((*dr).colours).offset((*dr).ncolours as isize)).g = g;
    (*((*dr).colours).offset((*dr).ncolours as isize)).b = b;
    (*((*dr).colours).offset((*dr).ncolours as isize)).grey = grey;
    let fresh0 = (*dr).ncolours;
    (*dr).ncolours = (*dr).ncolours + 1;
    return fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn print_mono_colour(
    mut dr: *mut drawing,
    mut grey: libc::c_int,
) -> libc::c_int {
    return print_generic_colour(
        dr,
        grey as libc::c_float,
        grey as libc::c_float,
        grey as libc::c_float,
        grey as libc::c_float,
        -(1 as libc::c_int),
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_grey_colour(
    mut dr: *mut drawing,
    mut grey: libc::c_float,
) -> libc::c_int {
    return print_generic_colour(
        dr,
        grey,
        grey,
        grey,
        grey,
        -(1 as libc::c_int),
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_hatched_colour(
    mut dr: *mut drawing,
    mut hatch: libc::c_int,
) -> libc::c_int {
    return print_generic_colour(
        dr,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        hatch,
        2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_rgb_mono_colour(
    mut dr: *mut drawing,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut grey: libc::c_int,
) -> libc::c_int {
    return print_generic_colour(
        dr,
        r,
        g,
        b,
        grey as libc::c_float,
        -(1 as libc::c_int),
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_rgb_grey_colour(
    mut dr: *mut drawing,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut grey: libc::c_float,
) -> libc::c_int {
    return print_generic_colour(dr, r, g, b, grey, -(1 as libc::c_int), 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn print_rgb_hatched_colour(
    mut dr: *mut drawing,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut hatch: libc::c_int,
) -> libc::c_int {
    return print_generic_colour(
        dr,
        r,
        g,
        b,
        0 as libc::c_int as libc::c_float,
        hatch,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_line_width(mut dr: *mut drawing, mut width: libc::c_int) {
    ((*(*dr).api).line_width).expect("non-null function pointer")(
        (*dr).handle,
        __tg_sqrt((*dr).scale) * width as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn print_line_dotted(mut dr: *mut drawing, mut dotted: bool) {
    ((*(*dr).api).line_dotted).expect("non-null function pointer")((*dr).handle, dotted);
}
