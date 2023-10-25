#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(c_variadic, extern_types, label_break_value)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type midend;
    pub type drawing;
    pub type blitter;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn print_get_colour(
        dr: *mut drawing,
        colour: libc::c_int,
        printing_in_colour: bool,
        hatch: *mut libc::c_int,
        r: *mut libc::c_float,
        g: *mut libc::c_float,
        b: *mut libc::c_float,
    );
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn drawing_new(
        api: *const drawing_api,
        me: *mut midend,
        handle: *mut libc::c_void,
    ) -> *mut drawing;
    fn drawing_free(dr: *mut drawing);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawing_api {
    pub draw_text: Option::<
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
    pub draw_rect: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_line: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_polygon: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_circle: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub draw_update: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub clip: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub unclip: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub start_draw: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub end_draw: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub status_bar: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char) -> (),
    >,
    pub blitter_new: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> *mut blitter,
    >,
    pub blitter_free: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut blitter) -> (),
    >,
    pub blitter_save: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut blitter,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub blitter_load: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut blitter,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub begin_doc: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub begin_page: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub begin_puzzle: Option::<
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
    pub end_puzzle: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub end_page: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub end_doc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub line_width: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_float) -> (),
    >,
    pub line_dotted: Option::<unsafe extern "C" fn(*mut libc::c_void, bool) -> ()>,
    pub text_fallback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const *const libc::c_char,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub draw_thick_line: Option::<
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct psdata {
    pub fp: *mut FILE,
    pub colour: bool,
    pub ytop: libc::c_int,
    pub clipped: bool,
    pub hatchthick: libc::c_float,
    pub hatchspace: libc::c_float,
    pub gamewidth: libc::c_int,
    pub gameheight: libc::c_int,
    pub drawing: *mut drawing,
}
unsafe extern "C" fn ps_printf(
    mut ps: *mut psdata,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vfprintf((*ps).fp, fmt, ap.as_va_list());
}
unsafe extern "C" fn ps_fill(mut ps: *mut psdata, mut colour: libc::c_int) {
    let mut hatch: libc::c_int = 0;
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    print_get_colour(
        (*ps).drawing,
        colour,
        (*ps).colour,
        &mut hatch,
        &mut r,
        &mut g,
        &mut b,
    );
    if hatch < 0 as libc::c_int {
        if (*ps).colour {
            ps_printf(
                ps,
                b"%g %g %g setrgbcolor fill\n\0" as *const u8 as *const libc::c_char,
                r as libc::c_double,
                g as libc::c_double,
                b as libc::c_double,
            );
        } else {
            ps_printf(
                ps,
                b"%g setgray fill\n\0" as *const u8 as *const libc::c_char,
                r as libc::c_double,
            );
        }
    } else {
        ps_printf(ps, b"gsave clip\n\0" as *const u8 as *const libc::c_char);
        ps_printf(ps, b"newpath\n\0" as *const u8 as *const libc::c_char);
        if hatch == 4 as libc::c_int || hatch == 5 as libc::c_int {
            ps_printf(
                ps,
                b"0 %g %d {\n  0 moveto 0 %d rlineto\n} for\n\0" as *const u8
                    as *const libc::c_char,
                (*ps).hatchspace as libc::c_double,
                (*ps).gamewidth,
                (*ps).gameheight,
            );
        }
        if hatch == 3 as libc::c_int || hatch == 5 as libc::c_int {
            ps_printf(
                ps,
                b"0 %g %d {\n  0 exch moveto %d 0 rlineto\n} for\n\0" as *const u8
                    as *const libc::c_char,
                (*ps).hatchspace as libc::c_double,
                (*ps).gameheight,
                (*ps).gamewidth,
            );
        }
        if hatch == 1 as libc::c_int || hatch == 6 as libc::c_int {
            ps_printf(
                ps,
                b"%d %g %d {\n  0 moveto %d dup rlineto\n} for\n\0" as *const u8
                    as *const libc::c_char,
                -(*ps).gameheight,
                (*ps).hatchspace as libc::c_double
                    * 1.414213562373095048801688724209698078569672f64,
                (*ps).gamewidth,
                if (*ps).gamewidth > (*ps).gameheight {
                    (*ps).gamewidth
                } else {
                    (*ps).gameheight
                },
            );
        }
        if hatch == 2 as libc::c_int || hatch == 6 as libc::c_int {
            ps_printf(
                ps,
                b"0 %g %d {\n  0 moveto %d neg dup neg rlineto\n} for\n\0" as *const u8
                    as *const libc::c_char,
                (*ps).hatchspace as libc::c_double
                    * 1.414213562373095048801688724209698078569672f64,
                (*ps).gamewidth + (*ps).gameheight,
                if (*ps).gamewidth > (*ps).gameheight {
                    (*ps).gamewidth
                } else {
                    (*ps).gameheight
                },
            );
        }
        ps_printf(
            ps,
            b"0 setgray %g setlinewidth stroke grestore\n\0" as *const u8
                as *const libc::c_char,
            (*ps).hatchthick as libc::c_double,
        );
    };
}
unsafe extern "C" fn ps_setcolour_internal(
    mut ps: *mut psdata,
    mut colour: libc::c_int,
    mut suffix: *const libc::c_char,
) {
    let mut hatch: libc::c_int = 0;
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    print_get_colour(
        (*ps).drawing,
        colour,
        (*ps).colour,
        &mut hatch,
        &mut r,
        &mut g,
        &mut b,
    );
    if hatch < 0 as libc::c_int {} else {
        __assert_fail(
            b"hatch < 0\0" as *const u8 as *const libc::c_char,
            b"/puzzles/ps.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"void ps_setcolour_internal(psdata *, int, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_4080: {
        if hatch < 0 as libc::c_int {} else {
            __assert_fail(
                b"hatch < 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/ps.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"void ps_setcolour_internal(psdata *, int, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*ps).colour {
        ps_printf(
            ps,
            b"%g %g %g setrgbcolor%s\n\0" as *const u8 as *const libc::c_char,
            r as libc::c_double,
            g as libc::c_double,
            b as libc::c_double,
            suffix,
        );
    } else {
        ps_printf(
            ps,
            b"%g setgray%s\n\0" as *const u8 as *const libc::c_char,
            r as libc::c_double,
            suffix,
        );
    };
}
unsafe extern "C" fn ps_setcolour(mut ps: *mut psdata, mut colour: libc::c_int) {
    ps_setcolour_internal(ps, colour, b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ps_stroke(mut ps: *mut psdata, mut colour: libc::c_int) {
    ps_setcolour_internal(ps, colour, b" stroke\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ps_draw_text(
    mut handle: *mut libc::c_void,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut fonttype: libc::c_int,
    mut fontsize: libc::c_int,
    mut align: libc::c_int,
    mut colour: libc::c_int,
    mut text: *const libc::c_char,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    y = (*ps).ytop - y;
    ps_setcolour(ps, colour);
    ps_printf(
        ps,
        b"/%s findfont %d scalefont setfont\n\0" as *const u8 as *const libc::c_char,
        if fonttype == 0 as libc::c_int {
            b"Courier-L1\0" as *const u8 as *const libc::c_char
        } else {
            b"Helvetica-L1\0" as *const u8 as *const libc::c_char
        },
        fontsize,
    );
    if align & 0x100 as libc::c_int != 0 {
        ps_printf(
            ps,
            b"newpath 0 0 moveto (X) true charpath flattenpath pathbbox\n3 -1 roll add 2 div %d exch sub %d exch moveto pop pop\n\0"
                as *const u8 as *const libc::c_char,
            y,
            x,
        );
    } else {
        ps_printf(ps, b"%d %d moveto\n\0" as *const u8 as *const libc::c_char, x, y);
    }
    ps_printf(ps, b"(\0" as *const u8 as *const libc::c_char);
    while *text != 0 {
        if *text as libc::c_int == '\\' as i32 || *text as libc::c_int == '(' as i32
            || *text as libc::c_int == ')' as i32
        {
            ps_printf(ps, b"\\\0" as *const u8 as *const libc::c_char);
        }
        ps_printf(ps, b"%c\0" as *const u8 as *const libc::c_char, *text as libc::c_int);
        text = text.offset(1);
        text;
    }
    ps_printf(ps, b") \0" as *const u8 as *const libc::c_char);
    if align & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        ps_printf(
            ps,
            b"dup stringwidth pop %sneg 0 rmoveto show\n\0" as *const u8
                as *const libc::c_char,
            if align & 0x1 as libc::c_int != 0 {
                b"2 div \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        ps_printf(ps, b"show\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn ps_draw_rect(
    mut handle: *mut libc::c_void,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut colour: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    y = (*ps).ytop - y;
    ps_printf(
        ps,
        b"newpath %g %g moveto %d 0 rlineto 0 %d rlineto %d 0 rlineto closepath\n\0"
            as *const u8 as *const libc::c_char,
        x as libc::c_double - 0.5f64,
        y as libc::c_double + 0.5f64,
        w,
        -h,
        -w,
    );
    ps_fill(ps, colour);
}
unsafe extern "C" fn ps_draw_line(
    mut handle: *mut libc::c_void,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut x2: libc::c_int,
    mut y2: libc::c_int,
    mut colour: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    y1 = (*ps).ytop - y1;
    y2 = (*ps).ytop - y2;
    ps_printf(
        ps,
        b"newpath %d %d moveto %d %d lineto\n\0" as *const u8 as *const libc::c_char,
        x1,
        y1,
        x2,
        y2,
    );
    ps_stroke(ps, colour);
}
unsafe extern "C" fn ps_draw_polygon(
    mut handle: *mut libc::c_void,
    mut coords: *const libc::c_int,
    mut npoints: libc::c_int,
    mut fillcolour: libc::c_int,
    mut outlinecolour: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    let mut i: libc::c_int = 0;
    ps_printf(
        ps,
        b"newpath %d %d moveto\n\0" as *const u8 as *const libc::c_char,
        *coords.offset(0 as libc::c_int as isize),
        (*ps).ytop - *coords.offset(1 as libc::c_int as isize),
    );
    i = 1 as libc::c_int;
    while i < npoints {
        ps_printf(
            ps,
            b"%d %d lineto\n\0" as *const u8 as *const libc::c_char,
            *coords.offset((i * 2 as libc::c_int) as isize),
            (*ps).ytop
                - *coords.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize),
        );
        i += 1;
        i;
    }
    ps_printf(ps, b"closepath\n\0" as *const u8 as *const libc::c_char);
    if fillcolour >= 0 as libc::c_int {
        ps_printf(ps, b"gsave\n\0" as *const u8 as *const libc::c_char);
        ps_fill(ps, fillcolour);
        ps_printf(ps, b"grestore\n\0" as *const u8 as *const libc::c_char);
    }
    ps_stroke(ps, outlinecolour);
}
unsafe extern "C" fn ps_draw_circle(
    mut handle: *mut libc::c_void,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut radius: libc::c_int,
    mut fillcolour: libc::c_int,
    mut outlinecolour: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    cy = (*ps).ytop - cy;
    ps_printf(
        ps,
        b"newpath %d %d %d 0 360 arc closepath\n\0" as *const u8 as *const libc::c_char,
        cx,
        cy,
        radius,
    );
    if fillcolour >= 0 as libc::c_int {
        ps_printf(ps, b"gsave\n\0" as *const u8 as *const libc::c_char);
        ps_fill(ps, fillcolour);
        ps_printf(ps, b"grestore\n\0" as *const u8 as *const libc::c_char);
    }
    ps_stroke(ps, outlinecolour);
}
unsafe extern "C" fn ps_unclip(mut handle: *mut libc::c_void) {
    let mut ps: *mut psdata = handle as *mut psdata;
    if (*ps).clipped {} else {
        __assert_fail(
            b"ps->clipped\0" as *const u8 as *const libc::c_char,
            b"/puzzles/ps.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void ps_unclip(void *)\0"))
                .as_ptr(),
        );
    }
    'c_3857: {
        if (*ps).clipped {} else {
            __assert_fail(
                b"ps->clipped\0" as *const u8 as *const libc::c_char,
                b"/puzzles/ps.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void ps_unclip(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    ps_printf(ps, b"grestore\n\0" as *const u8 as *const libc::c_char);
    (*ps).clipped = 0 as libc::c_int != 0;
}
unsafe extern "C" fn ps_clip(
    mut handle: *mut libc::c_void,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    if (*ps).clipped {
        ps_unclip(ps as *mut libc::c_void);
    }
    y = (*ps).ytop - y;
    ps_printf(ps, b"gsave\n\0" as *const u8 as *const libc::c_char);
    ps_printf(
        ps,
        b"newpath %g %g moveto %d 0 rlineto 0 %d rlineto %d 0 rlineto closepath\n\0"
            as *const u8 as *const libc::c_char,
        x as libc::c_double - 0.5f64,
        y as libc::c_double + 0.5f64,
        w,
        -h,
        -w,
    );
    ps_printf(ps, b"clip\n\0" as *const u8 as *const libc::c_char);
    (*ps).clipped = 1 as libc::c_int != 0;
}
unsafe extern "C" fn ps_line_width(
    mut handle: *mut libc::c_void,
    mut width: libc::c_float,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    ps_printf(
        ps,
        b"%g setlinewidth\n\0" as *const u8 as *const libc::c_char,
        width as libc::c_double,
    );
}
unsafe extern "C" fn ps_line_dotted(mut handle: *mut libc::c_void, mut dotted: bool) {
    let mut ps: *mut psdata = handle as *mut psdata;
    if dotted {
        ps_printf(
            ps,
            b"[ currentlinewidth 3 mul ] 0 setdash\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        ps_printf(ps, b"[ ] 0 setdash\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn ps_text_fallback(
    mut handle: *mut libc::c_void,
    mut strings: *const *const libc::c_char,
    mut nstrings: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    maxlen = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nstrings {
        let mut len: libc::c_int = strlen(*strings.offset(i as isize)) as libc::c_int;
        if maxlen < len {
            maxlen = len;
        }
        i += 1;
        i;
    }
    ret = smalloc(
        ((maxlen + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < nstrings {
        let mut p: *const libc::c_char = *strings.offset(i as isize);
        let mut q: *mut libc::c_char = ret;
        while *p != 0 {
            let fresh0 = p;
            p = p.offset(1);
            let mut c: libc::c_int = *fresh0 as libc::c_uchar as libc::c_int;
            if c < 0x80 as libc::c_int {
                let fresh1 = q;
                q = q.offset(1);
                *fresh1 = c as libc::c_char;
            } else {
                if !((c == 0xc2 as libc::c_int || c == 0xc3 as libc::c_int)
                    && *p as libc::c_int & 0xc0 as libc::c_int == 0x80 as libc::c_int)
                {
                    break;
                }
                let fresh2 = p;
                p = p.offset(1);
                let fresh3 = q;
                q = q.offset(1);
                *fresh3 = (c << 6 as libc::c_int
                    | *fresh2 as libc::c_int & 0x3f as libc::c_int) as libc::c_char;
            }
        }
        if *p == 0 {
            *q = '\0' as i32 as libc::c_char;
            return ret;
        }
        i += 1;
        i;
    }
    if (b"Should never reach here\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"!\"Should never reach here\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/ps.c\0" as *const u8 as *const libc::c_char,
            283 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"char *ps_text_fallback(void *, const char *const *, int)\0"))
                .as_ptr(),
        );
    }
    'c_3104: {
        if (b"Should never reach here\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"!\"Should never reach here\"\0" as *const u8 as *const libc::c_char,
                b"/puzzles/ps.c\0" as *const u8 as *const libc::c_char,
                283 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"char *ps_text_fallback(void *, const char *const *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn ps_begin_doc(
    mut handle: *mut libc::c_void,
    mut pages: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    fputs(b"%!PS-Adobe-3.0\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fputs(
        b"%%Creator: Simon Tatham's Portable Puzzle Collection\n\0" as *const u8
            as *const libc::c_char,
        (*ps).fp,
    );
    fputs(
        b"%%DocumentData: Clean7Bit\n\0" as *const u8 as *const libc::c_char,
        (*ps).fp,
    );
    fputs(b"%%LanguageLevel: 1\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fprintf((*ps).fp, b"%%%%Pages: %d\n\0" as *const u8 as *const libc::c_char, pages);
    fputs(
        b"%%DocumentNeededResources:\n\0" as *const u8 as *const libc::c_char,
        (*ps).fp,
    );
    fputs(b"%%+ font Helvetica\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fputs(b"%%+ font Courier\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fputs(b"%%EndComments\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fputs(b"%%BeginSetup\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fputs(
        b"%%IncludeResource: font Helvetica\n\0" as *const u8 as *const libc::c_char,
        (*ps).fp,
    );
    fputs(
        b"%%IncludeResource: font Courier\n\0" as *const u8 as *const libc::c_char,
        (*ps).fp,
    );
    fputs(b"%%EndSetup\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fputs(b"%%BeginProlog\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
    fputs(
        b"/Helvetica findfont dup maxlength dict dup begin exch {1 index /FID ne {def} {pop pop} ifelse} forall /Encoding ISOLatin1Encoding def /FontName /Helvetica-L1 def FontName end exch definefont\n\0"
            as *const u8 as *const libc::c_char,
        (*ps).fp,
    );
    fputs(
        b"/Courier findfont dup maxlength dict dup begin exch {1 index /FID ne {def} {pop pop} ifelse} forall /Encoding ISOLatin1Encoding def /FontName /Courier-L1 def FontName end exch definefont\n\0"
            as *const u8 as *const libc::c_char,
        (*ps).fp,
    );
    fputs(b"%%EndProlog\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
}
unsafe extern "C" fn ps_begin_page(
    mut handle: *mut libc::c_void,
    mut number: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    fprintf(
        (*ps).fp,
        b"%%%%Page: %d %d\ngsave save\n%g dup scale\n\0" as *const u8
            as *const libc::c_char,
        number,
        number,
        72.0f64 / 25.4f64,
    );
}
unsafe extern "C" fn ps_begin_puzzle(
    mut handle: *mut libc::c_void,
    mut xm: libc::c_float,
    mut xc: libc::c_float,
    mut ym: libc::c_float,
    mut yc: libc::c_float,
    mut pw: libc::c_int,
    mut ph: libc::c_int,
    mut wmm: libc::c_float,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    fprintf(
        (*ps).fp,
        b"gsave\nclippath flattenpath pathbbox pop pop translate\nclippath flattenpath pathbbox 4 2 roll pop pop\nexch %g mul %g add exch dup %g mul %g add sub translate\n%g dup scale\n0 -%d translate\n\0"
            as *const u8 as *const libc::c_char,
        xm as libc::c_double,
        xc as libc::c_double,
        ym as libc::c_double,
        yc as libc::c_double,
        (wmm / pw as libc::c_float) as libc::c_double,
        ph,
    );
    (*ps).ytop = ph;
    (*ps).clipped = 0 as libc::c_int != 0;
    (*ps).gamewidth = pw;
    (*ps).gameheight = ph;
    (*ps)
        .hatchthick = (0.2f64 * pw as libc::c_double / wmm as libc::c_double)
        as libc::c_float;
    (*ps)
        .hatchspace = (1.0f64 * pw as libc::c_double / wmm as libc::c_double)
        as libc::c_float;
}
unsafe extern "C" fn ps_end_puzzle(mut handle: *mut libc::c_void) {
    let mut ps: *mut psdata = handle as *mut psdata;
    fputs(b"grestore\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
}
unsafe extern "C" fn ps_end_page(
    mut handle: *mut libc::c_void,
    mut number: libc::c_int,
) {
    let mut ps: *mut psdata = handle as *mut psdata;
    fputs(
        b"restore grestore showpage\n\0" as *const u8 as *const libc::c_char,
        (*ps).fp,
    );
}
unsafe extern "C" fn ps_end_doc(mut handle: *mut libc::c_void) {
    let mut ps: *mut psdata = handle as *mut psdata;
    fputs(b"%%EOF\n\0" as *const u8 as *const libc::c_char, (*ps).fp);
}
static mut ps_drawing: drawing_api = unsafe {
    {
        let mut init = drawing_api {
            draw_text: Some(
                ps_draw_text
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        *const libc::c_char,
                    ) -> (),
            ),
            draw_rect: Some(
                ps_draw_rect
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            draw_line: Some(
                ps_draw_line
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            draw_polygon: Some(
                ps_draw_polygon
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            draw_circle: Some(
                ps_draw_circle
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            draw_update: None,
            clip: Some(
                ps_clip
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
            unclip: Some(ps_unclip as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            start_draw: None,
            end_draw: None,
            status_bar: None,
            blitter_new: None,
            blitter_free: None,
            blitter_save: None,
            blitter_load: None,
            begin_doc: Some(
                ps_begin_doc
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> (),
            ),
            begin_page: Some(
                ps_begin_page
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> (),
            ),
            begin_puzzle: Some(
                ps_begin_puzzle
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_float,
                        libc::c_float,
                        libc::c_float,
                        libc::c_float,
                        libc::c_int,
                        libc::c_int,
                        libc::c_float,
                    ) -> (),
            ),
            end_puzzle: Some(
                ps_end_puzzle as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            end_page: Some(
                ps_end_page as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> (),
            ),
            end_doc: Some(ps_end_doc as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            line_width: Some(
                ps_line_width
                    as unsafe extern "C" fn(*mut libc::c_void, libc::c_float) -> (),
            ),
            line_dotted: Some(
                ps_line_dotted as unsafe extern "C" fn(*mut libc::c_void, bool) -> (),
            ),
            text_fallback: Some(
                ps_text_fallback
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const *const libc::c_char,
                        libc::c_int,
                    ) -> *mut libc::c_char,
            ),
            draw_thick_line: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn ps_init(
    mut outfile: *mut FILE,
    mut colour: bool,
) -> *mut psdata {
    let mut ps: *mut psdata = smalloc(::core::mem::size_of::<psdata>() as libc::c_ulong)
        as *mut psdata;
    (*ps).fp = outfile;
    (*ps).colour = colour;
    (*ps).ytop = 0 as libc::c_int;
    (*ps).clipped = 0 as libc::c_int != 0;
    (*ps).gameheight = 0 as libc::c_int;
    (*ps).gamewidth = (*ps).gameheight;
    (*ps).hatchspace = (*ps).gamewidth as libc::c_float;
    (*ps).hatchthick = (*ps).hatchspace;
    (*ps).drawing = drawing_new(&ps_drawing, 0 as *mut midend, ps as *mut libc::c_void);
    return ps;
}
#[no_mangle]
pub unsafe extern "C" fn ps_free(mut ps: *mut psdata) {
    drawing_free((*ps).drawing);
    sfree(ps as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ps_drawing_api(mut ps: *mut psdata) -> *mut drawing {
    return (*ps).drawing;
}
