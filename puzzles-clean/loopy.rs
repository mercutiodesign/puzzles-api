#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type frontend;
    pub type midend;
    pub type random_state;
    pub type drawing;
    pub type DSF;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn preset_menu_new() -> *mut preset_menu;
    fn preset_menu_add_submenu(
        parent: *mut preset_menu,
        title: *mut libc::c_char,
    ) -> *mut preset_menu;
    fn preset_menu_add_preset(
        menu: *mut preset_menu,
        title: *mut libc::c_char,
        params: *mut game_params,
    );
    fn frontend_default_colour(fe: *mut frontend, output: *mut libc::c_float);
    fn draw_text(
        dr: *mut drawing,
        x: libc::c_int,
        y: libc::c_int,
        fonttype: libc::c_int,
        fontsize: libc::c_int,
        align: libc::c_int,
        colour: libc::c_int,
        text: *const libc::c_char,
    );
    fn draw_rect(
        dr: *mut drawing,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        colour: libc::c_int,
    );
    fn draw_polygon(
        dr: *mut drawing,
        coords: *const libc::c_int,
        npoints: libc::c_int,
        fillcolour: libc::c_int,
        outlinecolour: libc::c_int,
    );
    fn draw_circle(
        dr: *mut drawing,
        cx: libc::c_int,
        cy: libc::c_int,
        radius: libc::c_int,
        fillcolour: libc::c_int,
        outlinecolour: libc::c_int,
    );
    fn draw_thick_line(
        dr: *mut drawing,
        thickness: libc::c_float,
        x1: libc::c_float,
        y1: libc::c_float,
        x2: libc::c_float,
        y2: libc::c_float,
        colour: libc::c_int,
    );
    fn clip(
        dr: *mut drawing,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn unclip(dr: *mut drawing);
    fn draw_update(
        dr: *mut drawing,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn print_mono_colour(dr: *mut drawing, grey: libc::c_int) -> libc::c_int;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn srealloc(p: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn dupstr(s: *const libc::c_char) -> *mut libc::c_char;
    fn getenv_bool(name: *const libc::c_char, dflt: libc::c_int) -> libc::c_int;
    fn shuffle(
        array: *mut libc::c_void,
        nelts: libc::c_int,
        eltsize: libc::c_int,
        rs: *mut random_state,
    );
    fn dsf_new(size: libc::c_int) -> *mut DSF;
    fn dsf_free(dsf: *mut DSF);
    fn dsf_copy(to: *mut DSF, from: *mut DSF);
    fn dsf_canonify(dsf: *mut DSF, n: libc::c_int) -> libc::c_int;
    fn dsf_size(dsf: *mut DSF, n: libc::c_int) -> libc::c_int;
    fn dsf_merge(dsf: *mut DSF, n1: libc::c_int, n2: libc::c_int);
    fn dsf_new_flip(size: libc::c_int) -> *mut DSF;
    fn dsf_merge_flip(dsf: *mut DSF, n1: libc::c_int, n2: libc::c_int, flip: bool);
    fn dsf_canonify_flip(dsf: *mut DSF, n: libc::c_int, flip: *mut bool) -> libc::c_int;
    fn random_new(seed: *const libc::c_char, len: libc::c_int) -> *mut random_state;
    fn grid_validate_params(
        type_0: grid_type,
        width: libc::c_int,
        height: libc::c_int,
    ) -> *const libc::c_char;
    fn grid_new_desc(
        type_0: grid_type,
        width: libc::c_int,
        height: libc::c_int,
        rs: *mut random_state,
    ) -> *mut libc::c_char;
    fn grid_validate_desc(
        type_0: grid_type,
        width: libc::c_int,
        height: libc::c_int,
        desc: *const libc::c_char,
    ) -> *const libc::c_char;
    fn grid_new(
        type_0: grid_type,
        width: libc::c_int,
        height: libc::c_int,
        desc: *const libc::c_char,
    ) -> *mut grid;
    fn grid_free(g: *mut grid);
    fn grid_nearest_edge(g: *mut grid, x: libc::c_int, y: libc::c_int) -> *mut grid_edge;
    fn grid_compute_size(
        type_0: grid_type,
        width: libc::c_int,
        height: libc::c_int,
        tilesize: *mut libc::c_int,
        xextent: *mut libc::c_int,
        yextent: *mut libc::c_int,
    );
    fn grid_find_incentre(f: *mut grid_face);
    fn generate_loop(
        g: *mut grid,
        board: *mut libc::c_char,
        rs: *mut random_state,
        bias: loopgen_bias_fn_t,
        biasctx: *mut libc::c_void,
    );
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MOD_MASK: C2RustUnnamed_0 = 28672;
pub const MOD_NUM_KEYPAD: C2RustUnnamed_0 = 16384;
pub const MOD_SHFT: C2RustUnnamed_0 = 8192;
pub const MOD_CTRL: C2RustUnnamed_0 = 4096;
pub const UI_UPPER_BOUND: C2RustUnnamed_0 = 533;
pub const UI_REDO: C2RustUnnamed_0 = 532;
pub const UI_UNDO: C2RustUnnamed_0 = 531;
pub const UI_SOLVE: C2RustUnnamed_0 = 530;
pub const UI_NEWGAME: C2RustUnnamed_0 = 529;
pub const UI_QUIT: C2RustUnnamed_0 = 528;
pub const UI_LOWER_BOUND: C2RustUnnamed_0 = 527;
pub const CURSOR_SELECT2: C2RustUnnamed_0 = 526;
pub const CURSOR_SELECT: C2RustUnnamed_0 = 525;
pub const CURSOR_RIGHT: C2RustUnnamed_0 = 524;
pub const CURSOR_LEFT: C2RustUnnamed_0 = 523;
pub const CURSOR_DOWN: C2RustUnnamed_0 = 522;
pub const CURSOR_UP: C2RustUnnamed_0 = 521;
pub const RIGHT_RELEASE: C2RustUnnamed_0 = 520;
pub const MIDDLE_RELEASE: C2RustUnnamed_0 = 519;
pub const LEFT_RELEASE: C2RustUnnamed_0 = 518;
pub const RIGHT_DRAG: C2RustUnnamed_0 = 517;
pub const MIDDLE_DRAG: C2RustUnnamed_0 = 516;
pub const LEFT_DRAG: C2RustUnnamed_0 = 515;
pub const RIGHT_BUTTON: C2RustUnnamed_0 = 514;
pub const MIDDLE_BUTTON: C2RustUnnamed_0 = 513;
pub const LEFT_BUTTON: C2RustUnnamed_0 = 512;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_item {
    pub name: *const libc::c_char,
    pub kw: *const libc::c_char,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub string: C2RustUnnamed_4,
    pub choices: C2RustUnnamed_3,
    pub boolean: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub bval: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub choicenames: *const libc::c_char,
    pub choicekws: *const libc::c_char,
    pub selected: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub sval: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_params {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub diff: libc::c_int,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_state {
    pub game_grid: *mut grid,
    pub clues: *mut libc::c_schar,
    pub lines: *mut libc::c_char,
    pub line_errors: *mut bool,
    pub exactly_one_loop: bool,
    pub solved: bool,
    pub cheated: bool,
    pub grid_type: libc::c_int,
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
pub struct grid_edge {
    pub dot1: *mut grid_dot,
    pub dot2: *mut grid_dot,
    pub face1: *mut grid_face,
    pub face2: *mut grid_face,
    pub index: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_ui {
    pub draw_faint_lines: bool,
    pub autofollow: C2RustUnnamed_5,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const AF_ADAPTIVE: C2RustUnnamed_5 = 2;
pub const AF_FIXED: C2RustUnnamed_5 = 1;
pub const AF_OFF: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_drawstate {
    pub started: bool,
    pub tilesize: libc::c_int,
    pub flashing: bool,
    pub textx: *mut libc::c_int,
    pub texty: *mut libc::c_int,
    pub lines: *mut libc::c_char,
    pub clue_error: *mut bool,
    pub clue_satisfied: *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game {
    pub name: *const libc::c_char,
    pub winhelp_topic: *const libc::c_char,
    pub htmlhelp_topic: *const libc::c_char,
    pub default_params: Option::<unsafe extern "C" fn() -> *mut game_params>,
    pub fetch_preset: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut *mut libc::c_char,
            *mut *mut game_params,
        ) -> bool,
    >,
    pub preset_menu: Option::<unsafe extern "C" fn() -> *mut preset_menu>,
    pub decode_params: Option::<
        unsafe extern "C" fn(*mut game_params, *const libc::c_char) -> (),
    >,
    pub encode_params: Option::<
        unsafe extern "C" fn(*const game_params, bool) -> *mut libc::c_char,
    >,
    pub free_params: Option::<unsafe extern "C" fn(*mut game_params) -> ()>,
    pub dup_params: Option::<
        unsafe extern "C" fn(*const game_params) -> *mut game_params,
    >,
    pub can_configure: bool,
    pub configure: Option::<
        unsafe extern "C" fn(*const game_params) -> *mut config_item,
    >,
    pub custom_params: Option::<
        unsafe extern "C" fn(*const config_item) -> *mut game_params,
    >,
    pub validate_params: Option::<
        unsafe extern "C" fn(*const game_params, bool) -> *const libc::c_char,
    >,
    pub new_desc: Option::<
        unsafe extern "C" fn(
            *const game_params,
            *mut random_state,
            *mut *mut libc::c_char,
            bool,
        ) -> *mut libc::c_char,
    >,
    pub validate_desc: Option::<
        unsafe extern "C" fn(
            *const game_params,
            *const libc::c_char,
        ) -> *const libc::c_char,
    >,
    pub new_game: Option::<
        unsafe extern "C" fn(
            *mut midend,
            *const game_params,
            *const libc::c_char,
        ) -> *mut game_state,
    >,
    pub dup_game: Option::<unsafe extern "C" fn(*const game_state) -> *mut game_state>,
    pub free_game: Option::<unsafe extern "C" fn(*mut game_state) -> ()>,
    pub can_solve: bool,
    pub solve: Option::<
        unsafe extern "C" fn(
            *const game_state,
            *const game_state,
            *const libc::c_char,
            *mut *const libc::c_char,
        ) -> *mut libc::c_char,
    >,
    pub can_format_as_text_ever: bool,
    pub can_format_as_text_now: Option::<
        unsafe extern "C" fn(*const game_params) -> bool,
    >,
    pub text_format: Option::<
        unsafe extern "C" fn(*const game_state) -> *mut libc::c_char,
    >,
    pub get_prefs: Option::<unsafe extern "C" fn(*mut game_ui) -> *mut config_item>,
    pub set_prefs: Option::<
        unsafe extern "C" fn(*mut game_ui, *const config_item) -> (),
    >,
    pub new_ui: Option::<unsafe extern "C" fn(*const game_state) -> *mut game_ui>,
    pub free_ui: Option::<unsafe extern "C" fn(*mut game_ui) -> ()>,
    pub encode_ui: Option::<unsafe extern "C" fn(*const game_ui) -> *mut libc::c_char>,
    pub decode_ui: Option::<
        unsafe extern "C" fn(*mut game_ui, *const libc::c_char, *const game_state) -> (),
    >,
    pub request_keys: Option::<
        unsafe extern "C" fn(*const game_params, *mut libc::c_int) -> *mut key_label,
    >,
    pub changed_state: Option::<
        unsafe extern "C" fn(*mut game_ui, *const game_state, *const game_state) -> (),
    >,
    pub current_key_label: Option::<
        unsafe extern "C" fn(
            *const game_ui,
            *const game_state,
            libc::c_int,
        ) -> *const libc::c_char,
    >,
    pub interpret_move: Option::<
        unsafe extern "C" fn(
            *const game_state,
            *mut game_ui,
            *const game_drawstate,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub execute_move: Option::<
        unsafe extern "C" fn(*const game_state, *const libc::c_char) -> *mut game_state,
    >,
    pub preferred_tilesize: libc::c_int,
    pub compute_size: Option::<
        unsafe extern "C" fn(
            *const game_params,
            libc::c_int,
            *const game_ui,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> (),
    >,
    pub set_size: Option::<
        unsafe extern "C" fn(
            *mut drawing,
            *mut game_drawstate,
            *const game_params,
            libc::c_int,
        ) -> (),
    >,
    pub colours: Option::<
        unsafe extern "C" fn(*mut frontend, *mut libc::c_int) -> *mut libc::c_float,
    >,
    pub new_drawstate: Option::<
        unsafe extern "C" fn(*mut drawing, *const game_state) -> *mut game_drawstate,
    >,
    pub free_drawstate: Option::<
        unsafe extern "C" fn(*mut drawing, *mut game_drawstate) -> (),
    >,
    pub redraw: Option::<
        unsafe extern "C" fn(
            *mut drawing,
            *mut game_drawstate,
            *const game_state,
            *const game_state,
            libc::c_int,
            *const game_ui,
            libc::c_float,
            libc::c_float,
        ) -> (),
    >,
    pub anim_length: Option::<
        unsafe extern "C" fn(
            *const game_state,
            *const game_state,
            libc::c_int,
            *mut game_ui,
        ) -> libc::c_float,
    >,
    pub flash_length: Option::<
        unsafe extern "C" fn(
            *const game_state,
            *const game_state,
            libc::c_int,
            *mut game_ui,
        ) -> libc::c_float,
    >,
    pub get_cursor_location: Option::<
        unsafe extern "C" fn(
            *const game_ui,
            *const game_drawstate,
            *const game_state,
            *const game_params,
            *mut libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> (),
    >,
    pub status: Option::<unsafe extern "C" fn(*const game_state) -> libc::c_int>,
    pub can_print: bool,
    pub can_print_in_colour: bool,
    pub print_size: Option::<
        unsafe extern "C" fn(
            *const game_params,
            *const game_ui,
            *mut libc::c_float,
            *mut libc::c_float,
        ) -> (),
    >,
    pub print: Option::<
        unsafe extern "C" fn(
            *mut drawing,
            *const game_state,
            *const game_ui,
            libc::c_int,
        ) -> (),
    >,
    pub wants_statusbar: bool,
    pub is_timed: bool,
    pub timing_state: Option::<
        unsafe extern "C" fn(*const game_state, *mut game_ui) -> bool,
    >,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_label {
    pub label: *mut libc::c_char,
    pub button: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct preset_menu {
    pub n_entries: libc::c_int,
    pub entries_size: libc::c_int,
    pub entries: *mut preset_menu_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct preset_menu_entry {
    pub title: *mut libc::c_char,
    pub params: *mut game_params,
    pub submenu: *mut preset_menu,
    pub id: libc::c_int,
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const C_END: C2RustUnnamed_6 = 3;
pub const C_BOOLEAN: C2RustUnnamed_6 = 2;
pub const C_CHOICES: C2RustUnnamed_6 = 1;
pub const C_STRING: C2RustUnnamed_6 = 0;
pub const LINE_YES: line_state = 0;
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
pub const COL_FOREGROUND: C2RustUnnamed_9 = 1;
pub const COL_FAINT: C2RustUnnamed_9 = 6;
pub const COL_MISTAKE: C2RustUnnamed_9 = 4;
pub const COL_HIGHLIGHT: C2RustUnnamed_9 = 3;
pub const COL_LINEUNKNOWN: C2RustUnnamed_9 = 2;
pub const LINE_NO: line_state = 2;
pub const LINE_UNKNOWN: line_state = 1;
pub const COL_SATISFIED: C2RustUnnamed_9 = 5;
pub const COL_BACKGROUND: C2RustUnnamed_9 = 0;
pub const DS_LINE_ERROR: line_drawstate = 3;
pub const NCOLOURS: C2RustUnnamed_9 = 7;
pub const COMP_LOOP: C2RustUnnamed_7 = 1;
pub const COMP_PATH: C2RustUnnamed_7 = 2;
pub const COMP_SILLY: C2RustUnnamed_7 = 3;
pub const COMP_EMPTY: C2RustUnnamed_7 = 4;
pub const COMP_NONE: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub type line_state = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solver_state {
    pub state: *mut game_state,
    pub solver_status: solver_status,
    pub looplen: *mut libc::c_int,
    pub diff: libc::c_int,
    pub dot_yes_count: *mut libc::c_char,
    pub dot_no_count: *mut libc::c_char,
    pub face_yes_count: *mut libc::c_char,
    pub face_no_count: *mut libc::c_char,
    pub dot_solved: *mut bool,
    pub face_solved: *mut bool,
    pub dotdsf: *mut DSF,
    pub dlines: *mut libc::c_char,
    pub linedsf: *mut DSF,
}
pub type solver_status = libc::c_uint;
pub const SOLVER_INCOMPLETE: solver_status = 3;
pub const SOLVER_AMBIGUOUS: solver_status = 2;
pub const SOLVER_MISTAKE: solver_status = 1;
pub const SOLVER_SOLVED: solver_status = 0;
pub const DIFF_MAX: C2RustUnnamed_10 = 4;
pub const DIFF_EASY: C2RustUnnamed_10 = 0;
pub const DIFF_HARD: C2RustUnnamed_10 = 3;
pub const DIFF_NORMAL: C2RustUnnamed_10 = 1;
pub const DIFF_TRICKY: C2RustUnnamed_10 = 2;
pub type face_colour = libc::c_uint;
pub const FACE_BLACK: face_colour = 2;
pub const FACE_GREY: face_colour = 1;
pub const FACE_WHITE: face_colour = 0;
pub type loopgen_bias_fn_t = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub amin: libc::c_int,
    pub omin: libc::c_int,
    pub aerr: *const libc::c_char,
    pub oerr: *const libc::c_char,
}
pub const LOOPY_GRID_SPECTRES: C2RustUnnamed_11 = 17;
pub const LOOPY_GRID_HATS: C2RustUnnamed_11 = 16;
pub const LOOPY_GRID_COMPASSDODECAGONAL: C2RustUnnamed_11 = 15;
pub const LOOPY_GRID_GREATGREATDODECAGONAL: C2RustUnnamed_11 = 13;
pub const LOOPY_GRID_GREATDODECAGONAL: C2RustUnnamed_11 = 10;
pub const LOOPY_GRID_DODECAGONAL: C2RustUnnamed_11 = 9;
pub const LOOPY_GRID_FLORET: C2RustUnnamed_11 = 8;
pub const LOOPY_GRID_OCTAGONAL: C2RustUnnamed_11 = 6;
pub const LOOPY_GRID_KAGOME: C2RustUnnamed_11 = 14;
pub const LOOPY_GRID_GREATHEXAGONAL: C2RustUnnamed_11 = 5;
pub const LOOPY_GRID_HONEYCOMB: C2RustUnnamed_11 = 2;
pub const LOOPY_GRID_PENROSE_P3: C2RustUnnamed_11 = 12;
pub const LOOPY_GRID_PENROSE_P2: C2RustUnnamed_11 = 11;
pub const LOOPY_GRID_KITE: C2RustUnnamed_11 = 7;
pub const LOOPY_GRID_CAIRO: C2RustUnnamed_11 = 4;
pub const LOOPY_GRID_SNUBSQUARE: C2RustUnnamed_11 = 3;
pub const LOOPY_GRID_TRIANGULAR: C2RustUnnamed_11 = 1;
pub const LOOPY_GRID_SQUARE: C2RustUnnamed_11 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub type C2RustUnnamed_10 = libc::c_uint;
pub type line_drawstate = libc::c_uint;
pub const DS_LINE_NO: line_drawstate = 2;
pub const DS_LINE_UNKNOWN: line_drawstate = 1;
pub const DS_LINE_YES: line_drawstate = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const LOOPY_GRID_DUMMY_TERMINATOR: C2RustUnnamed_11 = 18;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_double) -> libc::c_double {
    return sqrt(__x);
}
static mut diffnames: [*const libc::c_char; 4] = [
    b"Easy\0" as *const u8 as *const libc::c_char,
    b"Normal\0" as *const u8 as *const libc::c_char,
    b"Tricky\0" as *const u8 as *const libc::c_char,
    b"Hard\0" as *const u8 as *const libc::c_char,
];
static mut diffchars: [libc::c_char; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"enth\0")
};
static mut solver_fns: [Option::<
    unsafe extern "C" fn(*mut solver_state) -> libc::c_int,
>; 4] = [
    Some(trivial_deductions as unsafe extern "C" fn(*mut solver_state) -> libc::c_int),
    Some(dline_deductions as unsafe extern "C" fn(*mut solver_state) -> libc::c_int),
    Some(linedsf_deductions as unsafe extern "C" fn(*mut solver_state) -> libc::c_int),
    Some(loop_deductions as unsafe extern "C" fn(*mut solver_state) -> libc::c_int),
];
static mut solver_diffs: [libc::c_int; 4] = [
    DIFF_EASY as libc::c_int,
    DIFF_NORMAL as libc::c_int,
    DIFF_HARD as libc::c_int,
    DIFF_EASY as libc::c_int,
];
static mut NUM_SOLVERS: libc::c_int = 0;
static mut gridnames: [*const libc::c_char; 18] = [
    b"Squares\0" as *const u8 as *const libc::c_char,
    b"Triangular\0" as *const u8 as *const libc::c_char,
    b"Honeycomb\0" as *const u8 as *const libc::c_char,
    b"Snub-Square\0" as *const u8 as *const libc::c_char,
    b"Cairo\0" as *const u8 as *const libc::c_char,
    b"Great-Hexagonal\0" as *const u8 as *const libc::c_char,
    b"Octagonal\0" as *const u8 as *const libc::c_char,
    b"Kites\0" as *const u8 as *const libc::c_char,
    b"Floret\0" as *const u8 as *const libc::c_char,
    b"Dodecagonal\0" as *const u8 as *const libc::c_char,
    b"Great-Dodecagonal\0" as *const u8 as *const libc::c_char,
    b"Penrose (kite/dart)\0" as *const u8 as *const libc::c_char,
    b"Penrose (rhombs)\0" as *const u8 as *const libc::c_char,
    b"Great-Great-Dodecagonal\0" as *const u8 as *const libc::c_char,
    b"Kagome\0" as *const u8 as *const libc::c_char,
    b"Compass-Dodecagonal\0" as *const u8 as *const libc::c_char,
    b"Hats\0" as *const u8 as *const libc::c_char,
    b"Spectres\0" as *const u8 as *const libc::c_char,
];
static mut grid_types: [grid_type; 18] = [
    GRID_SQUARE,
    GRID_TRIANGULAR,
    GRID_HONEYCOMB,
    GRID_SNUBSQUARE,
    GRID_CAIRO,
    GRID_GREATHEXAGONAL,
    GRID_OCTAGONAL,
    GRID_KITE,
    GRID_FLORET,
    GRID_DODECAGONAL,
    GRID_GREATDODECAGONAL,
    GRID_PENROSE_P2,
    GRID_PENROSE_P3,
    GRID_GREATGREATDODECAGONAL,
    GRID_KAGOME,
    GRID_COMPASSDODECAGONAL,
    GRID_HATS,
    GRID_SPECTRES,
];
static mut grid_size_limits: [C2RustUnnamed_8; 18] = [
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 4 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 4\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 1 as libc::c_int,
            omin: 2 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 1\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 2\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 2 as libc::c_int,
            omin: 2 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 2\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 2\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 2 as libc::c_int,
            omin: 2 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 2\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 2\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 2 as libc::c_int,
            omin: 2 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 2\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 2\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 3 as libc::c_int,
            omin: 3 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 3\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 3\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 2 as libc::c_int,
            omin: 2 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 2\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 2\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 6 as libc::c_int,
            omin: 6 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 6\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 6\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_8 {
            amin: 6 as libc::c_int,
            omin: 6 as libc::c_int,
            aerr: b"Width and height for this grid type must both be at least 6\0"
                as *const u8 as *const libc::c_char,
            oerr: b"At least one of width and height for this grid type must be at least 6\0"
                as *const u8 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn loopy_generate_grid(
    mut params: *const game_params,
    mut grid_desc: *const libc::c_char,
) -> *mut grid {
    return grid_new(
        grid_types[(*params).type_0 as usize],
        (*params).w,
        (*params).h,
        grid_desc,
    );
}
unsafe extern "C" fn dup_game(mut state: *const game_state) -> *mut game_state {
    let mut ret: *mut game_state = smalloc(
        ::core::mem::size_of::<game_state>() as libc::c_ulong,
    ) as *mut game_state;
    (*ret).game_grid = (*state).game_grid;
    (*(*ret).game_grid).refcount += 1;
    (*(*ret).game_grid).refcount;
    (*ret).solved = (*state).solved;
    (*ret).cheated = (*state).cheated;
    (*ret)
        .clues = smalloc(
        ((*(*state).game_grid).num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_schar>() as libc::c_ulong),
    ) as *mut libc::c_schar;
    memcpy(
        (*ret).clues as *mut libc::c_void,
        (*state).clues as *const libc::c_void,
        (*(*state).game_grid).num_faces as libc::c_ulong,
    );
    (*ret)
        .lines = smalloc(
        ((*(*state).game_grid).num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*ret).lines as *mut libc::c_void,
        (*state).lines as *const libc::c_void,
        (*(*state).game_grid).num_edges as libc::c_ulong,
    );
    (*ret)
        .line_errors = smalloc(
        ((*(*state).game_grid).num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    memcpy(
        (*ret).line_errors as *mut libc::c_void,
        (*state).line_errors as *const libc::c_void,
        ((*(*state).game_grid).num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    (*ret).exactly_one_loop = (*state).exactly_one_loop;
    (*ret).grid_type = (*state).grid_type;
    return ret;
}
unsafe extern "C" fn free_game(mut state: *mut game_state) {
    if !state.is_null() {
        grid_free((*state).game_grid);
        sfree((*state).clues as *mut libc::c_void);
        sfree((*state).lines as *mut libc::c_void);
        sfree((*state).line_errors as *mut libc::c_void);
        sfree(state as *mut libc::c_void);
    }
}
unsafe extern "C" fn new_solver_state(
    mut state: *const game_state,
    mut diff: libc::c_int,
) -> *mut solver_state {
    let mut i: libc::c_int = 0;
    let mut num_dots: libc::c_int = (*(*state).game_grid).num_dots;
    let mut num_faces: libc::c_int = (*(*state).game_grid).num_faces;
    let mut num_edges: libc::c_int = (*(*state).game_grid).num_edges;
    let mut ret: *mut solver_state = smalloc(
        ::core::mem::size_of::<solver_state>() as libc::c_ulong,
    ) as *mut solver_state;
    (*ret).state = dup_game(state);
    (*ret).solver_status = SOLVER_INCOMPLETE;
    (*ret).diff = diff;
    (*ret).dotdsf = dsf_new(num_dots);
    (*ret)
        .looplen = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < num_dots {
        *((*ret).looplen).offset(i as isize) = 1 as libc::c_int;
        i += 1;
        i;
    }
    (*ret)
        .dot_solved = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    (*ret)
        .face_solved = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    memset(
        (*ret).dot_solved as *mut libc::c_void,
        0 as libc::c_int,
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    memset(
        (*ret).face_solved as *mut libc::c_void,
        0 as libc::c_int,
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    (*ret)
        .dot_yes_count = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        (*ret).dot_yes_count as *mut libc::c_void,
        0 as libc::c_int,
        num_dots as libc::c_ulong,
    );
    (*ret)
        .dot_no_count = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        (*ret).dot_no_count as *mut libc::c_void,
        0 as libc::c_int,
        num_dots as libc::c_ulong,
    );
    (*ret)
        .face_yes_count = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        (*ret).face_yes_count as *mut libc::c_void,
        0 as libc::c_int,
        num_faces as libc::c_ulong,
    );
    (*ret)
        .face_no_count = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        (*ret).face_no_count as *mut libc::c_void,
        0 as libc::c_int,
        num_faces as libc::c_ulong,
    );
    if diff < DIFF_NORMAL as libc::c_int {
        (*ret).dlines = 0 as *mut libc::c_char;
    } else {
        (*ret)
            .dlines = smalloc(
            ((2 as libc::c_int * num_edges) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        memset(
            (*ret).dlines as *mut libc::c_void,
            0 as libc::c_int,
            (2 as libc::c_int * num_edges) as libc::c_ulong,
        );
    }
    if diff < DIFF_HARD as libc::c_int {
        (*ret).linedsf = 0 as *mut DSF;
    } else {
        (*ret).linedsf = dsf_new_flip((*(*state).game_grid).num_edges);
    }
    return ret;
}
unsafe extern "C" fn free_solver_state(mut sstate: *mut solver_state) {
    if !sstate.is_null() {
        free_game((*sstate).state);
        dsf_free((*sstate).dotdsf);
        sfree((*sstate).looplen as *mut libc::c_void);
        sfree((*sstate).dot_solved as *mut libc::c_void);
        sfree((*sstate).face_solved as *mut libc::c_void);
        sfree((*sstate).dot_yes_count as *mut libc::c_void);
        sfree((*sstate).dot_no_count as *mut libc::c_void);
        sfree((*sstate).face_yes_count as *mut libc::c_void);
        sfree((*sstate).face_no_count as *mut libc::c_void);
        sfree((*sstate).dlines as *mut libc::c_void);
        dsf_free((*sstate).linedsf);
        sfree(sstate as *mut libc::c_void);
    }
}
unsafe extern "C" fn dup_solver_state(
    mut sstate: *const solver_state,
) -> *mut solver_state {
    let mut state: *mut game_state = (*sstate).state;
    let mut num_dots: libc::c_int = (*(*state).game_grid).num_dots;
    let mut num_faces: libc::c_int = (*(*state).game_grid).num_faces;
    let mut num_edges: libc::c_int = (*(*state).game_grid).num_edges;
    let mut ret: *mut solver_state = smalloc(
        ::core::mem::size_of::<solver_state>() as libc::c_ulong,
    ) as *mut solver_state;
    state = dup_game((*sstate).state);
    (*ret).state = state;
    (*ret).solver_status = (*sstate).solver_status;
    (*ret).diff = (*sstate).diff;
    (*ret).dotdsf = dsf_new(num_dots);
    (*ret)
        .looplen = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    dsf_copy((*ret).dotdsf, (*sstate).dotdsf);
    memcpy(
        (*ret).looplen as *mut libc::c_void,
        (*sstate).looplen as *const libc::c_void,
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*ret)
        .dot_solved = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    (*ret)
        .face_solved = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    memcpy(
        (*ret).dot_solved as *mut libc::c_void,
        (*sstate).dot_solved as *const libc::c_void,
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    memcpy(
        (*ret).face_solved as *mut libc::c_void,
        (*sstate).face_solved as *const libc::c_void,
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    (*ret)
        .dot_yes_count = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*ret).dot_yes_count as *mut libc::c_void,
        (*sstate).dot_yes_count as *const libc::c_void,
        num_dots as libc::c_ulong,
    );
    (*ret)
        .dot_no_count = smalloc(
        (num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*ret).dot_no_count as *mut libc::c_void,
        (*sstate).dot_no_count as *const libc::c_void,
        num_dots as libc::c_ulong,
    );
    (*ret)
        .face_yes_count = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*ret).face_yes_count as *mut libc::c_void,
        (*sstate).face_yes_count as *const libc::c_void,
        num_faces as libc::c_ulong,
    );
    (*ret)
        .face_no_count = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        (*ret).face_no_count as *mut libc::c_void,
        (*sstate).face_no_count as *const libc::c_void,
        num_faces as libc::c_ulong,
    );
    if !((*sstate).dlines).is_null() {
        (*ret)
            .dlines = smalloc(
            ((2 as libc::c_int * num_edges) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        memcpy(
            (*ret).dlines as *mut libc::c_void,
            (*sstate).dlines as *const libc::c_void,
            (2 as libc::c_int * num_edges) as libc::c_ulong,
        );
    } else {
        (*ret).dlines = 0 as *mut libc::c_char;
    }
    if !((*sstate).linedsf).is_null() {
        (*ret).linedsf = dsf_new_flip(num_edges);
        dsf_copy((*ret).linedsf, (*sstate).linedsf);
    } else {
        (*ret).linedsf = 0 as *mut DSF;
    }
    return ret;
}
unsafe extern "C" fn default_params() -> *mut game_params {
    let mut ret: *mut game_params = smalloc(
        ::core::mem::size_of::<game_params>() as libc::c_ulong,
    ) as *mut game_params;
    (*ret).h = 10 as libc::c_int;
    (*ret).w = 10 as libc::c_int;
    (*ret).diff = DIFF_EASY as libc::c_int;
    (*ret).type_0 = 0 as libc::c_int;
    return ret;
}
unsafe extern "C" fn dup_params(mut params: *const game_params) -> *mut game_params {
    let mut ret: *mut game_params = smalloc(
        ::core::mem::size_of::<game_params>() as libc::c_ulong,
    ) as *mut game_params;
    *ret = *params;
    return ret;
}
static mut loopy_presets_top: [game_params; 12] = [
    {
        let mut init = game_params {
            w: 7 as libc::c_int,
            h: 7 as libc::c_int,
            diff: DIFF_EASY as libc::c_int,
            type_0: LOOPY_GRID_SQUARE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_EASY as libc::c_int,
            type_0: LOOPY_GRID_SQUARE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 7 as libc::c_int,
            h: 7 as libc::c_int,
            diff: DIFF_NORMAL as libc::c_int,
            type_0: LOOPY_GRID_SQUARE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_NORMAL as libc::c_int,
            type_0: LOOPY_GRID_SQUARE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 7 as libc::c_int,
            h: 7 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_SQUARE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_SQUARE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 12 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_TRIANGULAR as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 7 as libc::c_int,
            h: 7 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_SNUBSQUARE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 9 as libc::c_int,
            h: 9 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_CAIRO as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 5 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_KITE as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_PENROSE_P2 as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_PENROSE_P3 as libc::c_int,
        };
        init
    },
];
static mut loopy_presets_more: [game_params; 11] = [
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_HONEYCOMB as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 4 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_GREATHEXAGONAL as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 4 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_KAGOME as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 7 as libc::c_int,
            h: 7 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_OCTAGONAL as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 5 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_FLORET as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 4 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_DODECAGONAL as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 4 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_GREATDODECAGONAL as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 3 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_GREATGREATDODECAGONAL as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 5 as libc::c_int,
            h: 4 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_COMPASSDODECAGONAL as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_HATS as libc::c_int,
        };
        init
    },
    {
        let mut init = game_params {
            w: 10 as libc::c_int,
            h: 10 as libc::c_int,
            diff: DIFF_HARD as libc::c_int,
            type_0: LOOPY_GRID_SPECTRES as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn preset_menu_add_preset_with_title(
    mut menu: *mut preset_menu,
    mut params: *const game_params,
) {
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut dup_params_0: *mut game_params = 0 as *mut game_params;
    sprintf(
        buf.as_mut_ptr(),
        b"%dx%d %s - %s\0" as *const u8 as *const libc::c_char,
        (*params).h,
        (*params).w,
        gridnames[(*params).type_0 as usize],
        diffnames[(*params).diff as usize],
    );
    dup_params_0 = smalloc(::core::mem::size_of::<game_params>() as libc::c_ulong)
        as *mut game_params;
    *dup_params_0 = *params;
    preset_menu_add_preset(menu, dupstr(buf.as_mut_ptr()), dup_params_0);
}
unsafe extern "C" fn game_preset_menu() -> *mut preset_menu {
    let mut top: *mut preset_menu = 0 as *mut preset_menu;
    let mut more: *mut preset_menu = 0 as *mut preset_menu;
    let mut i: libc::c_int = 0;
    top = preset_menu_new();
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[game_params; 12]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<game_params>() as libc::c_ulong)
    {
        preset_menu_add_preset_with_title(
            top,
            &*loopy_presets_top.as_ptr().offset(i as isize),
        );
        i += 1;
        i;
    }
    more = preset_menu_add_submenu(
        top,
        dupstr(b"More...\0" as *const u8 as *const libc::c_char),
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[game_params; 11]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<game_params>() as libc::c_ulong)
    {
        preset_menu_add_preset_with_title(
            more,
            &*loopy_presets_more.as_ptr().offset(i as isize),
        );
        i += 1;
        i;
    }
    return top;
}
unsafe extern "C" fn free_params(mut params: *mut game_params) {
    sfree(params as *mut libc::c_void);
}
unsafe extern "C" fn decode_params(
    mut params: *mut game_params,
    mut string: *const libc::c_char,
) {
    (*params).w = atoi(string);
    (*params).h = (*params).w;
    (*params).diff = DIFF_EASY as libc::c_int;
    while *string as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*string as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        string = string.offset(1);
        string;
    }
    if *string as libc::c_int == 'x' as i32 {
        string = string.offset(1);
        string;
        (*params).h = atoi(string);
        while *string as libc::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*string as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            string = string.offset(1);
            string;
        }
    }
    if *string as libc::c_int == 't' as i32 {
        string = string.offset(1);
        string;
        (*params).type_0 = atoi(string);
        while *string as libc::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*string as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            string = string.offset(1);
            string;
        }
    }
    if *string as libc::c_int == 'd' as i32 {
        let mut i: libc::c_int = 0;
        string = string.offset(1);
        string;
        i = 0 as libc::c_int;
        while i < DIFF_MAX as libc::c_int {
            if *string as libc::c_int == diffchars[i as usize] as libc::c_int {
                (*params).diff = i;
            }
            i += 1;
            i;
        }
        if *string != 0 {
            string = string.offset(1);
            string;
        }
    }
}
unsafe extern "C" fn encode_params(
    mut params: *const game_params,
    mut full: bool,
) -> *mut libc::c_char {
    let mut str: [libc::c_char; 80] = [0; 80];
    sprintf(
        str.as_mut_ptr(),
        b"%dx%dt%d\0" as *const u8 as *const libc::c_char,
        (*params).w,
        (*params).h,
        (*params).type_0,
    );
    if full {
        sprintf(
            str.as_mut_ptr().offset(strlen(str.as_mut_ptr()) as isize),
            b"d%c\0" as *const u8 as *const libc::c_char,
            diffchars[(*params).diff as usize] as libc::c_int,
        );
    }
    return dupstr(str.as_mut_ptr());
}
unsafe extern "C" fn game_configure(mut params: *const game_params) -> *mut config_item {
    let mut ret: *mut config_item = 0 as *mut config_item;
    let mut buf: [libc::c_char; 80] = [0; 80];
    ret = smalloc(
        (5 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<config_item>() as libc::c_ulong),
    ) as *mut config_item;
    let ref mut fresh0 = (*ret.offset(0 as libc::c_int as isize)).name;
    *fresh0 = b"Width\0" as *const u8 as *const libc::c_char;
    (*ret.offset(0 as libc::c_int as isize)).type_0 = C_STRING as libc::c_int;
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, (*params).w);
    let ref mut fresh1 = (*ret.offset(0 as libc::c_int as isize)).u.string.sval;
    *fresh1 = dupstr(buf.as_mut_ptr());
    let ref mut fresh2 = (*ret.offset(1 as libc::c_int as isize)).name;
    *fresh2 = b"Height\0" as *const u8 as *const libc::c_char;
    (*ret.offset(1 as libc::c_int as isize)).type_0 = C_STRING as libc::c_int;
    sprintf(buf.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, (*params).h);
    let ref mut fresh3 = (*ret.offset(1 as libc::c_int as isize)).u.string.sval;
    *fresh3 = dupstr(buf.as_mut_ptr());
    let ref mut fresh4 = (*ret.offset(2 as libc::c_int as isize)).name;
    *fresh4 = b"Grid type\0" as *const u8 as *const libc::c_char;
    (*ret.offset(2 as libc::c_int as isize)).type_0 = C_CHOICES as libc::c_int;
    let ref mut fresh5 = (*ret.offset(2 as libc::c_int as isize)).u.choices.choicenames;
    *fresh5 = b":Squares:Triangular:Honeycomb:Snub-Square:Cairo:Great-Hexagonal:Octagonal:Kites:Floret:Dodecagonal:Great-Dodecagonal:Penrose (kite/dart):Penrose (rhombs):Great-Great-Dodecagonal:Kagome:Compass-Dodecagonal:Hats:Spectres\0"
        as *const u8 as *const libc::c_char;
    (*ret.offset(2 as libc::c_int as isize)).u.choices.selected = (*params).type_0;
    let ref mut fresh6 = (*ret.offset(3 as libc::c_int as isize)).name;
    *fresh6 = b"Difficulty\0" as *const u8 as *const libc::c_char;
    (*ret.offset(3 as libc::c_int as isize)).type_0 = C_CHOICES as libc::c_int;
    let ref mut fresh7 = (*ret.offset(3 as libc::c_int as isize)).u.choices.choicenames;
    *fresh7 = b":Easy:Normal:Tricky:Hard\0" as *const u8 as *const libc::c_char;
    (*ret.offset(3 as libc::c_int as isize)).u.choices.selected = (*params).diff;
    let ref mut fresh8 = (*ret.offset(4 as libc::c_int as isize)).name;
    *fresh8 = 0 as *const libc::c_char;
    (*ret.offset(4 as libc::c_int as isize)).type_0 = C_END as libc::c_int;
    return ret;
}
unsafe extern "C" fn custom_params(mut cfg: *const config_item) -> *mut game_params {
    let mut ret: *mut game_params = smalloc(
        ::core::mem::size_of::<game_params>() as libc::c_ulong,
    ) as *mut game_params;
    (*ret).w = atoi((*cfg.offset(0 as libc::c_int as isize)).u.string.sval);
    (*ret).h = atoi((*cfg.offset(1 as libc::c_int as isize)).u.string.sval);
    (*ret).type_0 = (*cfg.offset(2 as libc::c_int as isize)).u.choices.selected;
    (*ret).diff = (*cfg.offset(3 as libc::c_int as isize)).u.choices.selected;
    return ret;
}
unsafe extern "C" fn validate_params(
    mut params: *const game_params,
    mut full: bool,
) -> *const libc::c_char {
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    if (*params).type_0 < 0 as libc::c_int
        || (*params).type_0 as libc::c_ulong
            >= (::core::mem::size_of::<[grid_type; 18]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<grid_type>() as libc::c_ulong)
    {
        return b"Illegal grid type\0" as *const u8 as *const libc::c_char;
    }
    if (*params).w < grid_size_limits[(*params).type_0 as usize].amin
        || (*params).h < grid_size_limits[(*params).type_0 as usize].amin
    {
        return grid_size_limits[(*params).type_0 as usize].aerr;
    }
    if (*params).w < grid_size_limits[(*params).type_0 as usize].omin
        && (*params).h < grid_size_limits[(*params).type_0 as usize].omin
    {
        return grid_size_limits[(*params).type_0 as usize].oerr;
    }
    err = grid_validate_params(
        grid_types[(*params).type_0 as usize],
        (*params).w,
        (*params).h,
    );
    if !err.is_null() {
        return err;
    }
    if (*params).diff < DIFF_MAX as libc::c_int {} else {
        __assert_fail(
            b"params->diff < DIFF_MAX\0" as *const u8 as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            713 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"const char *validate_params(const game_params *, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_24277: {
        if (*params).diff < DIFF_MAX as libc::c_int {} else {
            __assert_fail(
                b"params->diff < DIFF_MAX\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                713 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"const char *validate_params(const game_params *, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn state_to_text(mut state: *const game_state) -> *mut libc::c_char {
    let mut g: *mut grid = (*state).game_grid;
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_faces: libc::c_int = (*g).num_faces;
    let mut description: *mut libc::c_char = smalloc(
        ((num_faces + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut dp: *mut libc::c_char = description;
    let mut empty_count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_faces {
        if (*((*state).clues).offset(i as isize) as libc::c_int) < 0 as libc::c_int {
            if empty_count > 25 as libc::c_int {
                dp = dp
                    .offset(
                        sprintf(
                            dp,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            empty_count + 'a' as i32 - 1 as libc::c_int,
                        ) as isize,
                    );
                empty_count = 0 as libc::c_int;
            }
            empty_count += 1;
            empty_count;
        } else {
            if empty_count != 0 {
                dp = dp
                    .offset(
                        sprintf(
                            dp,
                            b"%c\0" as *const u8 as *const libc::c_char,
                            empty_count + 'a' as i32 - 1 as libc::c_int,
                        ) as isize,
                    );
                empty_count = 0 as libc::c_int;
            }
            dp = dp
                .offset(
                    sprintf(
                        dp,
                        b"%c\0" as *const u8 as *const libc::c_char,
                        if (*((*state).clues).offset(i as isize) as libc::c_int)
                            < 0 as libc::c_int
                        {
                            ' ' as i32
                        } else if (*((*state).clues).offset(i as isize) as libc::c_int)
                            < 10 as libc::c_int
                        {
                            *((*state).clues).offset(i as isize) as libc::c_int
                                + '0' as i32
                        } else {
                            *((*state).clues).offset(i as isize) as libc::c_int
                                - 10 as libc::c_int + 'A' as i32
                        },
                    ) as isize,
                );
        }
        i += 1;
        i;
    }
    if empty_count != 0 {
        dp = dp
            .offset(
                sprintf(
                    dp,
                    b"%c\0" as *const u8 as *const libc::c_char,
                    empty_count + 'a' as i32 - 1 as libc::c_int,
                ) as isize,
            );
    }
    retval = dupstr(description);
    sfree(description as *mut libc::c_void);
    return retval;
}
unsafe extern "C" fn extract_grid_desc(
    mut desc: *mut *const libc::c_char,
) -> *mut libc::c_char {
    let mut sep: *mut libc::c_char = strchr(*desc, '_' as i32);
    let mut gd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gd_len: libc::c_int = 0;
    if sep.is_null() {
        return 0 as *mut libc::c_char;
    }
    gd_len = sep.offset_from(*desc) as libc::c_long as libc::c_int;
    gd = smalloc(
        ((gd_len + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    memcpy(
        gd as *mut libc::c_void,
        *desc as *const libc::c_void,
        gd_len as libc::c_ulong,
    );
    *gd.offset(gd_len as isize) = '\0' as i32 as libc::c_char;
    *desc = sep.offset(1 as libc::c_int as isize);
    return gd;
}
unsafe extern "C" fn validate_desc(
    mut params: *const game_params,
    mut desc: *const libc::c_char,
) -> *const libc::c_char {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut g: *mut grid = 0 as *mut grid;
    let mut grid_desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *const libc::c_char = 0 as *const libc::c_char;
    grid_desc = extract_grid_desc(&mut desc);
    ret = grid_validate_desc(
        grid_types[(*params).type_0 as usize],
        (*params).w,
        (*params).h,
        grid_desc,
    );
    if !ret.is_null() {
        sfree(grid_desc as *mut libc::c_void);
        return ret;
    }
    g = loopy_generate_grid(params, grid_desc);
    sfree(grid_desc as *mut libc::c_void);
    while *desc != 0 {
        if *desc as libc::c_int >= '0' as i32 && *desc as libc::c_int <= '9' as i32
            || *desc as libc::c_int >= 'A' as i32 && *desc as libc::c_int <= 'Z' as i32
        {
            count += 1;
            count;
        } else if *desc as libc::c_int >= 'a' as i32 {
            count += *desc as libc::c_int - 'a' as i32 + 1 as libc::c_int;
        } else {
            grid_free(g);
            return b"Unknown character in description\0" as *const u8
                as *const libc::c_char;
        }
        desc = desc.offset(1);
        desc;
    }
    if count < (*g).num_faces {
        grid_free(g);
        return b"Description too short for board size\0" as *const u8
            as *const libc::c_char;
    }
    if count > (*g).num_faces {
        grid_free(g);
        return b"Description too long for board size\0" as *const u8
            as *const libc::c_char;
    }
    grid_free(g);
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn len_0_to_n(mut n: libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < n {
        len += if n - i > 0 as libc::c_int { n - i } else { 0 as libc::c_int };
        i *= 10 as libc::c_int;
    }
    return len;
}
unsafe extern "C" fn encode_solve_move(
    mut state: *const game_state,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut num_edges: libc::c_int = (*(*state).game_grid).num_edges;
    len = 1 as libc::c_int;
    len += len_0_to_n(num_edges);
    len += num_edges;
    ret = smalloc(
        ((len + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    p = ret;
    p = p.offset(sprintf(p, b"S\0" as *const u8 as *const libc::c_char) as isize);
    i = 0 as libc::c_int;
    while i < num_edges {
        match *((*state).lines).offset(i as isize) as libc::c_int {
            0 => {
                p = p
                    .offset(
                        sprintf(p, b"%dy\0" as *const u8 as *const libc::c_char, i)
                            as isize,
                    );
            }
            2 => {
                p = p
                    .offset(
                        sprintf(p, b"%dn\0" as *const u8 as *const libc::c_char, i)
                            as isize,
                    );
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if strlen(ret) <= len as size_t {} else {
        __assert_fail(
            b"strlen(ret) <= (size_t)len\0" as *const u8 as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            872 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"char *encode_solve_move(const game_state *)\0"))
                .as_ptr(),
        );
    }
    'c_14401: {
        if strlen(ret) <= len as size_t {} else {
            __assert_fail(
                b"strlen(ret) <= (size_t)len\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                872 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"char *encode_solve_move(const game_state *)\0"))
                    .as_ptr(),
            );
        }
    };
    return ret;
}
unsafe extern "C" fn legacy_prefs_override(mut ui_out: *mut game_ui) {
    static mut initialised: bool = 0 as libc::c_int != 0;
    static mut draw_faint_lines: libc::c_int = -(1 as libc::c_int);
    static mut autofollow: libc::c_int = -(1 as libc::c_int);
    if !initialised {
        let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
        initialised = 1 as libc::c_int != 0;
        draw_faint_lines = getenv_bool(
            b"LOOPY_FAINT_LINES\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        env = getenv(b"LOOPY_AUTOFOLLOW\0" as *const u8 as *const libc::c_char);
        if !env.is_null() {
            if strcmp(env, b"off\0" as *const u8 as *const libc::c_char) == 0 {
                autofollow = AF_OFF as libc::c_int;
            } else if strcmp(env, b"fixed\0" as *const u8 as *const libc::c_char) == 0 {
                autofollow = AF_FIXED as libc::c_int;
            } else if strcmp(env, b"adaptive\0" as *const u8 as *const libc::c_char) == 0
            {
                autofollow = AF_ADAPTIVE as libc::c_int;
            }
        }
    }
    if draw_faint_lines != -(1 as libc::c_int) {
        (*ui_out).draw_faint_lines = draw_faint_lines != 0;
    }
    if autofollow != -(1 as libc::c_int) {
        (*ui_out).autofollow = autofollow as C2RustUnnamed_5;
    }
}
unsafe extern "C" fn new_ui(mut state: *const game_state) -> *mut game_ui {
    let mut ui: *mut game_ui = smalloc(
        ::core::mem::size_of::<game_ui>() as libc::c_ulong,
    ) as *mut game_ui;
    (*ui).draw_faint_lines = 1 as libc::c_int != 0;
    (*ui).autofollow = AF_OFF;
    legacy_prefs_override(ui);
    return ui;
}
unsafe extern "C" fn free_ui(mut ui: *mut game_ui) {
    sfree(ui as *mut libc::c_void);
}
unsafe extern "C" fn get_prefs(mut ui: *mut game_ui) -> *mut config_item {
    let mut ret: *mut config_item = 0 as *mut config_item;
    ret = smalloc(
        (3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<config_item>() as libc::c_ulong),
    ) as *mut config_item;
    let ref mut fresh9 = (*ret.offset(0 as libc::c_int as isize)).name;
    *fresh9 = b"Draw excluded grid lines faintly\0" as *const u8 as *const libc::c_char;
    let ref mut fresh10 = (*ret.offset(0 as libc::c_int as isize)).kw;
    *fresh10 = b"draw-faint-lines\0" as *const u8 as *const libc::c_char;
    (*ret.offset(0 as libc::c_int as isize)).type_0 = C_BOOLEAN as libc::c_int;
    (*ret.offset(0 as libc::c_int as isize)).u.boolean.bval = (*ui).draw_faint_lines;
    let ref mut fresh11 = (*ret.offset(1 as libc::c_int as isize)).name;
    *fresh11 = b"Auto-follow unique paths of edges\0" as *const u8
        as *const libc::c_char;
    let ref mut fresh12 = (*ret.offset(1 as libc::c_int as isize)).kw;
    *fresh12 = b"auto-follow\0" as *const u8 as *const libc::c_char;
    (*ret.offset(1 as libc::c_int as isize)).type_0 = C_CHOICES as libc::c_int;
    let ref mut fresh13 = (*ret.offset(1 as libc::c_int as isize)).u.choices.choicenames;
    *fresh13 = b":No:Based on grid only:Based on grid and game state\0" as *const u8
        as *const libc::c_char;
    let ref mut fresh14 = (*ret.offset(1 as libc::c_int as isize)).u.choices.choicekws;
    *fresh14 = b":off:fixed:adaptive\0" as *const u8 as *const libc::c_char;
    (*ret.offset(1 as libc::c_int as isize))
        .u
        .choices
        .selected = (*ui).autofollow as libc::c_int;
    let ref mut fresh15 = (*ret.offset(2 as libc::c_int as isize)).name;
    *fresh15 = 0 as *const libc::c_char;
    (*ret.offset(2 as libc::c_int as isize)).type_0 = C_END as libc::c_int;
    return ret;
}
unsafe extern "C" fn set_prefs(mut ui: *mut game_ui, mut cfg: *const config_item) {
    (*ui).draw_faint_lines = (*cfg.offset(0 as libc::c_int as isize)).u.boolean.bval;
    (*ui)
        .autofollow = (*cfg.offset(1 as libc::c_int as isize)).u.choices.selected
        as C2RustUnnamed_5;
}
unsafe extern "C" fn game_changed_state(
    mut ui: *mut game_ui,
    mut oldstate: *const game_state,
    mut newstate: *const game_state,
) {}
unsafe extern "C" fn game_compute_size(
    mut params: *const game_params,
    mut tilesize: libc::c_int,
    mut ui: *const game_ui,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
) {
    let mut grid_width: libc::c_int = 0;
    let mut grid_height: libc::c_int = 0;
    let mut rendered_width: libc::c_int = 0;
    let mut rendered_height: libc::c_int = 0;
    let mut g_tilesize: libc::c_int = 0;
    grid_compute_size(
        grid_types[(*params).type_0 as usize],
        (*params).w,
        (*params).h,
        &mut g_tilesize,
        &mut grid_width,
        &mut grid_height,
    );
    rendered_width = grid_width * tilesize / g_tilesize;
    rendered_height = grid_height * tilesize / g_tilesize;
    *x = rendered_width + 2 as libc::c_int * (tilesize / 2 as libc::c_int)
        + 1 as libc::c_int;
    *y = rendered_height + 2 as libc::c_int * (tilesize / 2 as libc::c_int)
        + 1 as libc::c_int;
}
unsafe extern "C" fn game_set_size(
    mut dr: *mut drawing,
    mut ds: *mut game_drawstate,
    mut params: *const game_params,
    mut tilesize: libc::c_int,
) {
    (*ds).tilesize = tilesize;
}
unsafe extern "C" fn game_colours(
    mut fe: *mut frontend,
    mut ncolours: *mut libc::c_int,
) -> *mut libc::c_float {
    let mut ret: *mut libc::c_float = smalloc(
        ((3 as libc::c_int * NCOLOURS as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong),
    ) as *mut libc::c_float;
    frontend_default_colour(
        fe,
        &mut *ret.offset((COL_BACKGROUND as libc::c_int * 3 as libc::c_int) as isize),
    );
    *ret
        .offset(
            (COL_FOREGROUND as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_FOREGROUND as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_FOREGROUND as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
                as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_LINEUNKNOWN as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as isize,
        ) = *ret
        .offset(
            (COL_BACKGROUND as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as isize,
        ) * 0.9f32;
    *ret
        .offset(
            (COL_LINEUNKNOWN as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as isize,
        ) = *ret
        .offset(
            (COL_BACKGROUND as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as isize,
        ) * 0.9f32;
    *ret
        .offset(
            (COL_LINEUNKNOWN as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
                as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_HIGHLIGHT as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as isize,
        ) = 1.0f32;
    *ret
        .offset(
            (COL_HIGHLIGHT as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as isize,
        ) = 1.0f32;
    *ret
        .offset(
            (COL_HIGHLIGHT as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) as isize,
        ) = 1.0f32;
    *ret
        .offset(
            (COL_MISTAKE as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as isize,
        ) = 1.0f32;
    *ret
        .offset(
            (COL_MISTAKE as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_MISTAKE as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_SATISFIED as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_SATISFIED as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_SATISFIED as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) as isize,
        ) = 0.0f32;
    *ret
        .offset(
            (COL_FAINT as libc::c_int * 3 as libc::c_int + 0 as libc::c_int) as isize,
        ) = *ret
        .offset(
            (COL_BACKGROUND as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                as isize,
        ) * 0.9f32;
    *ret
        .offset(
            (COL_FAINT as libc::c_int * 3 as libc::c_int + 1 as libc::c_int) as isize,
        ) = *ret
        .offset(
            (COL_BACKGROUND as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                as isize,
        ) * 0.9f32;
    *ret
        .offset(
            (COL_FAINT as libc::c_int * 3 as libc::c_int + 2 as libc::c_int) as isize,
        ) = *ret
        .offset(
            (COL_BACKGROUND as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
                as isize,
        ) * 0.9f32;
    *ncolours = NCOLOURS as libc::c_int;
    return ret;
}
unsafe extern "C" fn game_new_drawstate(
    mut dr: *mut drawing,
    mut state: *const game_state,
) -> *mut game_drawstate {
    let mut ds: *mut game_drawstate = smalloc(
        ::core::mem::size_of::<game_drawstate>() as libc::c_ulong,
    ) as *mut game_drawstate;
    let mut num_faces: libc::c_int = (*(*state).game_grid).num_faces;
    let mut num_edges: libc::c_int = (*(*state).game_grid).num_edges;
    let mut i: libc::c_int = 0;
    (*ds).tilesize = 0 as libc::c_int;
    (*ds).started = 0 as libc::c_int != 0;
    (*ds)
        .lines = smalloc(
        (num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    (*ds)
        .clue_error = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    (*ds)
        .clue_satisfied = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    (*ds)
        .textx = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*ds)
        .texty = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*ds).flashing = 0 as libc::c_int != 0;
    memset(
        (*ds).lines as *mut libc::c_void,
        LINE_UNKNOWN as libc::c_int,
        num_edges as libc::c_ulong,
    );
    memset(
        (*ds).clue_error as *mut libc::c_void,
        0 as libc::c_int,
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    memset(
        (*ds).clue_satisfied as *mut libc::c_void,
        0 as libc::c_int,
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < num_faces {
        let ref mut fresh16 = *((*ds).texty).offset(i as isize);
        *fresh16 = -(1 as libc::c_int);
        *((*ds).textx).offset(i as isize) = *fresh16;
        i += 1;
        i;
    }
    return ds;
}
unsafe extern "C" fn game_free_drawstate(
    mut dr: *mut drawing,
    mut ds: *mut game_drawstate,
) {
    sfree((*ds).textx as *mut libc::c_void);
    sfree((*ds).texty as *mut libc::c_void);
    sfree((*ds).clue_error as *mut libc::c_void);
    sfree((*ds).clue_satisfied as *mut libc::c_void);
    sfree((*ds).lines as *mut libc::c_void);
    sfree(ds as *mut libc::c_void);
}
unsafe extern "C" fn game_anim_length(
    mut oldstate: *const game_state,
    mut newstate: *const game_state,
    mut dir: libc::c_int,
    mut ui: *mut game_ui,
) -> libc::c_float {
    return 0.0f32;
}
unsafe extern "C" fn game_can_format_as_text_now(
    mut params: *const game_params,
) -> bool {
    if (*params).type_0 != 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn game_text_format(
    mut state: *const game_state,
) -> *mut libc::c_char {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut W: libc::c_int = 0;
    let mut H: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cell_size: libc::c_int = 0;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut grid = (*state).game_grid;
    let mut f: *mut grid_face = 0 as *mut grid_face;
    if (*state).grid_type == 0 as libc::c_int {} else {
        __assert_fail(
            b"state->grid_type == 0\0" as *const u8 as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            1098 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"char *game_text_format(const game_state *)\0"))
                .as_ptr(),
        );
    }
    'c_14132: {
        if (*state).grid_type == 0 as libc::c_int {} else {
            __assert_fail(
                b"state->grid_type == 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1098 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"char *game_text_format(const game_state *)\0"))
                    .as_ptr(),
            );
        }
    };
    f = *((*g).faces).offset(0 as libc::c_int as isize);
    if (*f).order == 4 as libc::c_int {} else {
        __assert_fail(
            b"f->order == 4\0" as *const u8 as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            1102 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"char *game_text_format(const game_state *)\0"))
                .as_ptr(),
        );
    }
    'c_14083: {
        if (*f).order == 4 as libc::c_int {} else {
            __assert_fail(
                b"f->order == 4\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1102 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"char *game_text_format(const game_state *)\0"))
                    .as_ptr(),
            );
        }
    };
    cell_size = abs(
        (**((*f).dots).offset(0 as libc::c_int as isize)).x
            - (**((*f).dots).offset(2 as libc::c_int as isize)).x,
    );
    w = ((*g).highest_x - (*g).lowest_x) / cell_size;
    h = ((*g).highest_y - (*g).lowest_y) / cell_size;
    W = 2 as libc::c_int * w + 2 as libc::c_int;
    H = 2 as libc::c_int * h + 1 as libc::c_int;
    ret = smalloc(
        ((W * H + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    y = 0 as libc::c_int;
    while y < H {
        x = 0 as libc::c_int;
        while x < W - 1 as libc::c_int {
            *ret.offset((y * W + x) as isize) = ' ' as i32 as libc::c_char;
            x += 1;
            x;
        }
        *ret
            .offset(
                (y * W + W - 1 as libc::c_int) as isize,
            ) = '\n' as i32 as libc::c_char;
        y += 1;
        y;
    }
    *ret.offset((H * W) as isize) = '\0' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut e: *mut grid_edge = *((*g).edges).offset(i as isize);
        let mut x1: libc::c_int = ((*(*e).dot1).x - (*g).lowest_x) / cell_size;
        let mut x2: libc::c_int = ((*(*e).dot2).x - (*g).lowest_x) / cell_size;
        let mut y1: libc::c_int = ((*(*e).dot1).y - (*g).lowest_y) / cell_size;
        let mut y2: libc::c_int = ((*(*e).dot2).y - (*g).lowest_y) / cell_size;
        x = x1 + x2;
        y = y1 + y2;
        match *((*state).lines).offset(i as isize) as libc::c_int {
            0 => {
                *ret
                    .offset(
                        (y * W + x) as isize,
                    ) = (if y1 == y2 { '-' as i32 } else { '|' as i32 }) as libc::c_char;
            }
            2 => {
                *ret.offset((y * W + x) as isize) = 'x' as i32 as libc::c_char;
            }
            1 => {}
            _ => {
                if (b"Illegal line state\0" as *const u8 as *const libc::c_char)
                    .is_null()
                {} else {
                    __assert_fail(
                        b"!\"Illegal line state\"\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                        1144 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 43],
                            &[libc::c_char; 43],
                        >(b"char *game_text_format(const game_state *)\0"))
                            .as_ptr(),
                    );
                }
                'c_13735: {
                    if (b"Illegal line state\0" as *const u8 as *const libc::c_char)
                        .is_null()
                    {} else {
                        __assert_fail(
                            b"!\"Illegal line state\"\0" as *const u8
                                as *const libc::c_char,
                            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                            1144 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 43],
                                &[libc::c_char; 43],
                            >(b"char *game_text_format(const game_state *)\0"))
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
    while i < (*g).num_faces {
        let mut x1_0: libc::c_int = 0;
        let mut x2_0: libc::c_int = 0;
        let mut y1_0: libc::c_int = 0;
        let mut y2_0: libc::c_int = 0;
        f = *((*g).faces).offset(i as isize);
        if (*f).order == 4 as libc::c_int {} else {
            __assert_fail(
                b"f->order == 4\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1153 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"char *game_text_format(const game_state *)\0"))
                    .as_ptr(),
            );
        }
        'c_13650: {
            if (*f).order == 4 as libc::c_int {} else {
                __assert_fail(
                    b"f->order == 4\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                    1153 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 43],
                        &[libc::c_char; 43],
                    >(b"char *game_text_format(const game_state *)\0"))
                        .as_ptr(),
                );
            }
        };
        x1_0 = ((**((*f).dots).offset(0 as libc::c_int as isize)).x - (*g).lowest_x)
            / cell_size;
        x2_0 = ((**((*f).dots).offset(2 as libc::c_int as isize)).x - (*g).lowest_x)
            / cell_size;
        y1_0 = ((**((*f).dots).offset(0 as libc::c_int as isize)).y - (*g).lowest_y)
            / cell_size;
        y2_0 = ((**((*f).dots).offset(2 as libc::c_int as isize)).y - (*g).lowest_y)
            / cell_size;
        x = x1_0 + x2_0;
        y = y1_0 + y2_0;
        *ret
            .offset(
                (y * W + x) as isize,
            ) = (if (*((*state).clues).offset(i as isize) as libc::c_int)
            < 0 as libc::c_int
        {
            ' ' as i32
        } else if (*((*state).clues).offset(i as isize) as libc::c_int)
            < 10 as libc::c_int
        {
            *((*state).clues).offset(i as isize) as libc::c_int + '0' as i32
        } else {
            *((*state).clues).offset(i as isize) as libc::c_int - 10 as libc::c_int
                + 'A' as i32
        }) as libc::c_char;
        i += 1;
        i;
    }
    return ret;
}
unsafe extern "C" fn solver_set_line(
    mut sstate: *mut solver_state,
    mut i: libc::c_int,
    mut line_new: line_state,
) -> bool {
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = 0 as *mut grid;
    let mut e: *mut grid_edge = 0 as *mut grid_edge;
    if line_new as libc::c_uint != LINE_UNKNOWN as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"line_new != LINE_UNKNOWN\0" as *const u8 as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            1216 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"_Bool solver_set_line(solver_state *, int, enum line_state)\0"))
                .as_ptr(),
        );
    }
    'c_15142: {
        if line_new as libc::c_uint != LINE_UNKNOWN as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"line_new != LINE_UNKNOWN\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1216 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[libc::c_char; 60],
                >(b"_Bool solver_set_line(solver_state *, int, enum line_state)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*state).lines).offset(i as isize) as libc::c_uint == line_new as libc::c_uint {
        return 0 as libc::c_int != 0;
    }
    *((*state).lines).offset(i as isize) = line_new as libc::c_char;
    g = (*state).game_grid;
    e = *((*g).edges).offset(i as isize);
    if line_new as libc::c_uint == LINE_YES as libc::c_int as libc::c_uint {
        let ref mut fresh17 = *((*sstate).dot_yes_count)
            .offset((*(*e).dot1).index as isize);
        *fresh17 += 1;
        *fresh17;
        let ref mut fresh18 = *((*sstate).dot_yes_count)
            .offset((*(*e).dot2).index as isize);
        *fresh18 += 1;
        *fresh18;
        if !((*e).face1).is_null() {
            let ref mut fresh19 = *((*sstate).face_yes_count)
                .offset((*(*e).face1).index as isize);
            *fresh19 += 1;
            *fresh19;
        }
        if !((*e).face2).is_null() {
            let ref mut fresh20 = *((*sstate).face_yes_count)
                .offset((*(*e).face2).index as isize);
            *fresh20 += 1;
            *fresh20;
        }
    } else {
        let ref mut fresh21 = *((*sstate).dot_no_count)
            .offset((*(*e).dot1).index as isize);
        *fresh21 += 1;
        *fresh21;
        let ref mut fresh22 = *((*sstate).dot_no_count)
            .offset((*(*e).dot2).index as isize);
        *fresh22 += 1;
        *fresh22;
        if !((*e).face1).is_null() {
            let ref mut fresh23 = *((*sstate).face_no_count)
                .offset((*(*e).face1).index as isize);
            *fresh23 += 1;
            *fresh23;
        }
        if !((*e).face2).is_null() {
            let ref mut fresh24 = *((*sstate).face_no_count)
                .offset((*(*e).face2).index as isize);
            *fresh24 += 1;
            *fresh24;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn merge_dots(
    mut sstate: *mut solver_state,
    mut edge_index: libc::c_int,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut g: *mut grid = (*(*sstate).state).game_grid;
    let mut e: *mut grid_edge = *((*g).edges).offset(edge_index as isize);
    i = (*(*e).dot1).index;
    j = (*(*e).dot2).index;
    i = dsf_canonify((*sstate).dotdsf, i);
    j = dsf_canonify((*sstate).dotdsf, j);
    if i == j {
        return 1 as libc::c_int != 0
    } else {
        len = *((*sstate).looplen).offset(i as isize)
            + *((*sstate).looplen).offset(j as isize);
        dsf_merge((*sstate).dotdsf, i, j);
        i = dsf_canonify((*sstate).dotdsf, i);
        *((*sstate).looplen).offset(i as isize) = len;
        return 0 as libc::c_int != 0;
    };
}
unsafe extern "C" fn merge_lines(
    mut sstate: *mut solver_state,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut inverse: bool,
) -> bool {
    let mut inv_tmp: bool = false;
    if i < (*(*(*sstate).state).game_grid).num_edges {} else {
        __assert_fail(
            b"i < sstate->state->game_grid->num_edges\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            1305 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"_Bool merge_lines(solver_state *, int, int, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_16270: {
        if i < (*(*(*sstate).state).game_grid).num_edges {} else {
            __assert_fail(
                b"i < sstate->state->game_grid->num_edges\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1305 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"_Bool merge_lines(solver_state *, int, int, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    if j < (*(*(*sstate).state).game_grid).num_edges {} else {
        __assert_fail(
            b"j < sstate->state->game_grid->num_edges\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            1306 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"_Bool merge_lines(solver_state *, int, int, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_16218: {
        if j < (*(*(*sstate).state).game_grid).num_edges {} else {
            __assert_fail(
                b"j < sstate->state->game_grid->num_edges\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1306 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"_Bool merge_lines(solver_state *, int, int, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    i = dsf_canonify_flip((*sstate).linedsf, i, &mut inv_tmp);
    inverse = (inverse as libc::c_int ^ inv_tmp as libc::c_int) as bool;
    j = dsf_canonify_flip((*sstate).linedsf, j, &mut inv_tmp);
    inverse = (inverse as libc::c_int ^ inv_tmp as libc::c_int) as bool;
    dsf_merge_flip((*sstate).linedsf, i, j, inverse);
    return i != j;
}
unsafe extern "C" fn dot_order(
    mut state: *const game_state,
    mut dot: libc::c_int,
    mut line_type: libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut g: *mut grid = (*state).game_grid;
    let mut d: *mut grid_dot = *((*g).dots).offset(dot as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*d).order {
        let mut e: *mut grid_edge = *((*d).edges).offset(i as isize);
        if *((*state).lines).offset((*e).index as isize) as libc::c_int
            == line_type as libc::c_int
        {
            n += 1;
            n;
        }
        i += 1;
        i;
    }
    return n;
}
unsafe extern "C" fn face_order(
    mut state: *const game_state,
    mut face: libc::c_int,
    mut line_type: libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut g: *mut grid = (*state).game_grid;
    let mut f: *mut grid_face = *((*g).faces).offset(face as isize);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*f).order {
        let mut e: *mut grid_edge = *((*f).edges).offset(i as isize);
        if *((*state).lines).offset((*e).index as isize) as libc::c_int
            == line_type as libc::c_int
        {
            n += 1;
            n;
        }
        i += 1;
        i;
    }
    return n;
}
unsafe extern "C" fn dot_setall(
    mut sstate: *mut solver_state,
    mut dot: libc::c_int,
    mut old_type: libc::c_char,
    mut new_type: libc::c_char,
) -> bool {
    let mut retval: bool = 0 as libc::c_int != 0;
    let mut r: bool = false;
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = 0 as *mut grid;
    let mut d: *mut grid_dot = 0 as *mut grid_dot;
    let mut i: libc::c_int = 0;
    if old_type as libc::c_int == new_type as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    g = (*state).game_grid;
    d = *((*g).dots).offset(dot as isize);
    i = 0 as libc::c_int;
    while i < (*d).order {
        let mut line_index: libc::c_int = (**((*d).edges).offset(i as isize)).index;
        if *((*state).lines).offset(line_index as isize) as libc::c_int
            == old_type as libc::c_int
        {
            r = solver_set_line(sstate, line_index, new_type as line_state);
            if r {} else {
                __assert_fail(
                    b"r\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                    1385 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"_Bool dot_setall(solver_state *, int, char, char)\0"))
                        .as_ptr(),
                );
            }
            'c_20241: {
                if r {} else {
                    __assert_fail(
                        b"r\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                        1385 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[libc::c_char; 50],
                        >(b"_Bool dot_setall(solver_state *, int, char, char)\0"))
                            .as_ptr(),
                    );
                }
            };
            retval = 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return retval;
}
unsafe extern "C" fn face_setall(
    mut sstate: *mut solver_state,
    mut face: libc::c_int,
    mut old_type: libc::c_char,
    mut new_type: libc::c_char,
) -> bool {
    let mut retval: bool = 0 as libc::c_int != 0;
    let mut r: bool = false;
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = 0 as *mut grid;
    let mut f: *mut grid_face = 0 as *mut grid_face;
    let mut i: libc::c_int = 0;
    if old_type as libc::c_int == new_type as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    g = (*state).game_grid;
    f = *((*g).faces).offset(face as isize);
    i = 0 as libc::c_int;
    while i < (*f).order {
        let mut line_index: libc::c_int = (**((*f).edges).offset(i as isize)).index;
        if *((*state).lines).offset(line_index as isize) as libc::c_int
            == old_type as libc::c_int
        {
            r = solver_set_line(sstate, line_index, new_type as line_state);
            if r {} else {
                __assert_fail(
                    b"r\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                    1412 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 51],
                        &[libc::c_char; 51],
                    >(b"_Bool face_setall(solver_state *, int, char, char)\0"))
                        .as_ptr(),
                );
            }
            'c_21137: {
                if r {} else {
                    __assert_fail(
                        b"r\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                        1412 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 51],
                            &[libc::c_char; 51],
                        >(b"_Bool face_setall(solver_state *, int, char, char)\0"))
                            .as_ptr(),
                    );
                }
            };
            retval = 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return retval;
}
unsafe extern "C" fn add_full_clues(
    mut state: *mut game_state,
    mut rs: *mut random_state,
) {
    let mut clues: *mut libc::c_schar = (*state).clues;
    let mut g: *mut grid = (*state).game_grid;
    let mut board: *mut libc::c_char = smalloc(
        ((*g).num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    generate_loop(g, board, rs, None, 0 as *mut libc::c_void);
    memset(
        clues as *mut libc::c_void,
        0 as libc::c_int,
        (*g).num_faces as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut e: *mut grid_edge = *((*g).edges).offset(i as isize);
        let mut f1: *mut grid_face = (*e).face1;
        let mut f2: *mut grid_face = (*e).face2;
        let mut c1: face_colour = (if f1.is_null() {
            FACE_BLACK as libc::c_int
        } else {
            *board.offset((*f1).index as isize) as libc::c_int
        }) as face_colour;
        let mut c2: face_colour = (if f2.is_null() {
            FACE_BLACK as libc::c_int
        } else {
            *board.offset((*f2).index as isize) as libc::c_int
        }) as face_colour;
        if c1 as libc::c_uint != FACE_GREY as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"c1 != FACE_GREY\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1443 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void add_full_clues(game_state *, random_state *)\0"))
                    .as_ptr(),
            );
        }
        'c_24030: {
            if c1 as libc::c_uint != FACE_GREY as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"c1 != FACE_GREY\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                    1443 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"void add_full_clues(game_state *, random_state *)\0"))
                        .as_ptr(),
                );
            }
        };
        if c2 as libc::c_uint != FACE_GREY as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"c2 != FACE_GREY\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1444 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void add_full_clues(game_state *, random_state *)\0"))
                    .as_ptr(),
            );
        }
        'c_23989: {
            if c2 as libc::c_uint != FACE_GREY as libc::c_int as libc::c_uint {} else {
                __assert_fail(
                    b"c2 != FACE_GREY\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                    1444 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[libc::c_char; 50],
                    >(b"void add_full_clues(game_state *, random_state *)\0"))
                        .as_ptr(),
                );
            }
        };
        if c1 as libc::c_uint != c2 as libc::c_uint {
            if !f1.is_null() {
                let ref mut fresh25 = *clues.offset((*f1).index as isize);
                *fresh25 += 1;
                *fresh25;
            }
            if !f2.is_null() {
                let ref mut fresh26 = *clues.offset((*f2).index as isize);
                *fresh26 += 1;
                *fresh26;
            }
        }
        i += 1;
        i;
    }
    sfree(board as *mut libc::c_void);
}
unsafe extern "C" fn game_has_unique_soln(
    mut state: *const game_state,
    mut diff: libc::c_int,
) -> bool {
    let mut ret: bool = false;
    let mut sstate_new: *mut solver_state = 0 as *mut solver_state;
    let mut sstate: *mut solver_state = new_solver_state(state, diff);
    sstate_new = solve_game_rec(sstate);
    if (*sstate_new).solver_status as libc::c_uint
        != SOLVER_MISTAKE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"sstate_new->solver_status != SOLVER_MISTAKE\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            1462 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"_Bool game_has_unique_soln(const game_state *, int)\0"))
                .as_ptr(),
        );
    }
    'c_23574: {
        if (*sstate_new).solver_status as libc::c_uint
            != SOLVER_MISTAKE as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"sstate_new->solver_status != SOLVER_MISTAKE\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1462 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"_Bool game_has_unique_soln(const game_state *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    ret = (*sstate_new).solver_status as libc::c_uint
        == SOLVER_SOLVED as libc::c_int as libc::c_uint;
    free_solver_state(sstate_new);
    free_solver_state(sstate);
    return ret;
}
unsafe extern "C" fn remove_clues(
    mut state: *mut game_state,
    mut rs: *mut random_state,
    mut diff: libc::c_int,
) -> *mut game_state {
    let mut face_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut num_faces: libc::c_int = (*(*state).game_grid).num_faces;
    let mut ret: *mut game_state = dup_game(state);
    let mut saved_ret: *mut game_state = 0 as *mut game_state;
    let mut n: libc::c_int = 0;
    face_list = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    n = 0 as libc::c_int;
    while n < num_faces {
        *face_list.offset(n as isize) = n;
        n += 1;
        n;
    }
    shuffle(
        face_list as *mut libc::c_void,
        num_faces,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        rs,
    );
    n = 0 as libc::c_int;
    while n < num_faces {
        saved_ret = dup_game(ret);
        *((*ret).clues)
            .offset(
                *face_list.offset(n as isize) as isize,
            ) = -(1 as libc::c_int) as libc::c_schar;
        if game_has_unique_soln(ret, diff) {
            free_game(saved_ret);
        } else {
            free_game(ret);
            ret = saved_ret;
        }
        n += 1;
        n;
    }
    sfree(face_list as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn new_game_desc(
    mut params: *const game_params,
    mut rs: *mut random_state,
    mut aux: *mut *mut libc::c_char,
    mut interactive: bool,
) -> *mut libc::c_char {
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut game_desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut grid_desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut grid = 0 as *mut grid;
    let mut state: *mut game_state = smalloc(
        ::core::mem::size_of::<game_state>() as libc::c_ulong,
    ) as *mut game_state;
    let mut state_new: *mut game_state = 0 as *mut game_state;
    grid_desc = grid_new_desc(
        grid_types[(*params).type_0 as usize],
        (*params).w,
        (*params).h,
        rs,
    );
    g = loopy_generate_grid(params, grid_desc);
    (*state).game_grid = g;
    (*state)
        .clues = smalloc(
        ((*g).num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_schar>() as libc::c_ulong),
    ) as *mut libc::c_schar;
    (*state)
        .lines = smalloc(
        ((*g).num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    (*state)
        .line_errors = smalloc(
        ((*g).num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    (*state).exactly_one_loop = 0 as libc::c_int != 0;
    (*state).grid_type = (*params).type_0;
    loop {
        memset(
            (*state).lines as *mut libc::c_void,
            LINE_UNKNOWN as libc::c_int,
            (*g).num_edges as libc::c_ulong,
        );
        memset(
            (*state).line_errors as *mut libc::c_void,
            0 as libc::c_int,
            ((*g).num_edges as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
        );
        (*state).solved = 0 as libc::c_int != 0;
        (*state).cheated = 0 as libc::c_int != 0;
        loop {
            add_full_clues(state, rs);
            if game_has_unique_soln(state, (*params).diff) {
                break;
            }
        }
        state_new = remove_clues(state, rs, (*params).diff);
        free_game(state);
        state = state_new;
        if !((*params).diff > 0 as libc::c_int
            && game_has_unique_soln(state, (*params).diff - 1 as libc::c_int)
                as libc::c_int != 0)
        {
            break;
        }
    }
    game_desc = state_to_text(state);
    free_game(state);
    if !grid_desc.is_null() {
        retval = smalloc(
            (strlen(grid_desc))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen(game_desc))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        sprintf(
            retval,
            b"%s%c%s\0" as *const u8 as *const libc::c_char,
            grid_desc,
            '_' as i32,
            game_desc,
        );
        sfree(grid_desc as *mut libc::c_void);
        sfree(game_desc as *mut libc::c_void);
    } else {
        retval = game_desc;
    }
    if (validate_desc(params, retval)).is_null() {} else {
        __assert_fail(
            b"!validate_desc(params, retval)\0" as *const u8 as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            1568 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"char *new_game_desc(const game_params *, random_state *, char **, _Bool)\0",
            ))
                .as_ptr(),
        );
    }
    'c_23127: {
        if (validate_desc(params, retval)).is_null() {} else {
            __assert_fail(
                b"!validate_desc(params, retval)\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                1568 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[libc::c_char; 73],
                >(
                    b"char *new_game_desc(const game_params *, random_state *, char **, _Bool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return retval;
}
unsafe extern "C" fn new_game(
    mut me: *mut midend,
    mut params: *const game_params,
    mut desc: *const libc::c_char,
) -> *mut game_state {
    let mut i: libc::c_int = 0;
    let mut state: *mut game_state = smalloc(
        ::core::mem::size_of::<game_state>() as libc::c_ulong,
    ) as *mut game_state;
    let mut empties_to_make: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut dp: *const libc::c_char = 0 as *const libc::c_char;
    let mut grid_desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut g: *mut grid = 0 as *mut grid;
    let mut num_faces: libc::c_int = 0;
    let mut num_edges: libc::c_int = 0;
    grid_desc = extract_grid_desc(&mut desc);
    g = loopy_generate_grid(params, grid_desc);
    (*state).game_grid = g;
    if !grid_desc.is_null() {
        sfree(grid_desc as *mut libc::c_void);
    }
    dp = desc;
    num_faces = (*g).num_faces;
    num_edges = (*g).num_edges;
    (*state)
        .clues = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_schar>() as libc::c_ulong),
    ) as *mut libc::c_schar;
    (*state)
        .lines = smalloc(
        (num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    (*state)
        .line_errors = smalloc(
        (num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    (*state).exactly_one_loop = 0 as libc::c_int != 0;
    (*state).cheated = 0 as libc::c_int != 0;
    (*state).solved = (*state).cheated;
    (*state).grid_type = (*params).type_0;
    i = 0 as libc::c_int;
    while i < num_faces {
        if empties_to_make != 0 {
            empties_to_make -= 1;
            empties_to_make;
            *((*state).clues).offset(i as isize) = -(1 as libc::c_int) as libc::c_schar;
        } else {
            if *dp != 0 {} else {
                __assert_fail(
                    b"*dp\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                    1610 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 66],
                        &[libc::c_char; 66],
                    >(
                        b"game_state *new_game(midend *, const game_params *, const char *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_22568: {
                if *dp != 0 {} else {
                    __assert_fail(
                        b"*dp\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                        1610 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 66],
                            &[libc::c_char; 66],
                        >(
                            b"game_state *new_game(midend *, const game_params *, const char *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            n = *dp as libc::c_int - '0' as i32;
            n2 = *dp as libc::c_int - 'A' as i32 + 10 as libc::c_int;
            if n >= 0 as libc::c_int && n < 10 as libc::c_int {
                *((*state).clues).offset(i as isize) = n as libc::c_schar;
            } else if n2 >= 10 as libc::c_int && n2 < 36 as libc::c_int {
                *((*state).clues).offset(i as isize) = n2 as libc::c_schar;
            } else {
                n = *dp as libc::c_int - 'a' as i32 + 1 as libc::c_int;
                if n > 0 as libc::c_int {} else {
                    __assert_fail(
                        b"n > 0\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                        1619 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 66],
                            &[libc::c_char; 66],
                        >(
                            b"game_state *new_game(midend *, const game_params *, const char *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_22459: {
                    if n > 0 as libc::c_int {} else {
                        __assert_fail(
                            b"n > 0\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                            1619 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 66],
                                &[libc::c_char; 66],
                            >(
                                b"game_state *new_game(midend *, const game_params *, const char *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                *((*state).clues)
                    .offset(i as isize) = -(1 as libc::c_int) as libc::c_schar;
                empties_to_make = n - 1 as libc::c_int;
            }
            dp = dp.offset(1);
            dp;
        }
        i += 1;
        i;
    }
    memset(
        (*state).lines as *mut libc::c_void,
        LINE_UNKNOWN as libc::c_int,
        num_edges as libc::c_ulong,
    );
    memset(
        (*state).line_errors as *mut libc::c_void,
        0 as libc::c_int,
        (num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    return state;
}
unsafe extern "C" fn check_completion(mut state: *mut game_state) -> bool {
    let mut g: *mut grid = (*state).game_grid;
    let mut i: libc::c_int = 0;
    let mut ret: bool = false;
    let mut dsf: *mut DSF = 0 as *mut DSF;
    let mut component_state: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nsilly: libc::c_int = 0;
    let mut nloop: libc::c_int = 0;
    let mut npath: libc::c_int = 0;
    let mut largest_comp: libc::c_int = 0;
    let mut largest_size: libc::c_int = 0;
    let mut total_pathsize: libc::c_int = 0;
    memset(
        (*state).line_errors as *mut libc::c_void,
        0 as libc::c_int,
        ((*g).num_edges as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    );
    dsf = dsf_new((*g).num_dots);
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        if *((*state).lines).offset(i as isize) as libc::c_int == LINE_YES as libc::c_int
        {
            let mut e: *mut grid_edge = *((*g).edges).offset(i as isize);
            let mut d1: libc::c_int = (*(*e).dot1).index;
            let mut d2: libc::c_int = (*(*e).dot2).index;
            dsf_merge(dsf, d1, d2);
        }
        i += 1;
        i;
    }
    component_state = smalloc(
        ((*g).num_dots as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        if dsf_canonify(dsf, i) == i {
            *component_state.offset(i as isize) = COMP_LOOP as libc::c_int;
        } else {
            *component_state.offset(i as isize) = COMP_NONE as libc::c_int;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut comp: libc::c_int = dsf_canonify(dsf, i);
        let mut yes: libc::c_int = dot_order(
            state,
            i,
            LINE_YES as libc::c_int as libc::c_char,
        );
        let mut unknown: libc::c_int = dot_order(
            state,
            i,
            LINE_UNKNOWN as libc::c_int as libc::c_char,
        );
        if yes == 1 as libc::c_int && unknown == 0 as libc::c_int
            || yes >= 3 as libc::c_int
        {
            let mut d: *mut grid_dot = *((*g).dots).offset(i as isize);
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < (*d).order {
                let mut e_0: libc::c_int = (**((*d).edges).offset(j as isize)).index;
                if *((*state).lines).offset(e_0 as isize) as libc::c_int
                    == LINE_YES as libc::c_int
                {
                    *((*state).line_errors).offset(e_0 as isize) = 1 as libc::c_int != 0;
                }
                j += 1;
                j;
            }
            *component_state.offset(comp as isize) = COMP_SILLY as libc::c_int;
        } else if yes == 0 as libc::c_int {
            *component_state.offset(comp as isize) = COMP_EMPTY as libc::c_int;
        } else if yes == 1 as libc::c_int {
            if *component_state.offset(comp as isize) != COMP_SILLY as libc::c_int {
                *component_state.offset(comp as isize) = COMP_PATH as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    npath = 0 as libc::c_int;
    nloop = npath;
    nsilly = nloop;
    total_pathsize = 0 as libc::c_int;
    largest_size = -(1 as libc::c_int);
    largest_comp = largest_size;
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        if *component_state.offset(i as isize) == COMP_SILLY as libc::c_int {
            nsilly += 1;
            nsilly;
        } else if *component_state.offset(i as isize) == COMP_PATH as libc::c_int {
            total_pathsize += dsf_size(dsf, i);
            npath = 1 as libc::c_int;
        } else if *component_state.offset(i as isize) == COMP_LOOP as libc::c_int {
            let mut this_size: libc::c_int = 0;
            nloop += 1;
            nloop;
            this_size = dsf_size(dsf, i);
            if this_size > largest_size {
                largest_comp = i;
                largest_size = this_size;
            }
        }
        i += 1;
        i;
    }
    if largest_size < total_pathsize {
        largest_comp = -(1 as libc::c_int);
        largest_size = total_pathsize;
    }
    if nloop > 0 as libc::c_int && nloop + npath > 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*g).num_edges {
            if *((*state).lines).offset(i as isize) as libc::c_int
                == LINE_YES as libc::c_int
            {
                let mut e_1: *mut grid_edge = *((*g).edges).offset(i as isize);
                let mut d1_0: libc::c_int = (*(*e_1).dot1).index;
                let mut comp_0: libc::c_int = dsf_canonify(dsf, d1_0);
                if *component_state.offset(comp_0 as isize) == COMP_PATH as libc::c_int
                    && -(1 as libc::c_int) != largest_comp
                    || *component_state.offset(comp_0 as isize)
                        == COMP_LOOP as libc::c_int && comp_0 != largest_comp
                {
                    *((*state).line_errors).offset(i as isize) = 1 as libc::c_int != 0;
                }
            }
            i += 1;
            i;
        }
    }
    if nloop == 1 as libc::c_int && npath == 0 as libc::c_int
        && nsilly == 0 as libc::c_int
    {
        ret = 1 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i < (*g).num_faces {
            let mut c: libc::c_int = *((*state).clues).offset(i as isize) as libc::c_int;
            if c >= 0 as libc::c_int
                && face_order(state, i, LINE_YES as libc::c_int as libc::c_char) != c
            {
                ret = 0 as libc::c_int != 0;
                break;
            } else {
                i += 1;
                i;
            }
        }
        (*state).exactly_one_loop = 1 as libc::c_int != 0;
    } else {
        ret = 0 as libc::c_int != 0;
        (*state).exactly_one_loop = 0 as libc::c_int != 0;
    }
    sfree(component_state as *mut libc::c_void);
    dsf_free(dsf);
    return ret;
}
unsafe extern "C" fn dline_index_from_dot(
    mut g: *mut grid,
    mut d: *mut grid_dot,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut e: *mut grid_edge = *((*d).edges).offset(i as isize);
    let mut ret: libc::c_int = 0;
    ret = 2 as libc::c_int * (*e).index
        + (if (*e).dot1 == d { 1 as libc::c_int } else { 0 as libc::c_int });
    return ret;
}
unsafe extern "C" fn dline_index_from_face(
    mut g: *mut grid,
    mut f: *mut grid_face,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut e: *mut grid_edge = *((*f).edges).offset(i as isize);
    let mut d: *mut grid_dot = *((*f).dots).offset(i as isize);
    let mut ret: libc::c_int = 0;
    ret = 2 as libc::c_int * (*e).index
        + (if (*e).dot1 == d { 1 as libc::c_int } else { 0 as libc::c_int });
    return ret;
}
unsafe extern "C" fn is_atleastone(
    mut dline_array: *const libc::c_char,
    mut index: libc::c_int,
) -> bool {
    return *dline_array.offset(index as isize) as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0;
}
unsafe extern "C" fn set_atleastone(
    mut dline_array: *mut libc::c_char,
    mut index: libc::c_int,
) -> bool {
    return if *dline_array.offset(index as isize) as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        0 as libc::c_int
    } else {
        let ref mut fresh28 = *dline_array.offset(index as isize);
        *fresh28 = (*fresh28 as libc::c_int | (1 as libc::c_int) << 0 as libc::c_int)
            as libc::c_char;
        1 as libc::c_int
    } != 0;
}
unsafe extern "C" fn is_atmostone(
    mut dline_array: *const libc::c_char,
    mut index: libc::c_int,
) -> bool {
    return *dline_array.offset(index as isize) as libc::c_int
        & (1 as libc::c_int) << 1 as libc::c_int != 0;
}
unsafe extern "C" fn set_atmostone(
    mut dline_array: *mut libc::c_char,
    mut index: libc::c_int,
) -> bool {
    return if *dline_array.offset(index as isize) as libc::c_int
        & (1 as libc::c_int) << 1 as libc::c_int != 0
    {
        0 as libc::c_int
    } else {
        let ref mut fresh30 = *dline_array.offset(index as isize);
        *fresh30 = (*fresh30 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int)
            as libc::c_char;
        1 as libc::c_int
    } != 0;
}
unsafe extern "C" fn array_setall(
    mut array: *mut libc::c_char,
    mut from: libc::c_char,
    mut to: libc::c_char,
    mut len: libc::c_int,
) {
    let mut p: *mut libc::c_char = array;
    let mut p_old: *mut libc::c_char = p;
    let mut len_remaining: libc::c_int = len;
    loop {
        p = memchr(
            p as *const libc::c_void,
            from as libc::c_int,
            len_remaining as libc::c_ulong,
        ) as *mut libc::c_char;
        if p.is_null() {
            break;
        }
        *p = to;
        len_remaining = (len_remaining as libc::c_long
            - p.offset_from(p_old) as libc::c_long) as libc::c_int;
        p_old = p;
    };
}
unsafe extern "C" fn dline_set_opp_atleastone(
    mut sstate: *mut solver_state,
    mut d: *mut grid_dot,
    mut edge: libc::c_int,
) -> bool {
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = (*state).game_grid;
    let mut N: libc::c_int = (*d).order;
    let mut opp: libc::c_int = 0;
    let mut opp2: libc::c_int = 0;
    opp = 0 as libc::c_int;
    while opp < N {
        let mut opp_dline_index: libc::c_int = 0;
        if !(opp == edge || opp == edge + 1 as libc::c_int
            || opp == edge - 1 as libc::c_int)
        {
            if !(opp == 0 as libc::c_int && edge == N - 1 as libc::c_int) {
                if !(opp == N - 1 as libc::c_int && edge == 0 as libc::c_int) {
                    opp2 = opp + 1 as libc::c_int;
                    if opp2 == N {
                        opp2 = 0 as libc::c_int;
                    }
                    if !(*((*state).lines)
                        .offset((**((*d).edges).offset(opp as isize)).index as isize)
                        as libc::c_int != LINE_UNKNOWN as libc::c_int)
                    {
                        if !(*((*state).lines)
                            .offset(
                                (**((*d).edges).offset(opp2 as isize)).index as isize,
                            ) as libc::c_int != LINE_UNKNOWN as libc::c_int)
                        {
                            opp_dline_index = dline_index_from_dot(g, d, opp);
                            return set_atleastone((*sstate).dlines, opp_dline_index);
                        }
                    }
                }
            }
        }
        opp += 1;
        opp;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn face_setall_identical(
    mut sstate: *mut solver_state,
    mut face_index: libc::c_int,
    mut line_new: line_state,
) -> bool {
    let mut retval: bool = 0 as libc::c_int != 0;
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = (*state).game_grid;
    let mut f: *mut grid_face = *((*g).faces).offset(face_index as isize);
    let mut N: libc::c_int = (*f).order;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut can1: libc::c_int = 0;
    let mut can2: libc::c_int = 0;
    let mut inv1: bool = false;
    let mut inv2: bool = false;
    i = 0 as libc::c_int;
    while i < N {
        let mut line1_index: libc::c_int = (**((*f).edges).offset(i as isize)).index;
        if !(*((*state).lines).offset(line1_index as isize) as libc::c_int
            != LINE_UNKNOWN as libc::c_int)
        {
            j = i + 1 as libc::c_int;
            while j < N {
                let mut line2_index: libc::c_int = (**((*f).edges).offset(j as isize))
                    .index;
                if !(*((*state).lines).offset(line2_index as isize) as libc::c_int
                    != LINE_UNKNOWN as libc::c_int)
                {
                    can1 = dsf_canonify_flip((*sstate).linedsf, line1_index, &mut inv1);
                    can2 = dsf_canonify_flip((*sstate).linedsf, line2_index, &mut inv2);
                    if can1 == can2 && inv1 as libc::c_int == inv2 as libc::c_int {
                        solver_set_line(sstate, line1_index, line_new);
                        solver_set_line(sstate, line2_index, line_new);
                    }
                }
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    return retval;
}
unsafe extern "C" fn find_unknowns(
    mut state: *mut game_state,
    mut edge_list: *mut *mut grid_edge,
    mut expected_count: libc::c_int,
    mut e: *mut libc::c_int,
) {
    let mut c: libc::c_int = 0 as libc::c_int;
    while c < expected_count {
        let mut line_index: libc::c_int = (**edge_list).index;
        if *((*state).lines).offset(line_index as isize) as libc::c_int
            == LINE_UNKNOWN as libc::c_int
        {
            *e.offset(c as isize) = line_index;
            c += 1;
            c;
        }
        edge_list = edge_list.offset(1);
        edge_list;
    }
}
unsafe extern "C" fn parity_deductions(
    mut sstate: *mut solver_state,
    mut edge_list: *mut *mut grid_edge,
    mut total_parity: libc::c_int,
    mut unknown_count: libc::c_int,
) -> libc::c_int {
    let mut state: *mut game_state = (*sstate).state;
    let mut diff: libc::c_int = DIFF_MAX as libc::c_int;
    let mut linedsf: *mut DSF = (*sstate).linedsf;
    if unknown_count == 2 as libc::c_int {
        let mut e: [libc::c_int; 2] = [0; 2];
        find_unknowns(state, edge_list, 2 as libc::c_int, e.as_mut_ptr());
        if merge_lines(
            sstate,
            e[0 as libc::c_int as usize],
            e[1 as libc::c_int as usize],
            total_parity != 0,
        ) {
            diff = if diff < DIFF_HARD as libc::c_int {
                diff
            } else {
                DIFF_HARD as libc::c_int
            };
        }
    } else if unknown_count == 3 as libc::c_int {
        let mut e_0: [libc::c_int; 3] = [0; 3];
        let mut can: [libc::c_int; 3] = [0; 3];
        let mut inv: [bool; 3] = [false; 3];
        find_unknowns(state, edge_list, 3 as libc::c_int, e_0.as_mut_ptr());
        can[0 as libc::c_int
            as usize] = dsf_canonify_flip(
            linedsf,
            e_0[0 as libc::c_int as usize],
            inv.as_mut_ptr(),
        );
        can[1 as libc::c_int
            as usize] = dsf_canonify_flip(
            linedsf,
            e_0[1 as libc::c_int as usize],
            inv.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        can[2 as libc::c_int
            as usize] = dsf_canonify_flip(
            linedsf,
            e_0[2 as libc::c_int as usize],
            inv.as_mut_ptr().offset(2 as libc::c_int as isize),
        );
        if can[0 as libc::c_int as usize] == can[1 as libc::c_int as usize] {
            if solver_set_line(
                sstate,
                e_0[2 as libc::c_int as usize],
                (if total_parity ^ inv[0 as libc::c_int as usize] as libc::c_int
                    ^ inv[1 as libc::c_int as usize] as libc::c_int != 0
                {
                    LINE_YES as libc::c_int
                } else {
                    LINE_NO as libc::c_int
                }) as line_state,
            ) {
                diff = if diff < DIFF_EASY as libc::c_int {
                    diff
                } else {
                    DIFF_EASY as libc::c_int
                };
            }
        }
        if can[0 as libc::c_int as usize] == can[2 as libc::c_int as usize] {
            if solver_set_line(
                sstate,
                e_0[1 as libc::c_int as usize],
                (if total_parity ^ inv[0 as libc::c_int as usize] as libc::c_int
                    ^ inv[2 as libc::c_int as usize] as libc::c_int != 0
                {
                    LINE_YES as libc::c_int
                } else {
                    LINE_NO as libc::c_int
                }) as line_state,
            ) {
                diff = if diff < DIFF_EASY as libc::c_int {
                    diff
                } else {
                    DIFF_EASY as libc::c_int
                };
            }
        }
        if can[1 as libc::c_int as usize] == can[2 as libc::c_int as usize] {
            if solver_set_line(
                sstate,
                e_0[0 as libc::c_int as usize],
                (if total_parity ^ inv[1 as libc::c_int as usize] as libc::c_int
                    ^ inv[2 as libc::c_int as usize] as libc::c_int != 0
                {
                    LINE_YES as libc::c_int
                } else {
                    LINE_NO as libc::c_int
                }) as line_state,
            ) {
                diff = if diff < DIFF_EASY as libc::c_int {
                    diff
                } else {
                    DIFF_EASY as libc::c_int
                };
            }
        }
    } else if unknown_count == 4 as libc::c_int {
        let mut e_1: [libc::c_int; 4] = [0; 4];
        let mut can_0: [libc::c_int; 4] = [0; 4];
        let mut inv_0: [bool; 4] = [false; 4];
        find_unknowns(state, edge_list, 4 as libc::c_int, e_1.as_mut_ptr());
        can_0[0 as libc::c_int
            as usize] = dsf_canonify_flip(
            linedsf,
            e_1[0 as libc::c_int as usize],
            inv_0.as_mut_ptr(),
        );
        can_0[1 as libc::c_int
            as usize] = dsf_canonify_flip(
            linedsf,
            e_1[1 as libc::c_int as usize],
            inv_0.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        can_0[2 as libc::c_int
            as usize] = dsf_canonify_flip(
            linedsf,
            e_1[2 as libc::c_int as usize],
            inv_0.as_mut_ptr().offset(2 as libc::c_int as isize),
        );
        can_0[3 as libc::c_int
            as usize] = dsf_canonify_flip(
            linedsf,
            e_1[3 as libc::c_int as usize],
            inv_0.as_mut_ptr().offset(3 as libc::c_int as isize),
        );
        if can_0[0 as libc::c_int as usize] == can_0[1 as libc::c_int as usize] {
            if merge_lines(
                sstate,
                e_1[2 as libc::c_int as usize],
                e_1[3 as libc::c_int as usize],
                total_parity ^ inv_0[0 as libc::c_int as usize] as libc::c_int
                    ^ inv_0[1 as libc::c_int as usize] as libc::c_int != 0,
            ) {
                diff = if diff < DIFF_HARD as libc::c_int {
                    diff
                } else {
                    DIFF_HARD as libc::c_int
                };
            }
        } else if can_0[0 as libc::c_int as usize] == can_0[2 as libc::c_int as usize] {
            if merge_lines(
                sstate,
                e_1[1 as libc::c_int as usize],
                e_1[3 as libc::c_int as usize],
                total_parity ^ inv_0[0 as libc::c_int as usize] as libc::c_int
                    ^ inv_0[2 as libc::c_int as usize] as libc::c_int != 0,
            ) {
                diff = if diff < DIFF_HARD as libc::c_int {
                    diff
                } else {
                    DIFF_HARD as libc::c_int
                };
            }
        } else if can_0[0 as libc::c_int as usize] == can_0[3 as libc::c_int as usize] {
            if merge_lines(
                sstate,
                e_1[1 as libc::c_int as usize],
                e_1[2 as libc::c_int as usize],
                total_parity ^ inv_0[0 as libc::c_int as usize] as libc::c_int
                    ^ inv_0[3 as libc::c_int as usize] as libc::c_int != 0,
            ) {
                diff = if diff < DIFF_HARD as libc::c_int {
                    diff
                } else {
                    DIFF_HARD as libc::c_int
                };
            }
        } else if can_0[1 as libc::c_int as usize] == can_0[2 as libc::c_int as usize] {
            if merge_lines(
                sstate,
                e_1[0 as libc::c_int as usize],
                e_1[3 as libc::c_int as usize],
                total_parity ^ inv_0[1 as libc::c_int as usize] as libc::c_int
                    ^ inv_0[2 as libc::c_int as usize] as libc::c_int != 0,
            ) {
                diff = if diff < DIFF_HARD as libc::c_int {
                    diff
                } else {
                    DIFF_HARD as libc::c_int
                };
            }
        } else if can_0[1 as libc::c_int as usize] == can_0[3 as libc::c_int as usize] {
            if merge_lines(
                sstate,
                e_1[0 as libc::c_int as usize],
                e_1[2 as libc::c_int as usize],
                total_parity ^ inv_0[1 as libc::c_int as usize] as libc::c_int
                    ^ inv_0[3 as libc::c_int as usize] as libc::c_int != 0,
            ) {
                diff = if diff < DIFF_HARD as libc::c_int {
                    diff
                } else {
                    DIFF_HARD as libc::c_int
                };
            }
        } else if can_0[2 as libc::c_int as usize] == can_0[3 as libc::c_int as usize] {
            if merge_lines(
                sstate,
                e_1[0 as libc::c_int as usize],
                e_1[1 as libc::c_int as usize],
                total_parity ^ inv_0[2 as libc::c_int as usize] as libc::c_int
                    ^ inv_0[3 as libc::c_int as usize] as libc::c_int != 0,
            ) {
                diff = if diff < DIFF_HARD as libc::c_int {
                    diff
                } else {
                    DIFF_HARD as libc::c_int
                };
            }
        }
    }
    return diff;
}
unsafe extern "C" fn trivial_deductions(mut sstate: *mut solver_state) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut current_yes: libc::c_int = 0;
    let mut current_no: libc::c_int = 0;
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = (*state).game_grid;
    let mut diff: libc::c_int = DIFF_MAX as libc::c_int;
    let mut current_block_35: u64;
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
        if !*((*sstate).face_solved).offset(i as isize) {
            current_yes = *((*sstate).face_yes_count).offset(i as isize) as libc::c_int;
            current_no = *((*sstate).face_no_count).offset(i as isize) as libc::c_int;
            if current_yes + current_no == (*f).order {
                *((*sstate).face_solved).offset(i as isize) = 1 as libc::c_int != 0;
            } else if !((*((*state).clues).offset(i as isize) as libc::c_int)
                < 0 as libc::c_int)
            {
                if (*((*state).clues).offset(i as isize) as libc::c_int) < current_yes {
                    (*sstate).solver_status = SOLVER_MISTAKE;
                    return DIFF_EASY as libc::c_int;
                }
                if *((*state).clues).offset(i as isize) as libc::c_int == current_yes {
                    if face_setall(
                        sstate,
                        i,
                        LINE_UNKNOWN as libc::c_int as libc::c_char,
                        LINE_NO as libc::c_int as libc::c_char,
                    ) {
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                    *((*sstate).face_solved).offset(i as isize) = 1 as libc::c_int != 0;
                } else {
                    if ((*f).order - *((*state).clues).offset(i as isize) as libc::c_int)
                        < current_no
                    {
                        (*sstate).solver_status = SOLVER_MISTAKE;
                        return DIFF_EASY as libc::c_int;
                    }
                    if (*f).order - *((*state).clues).offset(i as isize) as libc::c_int
                        == current_no
                    {
                        if face_setall(
                            sstate,
                            i,
                            LINE_UNKNOWN as libc::c_int as libc::c_char,
                            LINE_YES as libc::c_int as libc::c_char,
                        ) {
                            diff = if diff < DIFF_EASY as libc::c_int {
                                diff
                            } else {
                                DIFF_EASY as libc::c_int
                            };
                        }
                        *((*sstate).face_solved)
                            .offset(i as isize) = 1 as libc::c_int != 0;
                    } else if (*f).order
                        - *((*state).clues).offset(i as isize) as libc::c_int
                        == current_no + 1 as libc::c_int
                        && (*f).order - current_yes - current_no > 2 as libc::c_int
                    {
                        let mut j: libc::c_int = 0;
                        let mut k: libc::c_int = 0;
                        let mut e1: libc::c_int = 0;
                        let mut e2: libc::c_int = 0;
                        let mut e: libc::c_int = 0;
                        let mut d: libc::c_int = 0;
                        j = 0 as libc::c_int;
                        's_114: loop {
                            if !(j < (*f).order) {
                                current_block_35 = 8258075665625361029;
                                break;
                            }
                            e1 = (**((*f).edges).offset(j as isize)).index;
                            e2 = (**((*f).edges)
                                .offset(
                                    (if (j + 1 as libc::c_int) < (*f).order {
                                        j + 1 as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    }) as isize,
                                ))
                                .index;
                            if (**((*g).edges).offset(e1 as isize)).dot1
                                == (**((*g).edges).offset(e2 as isize)).dot1
                                || (**((*g).edges).offset(e1 as isize)).dot1
                                    == (**((*g).edges).offset(e2 as isize)).dot2
                            {
                                d = (*(**((*g).edges).offset(e1 as isize)).dot1).index;
                            } else {
                                if (**((*g).edges).offset(e1 as isize)).dot2
                                    == (**((*g).edges).offset(e2 as isize)).dot1
                                    || (**((*g).edges).offset(e1 as isize)).dot2
                                        == (**((*g).edges).offset(e2 as isize)).dot2
                                {} else {
                                    __assert_fail(
                                        b"g->edges[e1]->dot2 == g->edges[e2]->dot1 || g->edges[e1]->dot2 == g->edges[e2]->dot2\0"
                                            as *const u8 as *const libc::c_char,
                                        b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                                        2258 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<
                                            &[u8; 39],
                                            &[libc::c_char; 39],
                                        >(b"int trivial_deductions(solver_state *)\0"))
                                            .as_ptr(),
                                    );
                                }
                                'c_20807: {
                                    if (**((*g).edges).offset(e1 as isize)).dot2
                                        == (**((*g).edges).offset(e2 as isize)).dot1
                                        || (**((*g).edges).offset(e1 as isize)).dot2
                                            == (**((*g).edges).offset(e2 as isize)).dot2
                                    {} else {
                                        __assert_fail(
                                            b"g->edges[e1]->dot2 == g->edges[e2]->dot1 || g->edges[e1]->dot2 == g->edges[e2]->dot2\0"
                                                as *const u8 as *const libc::c_char,
                                            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                                            2258 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 39],
                                                &[libc::c_char; 39],
                                            >(b"int trivial_deductions(solver_state *)\0"))
                                                .as_ptr(),
                                        );
                                    }
                                };
                                d = (*(**((*g).edges).offset(e1 as isize)).dot2).index;
                            }
                            if *((*state).lines).offset(e1 as isize) as libc::c_int
                                == LINE_UNKNOWN as libc::c_int
                                && *((*state).lines).offset(e2 as isize) as libc::c_int
                                    == LINE_UNKNOWN as libc::c_int
                            {
                                k = 0 as libc::c_int;
                                while k < (**((*g).dots).offset(d as isize)).order {
                                    let mut e_0: libc::c_int = (**((**((*g).dots)
                                        .offset(d as isize))
                                        .edges)
                                        .offset(k as isize))
                                        .index;
                                    if *((*state).lines).offset(e_0 as isize) as libc::c_int
                                        == LINE_YES as libc::c_int
                                    {
                                        current_block_35 = 1354089132341423732;
                                        break 's_114;
                                    }
                                    k += 1;
                                    k;
                                }
                            }
                            j += 1;
                            j;
                        }
                        match current_block_35 {
                            8258075665625361029 => {}
                            _ => {
                                j = 0 as libc::c_int;
                                while j < (*f).order {
                                    e = (**((*f).edges).offset(j as isize)).index;
                                    if *((*state).lines).offset(e as isize) as libc::c_int
                                        == LINE_UNKNOWN as libc::c_int && e != e1 && e != e2
                                    {
                                        let mut r: bool = solver_set_line(sstate, e, LINE_YES);
                                        if r {} else {
                                            __assert_fail(
                                                b"r\0" as *const u8 as *const libc::c_char,
                                                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                                                2282 as libc::c_int as libc::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 39],
                                                    &[libc::c_char; 39],
                                                >(b"int trivial_deductions(solver_state *)\0"))
                                                    .as_ptr(),
                                            );
                                        }
                                        'c_20601: {
                                            if r {} else {
                                                __assert_fail(
                                                    b"r\0" as *const u8 as *const libc::c_char,
                                                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                                                    2282 as libc::c_int as libc::c_uint,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 39],
                                                        &[libc::c_char; 39],
                                                    >(b"int trivial_deductions(solver_state *)\0"))
                                                        .as_ptr(),
                                                );
                                            }
                                        };
                                        diff = if diff < DIFF_EASY as libc::c_int {
                                            diff
                                        } else {
                                            DIFF_EASY as libc::c_int
                                        };
                                    }
                                    j += 1;
                                    j;
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut d_0: *mut grid_dot = *((*g).dots).offset(i as isize);
        let mut yes: libc::c_int = 0;
        let mut no: libc::c_int = 0;
        let mut unknown: libc::c_int = 0;
        if !*((*sstate).dot_solved).offset(i as isize) {
            yes = *((*sstate).dot_yes_count).offset(i as isize) as libc::c_int;
            no = *((*sstate).dot_no_count).offset(i as isize) as libc::c_int;
            unknown = (*d_0).order - yes - no;
            if yes == 0 as libc::c_int {
                if unknown == 0 as libc::c_int {
                    *((*sstate).dot_solved).offset(i as isize) = 1 as libc::c_int != 0;
                } else if unknown == 1 as libc::c_int {
                    dot_setall(
                        sstate,
                        i,
                        LINE_UNKNOWN as libc::c_int as libc::c_char,
                        LINE_NO as libc::c_int as libc::c_char,
                    );
                    diff = if diff < DIFF_EASY as libc::c_int {
                        diff
                    } else {
                        DIFF_EASY as libc::c_int
                    };
                    *((*sstate).dot_solved).offset(i as isize) = 1 as libc::c_int != 0;
                }
            } else if yes == 1 as libc::c_int {
                if unknown == 0 as libc::c_int {
                    (*sstate).solver_status = SOLVER_MISTAKE;
                    return DIFF_EASY as libc::c_int;
                } else if unknown == 1 as libc::c_int {
                    dot_setall(
                        sstate,
                        i,
                        LINE_UNKNOWN as libc::c_int as libc::c_char,
                        LINE_YES as libc::c_int as libc::c_char,
                    );
                    diff = if diff < DIFF_EASY as libc::c_int {
                        diff
                    } else {
                        DIFF_EASY as libc::c_int
                    };
                }
            } else if yes == 2 as libc::c_int {
                if unknown > 0 as libc::c_int {
                    dot_setall(
                        sstate,
                        i,
                        LINE_UNKNOWN as libc::c_int as libc::c_char,
                        LINE_NO as libc::c_int as libc::c_char,
                    );
                    diff = if diff < DIFF_EASY as libc::c_int {
                        diff
                    } else {
                        DIFF_EASY as libc::c_int
                    };
                }
                *((*sstate).dot_solved).offset(i as isize) = 1 as libc::c_int != 0;
            } else {
                (*sstate).solver_status = SOLVER_MISTAKE;
                return DIFF_EASY as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    return diff;
}
unsafe extern "C" fn dline_deductions(mut sstate: *mut solver_state) -> libc::c_int {
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = (*state).game_grid;
    let mut dlines: *mut libc::c_char = (*sstate).dlines;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = DIFF_MAX as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut maxs: [[libc::c_int; 14]; 14] = [[0; 14]; 14];
        let mut mins: [[libc::c_int; 14]; 14] = [[0; 14]; 14];
        let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut N: libc::c_int = (*f).order;
        let mut j: libc::c_int = 0;
        let mut m: libc::c_int = 0;
        let mut clue: libc::c_int = *((*state).clues).offset(i as isize) as libc::c_int;
        if N <= 14 as libc::c_int {} else {
            __assert_fail(
                b"N <= MAX_FACE_SIZE\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                2394 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"int dline_deductions(solver_state *)\0"))
                    .as_ptr(),
            );
        }
        'c_20067: {
            if N <= 14 as libc::c_int {} else {
                __assert_fail(
                    b"N <= MAX_FACE_SIZE\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                    2394 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 37],
                        &[libc::c_char; 37],
                    >(b"int dline_deductions(solver_state *)\0"))
                        .as_ptr(),
                );
            }
        };
        if !*((*sstate).face_solved).offset(i as isize) {
            if !(clue < 0 as libc::c_int) {
                j = 0 as libc::c_int;
                while j < N {
                    let mut edge_index: libc::c_int = (**((*f).edges).offset(j as isize))
                        .index;
                    let mut dline_index: libc::c_int = 0;
                    let mut line1: line_state = *((*state).lines)
                        .offset(edge_index as isize) as line_state;
                    let mut line2: line_state = LINE_YES;
                    let mut tmp: libc::c_int = 0;
                    let mut k: libc::c_int = j + 1 as libc::c_int;
                    if k >= N {
                        k = 0 as libc::c_int;
                    }
                    maxs[j
                        as usize][k
                        as usize] = if line1 as libc::c_uint
                        == LINE_NO as libc::c_int as libc::c_uint
                    {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    mins[j
                        as usize][k
                        as usize] = if line1 as libc::c_uint
                        == LINE_YES as libc::c_int as libc::c_uint
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    dline_index = dline_index_from_face(g, f, k);
                    edge_index = (**((*f).edges).offset(k as isize)).index;
                    line2 = *((*state).lines).offset(edge_index as isize) as line_state;
                    k += 1;
                    k;
                    if k >= N {
                        k = 0 as libc::c_int;
                    }
                    tmp = 2 as libc::c_int;
                    if line1 as libc::c_uint == LINE_NO as libc::c_int as libc::c_uint {
                        tmp -= 1;
                        tmp;
                    }
                    if line2 as libc::c_uint == LINE_NO as libc::c_int as libc::c_uint {
                        tmp -= 1;
                        tmp;
                    }
                    if tmp == 2 as libc::c_int
                        && is_atmostone(dlines, dline_index) as libc::c_int != 0
                    {
                        tmp = 1 as libc::c_int;
                    }
                    maxs[j as usize][k as usize] = tmp;
                    tmp = 0 as libc::c_int;
                    if line1 as libc::c_uint == LINE_YES as libc::c_int as libc::c_uint {
                        tmp += 1;
                        tmp;
                    }
                    if line2 as libc::c_uint == LINE_YES as libc::c_int as libc::c_uint {
                        tmp += 1;
                        tmp;
                    }
                    if tmp == 0 as libc::c_int
                        && is_atleastone(dlines, dline_index) as libc::c_int != 0
                    {
                        tmp = 1 as libc::c_int;
                    }
                    mins[j as usize][k as usize] = tmp;
                    j += 1;
                    j;
                }
                m = 3 as libc::c_int;
                while m < N {
                    j = 0 as libc::c_int;
                    while j < N {
                        let mut k_0: libc::c_int = j + m;
                        let mut u: libc::c_int = j + 1 as libc::c_int;
                        let mut v: libc::c_int = j + 2 as libc::c_int;
                        let mut tmp_0: libc::c_int = 0;
                        if k_0 >= N {
                            k_0 -= N;
                        }
                        if u >= N {
                            u -= N;
                        }
                        if v >= N {
                            v -= N;
                        }
                        maxs[j
                            as usize][k_0
                            as usize] = maxs[j as usize][u as usize]
                            + maxs[u as usize][k_0 as usize];
                        mins[j
                            as usize][k_0
                            as usize] = mins[j as usize][u as usize]
                            + mins[u as usize][k_0 as usize];
                        tmp_0 = maxs[j as usize][v as usize]
                            + maxs[v as usize][k_0 as usize];
                        maxs[j
                            as usize][k_0
                            as usize] = if maxs[j as usize][k_0 as usize] < tmp_0 {
                            maxs[j as usize][k_0 as usize]
                        } else {
                            tmp_0
                        };
                        tmp_0 = mins[j as usize][v as usize]
                            + mins[v as usize][k_0 as usize];
                        mins[j
                            as usize][k_0
                            as usize] = if mins[j as usize][k_0 as usize] > tmp_0 {
                            mins[j as usize][k_0 as usize]
                        } else {
                            tmp_0
                        };
                        j += 1;
                        j;
                    }
                    m += 1;
                    m;
                }
                j = 0 as libc::c_int;
                while j < N {
                    let mut k_1: libc::c_int = 0;
                    let mut e: *mut grid_edge = *((*f).edges).offset(j as isize);
                    let mut line_index: libc::c_int = (*e).index;
                    let mut dline_index_0: libc::c_int = 0;
                    if !(*((*state).lines).offset(line_index as isize) as libc::c_int
                        != LINE_UNKNOWN as libc::c_int)
                    {
                        k_1 = j + 1 as libc::c_int;
                        if k_1 >= N {
                            k_1 = 0 as libc::c_int;
                        }
                        if mins[k_1 as usize][j as usize] > clue {
                            (*sstate).solver_status = SOLVER_MISTAKE;
                            return DIFF_EASY as libc::c_int;
                        }
                        if mins[k_1 as usize][j as usize] == clue {
                            solver_set_line(sstate, line_index, LINE_NO);
                            diff = if diff < DIFF_EASY as libc::c_int {
                                diff
                            } else {
                                DIFF_EASY as libc::c_int
                            };
                        }
                        if maxs[k_1 as usize][j as usize] < clue - 1 as libc::c_int {
                            (*sstate).solver_status = SOLVER_MISTAKE;
                            return DIFF_EASY as libc::c_int;
                        }
                        if maxs[k_1 as usize][j as usize] == clue - 1 as libc::c_int {
                            solver_set_line(sstate, line_index, LINE_YES);
                            diff = if diff < DIFF_EASY as libc::c_int {
                                diff
                            } else {
                                DIFF_EASY as libc::c_int
                            };
                        }
                        if (*sstate).diff >= DIFF_TRICKY as libc::c_int {
                            e = *((*f).edges).offset(k_1 as isize);
                            if !(*((*state).lines).offset((*e).index as isize)
                                as libc::c_int != LINE_UNKNOWN as libc::c_int)
                            {
                                dline_index_0 = dline_index_from_face(g, f, k_1);
                                k_1 += 1;
                                k_1;
                                if k_1 >= N {
                                    k_1 = 0 as libc::c_int;
                                }
                                if mins[k_1 as usize][j as usize] > clue - 2 as libc::c_int
                                {
                                    if set_atmostone(dlines, dline_index_0) {
                                        diff = if diff < DIFF_NORMAL as libc::c_int {
                                            diff
                                        } else {
                                            DIFF_NORMAL as libc::c_int
                                        };
                                    }
                                }
                                if maxs[k_1 as usize][j as usize] < clue {
                                    if set_atleastone(dlines, dline_index_0) {
                                        diff = if diff < DIFF_NORMAL as libc::c_int {
                                            diff
                                        } else {
                                            DIFF_NORMAL as libc::c_int
                                        };
                                    }
                                }
                            }
                        }
                    }
                    j += 1;
                    j;
                }
            }
        }
        i += 1;
        i;
    }
    if diff < DIFF_NORMAL as libc::c_int {
        return diff;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut d: *mut grid_dot = *((*g).dots).offset(i as isize);
        let mut N_0: libc::c_int = (*d).order;
        let mut yes: libc::c_int = 0;
        let mut no: libc::c_int = 0;
        let mut unknown: libc::c_int = 0;
        let mut j_0: libc::c_int = 0;
        if !*((*sstate).dot_solved).offset(i as isize) {
            yes = *((*sstate).dot_yes_count).offset(i as isize) as libc::c_int;
            no = *((*sstate).dot_no_count).offset(i as isize) as libc::c_int;
            unknown = N_0 - yes - no;
            j_0 = 0 as libc::c_int;
            while j_0 < N_0 {
                let mut k_2: libc::c_int = 0;
                let mut dline_index_1: libc::c_int = 0;
                let mut line1_index: libc::c_int = 0;
                let mut line2_index: libc::c_int = 0;
                let mut line1_0: line_state = LINE_YES;
                let mut line2_0: line_state = LINE_YES;
                k_2 = j_0 + 1 as libc::c_int;
                if k_2 >= N_0 {
                    k_2 = 0 as libc::c_int;
                }
                dline_index_1 = dline_index_from_dot(g, d, j_0);
                line1_index = (**((*d).edges).offset(j_0 as isize)).index;
                line2_index = (**((*d).edges).offset(k_2 as isize)).index;
                line1_0 = *((*state).lines).offset(line1_index as isize) as line_state;
                line2_0 = *((*state).lines).offset(line2_index as isize) as line_state;
                if line1_0 as libc::c_uint == LINE_NO as libc::c_int as libc::c_uint
                    || line2_0 as libc::c_uint == LINE_NO as libc::c_int as libc::c_uint
                {
                    if set_atmostone(dlines, dline_index_1) {
                        diff = if diff < DIFF_NORMAL as libc::c_int {
                            diff
                        } else {
                            DIFF_NORMAL as libc::c_int
                        };
                    }
                }
                if line1_0 as libc::c_uint == LINE_YES as libc::c_int as libc::c_uint
                    || line2_0 as libc::c_uint == LINE_YES as libc::c_int as libc::c_uint
                {
                    if set_atleastone(dlines, dline_index_1) {
                        diff = if diff < DIFF_NORMAL as libc::c_int {
                            diff
                        } else {
                            DIFF_NORMAL as libc::c_int
                        };
                    }
                }
                if is_atmostone(dlines, dline_index_1) {
                    if line1_0 as libc::c_uint == LINE_YES as libc::c_int as libc::c_uint
                        && line2_0 as libc::c_uint
                            == LINE_UNKNOWN as libc::c_int as libc::c_uint
                    {
                        solver_set_line(sstate, line2_index, LINE_NO);
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                    if line2_0 as libc::c_uint == LINE_YES as libc::c_int as libc::c_uint
                        && line1_0 as libc::c_uint
                            == LINE_UNKNOWN as libc::c_int as libc::c_uint
                    {
                        solver_set_line(sstate, line1_index, LINE_NO);
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                }
                if is_atleastone(dlines, dline_index_1) {
                    if line1_0 as libc::c_uint == LINE_NO as libc::c_int as libc::c_uint
                        && line2_0 as libc::c_uint
                            == LINE_UNKNOWN as libc::c_int as libc::c_uint
                    {
                        solver_set_line(sstate, line2_index, LINE_YES);
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                    if line2_0 as libc::c_uint == LINE_NO as libc::c_int as libc::c_uint
                        && line1_0 as libc::c_uint
                            == LINE_UNKNOWN as libc::c_int as libc::c_uint
                    {
                        solver_set_line(sstate, line1_index, LINE_YES);
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                }
                if !(line1_0 as libc::c_uint
                    != LINE_UNKNOWN as libc::c_int as libc::c_uint
                    || line2_0 as libc::c_uint
                        != LINE_UNKNOWN as libc::c_int as libc::c_uint)
                {
                    if yes == 0 as libc::c_int && unknown == 2 as libc::c_int {
                        if is_atmostone(dlines, dline_index_1) {
                            solver_set_line(sstate, line1_index, LINE_NO);
                            solver_set_line(sstate, line2_index, LINE_NO);
                            diff = if diff < DIFF_EASY as libc::c_int {
                                diff
                            } else {
                                DIFF_EASY as libc::c_int
                            };
                        }
                        if is_atleastone(dlines, dline_index_1) {
                            solver_set_line(sstate, line1_index, LINE_YES);
                            solver_set_line(sstate, line2_index, LINE_YES);
                            diff = if diff < DIFF_EASY as libc::c_int {
                                diff
                            } else {
                                DIFF_EASY as libc::c_int
                            };
                        }
                    }
                    if yes == 1 as libc::c_int {
                        if set_atmostone(dlines, dline_index_1) {
                            diff = if diff < DIFF_NORMAL as libc::c_int {
                                diff
                            } else {
                                DIFF_NORMAL as libc::c_int
                            };
                        }
                        if unknown == 2 as libc::c_int {
                            if set_atleastone(dlines, dline_index_1) {
                                diff = if diff < DIFF_NORMAL as libc::c_int {
                                    diff
                                } else {
                                    DIFF_NORMAL as libc::c_int
                                };
                            }
                        }
                    }
                    if (*sstate).diff >= DIFF_TRICKY as libc::c_int {
                        if is_atleastone(dlines, dline_index_1) {
                            let mut opp: libc::c_int = 0;
                            opp = 0 as libc::c_int;
                            while opp < N_0 {
                                let mut opp_dline_index: libc::c_int = 0;
                                if !(opp == j_0 || opp == j_0 + 1 as libc::c_int
                                    || opp == j_0 - 1 as libc::c_int)
                                {
                                    if !(j_0 == 0 as libc::c_int
                                        && opp == N_0 - 1 as libc::c_int)
                                    {
                                        if !(j_0 == N_0 - 1 as libc::c_int
                                            && opp == 0 as libc::c_int)
                                        {
                                            opp_dline_index = dline_index_from_dot(g, d, opp);
                                            if set_atmostone(dlines, opp_dline_index) {
                                                diff = if diff < DIFF_NORMAL as libc::c_int {
                                                    diff
                                                } else {
                                                    DIFF_NORMAL as libc::c_int
                                                };
                                            }
                                        }
                                    }
                                }
                                opp += 1;
                                opp;
                            }
                            if yes == 0 as libc::c_int
                                && is_atmostone(dlines, dline_index_1) as libc::c_int != 0
                            {
                                if unknown == 3 as libc::c_int {
                                    opp = 0 as libc::c_int;
                                    while opp < N_0 {
                                        let mut opp_index: libc::c_int = 0;
                                        if !(opp == j_0 || opp == k_2) {
                                            opp_index = (**((*d).edges).offset(opp as isize)).index;
                                            if *((*state).lines).offset(opp_index as isize)
                                                as libc::c_int == LINE_UNKNOWN as libc::c_int
                                            {
                                                solver_set_line(sstate, opp_index, LINE_YES);
                                                diff = if diff < DIFF_EASY as libc::c_int {
                                                    diff
                                                } else {
                                                    DIFF_EASY as libc::c_int
                                                };
                                            }
                                        }
                                        opp += 1;
                                        opp;
                                    }
                                } else if unknown == 4 as libc::c_int {
                                    if dline_set_opp_atleastone(sstate, d, j_0) {
                                        diff = if diff < DIFF_NORMAL as libc::c_int {
                                            diff
                                        } else {
                                            DIFF_NORMAL as libc::c_int
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
                j_0 += 1;
                j_0;
            }
        }
        i += 1;
        i;
    }
    return diff;
}
unsafe extern "C" fn linedsf_deductions(mut sstate: *mut solver_state) -> libc::c_int {
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = (*state).game_grid;
    let mut dlines: *mut libc::c_char = (*sstate).dlines;
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_int = DIFF_MAX as libc::c_int;
    let mut diff_tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut N: libc::c_int = 0;
        let mut yes: libc::c_int = 0;
        let mut no: libc::c_int = 0;
        let mut unknown: libc::c_int = 0;
        let mut clue: libc::c_int = 0;
        if !*((*sstate).face_solved).offset(i as isize) {
            clue = *((*state).clues).offset(i as isize) as libc::c_int;
            if !(clue < 0 as libc::c_int) {
                N = (**((*g).faces).offset(i as isize)).order;
                yes = *((*sstate).face_yes_count).offset(i as isize) as libc::c_int;
                if yes + 1 as libc::c_int == clue {
                    if face_setall_identical(sstate, i, LINE_NO) {
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                }
                no = *((*sstate).face_no_count).offset(i as isize) as libc::c_int;
                if no + 1 as libc::c_int == N - clue {
                    if face_setall_identical(sstate, i, LINE_YES) {
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                }
                yes = *((*sstate).face_yes_count).offset(i as isize) as libc::c_int;
                unknown = N - no - yes;
                diff_tmp = parity_deductions(
                    sstate,
                    (**((*g).faces).offset(i as isize)).edges,
                    (clue - yes) % 2 as libc::c_int,
                    unknown,
                );
                diff = if diff < diff_tmp { diff } else { diff_tmp };
            }
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut d: *mut grid_dot = *((*g).dots).offset(i as isize);
        let mut N_0: libc::c_int = (*d).order;
        let mut j: libc::c_int = 0;
        let mut yes_0: libc::c_int = 0;
        let mut no_0: libc::c_int = 0;
        let mut unknown_0: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < N_0 {
            let mut dline_index: libc::c_int = dline_index_from_dot(g, d, j);
            let mut line1_index: libc::c_int = 0;
            let mut line2_index: libc::c_int = 0;
            let mut can1: libc::c_int = 0;
            let mut can2: libc::c_int = 0;
            let mut inv1: bool = false;
            let mut inv2: bool = false;
            let mut j2: libc::c_int = 0;
            line1_index = (**((*d).edges).offset(j as isize)).index;
            if !(*((*state).lines).offset(line1_index as isize) as libc::c_int
                != LINE_UNKNOWN as libc::c_int)
            {
                j2 = j + 1 as libc::c_int;
                if j2 == N_0 {
                    j2 = 0 as libc::c_int;
                }
                line2_index = (**((*d).edges).offset(j2 as isize)).index;
                if !(*((*state).lines).offset(line2_index as isize) as libc::c_int
                    != LINE_UNKNOWN as libc::c_int)
                {
                    can1 = dsf_canonify_flip((*sstate).linedsf, line1_index, &mut inv1);
                    can2 = dsf_canonify_flip((*sstate).linedsf, line2_index, &mut inv2);
                    if can1 == can2 && inv1 as libc::c_int != inv2 as libc::c_int {
                        if set_atmostone(dlines, dline_index) {
                            diff = if diff < DIFF_NORMAL as libc::c_int {
                                diff
                            } else {
                                DIFF_NORMAL as libc::c_int
                            };
                        }
                        if set_atleastone(dlines, dline_index) {
                            diff = if diff < DIFF_NORMAL as libc::c_int {
                                diff
                            } else {
                                DIFF_NORMAL as libc::c_int
                            };
                        }
                    } else if is_atmostone(dlines, dline_index) as libc::c_int != 0
                        && is_atleastone(dlines, dline_index) as libc::c_int != 0
                    {
                        if merge_lines(
                            sstate,
                            line1_index,
                            line2_index,
                            1 as libc::c_int != 0,
                        ) {
                            diff = if diff < DIFF_HARD as libc::c_int {
                                diff
                            } else {
                                DIFF_HARD as libc::c_int
                            };
                        }
                    }
                }
            }
            j += 1;
            j;
        }
        yes_0 = *((*sstate).dot_yes_count).offset(i as isize) as libc::c_int;
        no_0 = *((*sstate).dot_no_count).offset(i as isize) as libc::c_int;
        unknown_0 = N_0 - yes_0 - no_0;
        diff_tmp = parity_deductions(
            sstate,
            (*d).edges,
            yes_0 % 2 as libc::c_int,
            unknown_0,
        );
        diff = if diff < diff_tmp { diff } else { diff_tmp };
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut can: libc::c_int = 0;
        let mut inv: bool = false;
        let mut s: line_state = LINE_YES;
        can = dsf_canonify_flip((*sstate).linedsf, i, &mut inv);
        if !(can == i) {
            s = *((*(*sstate).state).lines).offset(can as isize) as line_state;
            if s as libc::c_uint != LINE_UNKNOWN as libc::c_int as libc::c_uint {
                if solver_set_line(
                    sstate,
                    i,
                    (if inv as libc::c_int != 0 {
                        (2 as libc::c_int as libc::c_uint)
                            .wrapping_sub(s as libc::c_uint)
                    } else {
                        s as libc::c_uint
                    }) as line_state,
                ) {
                    diff = if diff < DIFF_EASY as libc::c_int {
                        diff
                    } else {
                        DIFF_EASY as libc::c_int
                    };
                }
            } else {
                s = *((*(*sstate).state).lines).offset(i as isize) as line_state;
                if s as libc::c_uint != LINE_UNKNOWN as libc::c_int as libc::c_uint {
                    if solver_set_line(
                        sstate,
                        can,
                        (if inv as libc::c_int != 0 {
                            (2 as libc::c_int as libc::c_uint)
                                .wrapping_sub(s as libc::c_uint)
                        } else {
                            s as libc::c_uint
                        }) as line_state,
                    ) {
                        diff = if diff < DIFF_EASY as libc::c_int {
                            diff
                        } else {
                            DIFF_EASY as libc::c_int
                        };
                    }
                }
            }
        }
        i += 1;
        i;
    }
    return diff;
}
unsafe extern "C" fn loop_deductions(mut sstate: *mut solver_state) -> libc::c_int {
    let mut edgecount: libc::c_int = 0 as libc::c_int;
    let mut clues: libc::c_int = 0 as libc::c_int;
    let mut satclues: libc::c_int = 0 as libc::c_int;
    let mut sm1clues: libc::c_int = 0 as libc::c_int;
    let mut state: *mut game_state = (*sstate).state;
    let mut g: *mut grid = (*state).game_grid;
    let mut shortest_chainlen: libc::c_int = (*g).num_dots;
    let mut dots_connected: libc::c_int = 0;
    let mut progress: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        if *((*state).lines).offset(i as isize) as libc::c_int == LINE_YES as libc::c_int
        {
            merge_dots(sstate, i);
            edgecount += 1;
            edgecount;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut c: libc::c_int = *((*state).clues).offset(i as isize) as libc::c_int;
        if c >= 0 as libc::c_int {
            let mut o: libc::c_int = *((*sstate).face_yes_count).offset(i as isize)
                as libc::c_int;
            if o == c {
                satclues += 1;
                satclues;
            } else if o == c - 1 as libc::c_int {
                sm1clues += 1;
                sm1clues;
            }
            clues += 1;
            clues;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        dots_connected = *((*sstate).looplen)
            .offset(dsf_canonify((*sstate).dotdsf, i) as isize);
        if dots_connected > 1 as libc::c_int {
            shortest_chainlen = if shortest_chainlen < dots_connected {
                shortest_chainlen
            } else {
                dots_connected
            };
        }
        i += 1;
        i;
    }
    if (*sstate).solver_status as libc::c_uint
        == SOLVER_INCOMPLETE as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"sstate->solver_status == SOLVER_INCOMPLETE\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
            2839 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int loop_deductions(solver_state *)\0"))
                .as_ptr(),
        );
    }
    'c_15441: {
        if (*sstate).solver_status as libc::c_uint
            == SOLVER_INCOMPLETE as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"sstate->solver_status == SOLVER_INCOMPLETE\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                2839 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"int loop_deductions(solver_state *)\0"))
                    .as_ptr(),
            );
        }
    };
    if satclues == clues && shortest_chainlen == edgecount {
        (*sstate).solver_status = SOLVER_SOLVED;
        progress = 1 as libc::c_int != 0;
    } else {
        i = 0 as libc::c_int;
        while i < (*g).num_edges {
            let mut e: *mut grid_edge = *((*g).edges).offset(i as isize);
            let mut d1: libc::c_int = (*(*e).dot1).index;
            let mut d2: libc::c_int = (*(*e).dot2).index;
            let mut eqclass: libc::c_int = 0;
            let mut val: libc::c_int = 0;
            if !(*((*state).lines).offset(i as isize) as libc::c_int
                != LINE_UNKNOWN as libc::c_int)
            {
                eqclass = dsf_canonify((*sstate).dotdsf, d1);
                if !(eqclass != dsf_canonify((*sstate).dotdsf, d2)) {
                    val = LINE_NO as libc::c_int;
                    if *((*sstate).looplen).offset(eqclass as isize)
                        == edgecount + 1 as libc::c_int
                    {
                        let mut sm1_nearby: libc::c_int = 0;
                        sm1_nearby = 0 as libc::c_int;
                        if !((*e).face1).is_null() {
                            let mut f: libc::c_int = (*(*e).face1).index;
                            let mut c_0: libc::c_int = *((*state).clues)
                                .offset(f as isize) as libc::c_int;
                            if c_0 >= 0 as libc::c_int
                                && *((*sstate).face_yes_count).offset(f as isize)
                                    as libc::c_int == c_0 - 1 as libc::c_int
                            {
                                sm1_nearby += 1;
                                sm1_nearby;
                            }
                        }
                        if !((*e).face2).is_null() {
                            let mut f_0: libc::c_int = (*(*e).face2).index;
                            let mut c_1: libc::c_int = *((*state).clues)
                                .offset(f_0 as isize) as libc::c_int;
                            if c_1 >= 0 as libc::c_int
                                && *((*sstate).face_yes_count).offset(f_0 as isize)
                                    as libc::c_int == c_1 - 1 as libc::c_int
                            {
                                sm1_nearby += 1;
                                sm1_nearby;
                            }
                        }
                        if sm1clues == sm1_nearby && sm1clues + satclues == clues {
                            val = LINE_YES as libc::c_int;
                        }
                    }
                    progress = solver_set_line(sstate, i, val as line_state);
                    if progress {} else {
                        __assert_fail(
                            b"progress\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                            2933 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 36],
                                &[libc::c_char; 36],
                            >(b"int loop_deductions(solver_state *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_14887: {
                        if progress {} else {
                            __assert_fail(
                                b"progress\0" as *const u8 as *const libc::c_char,
                                b"/puzzles/loopy.c\0" as *const u8 as *const libc::c_char,
                                2933 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 36],
                                    &[libc::c_char; 36],
                                >(b"int loop_deductions(solver_state *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if val == LINE_YES as libc::c_int {
                        (*sstate).solver_status = SOLVER_AMBIGUOUS;
                        break;
                    }
                }
            }
            i += 1;
            i;
        }
    }
    return if progress as libc::c_int != 0 {
        DIFF_EASY as libc::c_int
    } else {
        DIFF_MAX as libc::c_int
    };
}
unsafe extern "C" fn solve_game_rec(
    mut sstate_start: *const solver_state,
) -> *mut solver_state {
    let mut sstate: *mut solver_state = 0 as *mut solver_state;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut threshold_diff: libc::c_int = 0 as libc::c_int;
    let mut threshold_index: libc::c_int = 0 as libc::c_int;
    sstate = dup_solver_state(sstate_start);
    while i < NUM_SOLVERS {
        if (*sstate).solver_status as libc::c_uint
            == SOLVER_MISTAKE as libc::c_int as libc::c_uint
        {
            return sstate;
        }
        if (*sstate).solver_status as libc::c_uint
            == SOLVER_SOLVED as libc::c_int as libc::c_uint
            || (*sstate).solver_status as libc::c_uint
                == SOLVER_AMBIGUOUS as libc::c_int as libc::c_uint
        {
            break;
        }
        if (solver_diffs[i as usize] >= threshold_diff || i >= threshold_index)
            && solver_diffs[i as usize] <= (*sstate).diff
        {
            let mut next_diff: libc::c_int = (solver_fns[i as usize])
                .expect("non-null function pointer")(sstate);
            if next_diff != DIFF_MAX as libc::c_int {
                threshold_diff = next_diff;
                threshold_index = i;
                i = 0 as libc::c_int;
                continue;
            }
        }
        i += 1;
        i;
    }
    if (*sstate).solver_status as libc::c_uint
        == SOLVER_SOLVED as libc::c_int as libc::c_uint
        || (*sstate).solver_status as libc::c_uint
            == SOLVER_AMBIGUOUS as libc::c_int as libc::c_uint
    {
        array_setall(
            (*(*sstate).state).lines,
            LINE_UNKNOWN as libc::c_int as libc::c_char,
            LINE_NO as libc::c_int as libc::c_char,
            (*(*(*sstate).state).game_grid).num_edges,
        );
        return sstate;
    }
    return sstate;
}
unsafe extern "C" fn solve_game(
    mut state: *const game_state,
    mut currstate: *const game_state,
    mut aux: *const libc::c_char,
    mut error: *mut *const libc::c_char,
) -> *mut libc::c_char {
    let mut soln: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sstate: *mut solver_state = 0 as *mut solver_state;
    let mut new_sstate: *mut solver_state = 0 as *mut solver_state;
    sstate = new_solver_state(state, DIFF_MAX as libc::c_int);
    new_sstate = solve_game_rec(sstate);
    if (*new_sstate).solver_status as libc::c_uint
        == SOLVER_SOLVED as libc::c_int as libc::c_uint
    {
        soln = encode_solve_move((*new_sstate).state);
    } else if (*new_sstate).solver_status as libc::c_uint
        == SOLVER_AMBIGUOUS as libc::c_int as libc::c_uint
    {
        soln = encode_solve_move((*new_sstate).state);
    } else {
        soln = encode_solve_move((*new_sstate).state);
    }
    free_solver_state(new_sstate);
    free_solver_state(sstate);
    return soln;
}
unsafe extern "C" fn interpret_move(
    mut state: *const game_state,
    mut ui: *mut game_ui,
    mut ds: *const game_drawstate,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut button: libc::c_int,
) -> *mut libc::c_char {
    let mut g: *mut grid = (*state).game_grid;
    let mut e: *mut grid_edge = 0 as *mut grid_edge;
    let mut i: libc::c_int = 0;
    let mut movebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut movelen: libc::c_int = 0;
    let mut movesize: libc::c_int = 0;
    let mut button_char: libc::c_char = ' ' as i32 as libc::c_char;
    let mut old_state: line_state = LINE_YES;
    button &= !(MOD_MASK as libc::c_int);
    x -= (*ds).tilesize / 2 as libc::c_int;
    y -= (*ds).tilesize / 2 as libc::c_int;
    x = x * (*g).tilesize / (*ds).tilesize;
    y = y * (*g).tilesize / (*ds).tilesize;
    x += (*g).lowest_x;
    y += (*g).lowest_y;
    e = grid_nearest_edge(g, x, y);
    if e.is_null() {
        return 0 as *mut libc::c_char;
    }
    i = (*e).index;
    old_state = *((*state).lines).offset(i as isize) as line_state;
    match button {
        512 => {
            match old_state as libc::c_uint {
                1 => {
                    button_char = 'y' as i32 as libc::c_char;
                }
                0 | 2 => {
                    button_char = 'u' as i32 as libc::c_char;
                }
                _ => {}
            }
        }
        513 => {
            button_char = 'u' as i32 as libc::c_char;
        }
        514 => {
            match old_state as libc::c_uint {
                1 => {
                    button_char = 'n' as i32 as libc::c_char;
                }
                2 | 0 => {
                    button_char = 'u' as i32 as libc::c_char;
                }
                _ => {}
            }
        }
        _ => return 0 as *mut libc::c_char,
    }
    movelen = 0 as libc::c_int;
    movesize = 80 as libc::c_int;
    movebuf = smalloc(
        (movesize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    movelen = sprintf(
        movebuf,
        b"%d%c\0" as *const u8 as *const libc::c_char,
        i,
        button_char as libc::c_int,
    );
    if (*ui).autofollow as libc::c_uint != AF_OFF as libc::c_int as libc::c_uint {
        let mut dotid: libc::c_int = 0;
        dotid = 0 as libc::c_int;
        while dotid < 2 as libc::c_int {
            let mut dot: *mut grid_dot = if dotid == 0 as libc::c_int {
                (*e).dot1
            } else {
                (*e).dot2
            };
            let mut e_this: *mut grid_edge = e;
            loop {
                let mut j: libc::c_int = 0;
                let mut n_found: libc::c_int = 0;
                let mut e_next: *mut grid_edge = 0 as *mut grid_edge;
                n_found = 0 as libc::c_int;
                j = n_found;
                while j < (*dot).order {
                    let mut e_candidate: *mut grid_edge = *((*dot).edges)
                        .offset(j as isize);
                    let mut i_candidate: libc::c_int = (*e_candidate).index;
                    if e_candidate != e_this
                        && ((*ui).autofollow as libc::c_uint
                            == AF_FIXED as libc::c_int as libc::c_uint
                            || *((*state).lines).offset(i as isize) as libc::c_int
                                == LINE_NO as libc::c_int
                            || *((*state).lines).offset(i_candidate as isize)
                                as libc::c_int != LINE_NO as libc::c_int)
                    {
                        e_next = e_candidate;
                        n_found += 1;
                        n_found;
                    }
                    j += 1;
                    j;
                }
                if n_found != 1 as libc::c_int
                    || *((*state).lines).offset((*e_next).index as isize) as libc::c_int
                        != *((*state).lines).offset(i as isize) as libc::c_int
                {
                    break;
                }
                if e_next == e {
                    break;
                }
                dot = if (*e_next).dot1 != dot {
                    (*e_next).dot1
                } else {
                    (*e_next).dot2
                };
                if movelen > movesize - 40 as libc::c_int {
                    movesize = movesize * 5 as libc::c_int / 4 as libc::c_int
                        + 128 as libc::c_int;
                    movebuf = srealloc(
                        movebuf as *mut libc::c_void,
                        (movesize as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_char;
                }
                e_this = e_next;
                movelen
                    += sprintf(
                        movebuf.offset(movelen as isize),
                        b"%d%c\0" as *const u8 as *const libc::c_char,
                        (*e_this).index,
                        button_char as libc::c_int,
                    );
            }
            dotid += 1;
            dotid;
        }
    }
    return srealloc(
        movebuf as *mut libc::c_void,
        ((movelen + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
}
unsafe extern "C" fn execute_move(
    mut state: *const game_state,
    mut move_0: *const libc::c_char,
) -> *mut game_state {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut newstate: *mut game_state = dup_game(state);
    if *move_0.offset(0 as libc::c_int as isize) as libc::c_int == 'S' as i32 {
        move_0 = move_0.offset(1);
        move_0;
        (*newstate).cheated = 1 as libc::c_int != 0;
    }
    loop {
        if !(*move_0 != 0) {
            current_block = 13586036798005543211;
            break;
        }
        i = atoi(move_0);
        if i < 0 as libc::c_int || i >= (*(*newstate).game_grid).num_edges {
            current_block = 3870127747039110117;
            break;
        }
        move_0 = move_0
            .offset(
                strspn(move_0, b"1234567890\0" as *const u8 as *const libc::c_char)
                    as isize,
            );
        let fresh31 = move_0;
        move_0 = move_0.offset(1);
        match *fresh31 as libc::c_int {
            121 => {
                *((*newstate).lines)
                    .offset(i as isize) = LINE_YES as libc::c_int as libc::c_char;
            }
            110 => {
                *((*newstate).lines)
                    .offset(i as isize) = LINE_NO as libc::c_int as libc::c_char;
            }
            117 => {
                *((*newstate).lines)
                    .offset(i as isize) = LINE_UNKNOWN as libc::c_int as libc::c_char;
            }
            _ => {
                current_block = 3870127747039110117;
                break;
            }
        }
    }
    match current_block {
        3870127747039110117 => {
            free_game(newstate);
            return 0 as *mut game_state;
        }
        _ => {
            if check_completion(newstate) {
                (*newstate).solved = 1 as libc::c_int != 0;
            }
            return newstate;
        }
    };
}
unsafe extern "C" fn grid_to_screen(
    mut ds: *const game_drawstate,
    mut g: *const grid,
    mut grid_x: libc::c_int,
    mut grid_y: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
) {
    *x = grid_x - (*g).lowest_x;
    *y = grid_y - (*g).lowest_y;
    *x = *x * (*ds).tilesize / (*g).tilesize;
    *y = *y * (*ds).tilesize / (*g).tilesize;
    *x += (*ds).tilesize / 2 as libc::c_int;
    *y += (*ds).tilesize / 2 as libc::c_int;
}
unsafe extern "C" fn face_text_pos(
    mut ds: *const game_drawstate,
    mut g: *const grid,
    mut f: *mut grid_face,
    mut xret: *mut libc::c_int,
    mut yret: *mut libc::c_int,
) {
    let mut faceindex: libc::c_int = (*f).index;
    if *((*ds).textx).offset(faceindex as isize) >= 0 as libc::c_int {
        *xret = *((*ds).textx).offset(faceindex as isize);
        *yret = *((*ds).texty).offset(faceindex as isize);
        return;
    }
    grid_find_incentre(f);
    grid_to_screen(
        ds,
        g,
        (*f).ix,
        (*f).iy,
        &mut *((*ds).textx).offset(faceindex as isize),
        &mut *((*ds).texty).offset(faceindex as isize),
    );
    *xret = *((*ds).textx).offset(faceindex as isize);
    *yret = *((*ds).texty).offset(faceindex as isize);
}
unsafe extern "C" fn face_text_bbox(
    mut ds: *mut game_drawstate,
    mut g: *mut grid,
    mut f: *mut grid_face,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut w: *mut libc::c_int,
    mut h: *mut libc::c_int,
) {
    let mut xx: libc::c_int = 0;
    let mut yy: libc::c_int = 0;
    face_text_pos(ds, g, f, &mut xx, &mut yy);
    *x = xx - (*ds).tilesize * 5 as libc::c_int / 4 as libc::c_int - 1 as libc::c_int;
    *y = yy - (*ds).tilesize / 4 as libc::c_int - 3 as libc::c_int;
    *w = (*ds).tilesize * 5 as libc::c_int / 2 as libc::c_int + 2 as libc::c_int;
    *h = (*ds).tilesize / 2 as libc::c_int + 5 as libc::c_int;
}
unsafe extern "C" fn game_redraw_clue(
    mut dr: *mut drawing,
    mut ds: *mut game_drawstate,
    mut state: *const game_state,
    mut i: libc::c_int,
) {
    let mut g: *mut grid = (*state).game_grid;
    let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut c: [libc::c_char; 20] = [0; 20];
    sprintf(
        c.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        *((*state).clues).offset(i as isize) as libc::c_int,
    );
    face_text_pos(ds, g, f, &mut x, &mut y);
    draw_text(
        dr,
        x,
        y,
        1 as libc::c_int,
        (*ds).tilesize / 2 as libc::c_int,
        0x100 as libc::c_int | 0x1 as libc::c_int,
        if *((*ds).clue_error).offset(i as isize) as libc::c_int != 0 {
            COL_MISTAKE as libc::c_int
        } else if *((*ds).clue_satisfied).offset(i as isize) as libc::c_int != 0 {
            COL_SATISFIED as libc::c_int
        } else {
            COL_FOREGROUND as libc::c_int
        },
        c.as_mut_ptr(),
    );
}
unsafe extern "C" fn edge_bbox(
    mut ds: *mut game_drawstate,
    mut g: *mut grid,
    mut e: *mut grid_edge,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut w: *mut libc::c_int,
    mut h: *mut libc::c_int,
) {
    let mut x1: libc::c_int = (*(*e).dot1).x;
    let mut y1: libc::c_int = (*(*e).dot1).y;
    let mut x2: libc::c_int = (*(*e).dot2).x;
    let mut y2: libc::c_int = (*(*e).dot2).y;
    let mut xmin: libc::c_int = 0;
    let mut xmax: libc::c_int = 0;
    let mut ymin: libc::c_int = 0;
    let mut ymax: libc::c_int = 0;
    grid_to_screen(ds, g, x1, y1, &mut x1, &mut y1);
    grid_to_screen(ds, g, x2, y2, &mut x2, &mut y2);
    xmin = (if x1 < x2 { x1 } else { x2 })
        - ((*ds).tilesize + 15 as libc::c_int) / 16 as libc::c_int;
    xmax = (if x1 > x2 { x1 } else { x2 })
        + ((*ds).tilesize + 15 as libc::c_int) / 16 as libc::c_int;
    ymin = (if y1 < y2 { y1 } else { y2 })
        - ((*ds).tilesize + 15 as libc::c_int) / 16 as libc::c_int;
    ymax = (if y1 > y2 { y1 } else { y2 })
        + ((*ds).tilesize + 15 as libc::c_int) / 16 as libc::c_int;
    *x = xmin;
    *y = ymin;
    *w = xmax - xmin + 1 as libc::c_int;
    *h = ymax - ymin + 1 as libc::c_int;
}
unsafe extern "C" fn dot_bbox(
    mut ds: *mut game_drawstate,
    mut g: *mut grid,
    mut d: *mut grid_dot,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut w: *mut libc::c_int,
    mut h: *mut libc::c_int,
) {
    let mut x1: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut xmin: libc::c_int = 0;
    let mut xmax: libc::c_int = 0;
    let mut ymin: libc::c_int = 0;
    let mut ymax: libc::c_int = 0;
    grid_to_screen(ds, g, (*d).x, (*d).y, &mut x1, &mut y1);
    xmin = x1
        - ((*ds).tilesize * 5 as libc::c_int + 63 as libc::c_int) / 64 as libc::c_int;
    xmax = x1
        + ((*ds).tilesize * 5 as libc::c_int + 63 as libc::c_int) / 64 as libc::c_int;
    ymin = y1
        - ((*ds).tilesize * 5 as libc::c_int + 63 as libc::c_int) / 64 as libc::c_int;
    ymax = y1
        + ((*ds).tilesize * 5 as libc::c_int + 63 as libc::c_int) / 64 as libc::c_int;
    *x = xmin;
    *y = ymin;
    *w = xmax - xmin + 1 as libc::c_int;
    *h = ymax - ymin + 1 as libc::c_int;
}
static mut loopy_line_redraw_phases: [libc::c_int; 5] = [
    COL_FAINT as libc::c_int,
    COL_LINEUNKNOWN as libc::c_int,
    COL_FOREGROUND as libc::c_int,
    COL_HIGHLIGHT as libc::c_int,
    COL_MISTAKE as libc::c_int,
];
unsafe extern "C" fn game_redraw_line(
    mut dr: *mut drawing,
    mut ds: *mut game_drawstate,
    mut ui: *const game_ui,
    mut state: *const game_state,
    mut i: libc::c_int,
    mut phase: libc::c_int,
) {
    let mut g: *mut grid = (*state).game_grid;
    let mut e: *mut grid_edge = *((*g).edges).offset(i as isize);
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y1: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut line_colour: libc::c_int = 0;
    if *((*state).line_errors).offset(i as isize) {
        line_colour = COL_MISTAKE as libc::c_int;
    } else if *((*state).lines).offset(i as isize) as libc::c_int
        == LINE_UNKNOWN as libc::c_int
    {
        line_colour = COL_LINEUNKNOWN as libc::c_int;
    } else if *((*state).lines).offset(i as isize) as libc::c_int
        == LINE_NO as libc::c_int
    {
        line_colour = COL_FAINT as libc::c_int;
    } else if (*ds).flashing {
        line_colour = COL_HIGHLIGHT as libc::c_int;
    } else {
        line_colour = COL_FOREGROUND as libc::c_int;
    }
    if line_colour != loopy_line_redraw_phases[phase as usize] {
        return;
    }
    grid_to_screen(ds, g, (*(*e).dot1).x, (*(*e).dot1).y, &mut x1, &mut y1);
    grid_to_screen(ds, g, (*(*e).dot2).x, (*(*e).dot2).y, &mut x2, &mut y2);
    if line_colour == COL_FAINT as libc::c_int {
        if (*ui).draw_faint_lines {
            draw_thick_line(
                dr,
                ((*ds).tilesize as libc::c_double / 24.0f64) as libc::c_float,
                (x1 as libc::c_double + 0.5f64) as libc::c_float,
                (y1 as libc::c_double + 0.5f64) as libc::c_float,
                (x2 as libc::c_double + 0.5f64) as libc::c_float,
                (y2 as libc::c_double + 0.5f64) as libc::c_float,
                line_colour,
            );
        }
    } else {
        draw_thick_line(
            dr,
            (((*ds).tilesize * 3 as libc::c_int) as libc::c_double / 32.0f64)
                as libc::c_float,
            (x1 as libc::c_double + 0.5f64) as libc::c_float,
            (y1 as libc::c_double + 0.5f64) as libc::c_float,
            (x2 as libc::c_double + 0.5f64) as libc::c_float,
            (y2 as libc::c_double + 0.5f64) as libc::c_float,
            line_colour,
        );
    };
}
unsafe extern "C" fn game_redraw_dot(
    mut dr: *mut drawing,
    mut ds: *mut game_drawstate,
    mut state: *const game_state,
    mut i: libc::c_int,
) {
    let mut g: *mut grid = (*state).game_grid;
    let mut d: *mut grid_dot = *((*g).dots).offset(i as isize);
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    grid_to_screen(ds, g, (*d).x, (*d).y, &mut x, &mut y);
    draw_circle(
        dr,
        x,
        y,
        ((*ds).tilesize as libc::c_double * 2.5f64 / 32.0f64) as libc::c_int,
        COL_FOREGROUND as libc::c_int,
        COL_FOREGROUND as libc::c_int,
    );
}
unsafe extern "C" fn boxes_intersect(
    mut x0: libc::c_int,
    mut y0: libc::c_int,
    mut w0: libc::c_int,
    mut h0: libc::c_int,
    mut x1: libc::c_int,
    mut y1: libc::c_int,
    mut w1: libc::c_int,
    mut h1: libc::c_int,
) -> bool {
    return x0 < x1 + w1 && x1 < x0 + w0 && y0 < y1 + h1 && y1 < y0 + h0;
}
unsafe extern "C" fn game_redraw_in_rect(
    mut dr: *mut drawing,
    mut ds: *mut game_drawstate,
    mut ui: *const game_ui,
    mut state: *const game_state,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) {
    let mut g: *mut grid = (*state).game_grid;
    let mut i: libc::c_int = 0;
    let mut phase: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut by: libc::c_int = 0;
    let mut bw: libc::c_int = 0;
    let mut bh: libc::c_int = 0;
    clip(dr, x, y, w, h);
    draw_rect(dr, x, y, w, h, COL_BACKGROUND as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        if *((*state).clues).offset(i as isize) as libc::c_int >= 0 as libc::c_int {
            face_text_bbox(
                ds,
                g,
                *((*g).faces).offset(i as isize),
                &mut bx,
                &mut by,
                &mut bw,
                &mut bh,
            );
            if boxes_intersect(x, y, w, h, bx, by, bw, bh) {
                game_redraw_clue(dr, ds, state, i);
            }
        }
        i += 1;
        i;
    }
    phase = 0 as libc::c_int;
    while (phase as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        i = 0 as libc::c_int;
        while i < (*g).num_edges {
            edge_bbox(
                ds,
                g,
                *((*g).edges).offset(i as isize),
                &mut bx,
                &mut by,
                &mut bw,
                &mut bh,
            );
            if boxes_intersect(x, y, w, h, bx, by, bw, bh) {
                game_redraw_line(dr, ds, ui, state, i, phase);
            }
            i += 1;
            i;
        }
        phase += 1;
        phase;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        dot_bbox(
            ds,
            g,
            *((*g).dots).offset(i as isize),
            &mut bx,
            &mut by,
            &mut bw,
            &mut bh,
        );
        if boxes_intersect(x, y, w, h, bx, by, bw, bh) {
            game_redraw_dot(dr, ds, state, i);
        }
        i += 1;
        i;
    }
    unclip(dr);
    draw_update(dr, x, y, w, h);
}
unsafe extern "C" fn game_redraw(
    mut dr: *mut drawing,
    mut ds: *mut game_drawstate,
    mut oldstate: *const game_state,
    mut state: *const game_state,
    mut dir: libc::c_int,
    mut ui: *const game_ui,
    mut animtime: libc::c_float,
    mut flashtime: libc::c_float,
) {
    let mut g: *mut grid = (*state).game_grid;
    let mut border: libc::c_int = (*ds).tilesize / 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut flash_changed: bool = false;
    let mut redraw_everything: bool = 0 as libc::c_int != 0;
    let mut edges: [libc::c_int; 16] = [0; 16];
    let mut nedges: libc::c_int = 0 as libc::c_int;
    let mut faces: [libc::c_int; 16] = [0; 16];
    let mut nfaces: libc::c_int = 0 as libc::c_int;
    if !(*ds).started {
        redraw_everything = 1 as libc::c_int != 0;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut sides: libc::c_int = (*f).order;
        let mut yes_order: libc::c_int = 0;
        let mut no_order: libc::c_int = 0;
        let mut clue_mistake: bool = false;
        let mut clue_satisfied: bool = false;
        let mut n: libc::c_int = *((*state).clues).offset(i as isize) as libc::c_int;
        if !(n < 0 as libc::c_int) {
            yes_order = face_order(state, i, LINE_YES as libc::c_int as libc::c_char);
            if (*state).exactly_one_loop {
                no_order = sides - yes_order;
            } else {
                no_order = face_order(state, i, LINE_NO as libc::c_int as libc::c_char);
            }
            clue_mistake = yes_order > n || no_order > sides - n;
            clue_satisfied = yes_order == n && no_order == sides - n;
            if clue_mistake as libc::c_int
                != *((*ds).clue_error).offset(i as isize) as libc::c_int
                || clue_satisfied as libc::c_int
                    != *((*ds).clue_satisfied).offset(i as isize) as libc::c_int
            {
                *((*ds).clue_error).offset(i as isize) = clue_mistake;
                *((*ds).clue_satisfied).offset(i as isize) = clue_satisfied;
                if nfaces == 16 as libc::c_int {
                    redraw_everything = 1 as libc::c_int != 0;
                } else {
                    let fresh32 = nfaces;
                    nfaces = nfaces + 1;
                    faces[fresh32 as usize] = i;
                }
            }
        }
        i += 1;
        i;
    }
    if flashtime > 0 as libc::c_int as libc::c_float
        && (flashtime <= 0.5f32 / 3 as libc::c_int as libc::c_float
            || flashtime
                >= 0.5f32 * 2 as libc::c_int as libc::c_float
                    / 3 as libc::c_int as libc::c_float)
    {
        flash_changed = !(*ds).flashing;
        (*ds).flashing = 1 as libc::c_int != 0;
    } else {
        flash_changed = (*ds).flashing;
        (*ds).flashing = 0 as libc::c_int != 0;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut new_ds: libc::c_char = (if *((*state).line_errors).offset(i as isize)
            as libc::c_int != 0
        {
            DS_LINE_ERROR as libc::c_int
        } else {
            *((*state).lines).offset(i as isize) as libc::c_int
        }) as libc::c_char;
        if new_ds as libc::c_int != *((*ds).lines).offset(i as isize) as libc::c_int
            || flash_changed as libc::c_int != 0
                && *((*state).lines).offset(i as isize) as libc::c_int
                    == LINE_YES as libc::c_int
        {
            *((*ds).lines).offset(i as isize) = new_ds;
            if nedges == 16 as libc::c_int {
                redraw_everything = 1 as libc::c_int != 0;
            } else {
                let fresh33 = nedges;
                nedges = nedges + 1;
                edges[fresh33 as usize] = i;
            }
        }
        i += 1;
        i;
    }
    if redraw_everything {
        let mut grid_width: libc::c_int = (*g).highest_x - (*g).lowest_x;
        let mut grid_height: libc::c_int = (*g).highest_y - (*g).lowest_y;
        let mut w: libc::c_int = grid_width * (*ds).tilesize / (*g).tilesize;
        let mut h: libc::c_int = grid_height * (*ds).tilesize / (*g).tilesize;
        game_redraw_in_rect(
            dr,
            ds,
            ui,
            state,
            0 as libc::c_int,
            0 as libc::c_int,
            w + 2 as libc::c_int * border + 1 as libc::c_int,
            h + 2 as libc::c_int * border + 1 as libc::c_int,
        );
    } else {
        i = 0 as libc::c_int;
        while i < nfaces {
            let mut f_0: *mut grid_face = *((*g).faces)
                .offset(faces[i as usize] as isize);
            let mut x: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let mut w_0: libc::c_int = 0;
            let mut h_0: libc::c_int = 0;
            face_text_bbox(ds, g, f_0, &mut x, &mut y, &mut w_0, &mut h_0);
            game_redraw_in_rect(dr, ds, ui, state, x, y, w_0, h_0);
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < nedges {
            let mut e: *mut grid_edge = *((*g).edges).offset(edges[i as usize] as isize);
            let mut x_0: libc::c_int = 0;
            let mut y_0: libc::c_int = 0;
            let mut w_1: libc::c_int = 0;
            let mut h_1: libc::c_int = 0;
            edge_bbox(ds, g, e, &mut x_0, &mut y_0, &mut w_1, &mut h_1);
            game_redraw_in_rect(dr, ds, ui, state, x_0, y_0, w_1, h_1);
            i += 1;
            i;
        }
    }
    (*ds).started = 1 as libc::c_int != 0;
}
unsafe extern "C" fn game_flash_length(
    mut oldstate: *const game_state,
    mut newstate: *const game_state,
    mut dir: libc::c_int,
    mut ui: *mut game_ui,
) -> libc::c_float {
    if !(*oldstate).solved && (*newstate).solved as libc::c_int != 0
        && !(*oldstate).cheated && !(*newstate).cheated
    {
        return 0.5f32;
    }
    return 0.0f32;
}
unsafe extern "C" fn game_get_cursor_location(
    mut ui: *const game_ui,
    mut ds: *const game_drawstate,
    mut state: *const game_state,
    mut params: *const game_params,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut w: *mut libc::c_int,
    mut h: *mut libc::c_int,
) {}
unsafe extern "C" fn game_status(mut state: *const game_state) -> libc::c_int {
    return if (*state).solved as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn game_print_size(
    mut params: *const game_params,
    mut ui: *const game_ui,
    mut x: *mut libc::c_float,
    mut y: *mut libc::c_float,
) {
    let mut pw: libc::c_int = 0;
    let mut ph: libc::c_int = 0;
    game_compute_size(params, 700 as libc::c_int, ui, &mut pw, &mut ph);
    *x = pw as libc::c_float / 100.0f32;
    *y = ph as libc::c_float / 100.0f32;
}
unsafe extern "C" fn game_print(
    mut dr: *mut drawing,
    mut state: *const game_state,
    mut ui: *const game_ui,
    mut tilesize: libc::c_int,
) {
    let mut ink: libc::c_int = print_mono_colour(dr, 0 as libc::c_int);
    let mut i: libc::c_int = 0;
    let mut ads: game_drawstate = game_drawstate {
        started: false,
        tilesize: 0,
        flashing: false,
        textx: 0 as *mut libc::c_int,
        texty: 0 as *mut libc::c_int,
        lines: 0 as *mut libc::c_char,
        clue_error: 0 as *mut bool,
        clue_satisfied: 0 as *mut bool,
    };
    let mut ds: *mut game_drawstate = &mut ads;
    let mut g: *mut grid = (*state).game_grid;
    (*ds).tilesize = tilesize;
    (*ds)
        .textx = smalloc(
        ((*g).num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*ds)
        .texty = smalloc(
        ((*g).num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let ref mut fresh34 = *((*ds).texty).offset(i as isize);
        *fresh34 = -(1 as libc::c_int);
        *((*ds).textx).offset(i as isize) = *fresh34;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_dots {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        grid_to_screen(
            ds,
            g,
            (**((*g).dots).offset(i as isize)).x,
            (**((*g).dots).offset(i as isize)).y,
            &mut x,
            &mut y,
        );
        draw_circle(dr, x, y, (*ds).tilesize / 15 as libc::c_int, ink, ink);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_faces {
        let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut clue: libc::c_int = *((*state).clues).offset(i as isize) as libc::c_int;
        if clue >= 0 as libc::c_int {
            let mut c: [libc::c_char; 20] = [0; 20];
            let mut x_0: libc::c_int = 0;
            let mut y_0: libc::c_int = 0;
            sprintf(
                c.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                *((*state).clues).offset(i as isize) as libc::c_int,
            );
            face_text_pos(ds, g, f, &mut x_0, &mut y_0);
            draw_text(
                dr,
                x_0,
                y_0,
                1 as libc::c_int,
                (*ds).tilesize / 2 as libc::c_int,
                0x100 as libc::c_int | 0x1 as libc::c_int,
                ink,
                c.as_mut_ptr(),
            );
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*g).num_edges {
        let mut thickness: libc::c_int = if *((*state).lines).offset(i as isize)
            as libc::c_int == LINE_YES as libc::c_int
        {
            30 as libc::c_int
        } else {
            150 as libc::c_int
        };
        let mut e: *mut grid_edge = *((*g).edges).offset(i as isize);
        let mut x1: libc::c_int = 0;
        let mut y1: libc::c_int = 0;
        let mut x2: libc::c_int = 0;
        let mut y2: libc::c_int = 0;
        grid_to_screen(ds, g, (*(*e).dot1).x, (*(*e).dot1).y, &mut x1, &mut y1);
        grid_to_screen(ds, g, (*(*e).dot2).x, (*(*e).dot2).y, &mut x2, &mut y2);
        if *((*state).lines).offset(i as isize) as libc::c_int == LINE_YES as libc::c_int
        {
            let mut d: libc::c_double = __tg_sqrt(
                (x1 as libc::c_double - x2 as libc::c_double)
                    * (x1 as libc::c_double - x2 as libc::c_double)
                    + (y1 as libc::c_double - y2 as libc::c_double)
                        * (y1 as libc::c_double - y2 as libc::c_double),
            );
            let mut dx: libc::c_double = (x2 - x1) as libc::c_double / d;
            let mut dy: libc::c_double = (y2 - y1) as libc::c_double / d;
            let mut points: [libc::c_int; 8] = [0; 8];
            dx = dx * (*ds).tilesize as libc::c_double / thickness as libc::c_double;
            dy = dy * (*ds).tilesize as libc::c_double / thickness as libc::c_double;
            points[0 as libc::c_int as usize] = x1 + dy as libc::c_int;
            points[1 as libc::c_int as usize] = y1 - dx as libc::c_int;
            points[2 as libc::c_int as usize] = x1 - dy as libc::c_int;
            points[3 as libc::c_int as usize] = y1 + dx as libc::c_int;
            points[4 as libc::c_int as usize] = x2 - dy as libc::c_int;
            points[5 as libc::c_int as usize] = y2 + dx as libc::c_int;
            points[6 as libc::c_int as usize] = x2 + dy as libc::c_int;
            points[7 as libc::c_int as usize] = y2 - dx as libc::c_int;
            draw_polygon(dr, points.as_mut_ptr(), 4 as libc::c_int, ink, ink);
        } else {
            let mut divisions: libc::c_int = 6 as libc::c_int;
            let mut j: libc::c_int = 0;
            j = 1 as libc::c_int;
            while j < divisions {
                let mut x_1: libc::c_int = (x1 * (divisions - j) + x2 * j) / divisions;
                let mut y_1: libc::c_int = (y1 * (divisions - j) + y2 * j) / divisions;
                draw_circle(dr, x_1, y_1, (*ds).tilesize / thickness, ink, ink);
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    sfree((*ds).textx as *mut libc::c_void);
    sfree((*ds).texty as *mut libc::c_void);
}
#[no_mangle]
pub static mut thegame: game = unsafe {
    {
        let mut init = game {
            name: b"Loopy\0" as *const u8 as *const libc::c_char,
            winhelp_topic: b"games.loopy\0" as *const u8 as *const libc::c_char,
            htmlhelp_topic: b"loopy\0" as *const u8 as *const libc::c_char,
            default_params: Some(
                default_params as unsafe extern "C" fn() -> *mut game_params,
            ),
            fetch_preset: None,
            preset_menu: Some(
                game_preset_menu as unsafe extern "C" fn() -> *mut preset_menu,
            ),
            decode_params: Some(
                decode_params
                    as unsafe extern "C" fn(*mut game_params, *const libc::c_char) -> (),
            ),
            encode_params: Some(
                encode_params
                    as unsafe extern "C" fn(
                        *const game_params,
                        bool,
                    ) -> *mut libc::c_char,
            ),
            free_params: Some(
                free_params as unsafe extern "C" fn(*mut game_params) -> (),
            ),
            dup_params: Some(
                dup_params
                    as unsafe extern "C" fn(*const game_params) -> *mut game_params,
            ),
            can_configure: 1 as libc::c_int != 0,
            configure: Some(
                game_configure
                    as unsafe extern "C" fn(*const game_params) -> *mut config_item,
            ),
            custom_params: Some(
                custom_params
                    as unsafe extern "C" fn(*const config_item) -> *mut game_params,
            ),
            validate_params: Some(
                validate_params
                    as unsafe extern "C" fn(
                        *const game_params,
                        bool,
                    ) -> *const libc::c_char,
            ),
            new_desc: Some(
                new_game_desc
                    as unsafe extern "C" fn(
                        *const game_params,
                        *mut random_state,
                        *mut *mut libc::c_char,
                        bool,
                    ) -> *mut libc::c_char,
            ),
            validate_desc: Some(
                validate_desc
                    as unsafe extern "C" fn(
                        *const game_params,
                        *const libc::c_char,
                    ) -> *const libc::c_char,
            ),
            new_game: Some(
                new_game
                    as unsafe extern "C" fn(
                        *mut midend,
                        *const game_params,
                        *const libc::c_char,
                    ) -> *mut game_state,
            ),
            dup_game: Some(
                dup_game as unsafe extern "C" fn(*const game_state) -> *mut game_state,
            ),
            free_game: Some(free_game as unsafe extern "C" fn(*mut game_state) -> ()),
            can_solve: 1 as libc::c_int != 0,
            solve: Some(
                solve_game
                    as unsafe extern "C" fn(
                        *const game_state,
                        *const game_state,
                        *const libc::c_char,
                        *mut *const libc::c_char,
                    ) -> *mut libc::c_char,
            ),
            can_format_as_text_ever: 1 as libc::c_int != 0,
            can_format_as_text_now: Some(
                game_can_format_as_text_now
                    as unsafe extern "C" fn(*const game_params) -> bool,
            ),
            text_format: Some(
                game_text_format
                    as unsafe extern "C" fn(*const game_state) -> *mut libc::c_char,
            ),
            get_prefs: Some(
                get_prefs as unsafe extern "C" fn(*mut game_ui) -> *mut config_item,
            ),
            set_prefs: Some(
                set_prefs as unsafe extern "C" fn(*mut game_ui, *const config_item) -> (),
            ),
            new_ui: Some(
                new_ui as unsafe extern "C" fn(*const game_state) -> *mut game_ui,
            ),
            free_ui: Some(free_ui as unsafe extern "C" fn(*mut game_ui) -> ()),
            encode_ui: None,
            decode_ui: None,
            request_keys: None,
            changed_state: Some(
                game_changed_state
                    as unsafe extern "C" fn(
                        *mut game_ui,
                        *const game_state,
                        *const game_state,
                    ) -> (),
            ),
            current_key_label: None,
            interpret_move: Some(
                interpret_move
                    as unsafe extern "C" fn(
                        *const game_state,
                        *mut game_ui,
                        *const game_drawstate,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut libc::c_char,
            ),
            execute_move: Some(
                execute_move
                    as unsafe extern "C" fn(
                        *const game_state,
                        *const libc::c_char,
                    ) -> *mut game_state,
            ),
            preferred_tilesize: 32 as libc::c_int,
            compute_size: Some(
                game_compute_size
                    as unsafe extern "C" fn(
                        *const game_params,
                        libc::c_int,
                        *const game_ui,
                        *mut libc::c_int,
                        *mut libc::c_int,
                    ) -> (),
            ),
            set_size: Some(
                game_set_size
                    as unsafe extern "C" fn(
                        *mut drawing,
                        *mut game_drawstate,
                        *const game_params,
                        libc::c_int,
                    ) -> (),
            ),
            colours: Some(
                game_colours
                    as unsafe extern "C" fn(
                        *mut frontend,
                        *mut libc::c_int,
                    ) -> *mut libc::c_float,
            ),
            new_drawstate: Some(
                game_new_drawstate
                    as unsafe extern "C" fn(
                        *mut drawing,
                        *const game_state,
                    ) -> *mut game_drawstate,
            ),
            free_drawstate: Some(
                game_free_drawstate
                    as unsafe extern "C" fn(*mut drawing, *mut game_drawstate) -> (),
            ),
            redraw: Some(
                game_redraw
                    as unsafe extern "C" fn(
                        *mut drawing,
                        *mut game_drawstate,
                        *const game_state,
                        *const game_state,
                        libc::c_int,
                        *const game_ui,
                        libc::c_float,
                        libc::c_float,
                    ) -> (),
            ),
            anim_length: Some(
                game_anim_length
                    as unsafe extern "C" fn(
                        *const game_state,
                        *const game_state,
                        libc::c_int,
                        *mut game_ui,
                    ) -> libc::c_float,
            ),
            flash_length: Some(
                game_flash_length
                    as unsafe extern "C" fn(
                        *const game_state,
                        *const game_state,
                        libc::c_int,
                        *mut game_ui,
                    ) -> libc::c_float,
            ),
            get_cursor_location: Some(
                game_get_cursor_location
                    as unsafe extern "C" fn(
                        *const game_ui,
                        *const game_drawstate,
                        *const game_state,
                        *const game_params,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                        *mut libc::c_int,
                    ) -> (),
            ),
            status: Some(
                game_status as unsafe extern "C" fn(*const game_state) -> libc::c_int,
            ),
            can_print: 1 as libc::c_int != 0,
            can_print_in_colour: 0 as libc::c_int != 0,
            print_size: Some(
                game_print_size
                    as unsafe extern "C" fn(
                        *const game_params,
                        *const game_ui,
                        *mut libc::c_float,
                        *mut libc::c_float,
                    ) -> (),
            ),
            print: Some(
                game_print
                    as unsafe extern "C" fn(
                        *mut drawing,
                        *const game_state,
                        *const game_ui,
                        libc::c_int,
                    ) -> (),
            ),
            wants_statusbar: 0 as libc::c_int != 0,
            is_timed: 0 as libc::c_int != 0,
            timing_state: None,
            flags: 0 as libc::c_int,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn loopy_generator(
    mut params: *mut libc::c_char,
    mut seed: *mut libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut p: *mut game_params = 0 as *mut game_params;
    let mut rs: *mut random_state = 0 as *mut random_state;
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut seedlen: libc::c_int = 0;
    if params.is_null() || seed.is_null() {
        return 1 as libc::c_int;
    }
    seedlen = strlen(seed) as libc::c_int;
    p = default_params();
    decode_params(p, params);
    if (*p).w <= 0 as libc::c_int || (*p).h <= 0 as libc::c_int {
        return 2 as libc::c_int;
    }
    rs = random_new(seed, seedlen);
    loop {
        count -= 1;
        if !(count >= 0 as libc::c_int) {
            break;
        }
        desc = new_game_desc(p, rs, 0 as *mut *mut libc::c_char, 0 as libc::c_int != 0);
        printf(b"%s:%s\n\0" as *const u8 as *const libc::c_char, params, desc);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut quis: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn usage_exit(mut msg: *const libc::c_char) {
    if !msg.is_null() {
        fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const libc::c_char, quis, msg);
    }
    fprintf(
        stderr,
        b"Usage: %s [--seed SEED] [--count COUNT] <params>\n\0" as *const u8
            as *const libc::c_char,
        quis,
    );
    exit(1 as libc::c_int);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut game_params = 0 as *mut game_params;
    let mut rs: *mut random_state = 0 as *mut random_state;
    let mut params: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut desc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 1 as libc::c_int;
    let mut seed: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut seedtime: time_t = 0;
    let mut seedlen: libc::c_int = 0 as libc::c_int;
    quis = *argv.offset(0 as libc::c_int as isize);
    loop {
        argc -= 1;
        if !(argc > 0 as libc::c_int) {
            break;
        }
        argv = argv.offset(1);
        let mut p_0: *mut libc::c_char = *argv;
        if strcmp(p_0, b"--seed\0" as *const u8 as *const libc::c_char) == 0 {
            if argc < 2 as libc::c_int {
                usage_exit(
                    b"--seed needs an argument\0" as *const u8 as *const libc::c_char,
                );
            }
            argv = argv.offset(1);
            seed = *argv as *mut libc::c_void;
            seedlen = strlen(seed as *const libc::c_char) as libc::c_int;
            argc -= 1;
            argc;
        } else if strcmp(p_0, b"--count\0" as *const u8 as *const libc::c_char) == 0 {
            if argc < 2 as libc::c_int {
                usage_exit(
                    b"--count needs an argument\0" as *const u8 as *const libc::c_char,
                );
            }
            argv = argv.offset(1);
            count = atoi(*argv) as time_t as libc::c_int;
            argc -= 1;
            argc;
        } else if *p_0 as libc::c_int == '-' as i32 {
            fprintf(
                stderr,
                b"%s: unrecognized option `%s'\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
                p_0,
            );
            return 1 as libc::c_int;
        } else {
            if !params.is_null() {
                usage_exit(b"too many arguments\0" as *const u8 as *const libc::c_char);
            }
            params = p_0;
        }
    }
    if params.is_null() {
        usage_exit(0 as *const libc::c_char);
    }
    if seed.is_null() {
        seedtime = time(0 as *mut time_t);
        seed = &mut seedtime as *mut time_t as *mut libc::c_void;
        seedlen = ::core::mem::size_of::<time_t>() as libc::c_ulong as libc::c_int;
    }
    p = default_params();
    decode_params(p, params);
    if (*p).w <= 0 as libc::c_int {
        usage_exit(b"invalid width\0" as *const u8 as *const libc::c_char);
    }
    if (*p).h <= 0 as libc::c_int {
        usage_exit(b"invalid height\0" as *const u8 as *const libc::c_char);
    }
    rs = random_new(seed as *const libc::c_char, seedlen);
    loop {
        count -= 1;
        if !(count >= 0 as libc::c_int) {
            break;
        }
        desc = new_game_desc(p, rs, 0 as *mut *mut libc::c_char, 0 as libc::c_int != 0);
        printf(b"%s:%s\n\0" as *const u8 as *const libc::c_char, params, desc);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    NUM_SOLVERS = (::core::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
