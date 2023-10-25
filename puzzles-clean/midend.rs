use ::libc;
extern "C" {
    pub type frontend;
    pub type drawing;
    pub type game_state;
    pub type game_ui;
    pub type game_drawstate;
    pub type game_params;
    pub type random_state;
    pub type blitter;
    pub type document;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn deactivate_timer(fe: *mut frontend);
    fn activate_timer(fe: *mut frontend);
    fn get_random_seed(randseed: *mut *mut libc::c_void, randseedsize: *mut libc::c_int);
    fn drawing_new(
        api: *const drawing_api,
        me: *mut midend,
        handle: *mut libc::c_void,
    ) -> *mut drawing;
    fn drawing_free(dr: *mut drawing);
    fn draw_rect(
        dr: *mut drawing,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        colour: libc::c_int,
    );
    fn start_draw(dr: *mut drawing);
    fn draw_update(
        dr: *mut drawing,
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    );
    fn end_draw(dr: *mut drawing);
    fn status_bar(dr: *mut drawing, text: *const libc::c_char);
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn srealloc(p: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn dupstr(s: *const libc::c_char) -> *mut libc::c_char;
    fn free_cfg(cfg: *mut config_item);
    fn obfuscate_bitmap(bmp: *mut libc::c_uchar, bits: libc::c_int, decode: bool);
    fn bin2hex(in_0: *const libc::c_uchar, inlen: libc::c_int) -> *mut libc::c_char;
    fn hex2bin(in_0: *const libc::c_char, outlen: libc::c_int) -> *mut libc::c_uchar;
    fn copy_left_justified(buf: *mut libc::c_char, sz: size_t, str: *const libc::c_char);
    fn button2label(button: libc::c_int) -> *mut libc::c_char;
    fn random_new(seed: *const libc::c_char, len: libc::c_int) -> *mut random_state;
    fn random_upto(state: *mut random_state, limit: libc::c_ulong) -> libc::c_ulong;
    fn random_free(state: *mut random_state);
    fn document_add_puzzle(
        doc: *mut document,
        game: *const game,
        par: *mut game_params,
        ui: *mut game_ui,
        st: *mut game_state,
        st2: *mut game_state,
    );
    static mut MOVE_UI_UPDATE: [libc::c_char; 0];
    static mut MOVE_NO_EFFECT: [libc::c_char; 0];
    static mut MOVE_UNUSED: [libc::c_char; 0];
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
pub struct midend {
    pub frontend: *mut frontend,
    pub random: *mut random_state,
    pub ourgame: *const game,
    pub preset_menu: *mut preset_menu,
    pub encoded_presets: *mut *mut libc::c_char,
    pub n_encoded_presets: libc::c_int,
    pub desc: *mut libc::c_char,
    pub privdesc: *mut libc::c_char,
    pub seedstr: *mut libc::c_char,
    pub aux_info: *mut libc::c_char,
    pub genmode: C2RustUnnamed_5,
    pub nstates: libc::c_int,
    pub statesize: libc::c_int,
    pub statepos: libc::c_int,
    pub states: *mut midend_state_entry,
    pub newgame_undo: midend_serialise_buf,
    pub newgame_redo: midend_serialise_buf,
    pub newgame_can_store_undo: bool,
    pub params: *mut game_params,
    pub curparams: *mut game_params,
    pub drawstate: *mut game_drawstate,
    pub first_draw: bool,
    pub ui: *mut game_ui,
    pub oldstate: *mut game_state,
    pub anim_time: libc::c_float,
    pub anim_pos: libc::c_float,
    pub flash_time: libc::c_float,
    pub flash_pos: libc::c_float,
    pub dir: libc::c_int,
    pub timing: bool,
    pub elapsed: libc::c_float,
    pub laststatus: *mut libc::c_char,
    pub drawing: *mut drawing,
    pub pressed_mouse_button: libc::c_int,
    pub be_prefs: midend_serialise_buf,
    pub preferred_tilesize: libc::c_int,
    pub preferred_tilesize_dpr: libc::c_int,
    pub tilesize: libc::c_int,
    pub winwidth: libc::c_int,
    pub winheight: libc::c_int,
    pub game_id_change_notify_function: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> (),
    >,
    pub game_id_change_notify_ctx: *mut libc::c_void,
    pub one_key_shortcuts: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct midend_serialise_buf {
    pub buf: *mut libc::c_char,
    pub len: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct midend_state_entry {
    pub state: *mut game_state,
    pub movestr: *mut libc::c_char,
    pub movetype: libc::c_int,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const GOT_NOTHING: C2RustUnnamed_5 = 2;
pub const GOT_DESC: C2RustUnnamed_5 = 1;
pub const GOT_SEED: C2RustUnnamed_5 = 0;
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
pub type C2RustUnnamed_6 = libc::c_uint;
pub const C_END: C2RustUnnamed_6 = 3;
pub const C_BOOLEAN: C2RustUnnamed_6 = 2;
pub const C_CHOICES: C2RustUnnamed_6 = 1;
pub const C_STRING: C2RustUnnamed_6 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct midend_serialise_buf_read_ctx {
    pub ser: *mut midend_serialise_buf,
    pub len: libc::c_int,
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub data: *mut libc::c_char,
    pub len: size_t,
    pub size: size_t,
}
pub const NEWGAME: C2RustUnnamed_10 = 0;
pub const MOVE: C2RustUnnamed_10 = 1;
pub const RESTART: C2RustUnnamed_10 = 3;
pub const SOLVE: C2RustUnnamed_10 = 2;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const PKR_UNUSED: C2RustUnnamed_7 = 3;
pub const PKR_NO_EFFECT: C2RustUnnamed_7 = 2;
pub const PKR_SOME_EFFECT: C2RustUnnamed_7 = 1;
pub const PKR_QUIT: C2RustUnnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct newgame_undo_deserialise_check_ctx {
    pub refused: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deserialise_data {
    pub seed: *mut libc::c_char,
    pub parstr: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub privdesc: *mut libc::c_char,
    pub auxinfo: *mut libc::c_char,
    pub uistr: *mut libc::c_char,
    pub cparstr: *mut libc::c_char,
    pub elapsed: libc::c_float,
    pub params: *mut game_params,
    pub cparams: *mut game_params,
    pub ui: *mut game_ui,
    pub states: *mut midend_state_entry,
    pub nstates: libc::c_int,
    pub statepos: libc::c_int,
}
pub type C2RustUnnamed_8 = libc::c_uint;
pub const CFG_FRONTEND_SPECIFIC: C2RustUnnamed_8 = 4;
pub const CFG_PREFS: C2RustUnnamed_8 = 3;
pub const CFG_DESC: C2RustUnnamed_8 = 2;
pub const CFG_SEED: C2RustUnnamed_8 = 1;
pub const CFG_SETTINGS: C2RustUnnamed_8 = 0;
pub const DEF_DESC: C2RustUnnamed_9 = 2;
pub const DEF_SEED: C2RustUnnamed_9 = 1;
pub const DEF_PARAMS: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub type C2RustUnnamed_10 = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn midend_reset_tilesize(mut me: *mut midend) {
    (*me).preferred_tilesize = (*(*me).ourgame).preferred_tilesize;
    (*me).preferred_tilesize_dpr = 1.0f64 as libc::c_int;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut ts: libc::c_int = 0;
    sprintf(
        buf.as_mut_ptr(),
        b"%s_TILESIZE\0" as *const u8 as *const libc::c_char,
        (*(*me).ourgame).name,
    );
    k = 0 as libc::c_int;
    j = k;
    while buf[j as usize] != 0 {
        if *(*__ctype_b_loc())
            .offset(buf[j as usize] as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            let fresh0 = k;
            k = k + 1;
            buf[fresh0
                as usize] = toupper(buf[j as usize] as libc::c_uchar as libc::c_int)
                as libc::c_char;
        }
        j += 1;
        j;
    }
    buf[k as usize] = '\0' as i32 as libc::c_char;
    e = getenv(buf.as_mut_ptr());
    if !e.is_null()
        && sscanf(
            e,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut ts as *mut libc::c_int,
        ) == 1 as libc::c_int && ts > 0 as libc::c_int
    {
        (*me).preferred_tilesize = ts;
    }
}
#[no_mangle]
pub unsafe extern "C" fn midend_new(
    mut fe: *mut frontend,
    mut ourgame: *const game,
    mut drapi: *const drawing_api,
    mut drhandle: *mut libc::c_void,
) -> *mut midend {
    let mut me: *mut midend = smalloc(::core::mem::size_of::<midend>() as libc::c_ulong)
        as *mut midend;
    let mut randseed: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut randseedsize: libc::c_int = 0;
    get_random_seed(&mut randseed, &mut randseedsize);
    (*me).frontend = fe;
    (*me).ourgame = ourgame;
    (*me).random = random_new(randseed as *const libc::c_char, randseedsize);
    (*me).statepos = 0 as libc::c_int;
    (*me).statesize = (*me).statepos;
    (*me).nstates = (*me).statesize;
    (*me).states = 0 as *mut midend_state_entry;
    (*me).newgame_undo.buf = 0 as *mut libc::c_char;
    (*me).newgame_undo.len = 0 as libc::c_int;
    (*me).newgame_undo.size = (*me).newgame_undo.len;
    (*me).newgame_redo.buf = 0 as *mut libc::c_char;
    (*me).newgame_redo.len = 0 as libc::c_int;
    (*me).newgame_redo.size = (*me).newgame_redo.len;
    (*me).newgame_can_store_undo = 0 as libc::c_int != 0;
    (*me).params = ((*ourgame).default_params).expect("non-null function pointer")();
    (*me).game_id_change_notify_function = None;
    (*me).game_id_change_notify_ctx = 0 as *mut libc::c_void;
    (*me).encoded_presets = 0 as *mut *mut libc::c_char;
    (*me).n_encoded_presets = 0 as libc::c_int;
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    sprintf(
        buf.as_mut_ptr(),
        b"%s_DEFAULT\0" as *const u8 as *const libc::c_char,
        (*(*me).ourgame).name,
    );
    k = 0 as libc::c_int;
    j = k;
    while buf[j as usize] != 0 {
        if *(*__ctype_b_loc())
            .offset(buf[j as usize] as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            let fresh1 = k;
            k = k + 1;
            buf[fresh1
                as usize] = toupper(buf[j as usize] as libc::c_uchar as libc::c_int)
                as libc::c_char;
        }
        j += 1;
        j;
    }
    buf[k as usize] = '\0' as i32 as libc::c_char;
    e = getenv(buf.as_mut_ptr());
    if !e.is_null() {
        ((*(*me).ourgame).decode_params)
            .expect("non-null function pointer")((*me).params, e);
    }
    (*me).curparams = 0 as *mut game_params;
    (*me).privdesc = 0 as *mut libc::c_char;
    (*me).desc = (*me).privdesc;
    (*me).seedstr = 0 as *mut libc::c_char;
    (*me).aux_info = 0 as *mut libc::c_char;
    (*me).genmode = GOT_NOTHING;
    (*me).drawstate = 0 as *mut game_drawstate;
    (*me).first_draw = 1 as libc::c_int != 0;
    (*me).oldstate = 0 as *mut game_state;
    (*me).preset_menu = 0 as *mut preset_menu;
    (*me).anim_pos = 0.0f32;
    (*me).anim_time = (*me).anim_pos;
    (*me).flash_pos = 0.0f32;
    (*me).flash_time = (*me).flash_pos;
    (*me).dir = 0 as libc::c_int;
    (*me).ui = 0 as *mut game_ui;
    (*me).pressed_mouse_button = 0 as libc::c_int;
    (*me).laststatus = 0 as *mut libc::c_char;
    (*me).timing = 0 as libc::c_int != 0;
    (*me).elapsed = 0.0f32;
    (*me).winheight = 0 as libc::c_int;
    (*me).winwidth = (*me).winheight;
    (*me).tilesize = (*me).winwidth;
    if !drapi.is_null() {
        (*me).drawing = drawing_new(drapi, me, drhandle);
    } else {
        (*me).drawing = 0 as *mut drawing;
    }
    (*me).be_prefs.buf = 0 as *mut libc::c_char;
    (*me).be_prefs.len = 0 as libc::c_int;
    (*me).be_prefs.size = (*me).be_prefs.len;
    (*me).one_key_shortcuts = 1 as libc::c_int != 0;
    midend_reset_tilesize(me);
    sfree(randseed);
    return me;
}
#[no_mangle]
pub unsafe extern "C" fn midend_which_game(mut me: *mut midend) -> *const game {
    return (*me).ourgame;
}
unsafe extern "C" fn midend_purge_states(mut me: *mut midend) {
    while (*me).nstates > (*me).statepos {
        (*me).nstates -= 1;
        ((*(*me).ourgame).free_game)
            .expect(
                "non-null function pointer",
            )((*((*me).states).offset((*me).nstates as isize)).state);
        if !((*((*me).states).offset((*me).nstates as isize)).movestr).is_null() {
            sfree(
                (*((*me).states).offset((*me).nstates as isize)).movestr
                    as *mut libc::c_void,
            );
        }
    }
    (*me).newgame_redo.len = 0 as libc::c_int;
}
unsafe extern "C" fn midend_free_game(mut me: *mut midend) {
    while (*me).nstates > 0 as libc::c_int {
        (*me).nstates -= 1;
        (*me).nstates;
        ((*(*me).ourgame).free_game)
            .expect(
                "non-null function pointer",
            )((*((*me).states).offset((*me).nstates as isize)).state);
        sfree(
            (*((*me).states).offset((*me).nstates as isize)).movestr as *mut libc::c_void,
        );
    }
    if !((*me).drawstate).is_null() {
        ((*(*me).ourgame).free_drawstate)
            .expect("non-null function pointer")((*me).drawing, (*me).drawstate);
    }
}
unsafe extern "C" fn midend_free_preset_menu(
    mut me: *mut midend,
    mut menu: *mut preset_menu,
) {
    if !menu.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*menu).n_entries {
            sfree((*((*menu).entries).offset(i as isize)).title as *mut libc::c_void);
            if !((*((*menu).entries).offset(i as isize)).params).is_null() {
                ((*(*me).ourgame).free_params)
                    .expect(
                        "non-null function pointer",
                    )((*((*menu).entries).offset(i as isize)).params);
            }
            midend_free_preset_menu(me, (*((*menu).entries).offset(i as isize)).submenu);
            i += 1;
            i;
        }
        sfree((*menu).entries as *mut libc::c_void);
        sfree(menu as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn midend_free(mut me: *mut midend) {
    let mut i: libc::c_int = 0;
    midend_free_game(me);
    i = 0 as libc::c_int;
    while i < (*me).n_encoded_presets {
        sfree(*((*me).encoded_presets).offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    sfree((*me).encoded_presets as *mut libc::c_void);
    if !((*me).drawing).is_null() {
        drawing_free((*me).drawing);
    }
    random_free((*me).random);
    sfree((*me).newgame_undo.buf as *mut libc::c_void);
    sfree((*me).newgame_redo.buf as *mut libc::c_void);
    sfree((*me).states as *mut libc::c_void);
    sfree((*me).desc as *mut libc::c_void);
    sfree((*me).privdesc as *mut libc::c_void);
    sfree((*me).seedstr as *mut libc::c_void);
    sfree((*me).aux_info as *mut libc::c_void);
    sfree((*me).be_prefs.buf as *mut libc::c_void);
    ((*(*me).ourgame).free_params).expect("non-null function pointer")((*me).params);
    midend_free_preset_menu(me, (*me).preset_menu);
    if !((*me).ui).is_null() {
        ((*(*me).ourgame).free_ui).expect("non-null function pointer")((*me).ui);
    }
    if !((*me).curparams).is_null() {
        ((*(*me).ourgame).free_params)
            .expect("non-null function pointer")((*me).curparams);
    }
    sfree((*me).laststatus as *mut libc::c_void);
    sfree(me as *mut libc::c_void);
}
unsafe extern "C" fn midend_size_new_drawstate(mut me: *mut midend) {
    if (*me).tilesize > 0 as libc::c_int {
        ((*(*me).ourgame).compute_size)
            .expect(
                "non-null function pointer",
            )(
            (*me).params,
            (*me).tilesize,
            (*me).ui,
            &mut (*me).winwidth,
            &mut (*me).winheight,
        );
        ((*(*me).ourgame).set_size)
            .expect(
                "non-null function pointer",
            )((*me).drawing, (*me).drawstate, (*me).params, (*me).tilesize);
    }
}
unsafe extern "C" fn convert_tilesize(
    mut me: *mut midend,
    mut old_tilesize: libc::c_int,
    mut old_dpr: libc::c_double,
    mut new_dpr: libc::c_double,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut rx: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut defaults: *mut game_params = 0 as *mut game_params;
    if new_dpr == old_dpr {
        return old_tilesize;
    }
    defaults = ((*(*me).ourgame).default_params).expect("non-null function pointer")();
    ((*(*me).ourgame).compute_size)
        .expect(
            "non-null function pointer",
        )(defaults, old_tilesize, (*me).ui, &mut x, &mut y);
    x = (x as libc::c_double * (new_dpr / old_dpr)) as libc::c_int;
    y = (y as libc::c_double * (new_dpr / old_dpr)) as libc::c_int;
    max = 1 as libc::c_int;
    min = max;
    loop {
        max *= 2 as libc::c_int;
        ((*(*me).ourgame).compute_size)
            .expect(
                "non-null function pointer",
            )(defaults, max, (*me).ui, &mut rx, &mut ry);
        if !(rx <= x && ry <= y) {
            break;
        }
    }
    while max - min > 1 as libc::c_int {
        let mut mid: libc::c_int = (max + min) / 2 as libc::c_int;
        ((*(*me).ourgame).compute_size)
            .expect(
                "non-null function pointer",
            )(defaults, mid, (*me).ui, &mut rx, &mut ry);
        if rx <= x && ry <= y {
            min = mid;
        } else {
            max = mid;
        }
    }
    ((*(*me).ourgame).free_params).expect("non-null function pointer")(defaults);
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn midend_size(
    mut me: *mut midend,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut user_size: bool,
    mut device_pixel_ratio: libc::c_double,
) {
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut rx: libc::c_int = 0;
    let mut ry: libc::c_int = 0;
    if !((*me).drawstate).is_null() && (*me).tilesize > 0 as libc::c_int {
        ((*(*me).ourgame).free_drawstate)
            .expect("non-null function pointer")((*me).drawing, (*me).drawstate);
        (*me)
            .drawstate = ((*(*me).ourgame).new_drawstate)
            .expect(
                "non-null function pointer",
            )((*me).drawing, (*((*me).states).offset(0 as libc::c_int as isize)).state);
        (*me).first_draw = 1 as libc::c_int != 0;
    }
    if user_size {
        max = 1 as libc::c_int;
        loop {
            max *= 2 as libc::c_int;
            ((*(*me).ourgame).compute_size)
                .expect(
                    "non-null function pointer",
                )((*me).params, max, (*me).ui, &mut rx, &mut ry);
            if !(rx <= *x && ry <= *y) {
                break;
            }
        }
    } else {
        max = convert_tilesize(
            me,
            (*me).preferred_tilesize,
            (*me).preferred_tilesize_dpr as libc::c_double,
            device_pixel_ratio,
        ) + 1 as libc::c_int;
    }
    min = 1 as libc::c_int;
    while max - min > 1 as libc::c_int {
        let mut mid: libc::c_int = (max + min) / 2 as libc::c_int;
        ((*(*me).ourgame).compute_size)
            .expect(
                "non-null function pointer",
            )((*me).params, mid, (*me).ui, &mut rx, &mut ry);
        if rx <= *x && ry <= *y {
            min = mid;
        } else {
            max = mid;
        }
    }
    (*me).tilesize = min;
    if user_size {
        (*me).preferred_tilesize = (*me).tilesize;
        (*me).preferred_tilesize_dpr = device_pixel_ratio as libc::c_int;
    }
    midend_size_new_drawstate(me);
    *x = (*me).winwidth;
    *y = (*me).winheight;
}
#[no_mangle]
pub unsafe extern "C" fn midend_tilesize(mut me: *mut midend) -> libc::c_int {
    return (*me).tilesize;
}
#[no_mangle]
pub unsafe extern "C" fn midend_set_params(
    mut me: *mut midend,
    mut params: *mut game_params,
) {
    ((*(*me).ourgame).free_params).expect("non-null function pointer")((*me).params);
    (*me)
        .params = ((*(*me).ourgame).dup_params)
        .expect("non-null function pointer")(params);
}
#[no_mangle]
pub unsafe extern "C" fn midend_get_params(mut me: *mut midend) -> *mut game_params {
    return ((*(*me).ourgame).dup_params)
        .expect("non-null function pointer")((*me).params);
}
unsafe extern "C" fn encode_params(
    mut me: *mut midend,
    mut params: *const game_params,
    mut full: bool,
) -> *mut libc::c_char {
    let mut encoded: *mut libc::c_char = ((*(*me).ourgame).encode_params)
        .expect("non-null function pointer")(params, full);
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *encoded.offset(i as isize) != 0 {
        if *encoded.offset(i as isize) as libc::c_int >= 32 as libc::c_int
            && (*encoded.offset(i as isize) as libc::c_int) < 127 as libc::c_int
            && *encoded.offset(i as isize) as libc::c_int != '#' as i32
            && *encoded.offset(i as isize) as libc::c_int != ':' as i32
        {} else {
            __assert_fail(
                b"encoded[i] >= 32 && encoded[i] < 127 && encoded[i] != '#' && encoded[i] != ':'\0"
                    as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                469 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"char *encode_params(midend *, const game_params *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_9790: {
            if *encoded.offset(i as isize) as libc::c_int >= 32 as libc::c_int
                && (*encoded.offset(i as isize) as libc::c_int) < 127 as libc::c_int
                && *encoded.offset(i as isize) as libc::c_int != '#' as i32
                && *encoded.offset(i as isize) as libc::c_int != ':' as i32
            {} else {
                __assert_fail(
                    b"encoded[i] >= 32 && encoded[i] < 127 && encoded[i] != '#' && encoded[i] != ':'\0"
                        as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    469 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 58],
                        &[libc::c_char; 58],
                    >(b"char *encode_params(midend *, const game_params *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    return encoded;
}
unsafe extern "C" fn assert_printable_ascii(mut s: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *s.offset(i as isize) != 0 {
        if *s.offset(i as isize) as libc::c_int >= 32 as libc::c_int
            && (*s.offset(i as isize) as libc::c_int) < 127 as libc::c_int
        {} else {
            __assert_fail(
                b"s[i] >= 32 && s[i] < 127\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                479 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void assert_printable_ascii(const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_7195: {
            if *s.offset(i as isize) as libc::c_int >= 32 as libc::c_int
                && (*s.offset(i as isize) as libc::c_int) < 127 as libc::c_int
            {} else {
                __assert_fail(
                    b"s[i] >= 32 && s[i] < 127\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    479 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"void assert_printable_ascii(const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
}
unsafe extern "C" fn midend_set_timer(mut me: *mut midend) {
    (*me)
        .timing = (*(*me).ourgame).is_timed as libc::c_int != 0
        && ((*(*me).ourgame).timing_state)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
            (*me).ui,
        ) as libc::c_int != 0;
    if (*me).timing as libc::c_int != 0 || (*me).flash_time != 0.
        || (*me).anim_time != 0.
    {
        activate_timer((*me).frontend);
    } else {
        deactivate_timer((*me).frontend);
    };
}
#[no_mangle]
pub unsafe extern "C" fn midend_force_redraw(mut me: *mut midend) {
    if !((*me).drawstate).is_null() {
        ((*(*me).ourgame).free_drawstate)
            .expect("non-null function pointer")((*me).drawing, (*me).drawstate);
    }
    (*me)
        .drawstate = ((*(*me).ourgame).new_drawstate)
        .expect(
            "non-null function pointer",
        )((*me).drawing, (*((*me).states).offset(0 as libc::c_int as isize)).state);
    (*me).first_draw = 1 as libc::c_int != 0;
    midend_size_new_drawstate(me);
    midend_redraw(me);
}
unsafe extern "C" fn midend_serialise_buf_write(
    mut ctx: *mut libc::c_void,
    mut buf: *const libc::c_void,
    mut len: libc::c_int,
) {
    let mut ser: *mut midend_serialise_buf = ctx as *mut midend_serialise_buf;
    let mut new_len: libc::c_int = 0;
    if len < 2147483647 as libc::c_int - (*ser).len {} else {
        __assert_fail(
            b"len < INT_MAX - ser->len\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[libc::c_char; 59],
            >(b"void midend_serialise_buf_write(void *, const void *, int)\0"))
                .as_ptr(),
        );
    }
    'c_5133: {
        if len < 2147483647 as libc::c_int - (*ser).len {} else {
            __assert_fail(
                b"len < INT_MAX - ser->len\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                509 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[libc::c_char; 59],
                >(b"void midend_serialise_buf_write(void *, const void *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    new_len = (*ser).len + len;
    if new_len > (*ser).size {
        (*ser).size = new_len + new_len / 4 as libc::c_int + 1024 as libc::c_int;
        (*ser)
            .buf = srealloc(
            (*ser).buf as *mut libc::c_void,
            ((*ser).size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    memcpy(
        ((*ser).buf).offset((*ser).len as isize) as *mut libc::c_void,
        buf,
        len as libc::c_ulong,
    );
    (*ser).len = new_len;
}
unsafe extern "C" fn midend_serialise_buf_read(
    mut ctx: *mut libc::c_void,
    mut buf: *mut libc::c_void,
    mut len: libc::c_int,
) -> bool {
    let rctx: *mut midend_serialise_buf_read_ctx = ctx
        as *mut midend_serialise_buf_read_ctx;
    if len > (*rctx).len - (*rctx).pos {
        return 0 as libc::c_int != 0;
    }
    memcpy(
        buf,
        ((*(*rctx).ser).buf).offset((*rctx).pos as isize) as *const libc::c_void,
        len as libc::c_ulong,
    );
    (*rctx).pos += len;
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn midend_new_game(mut me: *mut midend) {
    (*me).newgame_undo.len = 0 as libc::c_int;
    if (*me).newgame_can_store_undo {
        midend_purge_states(me);
        midend_serialise(
            me,
            Some(
                midend_serialise_buf_write
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            &mut (*me).newgame_undo as *mut midend_serialise_buf as *mut libc::c_void,
        );
    }
    midend_stop_anim(me);
    midend_free_game(me);
    if (*me).nstates == 0 as libc::c_int {} else {
        __assert_fail(
            b"me->nstates == 0\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            556 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void midend_new_game(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_7512: {
        if (*me).nstates == 0 as libc::c_int {} else {
            __assert_fail(
                b"me->nstates == 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                556 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void midend_new_game(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*me).genmode as libc::c_uint == GOT_DESC as libc::c_int as libc::c_uint {
        (*me).genmode = GOT_NOTHING;
    } else {
        let mut rs: *mut random_state = 0 as *mut random_state;
        if (*me).genmode as libc::c_uint == GOT_SEED as libc::c_int as libc::c_uint {
            (*me).genmode = GOT_NOTHING;
        } else {
            let mut newseed: [libc::c_char; 16] = [0; 16];
            let mut i: libc::c_int = 0;
            newseed[15 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            newseed[0 as libc::c_int
                as usize] = ('1' as i32
                + random_upto((*me).random, 9 as libc::c_int as libc::c_ulong)
                    as libc::c_char as libc::c_int) as libc::c_char;
            i = 1 as libc::c_int;
            while i < 15 as libc::c_int {
                newseed[i
                    as usize] = ('0' as i32
                    + random_upto((*me).random, 10 as libc::c_int as libc::c_ulong)
                        as libc::c_char as libc::c_int) as libc::c_char;
                i += 1;
                i;
            }
            sfree((*me).seedstr as *mut libc::c_void);
            (*me).seedstr = dupstr(newseed.as_mut_ptr());
            if !((*me).curparams).is_null() {
                ((*(*me).ourgame).free_params)
                    .expect("non-null function pointer")((*me).curparams);
            }
            (*me)
                .curparams = ((*(*me).ourgame).dup_params)
                .expect("non-null function pointer")((*me).params);
        }
        sfree((*me).desc as *mut libc::c_void);
        sfree((*me).privdesc as *mut libc::c_void);
        sfree((*me).aux_info as *mut libc::c_void);
        (*me).aux_info = 0 as *mut libc::c_char;
        rs = random_new((*me).seedstr, strlen((*me).seedstr) as libc::c_int);
        (*me)
            .desc = ((*(*me).ourgame).new_desc)
            .expect(
                "non-null function pointer",
            )((*me).curparams, rs, &mut (*me).aux_info, !((*me).drawing).is_null());
        assert_printable_ascii((*me).desc);
        (*me).privdesc = 0 as *mut libc::c_char;
        random_free(rs);
    }
    if (*me).nstates >= (*me).statesize {
        (*me).statesize = (*me).nstates + 128 as libc::c_int;
        (*me)
            .states = srealloc(
            (*me).states as *mut libc::c_void,
            ((*me).statesize as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<midend_state_entry>() as libc::c_ulong,
                ),
        ) as *mut midend_state_entry;
    }
    let ref mut fresh2 = (*((*me).states).offset((*me).nstates as isize)).state;
    *fresh2 = ((*(*me).ourgame).new_game)
        .expect("non-null function pointer")(me, (*me).params, (*me).desc);
    if (*(*me).ourgame).can_solve as libc::c_int != 0 && !((*me).aux_info).is_null() {
        let mut s: *mut game_state = 0 as *mut game_state;
        let mut msg: *const libc::c_char = 0 as *const libc::c_char;
        let mut movestr: *mut libc::c_char = 0 as *mut libc::c_char;
        msg = 0 as *const libc::c_char;
        movestr = ((*(*me).ourgame).solve)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(0 as libc::c_int as isize)).state,
            (*((*me).states).offset(0 as libc::c_int as isize)).state,
            (*me).aux_info,
            &mut msg,
        );
        if !movestr.is_null() && msg.is_null() {} else {
            __assert_fail(
                b"movestr && !msg\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                639 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void midend_new_game(midend *)\0"))
                    .as_ptr(),
            );
        }
        'c_6971: {
            if !movestr.is_null() && msg.is_null() {} else {
                __assert_fail(
                    b"movestr && !msg\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    639 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void midend_new_game(midend *)\0"))
                        .as_ptr(),
                );
            }
        };
        s = ((*(*me).ourgame).execute_move)
            .expect(
                "non-null function pointer",
            )((*((*me).states).offset(0 as libc::c_int as isize)).state, movestr);
        if !s.is_null() {} else {
            __assert_fail(
                b"s\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                641 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void midend_new_game(midend *)\0"))
                    .as_ptr(),
            );
        }
        'c_6917: {
            if !s.is_null() {} else {
                __assert_fail(
                    b"s\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    641 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void midend_new_game(midend *)\0"))
                        .as_ptr(),
                );
            }
        };
        ((*(*me).ourgame).free_game).expect("non-null function pointer")(s);
        sfree(movestr as *mut libc::c_void);
    }
    let ref mut fresh3 = (*((*me).states).offset((*me).nstates as isize)).movestr;
    *fresh3 = 0 as *mut libc::c_char;
    (*((*me).states).offset((*me).nstates as isize)).movetype = NEWGAME as libc::c_int;
    (*me).nstates += 1;
    (*me).nstates;
    (*me).statepos = 1 as libc::c_int;
    (*me)
        .drawstate = ((*(*me).ourgame).new_drawstate)
        .expect(
            "non-null function pointer",
        )((*me).drawing, (*((*me).states).offset(0 as libc::c_int as isize)).state);
    (*me).first_draw = 1 as libc::c_int != 0;
    midend_size_new_drawstate(me);
    (*me).elapsed = 0.0f32;
    (*me).flash_time = 0.0f32;
    (*me).flash_pos = (*me).flash_time;
    (*me).anim_time = 0.0f32;
    (*me).anim_pos = (*me).anim_time;
    if !((*me).ui).is_null() {
        ((*(*me).ourgame).free_ui).expect("non-null function pointer")((*me).ui);
    }
    (*me)
        .ui = ((*(*me).ourgame).new_ui)
        .expect(
            "non-null function pointer",
        )((*((*me).states).offset(0 as libc::c_int as isize)).state);
    midend_apply_prefs(me, (*me).ui);
    midend_set_timer(me);
    (*me).pressed_mouse_button = 0 as libc::c_int;
    if ((*me).game_id_change_notify_function).is_some() {
        ((*me).game_id_change_notify_function)
            .expect("non-null function pointer")((*me).game_id_change_notify_ctx);
    }
    (*me).newgame_can_store_undo = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn midend_load_prefs(
    mut me: *mut midend,
    mut read: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, libc::c_int) -> bool,
    >,
    mut rctx: *mut libc::c_void,
) -> *const libc::c_char {
    let mut err: *const libc::c_char = midend_deserialise_prefs(
        me,
        0 as *mut game_ui,
        read,
        rctx,
    );
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn midend_save_prefs(
    mut me: *mut midend,
    mut write: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void, libc::c_int) -> (),
    >,
    mut wctx: *mut libc::c_void,
) {
    midend_serialise_prefs(me, 0 as *mut game_ui, write, wctx);
}
#[no_mangle]
pub unsafe extern "C" fn midend_can_undo(mut me: *mut midend) -> bool {
    return (*me).statepos > 1 as libc::c_int || (*me).newgame_undo.len != 0;
}
#[no_mangle]
pub unsafe extern "C" fn midend_can_redo(mut me: *mut midend) -> bool {
    return (*me).statepos < (*me).nstates || (*me).newgame_redo.len != 0;
}
unsafe extern "C" fn newgame_undo_deserialise_check(
    mut vctx: *mut libc::c_void,
    mut me: *mut midend,
    mut data: *const deserialise_data,
) -> *const libc::c_char {
    let mut ctx: *mut newgame_undo_deserialise_check_ctx = vctx
        as *mut newgame_undo_deserialise_check_ctx;
    let mut old: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    old = encode_params(me, (*me).params, 1 as libc::c_int != 0);
    new = encode_params(me, (*data).params, 1 as libc::c_int != 0);
    if strcmp(old, new) != 0 {
        (*ctx).refused = 1 as libc::c_int != 0;
        return b"Undoing this new-game operation would change params\0" as *const u8
            as *const libc::c_char;
    }
    old = encode_params(me, (*me).curparams, 1 as libc::c_int != 0);
    new = encode_params(me, (*data).cparams, 1 as libc::c_int != 0);
    if strcmp(old, new) != 0 {
        (*ctx).refused = 1 as libc::c_int != 0;
        return b"Undoing this new-game operation would change params\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn midend_undo(mut me: *mut midend) -> bool {
    let mut deserialise_error: *const libc::c_char = 0 as *const libc::c_char;
    if (*me).statepos > 1 as libc::c_int {
        if !((*me).ui).is_null() {
            ((*(*me).ourgame).changed_state)
                .expect(
                    "non-null function pointer",
                )(
                (*me).ui,
                (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                    .state,
                (*((*me).states).offset(((*me).statepos - 2 as libc::c_int) as isize))
                    .state,
            );
        }
        (*me).statepos -= 1;
        (*me).statepos;
        (*me).dir = -(1 as libc::c_int);
        return 1 as libc::c_int != 0;
    } else if (*me).newgame_undo.len != 0 {
        let mut rctx: midend_serialise_buf_read_ctx = midend_serialise_buf_read_ctx {
            ser: 0 as *mut midend_serialise_buf,
            len: 0,
            pos: 0,
        };
        let mut cctx: newgame_undo_deserialise_check_ctx = newgame_undo_deserialise_check_ctx {
            refused: false,
        };
        let mut serbuf: midend_serialise_buf = midend_serialise_buf {
            buf: 0 as *mut libc::c_char,
            len: 0,
            size: 0,
        };
        serbuf.buf = 0 as *mut libc::c_char;
        serbuf.size = 0 as libc::c_int;
        serbuf.len = serbuf.size;
        midend_serialise(
            me,
            Some(
                midend_serialise_buf_write
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            &mut serbuf as *mut midend_serialise_buf as *mut libc::c_void,
        );
        rctx.ser = &mut (*me).newgame_undo;
        rctx.len = (*me).newgame_undo.len;
        rctx.pos = 0 as libc::c_int;
        cctx.refused = 0 as libc::c_int != 0;
        deserialise_error = midend_deserialise_internal(
            me,
            Some(
                midend_serialise_buf_read
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> bool,
            ),
            &mut rctx as *mut midend_serialise_buf_read_ctx as *mut libc::c_void,
            Some(
                newgame_undo_deserialise_check
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut midend,
                        *const deserialise_data,
                    ) -> *const libc::c_char,
            ),
            &mut cctx as *mut newgame_undo_deserialise_check_ctx as *mut libc::c_void,
        );
        if cctx.refused {
            sfree(serbuf.buf as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        } else {
            if deserialise_error.is_null() {} else {
                __assert_fail(
                    b"!deserialise_error\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    800 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 28],
                        &[libc::c_char; 28],
                    >(b"_Bool midend_undo(midend *)\0"))
                        .as_ptr(),
                );
            }
            'c_14984: {
                if deserialise_error.is_null() {} else {
                    __assert_fail(
                        b"!deserialise_error\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        800 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 28],
                            &[libc::c_char; 28],
                        >(b"_Bool midend_undo(midend *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*me).newgame_undo.len = 0 as libc::c_int;
            (*me).newgame_redo.len = 0 as libc::c_int;
            midend_serialise_buf_write(
                &mut (*me).newgame_redo as *mut midend_serialise_buf
                    as *mut libc::c_void,
                serbuf.buf as *const libc::c_void,
                serbuf.len,
            );
            sfree(serbuf.buf as *mut libc::c_void);
            return 1 as libc::c_int != 0;
        }
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn midend_redo(mut me: *mut midend) -> bool {
    let mut deserialise_error: *const libc::c_char = 0 as *const libc::c_char;
    if (*me).statepos < (*me).nstates {
        if !((*me).ui).is_null() {
            ((*(*me).ourgame).changed_state)
                .expect(
                    "non-null function pointer",
                )(
                (*me).ui,
                (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                    .state,
                (*((*me).states).offset((*me).statepos as isize)).state,
            );
        }
        (*me).statepos += 1;
        (*me).statepos;
        (*me).dir = 1 as libc::c_int;
        return 1 as libc::c_int != 0;
    } else if (*me).newgame_redo.len != 0 {
        let mut rctx: midend_serialise_buf_read_ctx = midend_serialise_buf_read_ctx {
            ser: 0 as *mut midend_serialise_buf,
            len: 0,
            pos: 0,
        };
        let mut cctx: newgame_undo_deserialise_check_ctx = newgame_undo_deserialise_check_ctx {
            refused: false,
        };
        let mut serbuf: midend_serialise_buf = midend_serialise_buf {
            buf: 0 as *mut libc::c_char,
            len: 0,
            size: 0,
        };
        serbuf.buf = 0 as *mut libc::c_char;
        serbuf.size = 0 as libc::c_int;
        serbuf.len = serbuf.size;
        midend_serialise(
            me,
            Some(
                midend_serialise_buf_write
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            &mut serbuf as *mut midend_serialise_buf as *mut libc::c_void,
        );
        rctx.ser = &mut (*me).newgame_redo;
        rctx.len = (*me).newgame_redo.len;
        rctx.pos = 0 as libc::c_int;
        cctx.refused = 0 as libc::c_int != 0;
        deserialise_error = midend_deserialise_internal(
            me,
            Some(
                midend_serialise_buf_read
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> bool,
            ),
            &mut rctx as *mut midend_serialise_buf_read_ctx as *mut libc::c_void,
            Some(
                newgame_undo_deserialise_check
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut midend,
                        *const deserialise_data,
                    ) -> *const libc::c_char,
            ),
            &mut cctx as *mut newgame_undo_deserialise_check_ctx as *mut libc::c_void,
        );
        if cctx.refused {
            sfree(serbuf.buf as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        } else {
            if deserialise_error.is_null() {} else {
                __assert_fail(
                    b"!deserialise_error\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    874 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 28],
                        &[libc::c_char; 28],
                    >(b"_Bool midend_redo(midend *)\0"))
                        .as_ptr(),
                );
            }
            'c_12145: {
                if deserialise_error.is_null() {} else {
                    __assert_fail(
                        b"!deserialise_error\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        874 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 28],
                            &[libc::c_char; 28],
                        >(b"_Bool midend_redo(midend *)\0"))
                            .as_ptr(),
                    );
                }
            };
            (*me).newgame_redo.len = 0 as libc::c_int;
            (*me).newgame_undo.len = 0 as libc::c_int;
            midend_serialise_buf_write(
                &mut (*me).newgame_undo as *mut midend_serialise_buf
                    as *mut libc::c_void,
                serbuf.buf as *const libc::c_void,
                serbuf.len,
            );
            sfree(serbuf.buf as *mut libc::c_void);
            return 1 as libc::c_int != 0;
        }
    } else {
        return 0 as libc::c_int != 0
    };
}
unsafe extern "C" fn midend_finish_move(mut me: *mut midend) {
    let mut flashtime: libc::c_float = 0.;
    if (!((*me).oldstate).is_null() || (*me).statepos > 1 as libc::c_int)
        && ((*me).dir > 0 as libc::c_int
            && !((*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                .movetype != MOVE as libc::c_int)
            || (*me).dir < 0 as libc::c_int && (*me).statepos < (*me).nstates
                && !((*((*me).states).offset((*me).statepos as isize)).movetype
                    != MOVE as libc::c_int))
    {
        flashtime = ((*(*me).ourgame).flash_length)
            .expect(
                "non-null function pointer",
            )(
            if !((*me).oldstate).is_null() {
                (*me).oldstate
            } else {
                (*((*me).states).offset(((*me).statepos - 2 as libc::c_int) as isize))
                    .state
            },
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
            if !((*me).oldstate).is_null() { (*me).dir } else { 1 as libc::c_int },
            (*me).ui,
        );
        if flashtime > 0 as libc::c_int as libc::c_float {
            (*me).flash_pos = 0.0f32;
            (*me).flash_time = flashtime;
        }
    }
    if !((*me).oldstate).is_null() {
        ((*(*me).ourgame).free_game).expect("non-null function pointer")((*me).oldstate);
    }
    (*me).oldstate = 0 as *mut game_state;
    (*me).anim_time = 0 as libc::c_int as libc::c_float;
    (*me).anim_pos = (*me).anim_time;
    (*me).dir = 0 as libc::c_int;
    midend_set_timer(me);
}
#[no_mangle]
pub unsafe extern "C" fn midend_stop_anim(mut me: *mut midend) {
    if !((*me).oldstate).is_null()
        || (*me).anim_time != 0 as libc::c_int as libc::c_float
    {
        midend_finish_move(me);
        midend_redraw(me);
    }
}
#[no_mangle]
pub unsafe extern "C" fn midend_restart_game(mut me: *mut midend) {
    let mut s: *mut game_state = 0 as *mut game_state;
    if (*me).statepos >= 1 as libc::c_int {} else {
        __assert_fail(
            b"me->statepos >= 1\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            943 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void midend_restart_game(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_10736: {
        if (*me).statepos >= 1 as libc::c_int {} else {
            __assert_fail(
                b"me->statepos >= 1\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                943 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void midend_restart_game(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*me).statepos == 1 as libc::c_int {
        return;
    }
    s = ((*(*me).ourgame).new_game)
        .expect("non-null function pointer")(me, (*me).params, (*me).desc);
    midend_stop_anim(me);
    midend_purge_states(me);
    if (*me).nstates >= (*me).statesize {
        (*me).statesize = (*me).nstates + 128 as libc::c_int;
        (*me)
            .states = srealloc(
            (*me).states as *mut libc::c_void,
            ((*me).statesize as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<midend_state_entry>() as libc::c_ulong,
                ),
        ) as *mut midend_state_entry;
    }
    let ref mut fresh4 = (*((*me).states).offset((*me).nstates as isize)).state;
    *fresh4 = s;
    let ref mut fresh5 = (*((*me).states).offset((*me).nstates as isize)).movestr;
    *fresh5 = dupstr((*me).desc);
    (*((*me).states).offset((*me).nstates as isize)).movetype = RESTART as libc::c_int;
    (*me).nstates += 1;
    (*me).statepos = (*me).nstates;
    if !((*me).ui).is_null() {
        ((*(*me).ourgame).changed_state)
            .expect(
                "non-null function pointer",
            )(
            (*me).ui,
            (*((*me).states).offset(((*me).statepos - 2 as libc::c_int) as isize)).state,
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
        );
    }
    (*me).flash_time = 0.0f32;
    (*me).flash_pos = (*me).flash_time;
    midend_finish_move(me);
    midend_redraw(me);
    midend_set_timer(me);
}
unsafe extern "C" fn midend_really_process_key(
    mut me: *mut midend,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut button: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut oldstate: *mut game_state = ((*(*me).ourgame).dup_game)
        .expect(
            "non-null function pointer",
        )((*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state);
    let mut type_0: libc::c_int = MOVE as libc::c_int;
    let mut gottype: bool = 0 as libc::c_int != 0;
    let mut ret: libc::c_int = PKR_NO_EFFECT as libc::c_int;
    let mut anim_time: libc::c_float = 0.;
    let mut s: *mut game_state = 0 as *mut game_state;
    let mut movestr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(button > UI_LOWER_BOUND as libc::c_int
        && button < UI_UPPER_BOUND as libc::c_int)
    {
        movestr = ((*(*me).ourgame).interpret_move)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
            (*me).ui,
            (*me).drawstate,
            x,
            y,
            button,
        );
    }
    if movestr.is_null() || movestr == MOVE_UNUSED.as_mut_ptr() {
        if (*me).one_key_shortcuts as libc::c_int != 0
            && (button == 'n' as i32 || button == 'N' as i32) || button == '\u{e}' as i32
            || button == UI_NEWGAME as libc::c_int
        {
            midend_new_game(me);
            midend_redraw(me);
            ret = PKR_SOME_EFFECT as libc::c_int;
            current_block = 7514209646974688516;
        } else if (*me).one_key_shortcuts as libc::c_int != 0
            && (button == 'u' as i32 || button == 'U' as i32) || button == '*' as i32
            || button == '\u{1a}' as i32 || button == '\u{1f}' as i32
            || button == UI_UNDO as libc::c_int
        {
            midend_stop_anim(me);
            type_0 = (*((*me).states)
                .offset(((*me).statepos - 1 as libc::c_int) as isize))
                .movetype;
            gottype = 1 as libc::c_int != 0;
            if !midend_undo(me) {
                current_block = 7514209646974688516;
            } else {
                ret = PKR_SOME_EFFECT as libc::c_int;
                current_block = 3689906465960840878;
            }
        } else if (*me).one_key_shortcuts as libc::c_int != 0
            && (button == 'r' as i32 || button == 'R' as i32) || button == '#' as i32
            || button == '\u{12}' as i32 || button == '\u{19}' as i32
            || button == UI_REDO as libc::c_int
        {
            midend_stop_anim(me);
            if !midend_redo(me) {
                current_block = 7514209646974688516;
            } else {
                ret = PKR_SOME_EFFECT as libc::c_int;
                current_block = 3689906465960840878;
            }
        } else if (button == '\u{13}' as i32 || button == UI_SOLVE as libc::c_int)
            && (*(*me).ourgame).can_solve as libc::c_int != 0
        {
            ret = PKR_SOME_EFFECT as libc::c_int;
            if !(midend_solve(me)).is_null() {
                current_block = 7514209646974688516;
            } else {
                current_block = 3689906465960840878;
            }
        } else {
            if (*me).one_key_shortcuts as libc::c_int != 0
                && (button == 'q' as i32 || button == 'Q' as i32)
                || button == '\u{11}' as i32 || button == UI_QUIT as libc::c_int
            {
                ret = PKR_QUIT as libc::c_int;
            } else {
                ret = PKR_UNUSED as libc::c_int;
            }
            current_block = 7514209646974688516;
        }
    } else if movestr == MOVE_NO_EFFECT.as_mut_ptr() {
        ret = PKR_NO_EFFECT as libc::c_int;
        current_block = 7514209646974688516;
    } else {
        ret = PKR_SOME_EFFECT as libc::c_int;
        if movestr == MOVE_UI_UPDATE.as_mut_ptr() {
            s = (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                .state;
        } else {
            assert_printable_ascii(movestr);
            s = ((*(*me).ourgame).execute_move)
                .expect(
                    "non-null function pointer",
                )(
                (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                    .state,
                movestr,
            );
            if !s.is_null() {} else {
                __assert_fail(
                    b"s != NULL\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1040 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int midend_really_process_key(midend *, int, int, int)\0"))
                        .as_ptr(),
                );
            }
            'c_11367: {
                if !s.is_null() {} else {
                    __assert_fail(
                        b"s != NULL\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        1040 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"int midend_really_process_key(midend *, int, int, int)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        if s
            == (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                .state
        {
            midend_redraw(me);
            midend_set_timer(me);
            current_block = 7514209646974688516;
        } else if !s.is_null() {
            midend_stop_anim(me);
            midend_purge_states(me);
            if (*me).nstates >= (*me).statesize {
                (*me).statesize = (*me).nstates + 128 as libc::c_int;
                (*me)
                    .states = srealloc(
                    (*me).states as *mut libc::c_void,
                    ((*me).statesize as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<midend_state_entry>() as libc::c_ulong,
                        ),
                ) as *mut midend_state_entry;
            }
            if !movestr.is_null() {} else {
                __assert_fail(
                    b"movestr != NULL\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1056 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"int midend_really_process_key(midend *, int, int, int)\0"))
                        .as_ptr(),
                );
            }
            'c_11224: {
                if !movestr.is_null() {} else {
                    __assert_fail(
                        b"movestr != NULL\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        1056 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"int midend_really_process_key(midend *, int, int, int)\0"))
                            .as_ptr(),
                    );
                }
            };
            let ref mut fresh6 = (*((*me).states).offset((*me).nstates as isize)).state;
            *fresh6 = s;
            let ref mut fresh7 = (*((*me).states).offset((*me).nstates as isize))
                .movestr;
            *fresh7 = movestr;
            (*((*me).states).offset((*me).nstates as isize))
                .movetype = MOVE as libc::c_int;
            (*me).nstates += 1;
            (*me).statepos = (*me).nstates;
            (*me).dir = 1 as libc::c_int;
            if !((*me).ui).is_null() {
                ((*(*me).ourgame).changed_state)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*me).ui,
                    (*((*me).states)
                        .offset(((*me).statepos - 2 as libc::c_int) as isize))
                        .state,
                    (*((*me).states)
                        .offset(((*me).statepos - 1 as libc::c_int) as isize))
                        .state,
                );
            }
            current_block = 3689906465960840878;
        } else {
            current_block = 7514209646974688516;
        }
    }
    match current_block {
        3689906465960840878 => {
            if !gottype {
                type_0 = (*((*me).states)
                    .offset(((*me).statepos - 1 as libc::c_int) as isize))
                    .movetype;
            }
            if type_0 != MOVE as libc::c_int
                && !(type_0 == SOLVE as libc::c_int
                    && (*(*me).ourgame).flags & (1 as libc::c_int) << 9 as libc::c_int
                        != 0)
            {
                anim_time = 0 as libc::c_int as libc::c_float;
            } else {
                anim_time = ((*(*me).ourgame).anim_length)
                    .expect(
                        "non-null function pointer",
                    )(
                    oldstate,
                    (*((*me).states)
                        .offset(((*me).statepos - 1 as libc::c_int) as isize))
                        .state,
                    (*me).dir,
                    (*me).ui,
                );
            }
            (*me).oldstate = oldstate;
            oldstate = 0 as *mut game_state;
            if anim_time > 0 as libc::c_int as libc::c_float {
                (*me).anim_time = anim_time;
            } else {
                (*me).anim_time = 0.0f64 as libc::c_float;
                midend_finish_move(me);
            }
            (*me).anim_pos = 0.0f64 as libc::c_float;
            midend_redraw(me);
            midend_set_timer(me);
        }
        _ => {}
    }
    if !oldstate.is_null() {
        ((*(*me).ourgame).free_game).expect("non-null function pointer")(oldstate);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn midend_process_key(
    mut me: *mut midend,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut button: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = PKR_UNUSED as libc::c_int;
    let mut ret2: libc::c_int = 0;
    if (button - LEFT_DRAG as libc::c_int) as libc::c_uint
        <= (RIGHT_DRAG as libc::c_int - LEFT_DRAG as libc::c_int) as libc::c_uint
        || (button - LEFT_RELEASE as libc::c_int) as libc::c_uint
            <= (RIGHT_RELEASE as libc::c_int - LEFT_RELEASE as libc::c_int)
                as libc::c_uint
    {
        if (*me).pressed_mouse_button != 0 {
            if (button - LEFT_DRAG as libc::c_int) as libc::c_uint
                <= (RIGHT_DRAG as libc::c_int - LEFT_DRAG as libc::c_int) as libc::c_uint
            {
                button = (*me).pressed_mouse_button
                    + (LEFT_DRAG as libc::c_int - LEFT_BUTTON as libc::c_int);
            } else {
                button = (*me).pressed_mouse_button
                    + (LEFT_RELEASE as libc::c_int - LEFT_BUTTON as libc::c_int);
            }
        } else {
            return ret
        }
    } else if (button - LEFT_BUTTON as libc::c_int) as libc::c_uint
        <= (RIGHT_BUTTON as libc::c_int - LEFT_BUTTON as libc::c_int) as libc::c_uint
        && (*me).pressed_mouse_button != 0
    {
        if (*(*me).ourgame).flags
            & (1 as libc::c_int)
                << ((*me).pressed_mouse_button - LEFT_BUTTON as libc::c_int)
                    * 3 as libc::c_int + button - LEFT_BUTTON as libc::c_int != 0
        {
            return ret;
        }
        ret2 = midend_really_process_key(
            me,
            x,
            y,
            (*me).pressed_mouse_button
                + (LEFT_RELEASE as libc::c_int - LEFT_BUTTON as libc::c_int),
        );
        ret = if ret < ret2 { ret } else { ret2 };
    }
    if button & MOD_CTRL as libc::c_int != 0
        && button & !(MOD_MASK as libc::c_int) >= 0x40 as libc::c_int
        && button & !(MOD_MASK as libc::c_int) < 0x80 as libc::c_int
    {
        button = button
            & (0x1f as libc::c_int
                | MOD_MASK as libc::c_int & !(MOD_CTRL as libc::c_int));
    }
    if button & (!(MOD_MASK as libc::c_int) | MOD_SHFT as libc::c_int)
        == MOD_SHFT as libc::c_int | '\u{1a}' as i32
    {
        button = UI_REDO as libc::c_int;
    }
    if !(button & !(MOD_MASK as libc::c_int) == CURSOR_UP as libc::c_int
        || button & !(MOD_MASK as libc::c_int) == CURSOR_DOWN as libc::c_int
        || button & !(MOD_MASK as libc::c_int) == CURSOR_RIGHT as libc::c_int
        || button & !(MOD_MASK as libc::c_int) == CURSOR_LEFT as libc::c_int)
    {
        if button & MOD_CTRL as libc::c_int != 0
            && button & !(MOD_MASK as libc::c_int) >= 0x20 as libc::c_int
        {
            return PKR_UNUSED as libc::c_int;
        }
        button &= !(MOD_CTRL as libc::c_int | MOD_SHFT as libc::c_int);
    }
    if button & !(MOD_MASK as libc::c_int) < '0' as i32
        || button & !(MOD_MASK as libc::c_int) > '9' as i32
    {
        button &= !(MOD_NUM_KEYPAD as libc::c_int);
    }
    if button == '\n' as i32 || button == '\r' as i32 {
        button = CURSOR_SELECT as libc::c_int;
    }
    if button == ' ' as i32 {
        button = CURSOR_SELECT2 as libc::c_int;
    }
    if button == '\u{7f}' as i32 {
        button = '\u{8}' as i32;
    }
    ret2 = midend_really_process_key(me, x, y, button);
    ret = if ret < ret2 { ret } else { ret2 };
    if (button - LEFT_RELEASE as libc::c_int) as libc::c_uint
        <= (RIGHT_RELEASE as libc::c_int - LEFT_RELEASE as libc::c_int) as libc::c_uint
    {
        (*me).pressed_mouse_button = 0 as libc::c_int;
    } else if (button - LEFT_BUTTON as libc::c_int) as libc::c_uint
        <= (RIGHT_BUTTON as libc::c_int - LEFT_BUTTON as libc::c_int) as libc::c_uint
    {
        (*me).pressed_mouse_button = button;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn midend_request_keys(
    mut me: *mut midend,
    mut n: *mut libc::c_int,
) -> *mut key_label {
    let mut keys: *mut key_label = 0 as *mut key_label;
    let mut nkeys: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if ((*(*me).ourgame).request_keys).is_some() {
        keys = ((*(*me).ourgame).request_keys)
            .expect("non-null function pointer")(midend_get_params(me), &mut nkeys);
        i = 0 as libc::c_int;
        while i < nkeys {
            if ((*keys.offset(i as isize)).label).is_null() {
                let ref mut fresh8 = (*keys.offset(i as isize)).label;
                *fresh8 = button2label((*keys.offset(i as isize)).button);
            }
            i += 1;
            i;
        }
    }
    if !n.is_null() {
        *n = nkeys;
    }
    return keys;
}
#[no_mangle]
pub unsafe extern "C" fn midend_current_key_label(
    mut me: *mut midend,
    mut button: libc::c_int,
) -> *const libc::c_char {
    if button == CURSOR_SELECT as libc::c_int || button == CURSOR_SELECT2 as libc::c_int
    {} else {
        __assert_fail(
            b"IS_CURSOR_SELECT(button)\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1291 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"const char *midend_current_key_label(midend *, int)\0"))
                .as_ptr(),
        );
    }
    'c_15827: {
        if button == CURSOR_SELECT as libc::c_int
            || button == CURSOR_SELECT2 as libc::c_int
        {} else {
            __assert_fail(
                b"IS_CURSOR_SELECT(button)\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1291 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"const char *midend_current_key_label(midend *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*(*me).ourgame).current_key_label).is_none() {
        return b"\0" as *const u8 as *const libc::c_char;
    }
    return ((*(*me).ourgame).current_key_label)
        .expect(
            "non-null function pointer",
        )(
        (*me).ui,
        (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
        button,
    );
}
#[no_mangle]
pub unsafe extern "C" fn midend_redraw(mut me: *mut midend) {
    if !((*me).drawing).is_null() {} else {
        __assert_fail(
            b"me->drawing\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1299 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void midend_redraw(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_7835: {
        if !((*me).drawing).is_null() {} else {
            __assert_fail(
                b"me->drawing\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1299 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void midend_redraw(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*me).statepos > 0 as libc::c_int && !((*me).drawstate).is_null() {
        let mut first_draw: bool = (*me).first_draw;
        (*me).first_draw = 0 as libc::c_int != 0;
        start_draw((*me).drawing);
        if first_draw {
            draw_rect(
                (*me).drawing,
                0 as libc::c_int,
                0 as libc::c_int,
                (*me).winwidth,
                (*me).winheight,
                0 as libc::c_int,
            );
        }
        if !((*me).oldstate).is_null()
            && (*me).anim_time > 0 as libc::c_int as libc::c_float
            && (*me).anim_pos < (*me).anim_time
        {
            if (*me).dir != 0 as libc::c_int {} else {
                __assert_fail(
                    b"me->dir != 0\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1322 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void midend_redraw(midend *)\0"))
                        .as_ptr(),
                );
            }
            'c_7726: {
                if (*me).dir != 0 as libc::c_int {} else {
                    __assert_fail(
                        b"me->dir != 0\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        1322 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 29],
                            &[libc::c_char; 29],
                        >(b"void midend_redraw(midend *)\0"))
                            .as_ptr(),
                    );
                }
            };
            ((*(*me).ourgame).redraw)
                .expect(
                    "non-null function pointer",
                )(
                (*me).drawing,
                (*me).drawstate,
                (*me).oldstate,
                (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                    .state,
                (*me).dir,
                (*me).ui,
                (*me).anim_pos,
                (*me).flash_pos,
            );
        } else {
            ((*(*me).ourgame).redraw)
                .expect(
                    "non-null function pointer",
                )(
                (*me).drawing,
                (*me).drawstate,
                0 as *const game_state,
                (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize))
                    .state,
                1 as libc::c_int,
                (*me).ui,
                0.0f64 as libc::c_float,
                (*me).flash_pos,
            );
        }
        if first_draw {
            draw_update(
                (*me).drawing,
                0 as libc::c_int,
                0 as libc::c_int,
                (*me).winwidth,
                (*me).winheight,
            );
        }
        end_draw((*me).drawing);
    }
}
#[no_mangle]
pub unsafe extern "C" fn midend_freeze_timer(
    mut me: *mut midend,
    mut tprop: libc::c_float,
) {
    (*me).anim_pos = (*me).anim_time * tprop;
    midend_redraw(me);
    deactivate_timer((*me).frontend);
}
#[no_mangle]
pub unsafe extern "C" fn midend_timer(mut me: *mut midend, mut tplus: libc::c_float) {
    let mut need_redraw: bool = (*me).anim_time > 0 as libc::c_int as libc::c_float
        || (*me).flash_time > 0 as libc::c_int as libc::c_float;
    (*me).anim_pos += tplus;
    if (*me).anim_pos >= (*me).anim_time
        || (*me).anim_time == 0 as libc::c_int as libc::c_float
        || ((*me).oldstate).is_null()
    {
        if (*me).anim_time > 0 as libc::c_int as libc::c_float {
            midend_finish_move(me);
        }
    }
    (*me).flash_pos += tplus;
    if (*me).flash_pos >= (*me).flash_time
        || (*me).flash_time == 0 as libc::c_int as libc::c_float
    {
        (*me).flash_time = 0 as libc::c_int as libc::c_float;
        (*me).flash_pos = (*me).flash_time;
    }
    if need_redraw {
        midend_redraw(me);
    }
    if (*me).timing {
        let mut oldelapsed: libc::c_float = (*me).elapsed;
        (*me).elapsed += tplus;
        if oldelapsed as libc::c_int != (*me).elapsed as libc::c_int {
            status_bar(
                (*me).drawing,
                if !((*me).laststatus).is_null() {
                    (*me).laststatus as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
    }
    midend_set_timer(me);
}
#[no_mangle]
pub unsafe extern "C" fn midend_colours(
    mut me: *mut midend,
    mut ncolours: *mut libc::c_int,
) -> *mut libc::c_float {
    let mut ret: *mut libc::c_float = 0 as *mut libc::c_float;
    ret = ((*(*me).ourgame).colours)
        .expect("non-null function pointer")((*me).frontend, ncolours);
    if *ncolours >= 1 as libc::c_int {} else {
        __assert_fail(
            b"*ncolours >= 1\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1389 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"float *midend_colours(midend *, int *)\0"))
                .as_ptr(),
        );
    }
    'c_16390: {
        if *ncolours >= 1 as libc::c_int {} else {
            __assert_fail(
                b"*ncolours >= 1\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1389 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"float *midend_colours(midend *, int *)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < *ncolours {
        let mut buf: [libc::c_char; 80] = [0; 80];
        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r: libc::c_uint = 0;
        let mut g: libc::c_uint = 0;
        let mut b: libc::c_uint = 0;
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        sprintf(
            buf.as_mut_ptr(),
            b"%s_COLOUR_%d\0" as *const u8 as *const libc::c_char,
            (*(*me).ourgame).name,
            i,
        );
        k = 0 as libc::c_int;
        j = k;
        while buf[j as usize] != 0 {
            if *(*__ctype_b_loc())
                .offset(buf[j as usize] as libc::c_uchar as libc::c_int as isize)
                as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                let fresh9 = k;
                k = k + 1;
                buf[fresh9
                    as usize] = toupper(buf[j as usize] as libc::c_uchar as libc::c_int)
                    as libc::c_char;
            }
            j += 1;
            j;
        }
        buf[k as usize] = '\0' as i32 as libc::c_char;
        e = getenv(buf.as_mut_ptr());
        if !e.is_null()
            && sscanf(
                e,
                b"%2x%2x%2x\0" as *const u8 as *const libc::c_char,
                &mut r as *mut libc::c_uint,
                &mut g as *mut libc::c_uint,
                &mut b as *mut libc::c_uint,
            ) == 3 as libc::c_int
        {
            *ret
                .offset(
                    (i * 3 as libc::c_int + 0 as libc::c_int) as isize,
                ) = r as libc::c_float / 255.0f32;
            *ret
                .offset(
                    (i * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = g as libc::c_float / 255.0f32;
            *ret
                .offset(
                    (i * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = b as libc::c_float / 255.0f32;
        }
        if 0.0f32 <= *ret.offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
            && *ret.offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize) <= 1.0f32
        {} else {
            __assert_fail(
                b"0.0F <= ret[i*3 + 0] && ret[i*3 + 0] <= 1.0F\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1416 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"float *midend_colours(midend *, int *)\0"))
                    .as_ptr(),
            );
        }
        'c_16133: {
            if 0.0f32 <= *ret.offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
                && *ret.offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
                    <= 1.0f32
            {} else {
                __assert_fail(
                    b"0.0F <= ret[i*3 + 0] && ret[i*3 + 0] <= 1.0F\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1416 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"float *midend_colours(midend *, int *)\0"))
                        .as_ptr(),
                );
            }
        };
        if 0.0f32 <= *ret.offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
            && *ret.offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize) <= 1.0f32
        {} else {
            __assert_fail(
                b"0.0F <= ret[i*3 + 1] && ret[i*3 + 1] <= 1.0F\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1417 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"float *midend_colours(midend *, int *)\0"))
                    .as_ptr(),
            );
        }
        'c_16055: {
            if 0.0f32 <= *ret.offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                && *ret.offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                    <= 1.0f32
            {} else {
                __assert_fail(
                    b"0.0F <= ret[i*3 + 1] && ret[i*3 + 1] <= 1.0F\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1417 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"float *midend_colours(midend *, int *)\0"))
                        .as_ptr(),
                );
            }
        };
        if 0.0f32 <= *ret.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
            && *ret.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize) <= 1.0f32
        {} else {
            __assert_fail(
                b"0.0F <= ret[i*3 + 2] && ret[i*3 + 2] <= 1.0F\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1418 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"float *midend_colours(midend *, int *)\0"))
                    .as_ptr(),
            );
        }
        'c_15976: {
            if 0.0f32 <= *ret.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                && *ret.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                    <= 1.0f32
            {} else {
                __assert_fail(
                    b"0.0F <= ret[i*3 + 2] && ret[i*3 + 2] <= 1.0F\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1418 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"float *midend_colours(midend *, int *)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn preset_menu_new() -> *mut preset_menu {
    let mut menu: *mut preset_menu = smalloc(
        ::core::mem::size_of::<preset_menu>() as libc::c_ulong,
    ) as *mut preset_menu;
    (*menu).n_entries = 0 as libc::c_int;
    (*menu).entries_size = 0 as libc::c_int;
    (*menu).entries = 0 as *mut preset_menu_entry;
    return menu;
}
unsafe extern "C" fn preset_menu_add(
    mut menu: *mut preset_menu,
    mut title: *mut libc::c_char,
) -> *mut preset_menu_entry {
    let mut toret: *mut preset_menu_entry = 0 as *mut preset_menu_entry;
    if (*menu).n_entries >= (*menu).entries_size {
        (*menu)
            .entries_size = (*menu).n_entries * 5 as libc::c_int / 4 as libc::c_int
            + 10 as libc::c_int;
        (*menu)
            .entries = srealloc(
            (*menu).entries as *mut libc::c_void,
            ((*menu).entries_size as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<preset_menu_entry>() as libc::c_ulong,
                ),
        ) as *mut preset_menu_entry;
    }
    let fresh10 = (*menu).n_entries;
    (*menu).n_entries = (*menu).n_entries + 1;
    toret = &mut *((*menu).entries).offset(fresh10 as isize) as *mut preset_menu_entry;
    (*toret).title = title;
    (*toret).params = 0 as *mut game_params;
    (*toret).submenu = 0 as *mut preset_menu;
    return toret;
}
#[no_mangle]
pub unsafe extern "C" fn preset_menu_add_submenu(
    mut parent: *mut preset_menu,
    mut title: *mut libc::c_char,
) -> *mut preset_menu {
    let mut entry: *mut preset_menu_entry = preset_menu_add(parent, title);
    (*entry).submenu = preset_menu_new();
    return (*entry).submenu;
}
#[no_mangle]
pub unsafe extern "C" fn preset_menu_add_preset(
    mut parent: *mut preset_menu,
    mut title: *mut libc::c_char,
    mut params: *mut game_params,
) {
    let mut entry: *mut preset_menu_entry = preset_menu_add(parent, title);
    (*entry).params = params;
}
#[no_mangle]
pub unsafe extern "C" fn preset_menu_lookup_by_id(
    mut menu: *mut preset_menu,
    mut id: libc::c_int,
) -> *mut game_params {
    let mut i: libc::c_int = 0;
    let mut retd: *mut game_params = 0 as *mut game_params;
    i = 0 as libc::c_int;
    while i < (*menu).n_entries {
        if id == (*((*menu).entries).offset(i as isize)).id {
            return (*((*menu).entries).offset(i as isize)).params;
        }
        if !((*((*menu).entries).offset(i as isize)).submenu).is_null()
            && {
                retd = preset_menu_lookup_by_id(
                    (*((*menu).entries).offset(i as isize)).submenu,
                    id,
                );
                !retd.is_null()
            }
        {
            return retd;
        }
        i += 1;
        i;
    }
    return 0 as *mut game_params;
}
unsafe extern "C" fn preset_menu_add_from_user_env(
    mut me: *mut midend,
    mut menu: *mut preset_menu,
    mut p: *mut libc::c_char,
    mut top_level: bool,
) -> *mut libc::c_char {
    while *p != 0 {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut preset: *mut game_params = 0 as *mut game_params;
        name = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != ':' as i32 {
            p = p.offset(1);
            p;
        }
        if *p != 0 {
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = '\0' as i32 as libc::c_char;
        }
        val = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != ':' as i32 {
            p = p.offset(1);
            p;
        }
        if *p != 0 {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = '\0' as i32 as libc::c_char;
        }
        if strcmp(val, b"#\0" as *const u8 as *const libc::c_char) == 0 {
            if *name != 0 {
                let mut submenu: *mut preset_menu = preset_menu_add_submenu(
                    menu,
                    dupstr(name),
                );
                p = preset_menu_add_from_user_env(me, submenu, p, 0 as libc::c_int != 0);
            } else if !top_level {
                return p
            }
        } else {
            preset = ((*(*me).ourgame).default_params)
                .expect("non-null function pointer")();
            ((*(*me).ourgame).decode_params)
                .expect("non-null function pointer")(preset, val);
            if !(((*(*me).ourgame).validate_params)
                .expect("non-null function pointer")(preset, 1 as libc::c_int != 0))
                .is_null()
            {
                ((*(*me).ourgame).free_params)
                    .expect("non-null function pointer")(preset);
            } else {
                preset_menu_add_preset(menu, dupstr(name), preset);
            }
        }
    }
    return p;
}
unsafe extern "C" fn preset_menu_alloc_ids(
    mut me: *mut midend,
    mut menu: *mut preset_menu,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*menu).n_entries {
        let fresh13 = (*me).n_encoded_presets;
        (*me).n_encoded_presets = (*me).n_encoded_presets + 1;
        (*((*menu).entries).offset(i as isize)).id = fresh13;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*menu).n_entries {
        if !((*((*menu).entries).offset(i as isize)).submenu).is_null() {
            preset_menu_alloc_ids(me, (*((*menu).entries).offset(i as isize)).submenu);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn preset_menu_encode_params(
    mut me: *mut midend,
    mut menu: *mut preset_menu,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*menu).n_entries {
        if !((*((*menu).entries).offset(i as isize)).params).is_null() {
            let ref mut fresh14 = *((*me).encoded_presets)
                .offset((*((*menu).entries).offset(i as isize)).id as isize);
            *fresh14 = encode_params(
                me,
                (*((*menu).entries).offset(i as isize)).params,
                1 as libc::c_int != 0,
            );
        } else {
            preset_menu_encode_params(
                me,
                (*((*menu).entries).offset(i as isize)).submenu,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn midend_get_presets(
    mut me: *mut midend,
    mut id_limit: *mut libc::c_int,
) -> *mut preset_menu {
    let mut i: libc::c_int = 0;
    if !((*me).preset_menu).is_null() {
        return (*me).preset_menu;
    }
    if ((*(*me).ourgame).fetch_preset).is_some() {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut preset: *mut game_params = 0 as *mut game_params;
        if ((*(*me).ourgame).preset_menu).is_none() {} else {
            __assert_fail(
                b"!me->ourgame->preset_menu\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1576 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"struct preset_menu *midend_get_presets(midend *, int *)\0"))
                    .as_ptr(),
            );
        }
        'c_17302: {
            if ((*(*me).ourgame).preset_menu).is_none() {} else {
                __assert_fail(
                    b"!me->ourgame->preset_menu\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1576 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 56],
                        &[libc::c_char; 56],
                    >(b"struct preset_menu *midend_get_presets(midend *, int *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*me).preset_menu = preset_menu_new();
        i = 0 as libc::c_int;
        while ((*(*me).ourgame).fetch_preset)
            .expect("non-null function pointer")(i, &mut name, &mut preset)
        {
            preset_menu_add_preset((*me).preset_menu, name, preset);
            i += 1;
            i;
        }
    } else {
        (*me)
            .preset_menu = ((*(*me).ourgame).preset_menu)
            .expect("non-null function pointer")();
    }
    let mut buf: [libc::c_char; 80] = [0; 80];
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    sprintf(
        buf.as_mut_ptr(),
        b"%s_PRESETS\0" as *const u8 as *const libc::c_char,
        (*(*me).ourgame).name,
    );
    k = 0 as libc::c_int;
    j = k;
    while buf[j as usize] != 0 {
        if *(*__ctype_b_loc())
            .offset(buf[j as usize] as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            let fresh15 = k;
            k = k + 1;
            buf[fresh15
                as usize] = toupper(buf[j as usize] as libc::c_uchar as libc::c_int)
                as libc::c_char;
        }
        j += 1;
        j;
    }
    buf[k as usize] = '\0' as i32 as libc::c_char;
    e = getenv(buf.as_mut_ptr());
    if !e.is_null() {
        e = dupstr(e);
        preset_menu_add_from_user_env(me, (*me).preset_menu, e, 1 as libc::c_int != 0);
        sfree(e as *mut libc::c_void);
    }
    (*me).n_encoded_presets = 0 as libc::c_int;
    preset_menu_alloc_ids(me, (*me).preset_menu);
    (*me)
        .encoded_presets = smalloc(
        ((*me).n_encoded_presets as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < (*me).n_encoded_presets {
        let ref mut fresh16 = *((*me).encoded_presets).offset(i as isize);
        *fresh16 = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    preset_menu_encode_params(me, (*me).preset_menu);
    if !id_limit.is_null() {
        *id_limit = (*me).n_encoded_presets;
    }
    return (*me).preset_menu;
}
#[no_mangle]
pub unsafe extern "C" fn midend_which_preset(mut me: *mut midend) -> libc::c_int {
    let mut encoding: *mut libc::c_char = encode_params(
        me,
        (*me).params,
        1 as libc::c_int != 0,
    );
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    ret = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*me).n_encoded_presets {
        if !(*((*me).encoded_presets).offset(i as isize)).is_null()
            && strcmp(encoding, *((*me).encoded_presets).offset(i as isize)) == 0
        {
            ret = i;
            break;
        } else {
            i += 1;
            i;
        }
    }
    sfree(encoding as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn midend_wants_statusbar(mut me: *mut midend) -> bool {
    return (*(*me).ourgame).wants_statusbar;
}
#[no_mangle]
pub unsafe extern "C" fn midend_request_id_changes(
    mut me: *mut midend,
    mut notify: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut ctx: *mut libc::c_void,
) {
    (*me).game_id_change_notify_function = notify;
    (*me).game_id_change_notify_ctx = ctx;
}
#[no_mangle]
pub unsafe extern "C" fn midend_get_cursor_location(
    mut me: *mut midend,
    mut x_out: *mut libc::c_int,
    mut y_out: *mut libc::c_int,
    mut w_out: *mut libc::c_int,
    mut h_out: *mut libc::c_int,
) -> bool {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    y = -(1 as libc::c_int);
    x = y;
    h = 1 as libc::c_int;
    w = h;
    if ((*(*me).ourgame).get_cursor_location).is_some() {
        ((*(*me).ourgame).get_cursor_location)
            .expect(
                "non-null function pointer",
            )(
            (*me).ui,
            (*me).drawstate,
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
            (*me).params,
            &mut x,
            &mut y,
            &mut w,
            &mut h,
        );
    }
    if x == -(1 as libc::c_int) && y == -(1 as libc::c_int) {
        return 0 as libc::c_int != 0;
    }
    if !x_out.is_null() {
        *x_out = x;
    }
    if !y_out.is_null() {
        *y_out = y;
    }
    if !w_out.is_null() {
        *w_out = w;
    }
    if !h_out.is_null() {
        *h_out = h;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn midend_supersede_game_desc(
    mut me: *mut midend,
    mut desc: *const libc::c_char,
    mut privdesc: *const libc::c_char,
) {
    assert_printable_ascii(desc);
    if !privdesc.is_null() {
        assert_printable_ascii(privdesc);
    }
    sfree((*me).desc as *mut libc::c_void);
    sfree((*me).privdesc as *mut libc::c_void);
    (*me).desc = dupstr(desc);
    (*me)
        .privdesc = if !privdesc.is_null() {
        dupstr(privdesc)
    } else {
        0 as *mut libc::c_char
    };
    if ((*me).game_id_change_notify_function).is_some() {
        ((*me).game_id_change_notify_function)
            .expect("non-null function pointer")((*me).game_id_change_notify_ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn midend_get_config(
    mut me: *mut midend,
    mut which: libc::c_int,
    mut wintitle: *mut *mut libc::c_char,
) -> *mut config_item {
    let mut titlebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rest: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: *mut config_item = 0 as *mut config_item;
    let mut sep: libc::c_char = 0;
    if !wintitle.is_null() {} else {
        __assert_fail(
            b"wintitle\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1707 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"config_item *midend_get_config(midend *, int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_17891: {
        if !wintitle.is_null() {} else {
            __assert_fail(
                b"wintitle\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1707 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"config_item *midend_get_config(midend *, int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    titlebuf = smalloc(
        (40 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen((*(*me).ourgame).name))
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    match which {
        0 => {
            sprintf(
                titlebuf,
                b"%s configuration\0" as *const u8 as *const libc::c_char,
                (*(*me).ourgame).name,
            );
            *wintitle = titlebuf;
            return ((*(*me).ourgame).configure)
                .expect("non-null function pointer")((*me).params);
        }
        1 | 2 => {
            if ((*me).curparams).is_null() {
                sfree(titlebuf as *mut libc::c_void);
                return 0 as *mut config_item;
            }
            sprintf(
                titlebuf,
                b"%s %s selection\0" as *const u8 as *const libc::c_char,
                (*(*me).ourgame).name,
                if which == CFG_SEED as libc::c_int {
                    b"random\0" as *const u8 as *const libc::c_char
                } else {
                    b"game\0" as *const u8 as *const libc::c_char
                },
            );
            *wintitle = titlebuf;
            ret = smalloc(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<config_item>() as libc::c_ulong),
            ) as *mut config_item;
            (*ret.offset(0 as libc::c_int as isize)).type_0 = C_STRING as libc::c_int;
            if which == CFG_SEED as libc::c_int {
                let ref mut fresh17 = (*ret.offset(0 as libc::c_int as isize)).name;
                *fresh17 = b"Game random seed\0" as *const u8 as *const libc::c_char;
            } else {
                let ref mut fresh18 = (*ret.offset(0 as libc::c_int as isize)).name;
                *fresh18 = b"Game ID\0" as *const u8 as *const libc::c_char;
            }
            parstr = encode_params(
                me,
                (*me).curparams,
                which == CFG_SEED as libc::c_int,
            );
            if !parstr.is_null() {} else {
                __assert_fail(
                    b"parstr\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    1742 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[libc::c_char; 55],
                    >(b"config_item *midend_get_config(midend *, int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_17692: {
                if !parstr.is_null() {} else {
                    __assert_fail(
                        b"parstr\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        1742 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 55],
                            &[libc::c_char; 55],
                        >(b"config_item *midend_get_config(midend *, int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
            if which == CFG_DESC as libc::c_int {
                rest = if !((*me).desc).is_null() {
                    (*me).desc as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                };
                sep = ':' as i32 as libc::c_char;
            } else {
                rest = if !((*me).seedstr).is_null() {
                    (*me).seedstr as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                };
                sep = '#' as i32 as libc::c_char;
            }
            let ref mut fresh19 = (*ret.offset(0 as libc::c_int as isize)).u.string.sval;
            *fresh19 = smalloc(
                (strlen(parstr))
                    .wrapping_add(strlen(rest))
                    .wrapping_add(2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            sprintf(
                (*ret.offset(0 as libc::c_int as isize)).u.string.sval,
                b"%s%c%s\0" as *const u8 as *const libc::c_char,
                parstr,
                sep as libc::c_int,
                rest,
            );
            sfree(parstr as *mut libc::c_void);
            (*ret.offset(1 as libc::c_int as isize)).type_0 = C_END as libc::c_int;
            let ref mut fresh20 = (*ret.offset(1 as libc::c_int as isize)).name;
            *fresh20 = 0 as *const libc::c_char;
            return ret;
        }
        3 => {
            sprintf(
                titlebuf,
                b"%s preferences\0" as *const u8 as *const libc::c_char,
                (*(*me).ourgame).name,
            );
            *wintitle = titlebuf;
            return midend_get_prefs(me, 0 as *mut game_ui);
        }
        _ => {}
    }
    if (b"We shouldn't be here\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"!\"We shouldn't be here\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1764 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"config_item *midend_get_config(midend *, int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_17477: {
        if (b"We shouldn't be here\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"!\"We shouldn't be here\"\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1764 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"config_item *midend_get_config(midend *, int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    return 0 as *mut config_item;
}
unsafe extern "C" fn midend_game_id_int(
    mut me: *mut midend,
    mut id: *const libc::c_char,
    mut defmode: libc::c_int,
) -> *const libc::c_char {
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut par: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut desc: *const libc::c_char = 0 as *const libc::c_char;
    let mut seed: *const libc::c_char = 0 as *const libc::c_char;
    let mut newcurparams: *mut game_params = 0 as *mut game_params;
    let mut newparams: *mut game_params = 0 as *mut game_params;
    let mut oldparams1: *mut game_params = 0 as *mut game_params;
    let mut oldparams2: *mut game_params = 0 as *mut game_params;
    let mut free_params: bool = false;
    seed = strchr(id, '#' as i32);
    desc = strchr(id, ':' as i32);
    if !desc.is_null() && (seed.is_null() || desc < seed) {
        par = smalloc(
            ((desc.offset_from(id) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        strncpy(par, id, desc.offset_from(id) as libc::c_long as libc::c_ulong);
        *par
            .offset(
                desc.offset_from(id) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        desc = desc.offset(1);
        desc;
        seed = 0 as *const libc::c_char;
    } else if !seed.is_null() && (desc.is_null() || seed < desc) {
        par = smalloc(
            ((seed.offset_from(id) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        strncpy(par, id, seed.offset_from(id) as libc::c_long as libc::c_ulong);
        *par
            .offset(
                seed.offset_from(id) as libc::c_long as isize,
            ) = '\0' as i32 as libc::c_char;
        seed = seed.offset(1);
        seed;
        desc = 0 as *const libc::c_char;
    } else if defmode == DEF_SEED as libc::c_int {
        seed = id;
        par = 0 as *mut libc::c_char;
        desc = 0 as *const libc::c_char;
    } else if defmode == DEF_DESC as libc::c_int {
        desc = id;
        par = 0 as *mut libc::c_char;
        seed = 0 as *const libc::c_char;
    } else {
        par = dupstr(id);
        desc = 0 as *const libc::c_char;
        seed = desc;
    }
    oldparams2 = 0 as *mut game_params;
    oldparams1 = oldparams2;
    newparams = oldparams1;
    newcurparams = newparams;
    if !par.is_null() {
        if !desc.is_null() {
            newcurparams = ((*(*me).ourgame).dup_params)
                .expect("non-null function pointer")((*me).params);
        } else {
            newcurparams = ((*(*me).ourgame).default_params)
                .expect("non-null function pointer")();
        }
        ((*(*me).ourgame).decode_params)
            .expect("non-null function pointer")(newcurparams, par);
        sfree(par as *mut libc::c_void);
        error = ((*(*me).ourgame).validate_params)
            .expect("non-null function pointer")(newcurparams, desc.is_null());
        if !error.is_null() {
            ((*(*me).ourgame).free_params)
                .expect("non-null function pointer")(newcurparams);
            return error;
        }
        oldparams1 = (*me).curparams;
        oldparams2 = (*me).params;
        if !seed.is_null() || !desc.is_null() {
            let mut tmpstr: *mut libc::c_char = 0 as *mut libc::c_char;
            newparams = ((*(*me).ourgame).dup_params)
                .expect("non-null function pointer")((*me).params);
            tmpstr = encode_params(me, newcurparams, 0 as libc::c_int != 0);
            ((*(*me).ourgame).decode_params)
                .expect("non-null function pointer")(newparams, tmpstr);
            sfree(tmpstr as *mut libc::c_void);
        } else {
            newparams = ((*(*me).ourgame).dup_params)
                .expect("non-null function pointer")(newcurparams);
        }
        free_params = 1 as libc::c_int != 0;
    } else {
        newcurparams = (*me).curparams;
        newparams = (*me).params;
        free_params = 0 as libc::c_int != 0;
    }
    if !desc.is_null() {
        error = ((*(*me).ourgame).validate_desc)
            .expect("non-null function pointer")(newparams, desc);
        if !error.is_null() {
            if free_params {
                if !newcurparams.is_null() {
                    ((*(*me).ourgame).free_params)
                        .expect("non-null function pointer")(newcurparams);
                }
                if !newparams.is_null() {
                    ((*(*me).ourgame).free_params)
                        .expect("non-null function pointer")(newparams);
                }
            }
            return error;
        }
    }
    (*me).params = newparams;
    (*me).curparams = newcurparams;
    if !oldparams1.is_null() {
        ((*(*me).ourgame).free_params).expect("non-null function pointer")(oldparams1);
    }
    if !oldparams2.is_null() {
        ((*(*me).ourgame).free_params).expect("non-null function pointer")(oldparams2);
    }
    sfree((*me).desc as *mut libc::c_void);
    sfree((*me).privdesc as *mut libc::c_void);
    (*me).privdesc = 0 as *mut libc::c_char;
    (*me).desc = (*me).privdesc;
    sfree((*me).seedstr as *mut libc::c_void);
    (*me).seedstr = 0 as *mut libc::c_char;
    if !desc.is_null() {
        (*me).desc = dupstr(desc);
        (*me).genmode = GOT_DESC;
        sfree((*me).aux_info as *mut libc::c_void);
        (*me).aux_info = 0 as *mut libc::c_char;
    }
    if !seed.is_null() {
        (*me).seedstr = dupstr(seed);
        (*me).genmode = GOT_SEED;
    }
    (*me).newgame_can_store_undo = 0 as libc::c_int != 0;
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn midend_game_id(
    mut me: *mut midend,
    mut id: *const libc::c_char,
) -> *const libc::c_char {
    return midend_game_id_int(me, id, DEF_PARAMS as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn midend_get_game_id(mut me: *mut midend) -> *mut libc::c_char {
    let mut parstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    parstr = encode_params(me, (*me).curparams, 0 as libc::c_int != 0);
    if !parstr.is_null() {} else {
        __assert_fail(
            b"parstr\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1961 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"char *midend_get_game_id(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_18869: {
        if !parstr.is_null() {} else {
            __assert_fail(
                b"parstr\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1961 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"char *midend_get_game_id(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !((*me).desc).is_null() {} else {
        __assert_fail(
            b"me->desc\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1962 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"char *midend_get_game_id(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_18833: {
        if !((*me).desc).is_null() {} else {
            __assert_fail(
                b"me->desc\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1962 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"char *midend_get_game_id(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    ret = smalloc(
        (strlen(parstr))
            .wrapping_add(strlen((*me).desc))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(ret, b"%s:%s\0" as *const u8 as *const libc::c_char, parstr, (*me).desc);
    sfree(parstr as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn midend_get_random_seed(
    mut me: *mut midend,
) -> *mut libc::c_char {
    let mut parstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*me).seedstr).is_null() {
        return 0 as *mut libc::c_char;
    }
    parstr = encode_params(me, (*me).curparams, 1 as libc::c_int != 0);
    if !parstr.is_null() {} else {
        __assert_fail(
            b"parstr\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            1977 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"char *midend_get_random_seed(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_18977: {
        if !parstr.is_null() {} else {
            __assert_fail(
                b"parstr\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                1977 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"char *midend_get_random_seed(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    ret = smalloc(
        (strlen(parstr))
            .wrapping_add(strlen((*me).seedstr))
            .wrapping_add(2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    sprintf(ret, b"%s#%s\0" as *const u8 as *const libc::c_char, parstr, (*me).seedstr);
    sfree(parstr as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn midend_set_config(
    mut me: *mut midend,
    mut which: libc::c_int,
    mut cfg: *mut config_item,
) -> *const libc::c_char {
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut params: *mut game_params = 0 as *mut game_params;
    match which {
        0 => {
            params = ((*(*me).ourgame).custom_params)
                .expect("non-null function pointer")(cfg);
            error = ((*(*me).ourgame).validate_params)
                .expect("non-null function pointer")(params, 1 as libc::c_int != 0);
            if !error.is_null() {
                ((*(*me).ourgame).free_params)
                    .expect("non-null function pointer")(params);
                return error;
            }
            ((*(*me).ourgame).free_params)
                .expect("non-null function pointer")((*me).params);
            (*me).params = params;
        }
        1 | 2 => {
            error = midend_game_id_int(
                me,
                (*cfg.offset(0 as libc::c_int as isize)).u.string.sval,
                if which == CFG_SEED as libc::c_int {
                    DEF_SEED as libc::c_int
                } else {
                    DEF_DESC as libc::c_int
                },
            );
            if !error.is_null() {
                return error;
            }
        }
        3 => {
            midend_set_prefs(me, (*me).ui, cfg);
        }
        _ => {}
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn midend_can_format_as_text_now(mut me: *mut midend) -> bool {
    if (*(*me).ourgame).can_format_as_text_ever {
        return ((*(*me).ourgame).can_format_as_text_now)
            .expect("non-null function pointer")((*me).params)
    } else {
        return 0 as libc::c_int != 0
    };
}
#[no_mangle]
pub unsafe extern "C" fn midend_text_format(mut me: *mut midend) -> *mut libc::c_char {
    if (*(*me).ourgame).can_format_as_text_ever as libc::c_int != 0
        && (*me).statepos > 0 as libc::c_int
        && ((*(*me).ourgame).can_format_as_text_now)
            .expect("non-null function pointer")((*me).params) as libc::c_int != 0
    {
        return ((*(*me).ourgame).text_format)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
        )
    } else {
        return 0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn midend_solve(mut me: *mut midend) -> *const libc::c_char {
    let mut s: *mut game_state = 0 as *mut game_state;
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    let mut movestr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*(*me).ourgame).can_solve {
        return b"This game does not support the Solve operation\0" as *const u8
            as *const libc::c_char;
    }
    if (*me).statepos < 1 as libc::c_int {
        return b"No game set up to solve\0" as *const u8 as *const libc::c_char;
    }
    msg = 0 as *const libc::c_char;
    movestr = ((*(*me).ourgame).solve)
        .expect(
            "non-null function pointer",
        )(
        (*((*me).states).offset(0 as libc::c_int as isize)).state,
        (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
        (*me).aux_info,
        &mut msg,
    );
    if movestr != MOVE_UI_UPDATE.as_mut_ptr() {} else {
        __assert_fail(
            b"movestr != MOVE_UI_UPDATE\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            2052 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"const char *midend_solve(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_11935: {
        if movestr != MOVE_UI_UPDATE.as_mut_ptr() {} else {
            __assert_fail(
                b"movestr != MOVE_UI_UPDATE\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2052 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"const char *midend_solve(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    if movestr.is_null() {
        if msg.is_null() {
            msg = b"Solve operation failed\0" as *const u8 as *const libc::c_char;
        }
        return msg;
    }
    assert_printable_ascii(movestr);
    s = ((*(*me).ourgame).execute_move)
        .expect(
            "non-null function pointer",
        )(
        (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
        movestr,
    );
    if !s.is_null() {} else {
        __assert_fail(
            b"s\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            2060 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"const char *midend_solve(midend *)\0"))
                .as_ptr(),
        );
    }
    'c_11853: {
        if !s.is_null() {} else {
            __assert_fail(
                b"s\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2060 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"const char *midend_solve(midend *)\0"))
                    .as_ptr(),
            );
        }
    };
    midend_stop_anim(me);
    midend_purge_states(me);
    if (*me).nstates >= (*me).statesize {
        (*me).statesize = (*me).nstates + 128 as libc::c_int;
        (*me)
            .states = srealloc(
            (*me).states as *mut libc::c_void,
            ((*me).statesize as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<midend_state_entry>() as libc::c_ulong,
                ),
        ) as *mut midend_state_entry;
    }
    let ref mut fresh21 = (*((*me).states).offset((*me).nstates as isize)).state;
    *fresh21 = s;
    let ref mut fresh22 = (*((*me).states).offset((*me).nstates as isize)).movestr;
    *fresh22 = movestr;
    (*((*me).states).offset((*me).nstates as isize)).movetype = SOLVE as libc::c_int;
    (*me).nstates += 1;
    (*me).statepos = (*me).nstates;
    if !((*me).ui).is_null() {
        ((*(*me).ourgame).changed_state)
            .expect(
                "non-null function pointer",
            )(
            (*me).ui,
            (*((*me).states).offset(((*me).statepos - 2 as libc::c_int) as isize)).state,
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
        );
    }
    (*me).dir = 1 as libc::c_int;
    if (*(*me).ourgame).flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        (*me)
            .oldstate = ((*(*me).ourgame).dup_game)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(((*me).statepos - 2 as libc::c_int) as isize)).state,
        );
        (*me)
            .anim_time = ((*(*me).ourgame).anim_length)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(((*me).statepos - 2 as libc::c_int) as isize)).state,
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
            1 as libc::c_int,
            (*me).ui,
        );
        (*me).anim_pos = 0.0f64 as libc::c_float;
    } else {
        (*me).anim_time = 0.0f64 as libc::c_float;
        midend_finish_move(me);
    }
    if !((*me).drawing).is_null() {
        midend_redraw(me);
    }
    midend_set_timer(me);
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn midend_status(mut me: *mut midend) -> libc::c_int {
    if (*me).statepos == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return ((*(*me).ourgame).status)
        .expect(
            "non-null function pointer",
        )((*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state);
}
#[no_mangle]
pub unsafe extern "C" fn midend_rewrite_statusbar(
    mut me: *mut midend,
    mut text: *const libc::c_char,
) -> *mut libc::c_char {
    if (*me).laststatus != text as *mut libc::c_char {
        sfree((*me).laststatus as *mut libc::c_void);
        (*me).laststatus = dupstr(text);
    }
    if (*(*me).ourgame).is_timed {
        let mut timebuf: [libc::c_char; 100] = [0; 100];
        let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut min: libc::c_int = 0;
        let mut sec: libc::c_int = 0;
        sec = (*me).elapsed as libc::c_int;
        min = sec / 60 as libc::c_int;
        sec %= 60 as libc::c_int;
        sprintf(
            timebuf.as_mut_ptr(),
            b"[%d:%02d] \0" as *const u8 as *const libc::c_char,
            min,
            sec,
        );
        ret = smalloc(
            (strlen(timebuf.as_mut_ptr()))
                .wrapping_add(strlen(text))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(ret, timebuf.as_mut_ptr());
        strcat(ret, text);
        return ret;
    } else {
        return dupstr(text)
    };
}
#[no_mangle]
pub unsafe extern "C" fn midend_serialise(
    mut me: *mut midend,
    mut write: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void, libc::c_int) -> (),
    >,
    mut wctx: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut hbuf: [libc::c_char; 80] = [0; 80];
    let mut str: *const libc::c_char = b"Simon Tatham's Portable Puzzle Collection\0"
        as *const u8 as *const libc::c_char;
    let mut lbuf: [libc::c_char; 9] = [0; 9];
    copy_left_justified(
        lbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        b"SAVEFILE\0" as *const u8 as *const libc::c_char,
    );
    sprintf(
        hbuf.as_mut_ptr(),
        b"%s:%d:\0" as *const u8 as *const libc::c_char,
        lbuf.as_mut_ptr(),
        strlen(str) as libc::c_int,
    );
    assert_printable_ascii(hbuf.as_mut_ptr());
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        hbuf.as_mut_ptr() as *const libc::c_void,
        strlen(hbuf.as_mut_ptr()) as libc::c_int,
    );
    assert_printable_ascii(str);
    write
        .expect(
            "non-null function pointer",
        )(wctx, str as *const libc::c_void, strlen(str) as libc::c_int);
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int,
    );
    let mut hbuf_0: [libc::c_char; 80] = [0; 80];
    let mut str_0: *const libc::c_char = b"1\0" as *const u8 as *const libc::c_char;
    let mut lbuf_0: [libc::c_char; 9] = [0; 9];
    copy_left_justified(
        lbuf_0.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        b"VERSION\0" as *const u8 as *const libc::c_char,
    );
    sprintf(
        hbuf_0.as_mut_ptr(),
        b"%s:%d:\0" as *const u8 as *const libc::c_char,
        lbuf_0.as_mut_ptr(),
        strlen(str_0) as libc::c_int,
    );
    assert_printable_ascii(hbuf_0.as_mut_ptr());
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        hbuf_0.as_mut_ptr() as *const libc::c_void,
        strlen(hbuf_0.as_mut_ptr()) as libc::c_int,
    );
    assert_printable_ascii(str_0);
    write
        .expect(
            "non-null function pointer",
        )(wctx, str_0 as *const libc::c_void, strlen(str_0) as libc::c_int);
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int,
    );
    let mut s: *mut libc::c_char = dupstr((*(*me).ourgame).name);
    let mut hbuf_1: [libc::c_char; 80] = [0; 80];
    let mut str_1: *const libc::c_char = s;
    let mut lbuf_1: [libc::c_char; 9] = [0; 9];
    copy_left_justified(
        lbuf_1.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        b"GAME\0" as *const u8 as *const libc::c_char,
    );
    sprintf(
        hbuf_1.as_mut_ptr(),
        b"%s:%d:\0" as *const u8 as *const libc::c_char,
        lbuf_1.as_mut_ptr(),
        strlen(str_1) as libc::c_int,
    );
    assert_printable_ascii(hbuf_1.as_mut_ptr());
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        hbuf_1.as_mut_ptr() as *const libc::c_void,
        strlen(hbuf_1.as_mut_ptr()) as libc::c_int,
    );
    assert_printable_ascii(str_1);
    write
        .expect(
            "non-null function pointer",
        )(wctx, str_1 as *const libc::c_void, strlen(str_1) as libc::c_int);
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int,
    );
    sfree(s as *mut libc::c_void);
    if !((*me).params).is_null() {
        let mut s_0: *mut libc::c_char = encode_params(
            me,
            (*me).params,
            1 as libc::c_int != 0,
        );
        let mut hbuf_2: [libc::c_char; 80] = [0; 80];
        let mut str_2: *const libc::c_char = s_0;
        let mut lbuf_2: [libc::c_char; 9] = [0; 9];
        copy_left_justified(
            lbuf_2.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            b"PARAMS\0" as *const u8 as *const libc::c_char,
        );
        sprintf(
            hbuf_2.as_mut_ptr(),
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            lbuf_2.as_mut_ptr(),
            strlen(str_2) as libc::c_int,
        );
        assert_printable_ascii(hbuf_2.as_mut_ptr());
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            hbuf_2.as_mut_ptr() as *const libc::c_void,
            strlen(hbuf_2.as_mut_ptr()) as libc::c_int,
        );
        assert_printable_ascii(str_2);
        write
            .expect(
                "non-null function pointer",
            )(wctx, str_2 as *const libc::c_void, strlen(str_2) as libc::c_int);
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
        sfree(s_0 as *mut libc::c_void);
    }
    if !((*me).curparams).is_null() {
        let mut s_1: *mut libc::c_char = encode_params(
            me,
            (*me).curparams,
            1 as libc::c_int != 0,
        );
        let mut hbuf_3: [libc::c_char; 80] = [0; 80];
        let mut str_3: *const libc::c_char = s_1;
        let mut lbuf_3: [libc::c_char; 9] = [0; 9];
        copy_left_justified(
            lbuf_3.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            b"CPARAMS\0" as *const u8 as *const libc::c_char,
        );
        sprintf(
            hbuf_3.as_mut_ptr(),
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            lbuf_3.as_mut_ptr(),
            strlen(str_3) as libc::c_int,
        );
        assert_printable_ascii(hbuf_3.as_mut_ptr());
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            hbuf_3.as_mut_ptr() as *const libc::c_void,
            strlen(hbuf_3.as_mut_ptr()) as libc::c_int,
        );
        assert_printable_ascii(str_3);
        write
            .expect(
                "non-null function pointer",
            )(wctx, str_3 as *const libc::c_void, strlen(str_3) as libc::c_int);
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
        sfree(s_1 as *mut libc::c_void);
    }
    if !((*me).seedstr).is_null() {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while *((*me).seedstr).offset(i_0 as isize) != 0 {
            if (*((*me).seedstr).offset(i_0 as isize) as libc::c_int) < 32 as libc::c_int
                || *((*me).seedstr).offset(i_0 as isize) as libc::c_int
                    >= 127 as libc::c_int
            {
                break;
            }
            i_0 += 1;
            i_0;
        }
        if *((*me).seedstr).offset(i_0 as isize) != 0 {
            let mut hexseed: *mut libc::c_char = bin2hex(
                (*me).seedstr as *mut libc::c_uchar,
                strlen((*me).seedstr) as libc::c_int,
            );
            let mut hbuf_4: [libc::c_char; 80] = [0; 80];
            let mut str_4: *const libc::c_char = hexseed;
            let mut lbuf_4: [libc::c_char; 9] = [0; 9];
            copy_left_justified(
                lbuf_4.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                b"HEXSEED\0" as *const u8 as *const libc::c_char,
            );
            sprintf(
                hbuf_4.as_mut_ptr(),
                b"%s:%d:\0" as *const u8 as *const libc::c_char,
                lbuf_4.as_mut_ptr(),
                strlen(str_4) as libc::c_int,
            );
            assert_printable_ascii(hbuf_4.as_mut_ptr());
            write
                .expect(
                    "non-null function pointer",
                )(
                wctx,
                hbuf_4.as_mut_ptr() as *const libc::c_void,
                strlen(hbuf_4.as_mut_ptr()) as libc::c_int,
            );
            assert_printable_ascii(str_4);
            write
                .expect(
                    "non-null function pointer",
                )(wctx, str_4 as *const libc::c_void, strlen(str_4) as libc::c_int);
            write
                .expect(
                    "non-null function pointer",
                )(
                wctx,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int,
            );
            sfree(hexseed as *mut libc::c_void);
        } else {
            let mut hbuf_5: [libc::c_char; 80] = [0; 80];
            let mut str_5: *const libc::c_char = (*me).seedstr;
            let mut lbuf_5: [libc::c_char; 9] = [0; 9];
            copy_left_justified(
                lbuf_5.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                b"SEED\0" as *const u8 as *const libc::c_char,
            );
            sprintf(
                hbuf_5.as_mut_ptr(),
                b"%s:%d:\0" as *const u8 as *const libc::c_char,
                lbuf_5.as_mut_ptr(),
                strlen(str_5) as libc::c_int,
            );
            assert_printable_ascii(hbuf_5.as_mut_ptr());
            write
                .expect(
                    "non-null function pointer",
                )(
                wctx,
                hbuf_5.as_mut_ptr() as *const libc::c_void,
                strlen(hbuf_5.as_mut_ptr()) as libc::c_int,
            );
            assert_printable_ascii(str_5);
            write
                .expect(
                    "non-null function pointer",
                )(wctx, str_5 as *const libc::c_void, strlen(str_5) as libc::c_int);
            write
                .expect(
                    "non-null function pointer",
                )(
                wctx,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int,
            );
        }
    }
    if !((*me).desc).is_null() {
        let mut hbuf_6: [libc::c_char; 80] = [0; 80];
        let mut str_6: *const libc::c_char = (*me).desc;
        let mut lbuf_6: [libc::c_char; 9] = [0; 9];
        copy_left_justified(
            lbuf_6.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            b"DESC\0" as *const u8 as *const libc::c_char,
        );
        sprintf(
            hbuf_6.as_mut_ptr(),
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            lbuf_6.as_mut_ptr(),
            strlen(str_6) as libc::c_int,
        );
        assert_printable_ascii(hbuf_6.as_mut_ptr());
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            hbuf_6.as_mut_ptr() as *const libc::c_void,
            strlen(hbuf_6.as_mut_ptr()) as libc::c_int,
        );
        assert_printable_ascii(str_6);
        write
            .expect(
                "non-null function pointer",
            )(wctx, str_6 as *const libc::c_void, strlen(str_6) as libc::c_int);
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
    }
    if !((*me).privdesc).is_null() {
        let mut hbuf_7: [libc::c_char; 80] = [0; 80];
        let mut str_7: *const libc::c_char = (*me).privdesc;
        let mut lbuf_7: [libc::c_char; 9] = [0; 9];
        copy_left_justified(
            lbuf_7.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            b"PRIVDESC\0" as *const u8 as *const libc::c_char,
        );
        sprintf(
            hbuf_7.as_mut_ptr(),
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            lbuf_7.as_mut_ptr(),
            strlen(str_7) as libc::c_int,
        );
        assert_printable_ascii(hbuf_7.as_mut_ptr());
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            hbuf_7.as_mut_ptr() as *const libc::c_void,
            strlen(hbuf_7.as_mut_ptr()) as libc::c_int,
        );
        assert_printable_ascii(str_7);
        write
            .expect(
                "non-null function pointer",
            )(wctx, str_7 as *const libc::c_void, strlen(str_7) as libc::c_int);
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
    }
    if !((*me).aux_info).is_null() {
        let mut s1: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        len = strlen((*me).aux_info) as libc::c_int;
        s1 = smalloc(
            (len as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
        ) as *mut libc::c_uchar;
        memcpy(
            s1 as *mut libc::c_void,
            (*me).aux_info as *const libc::c_void,
            len as libc::c_ulong,
        );
        obfuscate_bitmap(s1, len * 8 as libc::c_int, 0 as libc::c_int != 0);
        s2 = bin2hex(s1, len);
        let mut hbuf_8: [libc::c_char; 80] = [0; 80];
        let mut str_8: *const libc::c_char = s2;
        let mut lbuf_8: [libc::c_char; 9] = [0; 9];
        copy_left_justified(
            lbuf_8.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            b"AUXINFO\0" as *const u8 as *const libc::c_char,
        );
        sprintf(
            hbuf_8.as_mut_ptr(),
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            lbuf_8.as_mut_ptr(),
            strlen(str_8) as libc::c_int,
        );
        assert_printable_ascii(hbuf_8.as_mut_ptr());
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            hbuf_8.as_mut_ptr() as *const libc::c_void,
            strlen(hbuf_8.as_mut_ptr()) as libc::c_int,
        );
        assert_printable_ascii(str_8);
        write
            .expect(
                "non-null function pointer",
            )(wctx, str_8 as *const libc::c_void, strlen(str_8) as libc::c_int);
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
        sfree(s2 as *mut libc::c_void);
        sfree(s1 as *mut libc::c_void);
    }
    if !((*me).ui).is_null() && ((*(*me).ourgame).encode_ui).is_some() {
        let mut s_2: *mut libc::c_char = ((*(*me).ourgame).encode_ui)
            .expect("non-null function pointer")((*me).ui);
        if !s_2.is_null() {
            let mut hbuf_9: [libc::c_char; 80] = [0; 80];
            let mut str_9: *const libc::c_char = s_2;
            let mut lbuf_9: [libc::c_char; 9] = [0; 9];
            copy_left_justified(
                lbuf_9.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                b"UI\0" as *const u8 as *const libc::c_char,
            );
            sprintf(
                hbuf_9.as_mut_ptr(),
                b"%s:%d:\0" as *const u8 as *const libc::c_char,
                lbuf_9.as_mut_ptr(),
                strlen(str_9) as libc::c_int,
            );
            assert_printable_ascii(hbuf_9.as_mut_ptr());
            write
                .expect(
                    "non-null function pointer",
                )(
                wctx,
                hbuf_9.as_mut_ptr() as *const libc::c_void,
                strlen(hbuf_9.as_mut_ptr()) as libc::c_int,
            );
            assert_printable_ascii(str_9);
            write
                .expect(
                    "non-null function pointer",
                )(wctx, str_9 as *const libc::c_void, strlen(str_9) as libc::c_int);
            write
                .expect(
                    "non-null function pointer",
                )(
                wctx,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int,
            );
            sfree(s_2 as *mut libc::c_void);
        }
    }
    if (*(*me).ourgame).is_timed {
        let mut buf: [libc::c_char; 80] = [0; 80];
        sprintf(
            buf.as_mut_ptr(),
            b"%g\0" as *const u8 as *const libc::c_char,
            (*me).elapsed as libc::c_double,
        );
        let mut hbuf_10: [libc::c_char; 80] = [0; 80];
        let mut str_10: *const libc::c_char = buf.as_mut_ptr();
        let mut lbuf_10: [libc::c_char; 9] = [0; 9];
        copy_left_justified(
            lbuf_10.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            b"TIME\0" as *const u8 as *const libc::c_char,
        );
        sprintf(
            hbuf_10.as_mut_ptr(),
            b"%s:%d:\0" as *const u8 as *const libc::c_char,
            lbuf_10.as_mut_ptr(),
            strlen(str_10) as libc::c_int,
        );
        assert_printable_ascii(hbuf_10.as_mut_ptr());
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            hbuf_10.as_mut_ptr() as *const libc::c_void,
            strlen(hbuf_10.as_mut_ptr()) as libc::c_int,
        );
        assert_printable_ascii(str_10);
        write
            .expect(
                "non-null function pointer",
            )(wctx, str_10 as *const libc::c_void, strlen(str_10) as libc::c_int);
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
    }
    let mut buf_0: [libc::c_char; 80] = [0; 80];
    sprintf(
        buf_0.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        (*me).nstates,
    );
    let mut hbuf_11: [libc::c_char; 80] = [0; 80];
    let mut str_11: *const libc::c_char = buf_0.as_mut_ptr();
    let mut lbuf_11: [libc::c_char; 9] = [0; 9];
    copy_left_justified(
        lbuf_11.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        b"NSTATES\0" as *const u8 as *const libc::c_char,
    );
    sprintf(
        hbuf_11.as_mut_ptr(),
        b"%s:%d:\0" as *const u8 as *const libc::c_char,
        lbuf_11.as_mut_ptr(),
        strlen(str_11) as libc::c_int,
    );
    assert_printable_ascii(hbuf_11.as_mut_ptr());
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        hbuf_11.as_mut_ptr() as *const libc::c_void,
        strlen(hbuf_11.as_mut_ptr()) as libc::c_int,
    );
    assert_printable_ascii(str_11);
    write
        .expect(
            "non-null function pointer",
        )(wctx, str_11 as *const libc::c_void, strlen(str_11) as libc::c_int);
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int,
    );
    if (*me).statepos >= 1 as libc::c_int && (*me).statepos <= (*me).nstates {} else {
        __assert_fail(
            b"me->statepos >= 1 && me->statepos <= me->nstates\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            2285 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 77],
                &[libc::c_char; 77],
            >(
                b"void midend_serialise(midend *, void (*)(void *, const void *, int), void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_8656: {
        if (*me).statepos >= 1 as libc::c_int && (*me).statepos <= (*me).nstates
        {} else {
            __assert_fail(
                b"me->statepos >= 1 && me->statepos <= me->nstates\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2285 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"void midend_serialise(midend *, void (*)(void *, const void *, int), void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    sprintf(
        buf_0.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        (*me).statepos,
    );
    let mut hbuf_12: [libc::c_char; 80] = [0; 80];
    let mut str_12: *const libc::c_char = buf_0.as_mut_ptr();
    let mut lbuf_12: [libc::c_char; 9] = [0; 9];
    copy_left_justified(
        lbuf_12.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
        b"STATEPOS\0" as *const u8 as *const libc::c_char,
    );
    sprintf(
        hbuf_12.as_mut_ptr(),
        b"%s:%d:\0" as *const u8 as *const libc::c_char,
        lbuf_12.as_mut_ptr(),
        strlen(str_12) as libc::c_int,
    );
    assert_printable_ascii(hbuf_12.as_mut_ptr());
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        hbuf_12.as_mut_ptr() as *const libc::c_void,
        strlen(hbuf_12.as_mut_ptr()) as libc::c_int,
    );
    assert_printable_ascii(str_12);
    write
        .expect(
            "non-null function pointer",
        )(wctx, str_12 as *const libc::c_void, strlen(str_12) as libc::c_int);
    write
        .expect(
            "non-null function pointer",
        )(
        wctx,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int,
    );
    i = 1 as libc::c_int;
    while i < (*me).nstates {
        if (*((*me).states).offset(i as isize)).movetype != NEWGAME as libc::c_int
        {} else {
            __assert_fail(
                b"me->states[i].movetype != NEWGAME\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2297 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 77],
                    &[libc::c_char; 77],
                >(
                    b"void midend_serialise(midend *, void (*)(void *, const void *, int), void *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8482: {
            if (*((*me).states).offset(i as isize)).movetype != NEWGAME as libc::c_int
            {} else {
                __assert_fail(
                    b"me->states[i].movetype != NEWGAME\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    2297 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 77],
                        &[libc::c_char; 77],
                    >(
                        b"void midend_serialise(midend *, void (*)(void *, const void *, int), void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        match (*((*me).states).offset(i as isize)).movetype {
            1 => {
                let mut hbuf_13: [libc::c_char; 80] = [0; 80];
                let mut str_13: *const libc::c_char = (*((*me).states)
                    .offset(i as isize))
                    .movestr;
                let mut lbuf_13: [libc::c_char; 9] = [0; 9];
                copy_left_justified(
                    lbuf_13.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                    b"MOVE\0" as *const u8 as *const libc::c_char,
                );
                sprintf(
                    hbuf_13.as_mut_ptr(),
                    b"%s:%d:\0" as *const u8 as *const libc::c_char,
                    lbuf_13.as_mut_ptr(),
                    strlen(str_13) as libc::c_int,
                );
                assert_printable_ascii(hbuf_13.as_mut_ptr());
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    hbuf_13.as_mut_ptr() as *const libc::c_void,
                    strlen(hbuf_13.as_mut_ptr()) as libc::c_int,
                );
                assert_printable_ascii(str_13);
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    str_13 as *const libc::c_void,
                    strlen(str_13) as libc::c_int,
                );
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int,
                );
            }
            2 => {
                let mut hbuf_14: [libc::c_char; 80] = [0; 80];
                let mut str_14: *const libc::c_char = (*((*me).states)
                    .offset(i as isize))
                    .movestr;
                let mut lbuf_14: [libc::c_char; 9] = [0; 9];
                copy_left_justified(
                    lbuf_14.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                    b"SOLVE\0" as *const u8 as *const libc::c_char,
                );
                sprintf(
                    hbuf_14.as_mut_ptr(),
                    b"%s:%d:\0" as *const u8 as *const libc::c_char,
                    lbuf_14.as_mut_ptr(),
                    strlen(str_14) as libc::c_int,
                );
                assert_printable_ascii(hbuf_14.as_mut_ptr());
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    hbuf_14.as_mut_ptr() as *const libc::c_void,
                    strlen(hbuf_14.as_mut_ptr()) as libc::c_int,
                );
                assert_printable_ascii(str_14);
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    str_14 as *const libc::c_void,
                    strlen(str_14) as libc::c_int,
                );
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int,
                );
            }
            3 => {
                let mut hbuf_15: [libc::c_char; 80] = [0; 80];
                let mut str_15: *const libc::c_char = (*((*me).states)
                    .offset(i as isize))
                    .movestr;
                let mut lbuf_15: [libc::c_char; 9] = [0; 9];
                copy_left_justified(
                    lbuf_15.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
                    b"RESTART\0" as *const u8 as *const libc::c_char,
                );
                sprintf(
                    hbuf_15.as_mut_ptr(),
                    b"%s:%d:\0" as *const u8 as *const libc::c_char,
                    lbuf_15.as_mut_ptr(),
                    strlen(str_15) as libc::c_int,
                );
                assert_printable_ascii(hbuf_15.as_mut_ptr());
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    hbuf_15.as_mut_ptr() as *const libc::c_void,
                    strlen(hbuf_15.as_mut_ptr()) as libc::c_int,
                );
                assert_printable_ascii(str_15);
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    str_15 as *const libc::c_void,
                    strlen(str_15) as libc::c_int,
                );
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int,
                );
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn midend_deserialise_internal(
    mut me: *mut midend,
    mut read: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, libc::c_int) -> bool,
    >,
    mut rctx: *mut libc::c_void,
    mut check: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut midend,
            *const deserialise_data,
        ) -> *const libc::c_char,
    >,
    mut cctx: *mut libc::c_void,
) -> *const libc::c_char {
    let mut current_block: u64;
    let mut data: deserialise_data = deserialise_data {
        seed: 0 as *mut libc::c_char,
        parstr: 0 as *mut libc::c_char,
        desc: 0 as *mut libc::c_char,
        privdesc: 0 as *mut libc::c_char,
        auxinfo: 0 as *mut libc::c_char,
        uistr: 0 as *mut libc::c_char,
        cparstr: 0 as *mut libc::c_char,
        elapsed: 0.,
        params: 0 as *mut game_params,
        cparams: 0 as *mut game_params,
        ui: 0 as *mut game_ui,
        states: 0 as *mut midend_state_entry,
        nstates: 0,
        statepos: 0,
    };
    let mut gotstates: libc::c_int = 0 as libc::c_int;
    let mut started: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_int = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *const libc::c_char = b"Data does not appear to be a saved game file\0"
        as *const u8 as *const libc::c_char;
    data.privdesc = 0 as *mut libc::c_char;
    data.desc = data.privdesc;
    data.parstr = data.desc;
    data.seed = data.parstr;
    data.cparstr = 0 as *mut libc::c_char;
    data.uistr = data.cparstr;
    data.auxinfo = data.uistr;
    data.elapsed = 0.0f32;
    data.cparams = 0 as *mut game_params;
    data.params = data.cparams;
    data.ui = 0 as *mut game_ui;
    data.states = 0 as *mut midend_state_entry;
    data.nstates = 0 as libc::c_int;
    data.statepos = -(1 as libc::c_int);
    's_37: loop {
        if !(data.nstates <= 0 as libc::c_int || data.statepos < 0 as libc::c_int
            || gotstates < data.nstates - 1 as libc::c_int)
        {
            current_block = 10109057886293123569;
            break;
        }
        let mut key: [libc::c_char; 9] = [0; 9];
        let mut c: libc::c_char = 0;
        let mut len: libc::c_int = 0;
        loop {
            if !read
                .expect(
                    "non-null function pointer",
                )(rctx, key.as_mut_ptr() as *mut libc::c_void, 1 as libc::c_int)
            {
                current_block = 17011192871324626656;
                break 's_37;
            } else if !(key[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
                || key[0 as libc::c_int as usize] as libc::c_int == '\n' as i32)
            {
                break;
            }
        }
        if !read
            .expect(
                "non-null function pointer",
            )(
            rctx,
            key.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
            8 as libc::c_int,
        )
        {
            current_block = 17011192871324626656;
            break;
        } else if key[8 as libc::c_int as usize] as libc::c_int != ':' as i32 {
            if started {
                ret = b"Data was incorrectly formatted for a saved game file\0"
                    as *const u8 as *const libc::c_char;
            }
            current_block = 17011192871324626656;
            break;
        } else {
            len = strcspn(key.as_mut_ptr(), b": \0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            if len <= 8 as libc::c_int {} else {
                __assert_fail(
                    b"len <= 8\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    2373 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 166],
                        &[libc::c_char; 166],
                    >(
                        b"const char *midend_deserialise_internal(midend *, _Bool (*)(void *, void *, int), void *, const char *(*)(void *, midend *, const struct deserialise_data *), void *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_14544: {
                if len <= 8 as libc::c_int {} else {
                    __assert_fail(
                        b"len <= 8\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        2373 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 166],
                            &[libc::c_char; 166],
                        >(
                            b"const char *midend_deserialise_internal(midend *, _Bool (*)(void *, void *, int), void *, const char *(*)(void *, midend *, const struct deserialise_data *), void *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            key[len as usize] = '\0' as i32 as libc::c_char;
            len = 0 as libc::c_int;
            loop {
                if !read
                    .expect(
                        "non-null function pointer",
                    )(
                    rctx,
                    &mut c as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int,
                )
                {
                    current_block = 17011192871324626656;
                    break 's_37;
                } else {
                    if c as libc::c_int == ':' as i32 {
                        break;
                    }
                    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
                        && len
                            < (2147483647 as libc::c_int - 10 as libc::c_int)
                                / 10 as libc::c_int
                    {
                        len = len * 10 as libc::c_int + (c as libc::c_int - '0' as i32);
                    } else {
                        if started {
                            ret = b"Data was incorrectly formatted for a saved game file\0"
                                as *const u8 as *const libc::c_char;
                        }
                        current_block = 17011192871324626656;
                        break 's_37;
                    }
                }
            }
            val = smalloc(
                ((len + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            if !read
                .expect("non-null function pointer")(rctx, val as *mut libc::c_void, len)
            {
                current_block = 17011192871324626656;
                break;
            } else {
                *val.offset(len as isize) = '\0' as i32 as libc::c_char;
                if strcmp(
                    key.as_mut_ptr(),
                    b"SEED\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    i = 0 as libc::c_int;
                    while *val.offset(i as isize) != 0 {
                        if (*val.offset(i as isize) as libc::c_int) < 32 as libc::c_int
                            || *val.offset(i as isize) as libc::c_int
                                >= 127 as libc::c_int
                        {
                            ret = b"Forbidden characters in saved game file\0"
                                as *const u8 as *const libc::c_char;
                            current_block = 17011192871324626656;
                            break 's_37;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                }
                if !started {
                    if strcmp(
                        key.as_mut_ptr(),
                        b"SAVEFILE\0" as *const u8 as *const libc::c_char,
                    ) != 0
                        || strcmp(
                            val,
                            b"Simon Tatham's Portable Puzzle Collection\0" as *const u8
                                as *const libc::c_char,
                        ) != 0
                    {
                        current_block = 17011192871324626656;
                        break;
                    }
                    ret = b"Saved data ended unexpectedly\0" as *const u8
                        as *const libc::c_char;
                    started = 1 as libc::c_int != 0;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"VERSION\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if strcmp(val, b"1\0" as *const u8 as *const libc::c_char) != 0 {
                        ret = b"Cannot handle this version of the saved game file format\0"
                            as *const u8 as *const libc::c_char;
                        current_block = 17011192871324626656;
                        break;
                    }
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"GAME\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if strcmp(val, (*(*me).ourgame).name) != 0 {
                        ret = b"Save file is from a different game\0" as *const u8
                            as *const libc::c_char;
                        current_block = 17011192871324626656;
                        break;
                    }
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"PARAMS\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sfree(data.parstr as *mut libc::c_void);
                    data.parstr = val;
                    val = 0 as *mut libc::c_char;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"CPARAMS\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sfree(data.cparstr as *mut libc::c_void);
                    data.cparstr = val;
                    val = 0 as *mut libc::c_char;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"HEXSEED\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut len_0: libc::c_int = (strlen(val))
                        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
                    tmp = hex2bin(val, len_0);
                    sfree(data.seed as *mut libc::c_void);
                    data
                        .seed = smalloc(
                        ((len_0 + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_char;
                    memcpy(
                        data.seed as *mut libc::c_void,
                        tmp as *const libc::c_void,
                        len_0 as libc::c_ulong,
                    );
                    *(data.seed).offset(len_0 as isize) = '\0' as i32 as libc::c_char;
                    sfree(tmp as *mut libc::c_void);
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"SEED\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sfree(data.seed as *mut libc::c_void);
                    data.seed = val;
                    val = 0 as *mut libc::c_char;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"DESC\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sfree(data.desc as *mut libc::c_void);
                    data.desc = val;
                    val = 0 as *mut libc::c_char;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"PRIVDESC\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sfree(data.privdesc as *mut libc::c_void);
                    data.privdesc = val;
                    val = 0 as *mut libc::c_char;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"AUXINFO\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    let mut tmp_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut len_1: libc::c_int = (strlen(val))
                        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
                    tmp_0 = hex2bin(val, len_1);
                    obfuscate_bitmap(
                        tmp_0,
                        len_1 * 8 as libc::c_int,
                        1 as libc::c_int != 0,
                    );
                    sfree(data.auxinfo as *mut libc::c_void);
                    data
                        .auxinfo = smalloc(
                        ((len_1 + 1 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ) as *mut libc::c_char;
                    memcpy(
                        data.auxinfo as *mut libc::c_void,
                        tmp_0 as *const libc::c_void,
                        len_1 as libc::c_ulong,
                    );
                    *(data.auxinfo).offset(len_1 as isize) = '\0' as i32 as libc::c_char;
                    sfree(tmp_0 as *mut libc::c_void);
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"UI\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sfree(data.uistr as *mut libc::c_void);
                    data.uistr = val;
                    val = 0 as *mut libc::c_char;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"TIME\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    data.elapsed = atof(val) as libc::c_float;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"NSTATES\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if !(data.states).is_null() {
                        ret = b"Two state counts provided in save file\0" as *const u8
                            as *const libc::c_char;
                        current_block = 17011192871324626656;
                        break;
                    } else {
                        data.nstates = atoi(val);
                        if data.nstates <= 0 as libc::c_int {
                            ret = b"Number of states in save file was negative\0"
                                as *const u8 as *const libc::c_char;
                            current_block = 17011192871324626656;
                            break;
                        } else {
                            data
                                .states = smalloc(
                                (data.nstates as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<midend_state_entry>()
                                            as libc::c_ulong,
                                    ),
                            ) as *mut midend_state_entry;
                            i = 0 as libc::c_int;
                            while i < data.nstates {
                                let ref mut fresh23 = (*(data.states).offset(i as isize))
                                    .state;
                                *fresh23 = 0 as *mut game_state;
                                let ref mut fresh24 = (*(data.states).offset(i as isize))
                                    .movestr;
                                *fresh24 = 0 as *mut libc::c_char;
                                (*(data.states).offset(i as isize))
                                    .movetype = NEWGAME as libc::c_int;
                                i += 1;
                                i;
                            }
                        }
                    }
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"STATEPOS\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    data.statepos = atoi(val);
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"MOVE\0" as *const u8 as *const libc::c_char,
                ) == 0
                    || strcmp(
                        key.as_mut_ptr(),
                        b"SOLVE\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || strcmp(
                        key.as_mut_ptr(),
                        b"RESTART\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    if (data.states).is_null() {
                        ret = b"No state count provided in save file\0" as *const u8
                            as *const libc::c_char;
                        current_block = 17011192871324626656;
                        break;
                    } else if data.statepos < 0 as libc::c_int {
                        ret = b"No game position provided in save file\0" as *const u8
                            as *const libc::c_char;
                        current_block = 17011192871324626656;
                        break;
                    } else {
                        gotstates += 1;
                        gotstates;
                        if gotstates < data.nstates {} else {
                            __assert_fail(
                                b"gotstates < data.nstates\0" as *const u8
                                    as *const libc::c_char,
                                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                                2505 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 166],
                                    &[libc::c_char; 166],
                                >(
                                    b"const char *midend_deserialise_internal(midend *, _Bool (*)(void *, void *, int), void *, const char *(*)(void *, midend *, const struct deserialise_data *), void *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_13673: {
                            if gotstates < data.nstates {} else {
                                __assert_fail(
                                    b"gotstates < data.nstates\0" as *const u8
                                        as *const libc::c_char,
                                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                                    2505 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 166],
                                        &[libc::c_char; 166],
                                    >(
                                        b"const char *midend_deserialise_internal(midend *, _Bool (*)(void *, void *, int), void *, const char *(*)(void *, midend *, const struct deserialise_data *), void *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        if strcmp(
                            key.as_mut_ptr(),
                            b"MOVE\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*(data.states).offset(gotstates as isize))
                                .movetype = MOVE as libc::c_int;
                        } else if strcmp(
                            key.as_mut_ptr(),
                            b"SOLVE\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*(data.states).offset(gotstates as isize))
                                .movetype = SOLVE as libc::c_int;
                        } else {
                            (*(data.states).offset(gotstates as isize))
                                .movetype = RESTART as libc::c_int;
                        }
                        let ref mut fresh25 = (*(data.states).offset(gotstates as isize))
                            .movestr;
                        *fresh25 = val;
                        val = 0 as *mut libc::c_char;
                    }
                }
                sfree(val as *mut libc::c_void);
                val = 0 as *mut libc::c_char;
            }
        }
    }
    match current_block {
        10109057886293123569 => {
            data
                .params = ((*(*me).ourgame).default_params)
                .expect("non-null function pointer")();
            if (data.parstr).is_null() {
                ret = b"Long-term parameters in save file are missing\0" as *const u8
                    as *const libc::c_char;
            } else {
                ((*(*me).ourgame).decode_params)
                    .expect("non-null function pointer")(data.params, data.parstr);
                if !(((*(*me).ourgame).validate_params)
                    .expect(
                        "non-null function pointer",
                    )(data.params, 1 as libc::c_int != 0))
                    .is_null()
                {
                    ret = b"Long-term parameters in save file are invalid\0" as *const u8
                        as *const libc::c_char;
                } else {
                    data
                        .cparams = ((*(*me).ourgame).default_params)
                        .expect("non-null function pointer")();
                    if (data.cparstr).is_null() {
                        ret = b"Short-term parameters in save file are missing\0"
                            as *const u8 as *const libc::c_char;
                    } else {
                        ((*(*me).ourgame).decode_params)
                            .expect(
                                "non-null function pointer",
                            )(data.cparams, data.cparstr);
                        if !(((*(*me).ourgame).validate_params)
                            .expect(
                                "non-null function pointer",
                            )(data.cparams, 0 as libc::c_int != 0))
                            .is_null()
                        {
                            ret = b"Short-term parameters in save file are invalid\0"
                                as *const u8 as *const libc::c_char;
                        } else {
                            if !(data.seed).is_null()
                                && !(((*(*me).ourgame).validate_params)
                                    .expect(
                                        "non-null function pointer",
                                    )(data.cparams, 1 as libc::c_int != 0))
                                    .is_null()
                            {
                                sfree(data.seed as *mut libc::c_void);
                                data.seed = 0 as *mut libc::c_char;
                            }
                            if (data.desc).is_null() {
                                ret = b"Game description in save file is missing\0"
                                    as *const u8 as *const libc::c_char;
                            } else if !(((*(*me).ourgame).validate_desc)
                                .expect(
                                    "non-null function pointer",
                                )(data.cparams, data.desc))
                                .is_null()
                            {
                                ret = b"Game description in save file is invalid\0"
                                    as *const u8 as *const libc::c_char;
                            } else if !(data.privdesc).is_null()
                                && !(((*(*me).ourgame).validate_desc)
                                    .expect(
                                        "non-null function pointer",
                                    )(data.cparams, data.privdesc))
                                    .is_null()
                            {
                                ret = b"Game private description in save file is invalid\0"
                                    as *const u8 as *const libc::c_char;
                            } else if data.statepos < 1 as libc::c_int
                                || data.statepos > data.nstates
                            {
                                ret = b"Game position in save file is out of range\0"
                                    as *const u8 as *const libc::c_char;
                            } else if (data.states).is_null() {
                                ret = b"No state count provided in save file\0" as *const u8
                                    as *const libc::c_char;
                            } else {
                                let ref mut fresh26 = (*(data.states)
                                    .offset(0 as libc::c_int as isize))
                                    .state;
                                *fresh26 = ((*(*me).ourgame).new_game)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    me,
                                    data.cparams,
                                    if !(data.privdesc).is_null() {
                                        data.privdesc
                                    } else {
                                        data.desc
                                    },
                                );
                                i = 1 as libc::c_int;
                                loop {
                                    if !(i < data.nstates) {
                                        current_block = 14954430942378083783;
                                        break;
                                    }
                                    if (*(data.states).offset(i as isize)).movetype
                                        != NEWGAME as libc::c_int
                                    {} else {
                                        __assert_fail(
                                            b"data.states[i].movetype != NEWGAME\0" as *const u8
                                                as *const libc::c_char,
                                            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                                            2574 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 166],
                                                &[libc::c_char; 166],
                                            >(
                                                b"const char *midend_deserialise_internal(midend *, _Bool (*)(void *, void *, int), void *, const char *(*)(void *, midend *, const struct deserialise_data *), void *)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_13194: {
                                        if (*(data.states).offset(i as isize)).movetype
                                            != NEWGAME as libc::c_int
                                        {} else {
                                            __assert_fail(
                                                b"data.states[i].movetype != NEWGAME\0" as *const u8
                                                    as *const libc::c_char,
                                                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                                                2574 as libc::c_int as libc::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 166],
                                                    &[libc::c_char; 166],
                                                >(
                                                    b"const char *midend_deserialise_internal(midend *, _Bool (*)(void *, void *, int), void *, const char *(*)(void *, midend *, const struct deserialise_data *), void *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    match (*(data.states).offset(i as isize)).movetype {
                                        1 | 2 => {
                                            let ref mut fresh27 = (*(data.states).offset(i as isize))
                                                .state;
                                            *fresh27 = ((*(*me).ourgame).execute_move)
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                (*(data.states).offset((i - 1 as libc::c_int) as isize))
                                                    .state,
                                                (*(data.states).offset(i as isize)).movestr,
                                            );
                                            if ((*(data.states).offset(i as isize)).state).is_null() {
                                                ret = b"Save file contained an invalid move\0" as *const u8
                                                    as *const libc::c_char;
                                                current_block = 17011192871324626656;
                                                break;
                                            }
                                        }
                                        3 => {
                                            if !(((*(*me).ourgame).validate_desc)
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                data.cparams,
                                                (*(data.states).offset(i as isize)).movestr,
                                            ))
                                                .is_null()
                                            {
                                                ret = b"Save file contained an invalid restart move\0"
                                                    as *const u8 as *const libc::c_char;
                                                current_block = 17011192871324626656;
                                                break;
                                            } else {
                                                let ref mut fresh28 = (*(data.states).offset(i as isize))
                                                    .state;
                                                *fresh28 = ((*(*me).ourgame).new_game)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(
                                                    me,
                                                    data.cparams,
                                                    (*(data.states).offset(i as isize)).movestr,
                                                );
                                            }
                                        }
                                        _ => {}
                                    }
                                    i += 1;
                                    i;
                                }
                                match current_block {
                                    17011192871324626656 => {}
                                    _ => {
                                        data
                                            .ui = ((*(*me).ourgame).new_ui)
                                            .expect(
                                                "non-null function pointer",
                                            )((*(data.states).offset(0 as libc::c_int as isize)).state);
                                        midend_apply_prefs(me, data.ui);
                                        if !(data.uistr).is_null()
                                            && ((*(*me).ourgame).decode_ui).is_some()
                                        {
                                            ((*(*me).ourgame).decode_ui)
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                data.ui,
                                                data.uistr,
                                                (*(data.states)
                                                    .offset((data.statepos - 1 as libc::c_int) as isize))
                                                    .state,
                                            );
                                        }
                                        if !(check.is_some()
                                            && {
                                                ret = check
                                                    .expect("non-null function pointer")(cctx, me, &mut data);
                                                !ret.is_null()
                                            })
                                        {
                                            let mut tmp_1: *mut libc::c_char = 0 as *mut libc::c_char;
                                            tmp_1 = (*me).desc;
                                            (*me).desc = data.desc;
                                            data.desc = tmp_1;
                                            tmp_1 = (*me).privdesc;
                                            (*me).privdesc = data.privdesc;
                                            data.privdesc = tmp_1;
                                            tmp_1 = (*me).seedstr;
                                            (*me).seedstr = data.seed;
                                            data.seed = tmp_1;
                                            tmp_1 = (*me).aux_info;
                                            (*me).aux_info = data.auxinfo;
                                            data.auxinfo = tmp_1;
                                            (*me).genmode = GOT_NOTHING;
                                            (*me).statesize = data.nstates;
                                            data.nstates = (*me).nstates;
                                            (*me).nstates = (*me).statesize;
                                            let mut tmp_2: *mut midend_state_entry = 0
                                                as *mut midend_state_entry;
                                            tmp_2 = (*me).states;
                                            (*me).states = data.states;
                                            data.states = tmp_2;
                                            (*me).statepos = data.statepos;
                                            (*me).newgame_undo.len = 0 as libc::c_int;
                                            (*me).newgame_redo.len = 0 as libc::c_int;
                                            let mut tmp_3: *mut game_params = 0 as *mut game_params;
                                            tmp_3 = (*me).params;
                                            (*me).params = data.params;
                                            data.params = tmp_3;
                                            tmp_3 = (*me).curparams;
                                            (*me).curparams = data.cparams;
                                            data.cparams = tmp_3;
                                            (*me).oldstate = 0 as *mut game_state;
                                            (*me).flash_pos = 0.0f32;
                                            (*me).flash_time = (*me).flash_pos;
                                            (*me).anim_pos = (*me).flash_time;
                                            (*me).anim_time = (*me).anim_pos;
                                            (*me).dir = 0 as libc::c_int;
                                            let mut tmp_4: *mut game_ui = 0 as *mut game_ui;
                                            tmp_4 = (*me).ui;
                                            (*me).ui = data.ui;
                                            data.ui = tmp_4;
                                            (*me).elapsed = data.elapsed;
                                            (*me).pressed_mouse_button = 0 as libc::c_int;
                                            midend_set_timer(me);
                                            if !((*me).drawstate).is_null() {
                                                ((*(*me).ourgame).free_drawstate)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )((*me).drawing, (*me).drawstate);
                                            }
                                            (*me)
                                                .drawstate = ((*(*me).ourgame).new_drawstate)
                                                .expect(
                                                    "non-null function pointer",
                                                )(
                                                (*me).drawing,
                                                (*((*me).states)
                                                    .offset(((*me).statepos - 1 as libc::c_int) as isize))
                                                    .state,
                                            );
                                            (*me).first_draw = 1 as libc::c_int != 0;
                                            midend_size_new_drawstate(me);
                                            if ((*me).game_id_change_notify_function).is_some() {
                                                ((*me).game_id_change_notify_function)
                                                    .expect(
                                                        "non-null function pointer",
                                                    )((*me).game_id_change_notify_ctx);
                                            }
                                            ret = 0 as *const libc::c_char;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    sfree(val as *mut libc::c_void);
    sfree(data.seed as *mut libc::c_void);
    sfree(data.parstr as *mut libc::c_void);
    sfree(data.cparstr as *mut libc::c_void);
    sfree(data.desc as *mut libc::c_void);
    sfree(data.privdesc as *mut libc::c_void);
    sfree(data.auxinfo as *mut libc::c_void);
    sfree(data.uistr as *mut libc::c_void);
    if !(data.params).is_null() {
        ((*(*me).ourgame).free_params).expect("non-null function pointer")(data.params);
    }
    if !(data.cparams).is_null() {
        ((*(*me).ourgame).free_params).expect("non-null function pointer")(data.cparams);
    }
    if !(data.ui).is_null() {
        ((*(*me).ourgame).free_ui).expect("non-null function pointer")(data.ui);
    }
    if !(data.states).is_null() {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < data.nstates {
            if !((*(data.states).offset(i_0 as isize)).state).is_null() {
                ((*(*me).ourgame).free_game)
                    .expect(
                        "non-null function pointer",
                    )((*(data.states).offset(i_0 as isize)).state);
            }
            sfree((*(data.states).offset(i_0 as isize)).movestr as *mut libc::c_void);
            i_0 += 1;
            i_0;
        }
        sfree(data.states as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn midend_deserialise(
    mut me: *mut midend,
    mut read: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, libc::c_int) -> bool,
    >,
    mut rctx: *mut libc::c_void,
) -> *const libc::c_char {
    return midend_deserialise_internal(me, read, rctx, None, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn identify_game(
    mut name: *mut *mut libc::c_char,
    mut read: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, libc::c_int) -> bool,
    >,
    mut rctx: *mut libc::c_void,
) -> *const libc::c_char {
    let mut nstates: libc::c_int = 0 as libc::c_int;
    let mut statepos: libc::c_int = -(1 as libc::c_int);
    let mut gotstates: libc::c_int = 0 as libc::c_int;
    let mut started: bool = 0 as libc::c_int != 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: *const libc::c_char = b"Data does not appear to be a saved game file\0"
        as *const u8 as *const libc::c_char;
    *name = 0 as *mut libc::c_char;
    's_12: while nstates <= 0 as libc::c_int || statepos < 0 as libc::c_int
        || gotstates < nstates - 1 as libc::c_int
    {
        let mut key: [libc::c_char; 9] = [0; 9];
        let mut c: libc::c_char = 0;
        let mut len: libc::c_int = 0;
        loop {
            if !read
                .expect(
                    "non-null function pointer",
                )(rctx, key.as_mut_ptr() as *mut libc::c_void, 1 as libc::c_int)
            {
                break 's_12;
            } else if !(key[0 as libc::c_int as usize] as libc::c_int == '\r' as i32
                || key[0 as libc::c_int as usize] as libc::c_int == '\n' as i32)
            {
                break;
            }
        }
        if !read
            .expect(
                "non-null function pointer",
            )(
            rctx,
            key.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
            8 as libc::c_int,
        )
        {
            break;
        } else if key[8 as libc::c_int as usize] as libc::c_int != ':' as i32 {
            if started {
                ret = b"Data was incorrectly formatted for a saved game file\0"
                    as *const u8 as *const libc::c_char;
            }
            break;
        } else {
            len = strcspn(key.as_mut_ptr(), b": \0" as *const u8 as *const libc::c_char)
                as libc::c_int;
            if len <= 8 as libc::c_int {} else {
                __assert_fail(
                    b"len <= 8\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    2782 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"const char *identify_game(char **, _Bool (*)(void *, void *, int), void *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_19749: {
                if len <= 8 as libc::c_int {} else {
                    __assert_fail(
                        b"len <= 8\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                        2782 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 75],
                            &[libc::c_char; 75],
                        >(
                            b"const char *identify_game(char **, _Bool (*)(void *, void *, int), void *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            key[len as usize] = '\0' as i32 as libc::c_char;
            len = 0 as libc::c_int;
            loop {
                if !read
                    .expect(
                        "non-null function pointer",
                    )(
                    rctx,
                    &mut c as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int,
                )
                {
                    break 's_12;
                } else {
                    if c as libc::c_int == ':' as i32 {
                        break;
                    }
                    if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
                        && len
                            < (2147483647 as libc::c_int - 10 as libc::c_int)
                                / 10 as libc::c_int
                    {
                        len = len * 10 as libc::c_int + (c as libc::c_int - '0' as i32);
                    } else {
                        if started {
                            ret = b"Data was incorrectly formatted for a saved game file\0"
                                as *const u8 as *const libc::c_char;
                        }
                        break 's_12;
                    }
                }
            }
            val = smalloc(
                ((len + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            if !read
                .expect("non-null function pointer")(rctx, val as *mut libc::c_void, len)
            {
                break;
            } else {
                *val.offset(len as isize) = '\0' as i32 as libc::c_char;
                if !started {
                    if strcmp(
                        key.as_mut_ptr(),
                        b"SAVEFILE\0" as *const u8 as *const libc::c_char,
                    ) != 0
                        || strcmp(
                            val,
                            b"Simon Tatham's Portable Puzzle Collection\0" as *const u8
                                as *const libc::c_char,
                        ) != 0
                    {
                        break;
                    }
                    ret = b"Saved data ended unexpectedly\0" as *const u8
                        as *const libc::c_char;
                    started = 1 as libc::c_int != 0;
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"VERSION\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if strcmp(val, b"1\0" as *const u8 as *const libc::c_char) != 0 {
                        ret = b"Cannot handle this version of the saved game file format\0"
                            as *const u8 as *const libc::c_char;
                        break;
                    }
                } else if strcmp(
                    key.as_mut_ptr(),
                    b"GAME\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    *name = dupstr(val);
                    ret = 0 as *const libc::c_char;
                    break;
                }
                sfree(val as *mut libc::c_void);
                val = 0 as *mut libc::c_char;
            }
        }
    }
    sfree(val as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn midend_print_puzzle(
    mut me: *mut midend,
    mut doc: *mut document,
    mut with_soln: bool,
) -> *const libc::c_char {
    let mut soln: *mut game_state = 0 as *mut game_state;
    if (*me).statepos < 1 as libc::c_int {
        return b"No game set up to print\0" as *const u8 as *const libc::c_char;
    }
    if with_soln {
        let mut msg: *const libc::c_char = 0 as *const libc::c_char;
        let mut movestr: *mut libc::c_char = 0 as *mut libc::c_char;
        if !(*(*me).ourgame).can_solve {
            return b"This game does not support the Solve operation\0" as *const u8
                as *const libc::c_char;
        }
        msg = b"Solve operation failed\0" as *const u8 as *const libc::c_char;
        movestr = ((*(*me).ourgame).solve)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(0 as libc::c_int as isize)).state,
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
            (*me).aux_info,
            &mut msg,
        );
        if movestr.is_null() {
            return msg;
        }
        soln = ((*(*me).ourgame).execute_move)
            .expect(
                "non-null function pointer",
            )(
            (*((*me).states).offset(((*me).statepos - 1 as libc::c_int) as isize)).state,
            movestr,
        );
        if !soln.is_null() {} else {
            __assert_fail(
                b"soln\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2864 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"const char *midend_print_puzzle(midend *, document *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_20171: {
            if !soln.is_null() {} else {
                __assert_fail(
                    b"soln\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    2864 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 61],
                        &[libc::c_char; 61],
                    >(b"const char *midend_print_puzzle(midend *, document *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
        sfree(movestr as *mut libc::c_void);
    } else {
        soln = 0 as *mut game_state;
    }
    let mut ui: *mut game_ui = ((*(*me).ourgame).new_ui)
        .expect(
            "non-null function pointer",
        )((*((*me).states).offset(0 as libc::c_int as isize)).state);
    midend_apply_prefs(me, ui);
    document_add_puzzle(
        doc,
        (*me).ourgame,
        ((*(*me).ourgame).dup_params)
            .expect("non-null function pointer")((*me).curparams),
        ui,
        ((*(*me).ourgame).dup_game)
            .expect(
                "non-null function pointer",
            )((*((*me).states).offset(0 as libc::c_int as isize)).state),
        soln,
    );
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn midend_apply_prefs(mut me: *mut midend, mut ui: *mut game_ui) {
    let mut rctx: [midend_serialise_buf_read_ctx; 1] = [midend_serialise_buf_read_ctx {
        ser: 0 as *mut midend_serialise_buf,
        len: 0,
        pos: 0,
    }; 1];
    let ref mut fresh29 = (*rctx.as_mut_ptr()).ser;
    *fresh29 = &mut (*me).be_prefs;
    (*rctx.as_mut_ptr()).len = (*me).be_prefs.len;
    (*rctx.as_mut_ptr()).pos = 0 as libc::c_int;
    let mut err: *const libc::c_char = midend_deserialise_prefs(
        me,
        ui,
        Some(
            midend_serialise_buf_read
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> bool,
        ),
        rctx.as_mut_ptr() as *mut libc::c_void,
    );
    if err.is_null()
        && !(b"Bad internal serialisation of preferences\0" as *const u8
            as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"!err && \"Bad internal serialisation of preferences\"\0" as *const u8
                as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            2895 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[libc::c_char; 45],
            >(b"void midend_apply_prefs(midend *, game_ui *)\0"))
                .as_ptr(),
        );
    }
    'c_4794: {
        if err.is_null()
            && !(b"Bad internal serialisation of preferences\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"!err && \"Bad internal serialisation of preferences\"\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2895 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void midend_apply_prefs(midend *, game_ui *)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn midend_get_prefs(
    mut me: *mut midend,
    mut ui: *mut game_ui,
) -> *mut config_item {
    let mut n_be_prefs: libc::c_int = 0;
    let mut n_me_prefs: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut all_prefs: *mut config_item = 0 as *mut config_item;
    let mut be_prefs: *mut config_item = 0 as *mut config_item;
    be_prefs = 0 as *mut config_item;
    n_be_prefs = 0 as libc::c_int;
    if ((*(*me).ourgame).get_prefs).is_some() {
        if !ui.is_null() {
            be_prefs = ((*(*me).ourgame).get_prefs)
                .expect("non-null function pointer")(ui);
        } else if !((*me).ui).is_null() {
            be_prefs = ((*(*me).ourgame).get_prefs)
                .expect("non-null function pointer")((*me).ui);
        } else {
            let mut tmp_ui: *mut game_ui = ((*(*me).ourgame).new_ui)
                .expect("non-null function pointer")(0 as *const game_state);
            be_prefs = ((*(*me).ourgame).get_prefs)
                .expect("non-null function pointer")(tmp_ui);
            ((*(*me).ourgame).free_ui).expect("non-null function pointer")(tmp_ui);
        }
        while (*be_prefs.offset(n_be_prefs as isize)).type_0 != C_END as libc::c_int {
            n_be_prefs += 1;
            n_be_prefs;
        }
    }
    n_me_prefs = 1 as libc::c_int;
    all_prefs = smalloc(
        ((n_me_prefs + n_be_prefs + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<config_item>() as libc::c_ulong),
    ) as *mut config_item;
    pos = 0 as libc::c_int;
    if pos < n_me_prefs {} else {
        __assert_fail(
            b"pos < n_me_prefs\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            2924 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[libc::c_char; 51],
            >(b"config_item *midend_get_prefs(midend *, game_ui *)\0"))
                .as_ptr(),
        );
    }
    'c_5745: {
        if pos < n_me_prefs {} else {
            __assert_fail(
                b"pos < n_me_prefs\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2924 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[libc::c_char; 51],
                >(b"config_item *midend_get_prefs(midend *, game_ui *)\0"))
                    .as_ptr(),
            );
        }
    };
    let ref mut fresh30 = (*all_prefs.offset(pos as isize)).name;
    *fresh30 = b"Keyboard shortcuts without Ctrl\0" as *const u8 as *const libc::c_char;
    let ref mut fresh31 = (*all_prefs.offset(pos as isize)).kw;
    *fresh31 = b"one-key-shortcuts\0" as *const u8 as *const libc::c_char;
    (*all_prefs.offset(pos as isize)).type_0 = C_BOOLEAN as libc::c_int;
    (*all_prefs.offset(pos as isize)).u.boolean.bval = (*me).one_key_shortcuts;
    pos += 1;
    pos;
    i = 0 as libc::c_int;
    while i < n_be_prefs {
        *all_prefs.offset(pos as isize) = *be_prefs.offset(i as isize);
        pos += 1;
        pos;
        i += 1;
        i;
    }
    let ref mut fresh32 = (*all_prefs.offset(pos as isize)).name;
    *fresh32 = 0 as *const libc::c_char;
    (*all_prefs.offset(pos as isize)).type_0 = C_END as libc::c_int;
    if !be_prefs.is_null() {
        sfree(be_prefs as *mut libc::c_void);
    }
    return all_prefs;
}
unsafe extern "C" fn midend_set_prefs(
    mut me: *mut midend,
    mut ui: *mut game_ui,
    mut all_prefs: *mut config_item,
) {
    let mut pos: libc::c_int = 0 as libc::c_int;
    let mut tmpui: *mut game_ui = 0 as *mut game_ui;
    (*me).one_key_shortcuts = (*all_prefs.offset(pos as isize)).u.boolean.bval;
    pos += 1;
    pos;
    if ((*(*me).ourgame).get_prefs).is_some() {
        if ui.is_null() {
            tmpui = ((*(*me).ourgame).new_ui)
                .expect("non-null function pointer")(0 as *const game_state);
            ui = tmpui;
        }
        ((*(*me).ourgame).set_prefs)
            .expect("non-null function pointer")(ui, all_prefs.offset(pos as isize));
    }
    (*me).be_prefs.len = 0 as libc::c_int;
    midend_serialise_prefs(
        me,
        ui,
        Some(
            midend_serialise_buf_write
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
        &mut (*me).be_prefs as *mut midend_serialise_buf as *mut libc::c_void,
    );
    if !tmpui.is_null() {
        ((*(*me).ourgame).free_ui).expect("non-null function pointer")(tmpui);
    }
}
unsafe extern "C" fn midend_serialise_prefs(
    mut me: *mut midend,
    mut ui: *mut game_ui,
    mut write: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void, libc::c_int) -> (),
    >,
    mut wctx: *mut libc::c_void,
) {
    let mut cfg: *mut config_item = 0 as *mut config_item;
    let mut i: libc::c_int = 0;
    cfg = midend_get_prefs(me, ui);
    if !cfg.is_null() {} else {
        __assert_fail(
            b"cfg\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            2977 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 94],
                &[libc::c_char; 94],
            >(
                b"void midend_serialise_prefs(midend *, game_ui *, void (*)(void *, const void *, int), void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_5574: {
        if !cfg.is_null() {} else {
            __assert_fail(
                b"cfg\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2977 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"void midend_serialise_prefs(midend *, game_ui *, void (*)(void *, const void *, int), void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while (*cfg.offset(i as isize)).type_0 != C_END as libc::c_int {
        let mut it: *mut config_item = &mut *cfg.offset(i as isize) as *mut config_item;
        if *((*it).kw)
            .offset(
                strspn(
                    (*it).kw,
                    b"abcdefghijklmnopqrstuvwxyz-\0" as *const u8 as *const libc::c_char,
                ) as isize,
            ) as libc::c_int == '\0' as i32
        {} else {
            __assert_fail(
                b"it->kw[strspn(it->kw, \"abcdefghijklmnopqrstuvwxyz-\")] == '\\0'\0"
                    as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                2983 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 94],
                    &[libc::c_char; 94],
                >(
                    b"void midend_serialise_prefs(midend *, game_ui *, void (*)(void *, const void *, int), void *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5496: {
            if *((*it).kw)
                .offset(
                    strspn(
                        (*it).kw,
                        b"abcdefghijklmnopqrstuvwxyz-\0" as *const u8
                            as *const libc::c_char,
                    ) as isize,
                ) as libc::c_int == '\0' as i32
            {} else {
                __assert_fail(
                    b"it->kw[strspn(it->kw, \"abcdefghijklmnopqrstuvwxyz-\")] == '\\0'\0"
                        as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    2983 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 94],
                        &[libc::c_char; 94],
                    >(
                        b"void midend_serialise_prefs(midend *, game_ui *, void (*)(void *, const void *, int), void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        write
            .expect(
                "non-null function pointer",
            )(wctx, (*it).kw as *const libc::c_void, strlen((*it).kw) as libc::c_int);
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
        match (*it).type_0 {
            2 => {
                if (*it).u.boolean.bval {
                    write
                        .expect(
                            "non-null function pointer",
                        )(
                        wctx,
                        b"true\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        4 as libc::c_int,
                    );
                } else {
                    write
                        .expect(
                            "non-null function pointer",
                        )(
                        wctx,
                        b"false\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        5 as libc::c_int,
                    );
                }
            }
            0 => {
                let mut p: *const libc::c_char = (*it).u.string.sval;
                while *p != 0 {
                    let fresh33 = p;
                    p = p.offset(1);
                    let mut c: libc::c_char = *fresh33;
                    write
                        .expect(
                            "non-null function pointer",
                        )(
                        wctx,
                        &mut c as *mut libc::c_char as *const libc::c_void,
                        1 as libc::c_int,
                    );
                    if c as libc::c_int == '\n' as i32 {
                        write
                            .expect(
                                "non-null function pointer",
                            )(
                            wctx,
                            b" \0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            1 as libc::c_int,
                        );
                    }
                }
            }
            1 => {
                let mut n: libc::c_int = (*it).u.choices.selected;
                let mut p_0: *const libc::c_char = (*it).u.choices.choicekws;
                let mut sepstr: [libc::c_char; 2] = [0; 2];
                let fresh34 = p_0;
                p_0 = p_0.offset(1);
                sepstr[0 as libc::c_int as usize] = *fresh34;
                sepstr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                while n > 0 as libc::c_int {
                    let mut q: *const libc::c_char = strchr(
                        p_0,
                        sepstr[0 as libc::c_int as usize] as libc::c_int,
                    );
                    if !q.is_null()
                        && !(b"Value out of range in C_CHOICES\0" as *const u8
                            as *const libc::c_char)
                            .is_null()
                    {} else {
                        __assert_fail(
                            b"q != NULL && \"Value out of range in C_CHOICES\"\0"
                                as *const u8 as *const libc::c_char,
                            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                            3015 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 94],
                                &[libc::c_char; 94],
                            >(
                                b"void midend_serialise_prefs(midend *, game_ui *, void (*)(void *, const void *, int), void *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_5312: {
                        if !q.is_null()
                            && !(b"Value out of range in C_CHOICES\0" as *const u8
                                as *const libc::c_char)
                                .is_null()
                        {} else {
                            __assert_fail(
                                b"q != NULL && \"Value out of range in C_CHOICES\"\0"
                                    as *const u8 as *const libc::c_char,
                                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                                3015 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 94],
                                    &[libc::c_char; 94],
                                >(
                                    b"void midend_serialise_prefs(midend *, game_ui *, void (*)(void *, const void *, int), void *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    p_0 = q.offset(1 as libc::c_int as isize);
                    n -= 1;
                    n;
                }
                write
                    .expect(
                        "non-null function pointer",
                    )(
                    wctx,
                    p_0 as *const libc::c_void,
                    strcspn(p_0, sepstr.as_mut_ptr()) as libc::c_int,
                );
            }
            _ => {}
        }
        write
            .expect(
                "non-null function pointer",
            )(
            wctx,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
        i += 1;
        i;
    }
    free_cfg(cfg);
}
unsafe extern "C" fn buffer_append(mut buf: *mut buffer, mut c: libc::c_char) {
    if ((*buf).len).wrapping_add(2 as libc::c_int as size_t) > (*buf).size {
        let mut new_size: size_t = ((*buf).size)
            .wrapping_add((*buf).size / 4 as libc::c_int as size_t)
            .wrapping_add(128 as libc::c_int as size_t);
        if new_size > (*buf).size {} else {
            __assert_fail(
                b"new_size > buf->size\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                3040 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void buffer_append(struct buffer *, char)\0"))
                    .as_ptr(),
            );
        }
        'c_6417: {
            if new_size > (*buf).size {} else {
                __assert_fail(
                    b"new_size > buf->size\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    3040 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"void buffer_append(struct buffer *, char)\0"))
                        .as_ptr(),
                );
            }
        };
        (*buf)
            .data = srealloc(
            (*buf).data as *mut libc::c_void,
            new_size
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        (*buf).size = new_size;
        if (*buf).len < (*buf).size {} else {
            __assert_fail(
                b"buf->len < buf->size\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                3043 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void buffer_append(struct buffer *, char)\0"))
                    .as_ptr(),
            );
        }
        'c_6329: {
            if (*buf).len < (*buf).size {} else {
                __assert_fail(
                    b"buf->len < buf->size\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                    3043 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"void buffer_append(struct buffer *, char)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    let fresh35 = (*buf).len;
    (*buf).len = ((*buf).len).wrapping_add(1);
    *((*buf).data).offset(fresh35 as isize) = c;
    if (*buf).len < (*buf).size {} else {
        __assert_fail(
            b"buf->len < buf->size\0" as *const u8 as *const libc::c_char,
            b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
            3046 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void buffer_append(struct buffer *, char)\0"))
                .as_ptr(),
        );
    }
    'c_6263: {
        if (*buf).len < (*buf).size {} else {
            __assert_fail(
                b"buf->len < buf->size\0" as *const u8 as *const libc::c_char,
                b"/puzzles/midend.c\0" as *const u8 as *const libc::c_char,
                3046 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void buffer_append(struct buffer *, char)\0"))
                    .as_ptr(),
            );
        }
    };
    *((*buf).data).offset((*buf).len as isize) = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn midend_deserialise_prefs(
    mut me: *mut midend,
    mut ui: *mut game_ui,
    mut read: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, libc::c_int) -> bool,
    >,
    mut rctx: *mut libc::c_void,
) -> *const libc::c_char {
    let mut cfg: *mut config_item = 0 as *mut config_item;
    let mut it: *mut config_item = 0 as *mut config_item;
    let mut i: libc::c_int = 0;
    let mut buf: [buffer; 1] = [
        {
            let mut init = buffer {
                data: 0 as *mut libc::c_char,
                len: 0 as libc::c_int as size_t,
                size: 0 as libc::c_int as size_t,
            };
            init
        },
    ];
    let mut errmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut read_char: libc::c_char = 0;
    let mut ungot_char: libc::c_char = '\0' as i32 as libc::c_char;
    let mut have_ungot_a_char: bool = 0 as libc::c_int != 0;
    let mut eof: bool = 0 as libc::c_int != 0;
    cfg = midend_get_prefs(me, ui);
    's_18: while !eof {
        if have_ungot_a_char {
            read_char = ungot_char;
            have_ungot_a_char = 0 as libc::c_int != 0;
        } else if !read
            .expect(
                "non-null function pointer",
            )(
            rctx,
            &mut read_char as *mut libc::c_char as *mut libc::c_void,
            1 as libc::c_int,
        )
        {
            break;
        }
        if read_char as libc::c_int == '#' as i32
            || read_char as libc::c_int == '\n' as i32
        {
            while read_char as libc::c_int != '\n' as i32 {
                if !read
                    .expect(
                        "non-null function pointer",
                    )(
                    rctx,
                    &mut read_char as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int,
                )
                {
                    break 's_18;
                }
            }
        } else {
            (*buf.as_mut_ptr()).len = 0 as libc::c_int as size_t;
            loop {
                buffer_append(buf.as_mut_ptr(), read_char);
                if !read
                    .expect(
                        "non-null function pointer",
                    )(
                    rctx,
                    &mut read_char as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int,
                )
                {
                    errmsg = b"Partial line at end of preferences file\0" as *const u8
                        as *const libc::c_char;
                    break 's_18;
                } else if read_char as libc::c_int == '\n' as i32 {
                    errmsg = b"Expected '=' after keyword\0" as *const u8
                        as *const libc::c_char;
                    break 's_18;
                } else if read_char as libc::c_int == '=' as i32 {
                    break;
                }
            }
            it = 0 as *mut config_item;
            i = 0 as libc::c_int;
            while (*cfg.offset(i as isize)).type_0 != C_END as libc::c_int {
                if strcmp((*buf.as_mut_ptr()).data, (*cfg.offset(i as isize)).kw) == 0 {
                    it = &mut *cfg.offset(i as isize) as *mut config_item;
                }
                i += 1;
                i;
            }
            (*buf.as_mut_ptr()).len = 0 as libc::c_int as size_t;
            loop {
                if !read
                    .expect(
                        "non-null function pointer",
                    )(
                    rctx,
                    &mut read_char as *mut libc::c_char as *mut libc::c_void,
                    1 as libc::c_int,
                )
                {
                    eof = 1 as libc::c_int != 0;
                    break;
                } else if read_char as libc::c_int == '\n' as i32 {
                    if read
                        .expect(
                            "non-null function pointer",
                        )(
                        rctx,
                        &mut read_char as *mut libc::c_char as *mut libc::c_void,
                        1 as libc::c_int,
                    )
                    {
                        if read_char as libc::c_int == ' ' as i32 {
                            buffer_append(buf.as_mut_ptr(), '\n' as i32 as libc::c_char);
                        } else {
                            ungot_char = read_char;
                            have_ungot_a_char = 1 as libc::c_int != 0;
                            break;
                        }
                    } else {
                        eof = 1 as libc::c_int != 0;
                        break;
                    }
                } else {
                    buffer_append(buf.as_mut_ptr(), read_char);
                }
            }
            if it.is_null() {
                continue;
            }
            match (*it).type_0 {
                2 => {
                    if strcmp(
                        (*buf.as_mut_ptr()).data,
                        b"true\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*it).u.boolean.bval = 1 as libc::c_int != 0;
                    } else if strcmp(
                        (*buf.as_mut_ptr()).data,
                        b"false\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*it).u.boolean.bval = 0 as libc::c_int != 0;
                    } else {
                        errmsg = b"Value for boolean was not 'true' or 'false'\0"
                            as *const u8 as *const libc::c_char;
                        break;
                    }
                }
                0 => {
                    sfree((*it).u.string.sval as *mut libc::c_void);
                    (*it).u.string.sval = (*buf.as_mut_ptr()).data;
                    let ref mut fresh36 = (*buf.as_mut_ptr()).data;
                    *fresh36 = 0 as *mut libc::c_char;
                    let ref mut fresh37 = (*buf.as_mut_ptr()).size;
                    *fresh37 = 0 as libc::c_int as size_t;
                    (*buf.as_mut_ptr()).len = *fresh37;
                }
                1 => {
                    let mut n: libc::c_int = 0 as libc::c_int;
                    let mut found: bool = 0 as libc::c_int != 0;
                    let mut p: *const libc::c_char = (*it).u.choices.choicekws;
                    let mut sepstr: [libc::c_char; 2] = [0; 2];
                    sepstr[0 as libc::c_int as usize] = *p;
                    sepstr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    loop {
                        let fresh38 = p;
                        p = p.offset(1);
                        if !(*fresh38 != 0) {
                            break;
                        }
                        let mut len: libc::c_int = strcspn(p, sepstr.as_mut_ptr())
                            as libc::c_int;
                        if (*buf.as_mut_ptr()).len == len as size_t
                            && memcmp(
                                p as *const libc::c_void,
                                (*buf.as_mut_ptr()).data as *const libc::c_void,
                                len as libc::c_ulong,
                            ) == 0
                        {
                            (*it).u.choices.selected = n;
                            found = 1 as libc::c_int != 0;
                            break;
                        } else {
                            p = p.offset(len as isize);
                            n += 1;
                            n;
                        }
                    }
                    if found {
                        continue;
                    }
                    errmsg = b"Invalid value for enumeration\0" as *const u8
                        as *const libc::c_char;
                    break;
                }
                _ => {}
            }
        }
    }
    if errmsg.is_null() {
        midend_set_prefs(me, ui, cfg);
    }
    free_cfg(cfg);
    sfree((*buf.as_mut_ptr()).data as *mut libc::c_void);
    return errmsg;
}
