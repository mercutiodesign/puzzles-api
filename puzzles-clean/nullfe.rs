use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type frontend;
    pub type midend;
    pub type random_state;
    pub type game_params;
    pub type game_state;
    pub type game_ui;
    pub type game_drawstate;
    pub type document;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn dupstr(s: *const libc::c_char) -> *mut libc::c_char;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_item {
    pub name: *const libc::c_char,
    pub kw: *const libc::c_char,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub string: C2RustUnnamed_2,
    pub choices: C2RustUnnamed_1,
    pub boolean: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub bval: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub choicenames: *const libc::c_char,
    pub choicekws: *const libc::c_char,
    pub selected: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub sval: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game {
    pub name: *const libc::c_char,
    pub winhelp_topic: *const libc::c_char,
    pub htmlhelp_topic: *const libc::c_char,
    pub default_params: Option<unsafe extern "C" fn() -> *mut game_params>,
    pub fetch_preset: Option<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char, *mut *mut game_params) -> bool,
    >,
    pub preset_menu: Option<unsafe extern "C" fn() -> *mut preset_menu>,
    pub decode_params: Option<unsafe extern "C" fn(*mut game_params, *const libc::c_char) -> ()>,
    pub encode_params: Option<unsafe extern "C" fn(*const game_params, bool) -> *mut libc::c_char>,
    pub free_params: Option<unsafe extern "C" fn(*mut game_params) -> ()>,
    pub dup_params: Option<unsafe extern "C" fn(*const game_params) -> *mut game_params>,
    pub can_configure: bool,
    pub configure: Option<unsafe extern "C" fn(*const game_params) -> *mut config_item>,
    pub custom_params: Option<unsafe extern "C" fn(*const config_item) -> *mut game_params>,
    pub validate_params:
        Option<unsafe extern "C" fn(*const game_params, bool) -> *const libc::c_char>,
    pub new_desc: Option<
        unsafe extern "C" fn(
            *const game_params,
            *mut random_state,
            *mut *mut libc::c_char,
            bool,
        ) -> *mut libc::c_char,
    >,
    pub validate_desc: Option<
        unsafe extern "C" fn(*const game_params, *const libc::c_char) -> *const libc::c_char,
    >,
    pub new_game: Option<
        unsafe extern "C" fn(
            *mut midend,
            *const game_params,
            *const libc::c_char,
        ) -> *mut game_state,
    >,
    pub dup_game: Option<unsafe extern "C" fn(*const game_state) -> *mut game_state>,
    pub free_game: Option<unsafe extern "C" fn(*mut game_state) -> ()>,
    pub can_solve: bool,
    pub solve: Option<
        unsafe extern "C" fn(
            *const game_state,
            *const game_state,
            *const libc::c_char,
            *mut *const libc::c_char,
        ) -> *mut libc::c_char,
    >,
    pub can_format_as_text_ever: bool,
    pub can_format_as_text_now: Option<unsafe extern "C" fn(*const game_params) -> bool>,
    pub text_format: Option<unsafe extern "C" fn(*const game_state) -> *mut libc::c_char>,
    pub get_prefs: Option<unsafe extern "C" fn(*mut game_ui) -> *mut config_item>,
    pub set_prefs: Option<unsafe extern "C" fn(*mut game_ui, *const config_item) -> ()>,
    pub new_ui: Option<unsafe extern "C" fn(*const game_state) -> *mut game_ui>,
    pub free_ui: Option<unsafe extern "C" fn(*mut game_ui) -> ()>,
    pub encode_ui: Option<unsafe extern "C" fn(*const game_ui) -> *mut libc::c_char>,
    pub decode_ui:
        Option<unsafe extern "C" fn(*mut game_ui, *const libc::c_char, *const game_state) -> ()>,
    pub request_keys:
        Option<unsafe extern "C" fn(*const game_params, *mut libc::c_int) -> *mut key_label>,
    pub changed_state:
        Option<unsafe extern "C" fn(*mut game_ui, *const game_state, *const game_state) -> ()>,
    pub current_key_label: Option<
        unsafe extern "C" fn(*const game_ui, *const game_state, libc::c_int) -> *const libc::c_char,
    >,
    pub interpret_move: Option<
        unsafe extern "C" fn(
            *const game_state,
            *mut game_ui,
            *const game_drawstate,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> *mut libc::c_char,
    >,
    pub execute_move:
        Option<unsafe extern "C" fn(*const game_state, *const libc::c_char) -> *mut game_state>,
    pub preferred_tilesize: libc::c_int,
    pub compute_size: Option<
        unsafe extern "C" fn(
            *const game_params,
            libc::c_int,
            *const game_ui,
            *mut libc::c_int,
            *mut libc::c_int,
        ) -> (),
    >,
    pub set_size: Option<
        unsafe extern "C" fn(
            *mut drawing,
            *mut game_drawstate,
            *const game_params,
            libc::c_int,
        ) -> (),
    >,
    pub colours:
        Option<unsafe extern "C" fn(*mut frontend, *mut libc::c_int) -> *mut libc::c_float>,
    pub new_drawstate:
        Option<unsafe extern "C" fn(*mut drawing, *const game_state) -> *mut game_drawstate>,
    pub free_drawstate: Option<unsafe extern "C" fn(*mut drawing, *mut game_drawstate) -> ()>,
    pub redraw: Option<
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
    pub anim_length: Option<
        unsafe extern "C" fn(
            *const game_state,
            *const game_state,
            libc::c_int,
            *mut game_ui,
        ) -> libc::c_float,
    >,
    pub flash_length: Option<
        unsafe extern "C" fn(
            *const game_state,
            *const game_state,
            libc::c_int,
            *mut game_ui,
        ) -> libc::c_float,
    >,
    pub get_cursor_location: Option<
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
    pub status: Option<unsafe extern "C" fn(*const game_state) -> libc::c_int>,
    pub can_print: bool,
    pub can_print_in_colour: bool,
    pub print_size: Option<
        unsafe extern "C" fn(
            *const game_params,
            *const game_ui,
            *mut libc::c_float,
            *mut libc::c_float,
        ) -> (),
    >,
    pub print: Option<
        unsafe extern "C" fn(*mut drawing, *const game_state, *const game_ui, libc::c_int) -> (),
    >,
    pub wants_statusbar: bool,
    pub is_timed: bool,
    pub timing_state: Option<unsafe extern "C" fn(*const game_state, *mut game_ui) -> bool>,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawing {
    pub dummy: libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blitter {
    pub dummy: libc::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn fatal(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(
        stderr,
        b"fatal error: \0" as *const u8 as *const libc::c_char,
    );
    ap = args.clone();
    vfprintf(stderr, fmt, ap.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
