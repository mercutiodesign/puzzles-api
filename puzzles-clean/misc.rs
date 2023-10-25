#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
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
    pub type drawing;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fatal(fmt: *const libc::c_char, _: ...);
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
    fn draw_line(
        dr: *mut drawing,
        x1: libc::c_int,
        y1: libc::c_int,
        x2: libc::c_int,
        y2: libc::c_int,
        colour: libc::c_int,
    );
    fn draw_polygon(
        dr: *mut drawing,
        coords: *const libc::c_int,
        npoints: libc::c_int,
        fillcolour: libc::c_int,
        outlinecolour: libc::c_int,
    );
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn srealloc(p: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn dupstr(s: *const libc::c_char) -> *mut libc::c_char;
    fn random_upto(state: *mut random_state, limit: libc::c_ulong) -> libc::c_ulong;
    fn SHA_Init(s: *mut SHA_State);
    fn SHA_Bytes(s: *mut SHA_State, p: *const libc::c_void, len: libc::c_int);
    fn SHA_Final(s: *mut SHA_State, output: *mut libc::c_uchar);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_uint;
pub const MOD_MASK: C2RustUnnamed = 28672;
pub const MOD_NUM_KEYPAD: C2RustUnnamed = 16384;
pub const MOD_SHFT: C2RustUnnamed = 8192;
pub const MOD_CTRL: C2RustUnnamed = 4096;
pub const UI_UPPER_BOUND: C2RustUnnamed = 533;
pub const UI_REDO: C2RustUnnamed = 532;
pub const UI_UNDO: C2RustUnnamed = 531;
pub const UI_SOLVE: C2RustUnnamed = 530;
pub const UI_NEWGAME: C2RustUnnamed = 529;
pub const UI_QUIT: C2RustUnnamed = 528;
pub const UI_LOWER_BOUND: C2RustUnnamed = 527;
pub const CURSOR_SELECT2: C2RustUnnamed = 526;
pub const CURSOR_SELECT: C2RustUnnamed = 525;
pub const CURSOR_RIGHT: C2RustUnnamed = 524;
pub const CURSOR_LEFT: C2RustUnnamed = 523;
pub const CURSOR_DOWN: C2RustUnnamed = 522;
pub const CURSOR_UP: C2RustUnnamed = 521;
pub const RIGHT_RELEASE: C2RustUnnamed = 520;
pub const MIDDLE_RELEASE: C2RustUnnamed = 519;
pub const LEFT_RELEASE: C2RustUnnamed = 518;
pub const RIGHT_DRAG: C2RustUnnamed = 517;
pub const MIDDLE_DRAG: C2RustUnnamed = 516;
pub const LEFT_DRAG: C2RustUnnamed = 515;
pub const RIGHT_BUTTON: C2RustUnnamed = 514;
pub const MIDDLE_BUTTON: C2RustUnnamed = 513;
pub const LEFT_BUTTON: C2RustUnnamed = 512;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_item {
    pub name: *const libc::c_char,
    pub kw: *const libc::c_char,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub string: C2RustUnnamed_3,
    pub choices: C2RustUnnamed_2,
    pub boolean: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub bval: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub choicenames: *const libc::c_char,
    pub choicekws: *const libc::c_char,
    pub selected: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub sval: *mut libc::c_char,
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const C_END: C2RustUnnamed_4 = 3;
pub const C_BOOLEAN: C2RustUnnamed_4 = 2;
pub const C_CHOICES: C2RustUnnamed_4 = 1;
pub const C_STRING: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct step {
    pub seedstart: *mut libc::c_uchar,
    pub seedlen: libc::c_int,
    pub targetstart: *mut libc::c_uchar,
    pub targetlen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA_State {
    pub h: [uint32; 5],
    pub block: [libc::c_uchar; 64],
    pub blkused: libc::c_int,
    pub lenhi: uint32,
    pub lenlo: uint32,
}
pub type uint32 = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn __tg_sqrt(mut __x: libc::c_float) -> libc::c_float {
    return sqrtf(__x);
}
#[inline(always)]
unsafe extern "C" fn __tg_sqrt_0(mut __x: libc::c_double) -> libc::c_double {
    return sqrt(__x);
}
#[no_mangle]
pub static mut MOVE_UI_UPDATE: [libc::c_char; 1] = unsafe {
    *::core::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0")
};
#[no_mangle]
pub static mut MOVE_NO_EFFECT: [libc::c_char; 1] = unsafe {
    *::core::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0")
};
#[no_mangle]
pub static mut MOVE_UNUSED: [libc::c_char; 1] = unsafe {
    *::core::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0")
};
#[no_mangle]
pub unsafe extern "C" fn free_cfg(mut cfg: *mut config_item) {
    let mut i: *mut config_item = 0 as *mut config_item;
    i = cfg;
    while (*i).type_0 != C_END as libc::c_int {
        if (*i).type_0 == C_STRING as libc::c_int {
            sfree((*i).u.string.sval as *mut libc::c_void);
        }
        i = i.offset(1);
        i;
    }
    sfree(cfg as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn free_keys(mut keys: *mut key_label, mut nkeys: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nkeys {
        sfree((*keys.offset(i as isize)).label as *mut libc::c_void);
        i += 1;
        i;
    }
    sfree(keys as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn obfuscate_bitmap(
    mut bmp: *mut libc::c_uchar,
    mut bits: libc::c_int,
    mut decode: bool,
) {
    let mut bytes: libc::c_int = 0;
    let mut firsthalf: libc::c_int = 0;
    let mut secondhalf: libc::c_int = 0;
    let mut steps: [step; 2] = [step {
        seedstart: 0 as *mut libc::c_uchar,
        seedlen: 0,
        targetstart: 0 as *mut libc::c_uchar,
        targetlen: 0,
    }; 2];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    bytes = (bits + 7 as libc::c_int) / 8 as libc::c_int;
    firsthalf = bytes / 2 as libc::c_int;
    secondhalf = bytes - firsthalf;
    steps[(if decode as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
            as usize]
        .seedstart = bmp.offset(firsthalf as isize);
    steps[(if decode as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
            as usize]
        .seedlen = secondhalf;
    steps[(if decode as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
            as usize]
        .targetstart = bmp;
    steps[(if decode as libc::c_int != 0 { 1 as libc::c_int } else { 0 as libc::c_int })
            as usize]
        .targetlen = firsthalf;
    steps[(if decode as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int })
            as usize]
        .seedstart = bmp;
    steps[(if decode as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int })
            as usize]
        .seedlen = firsthalf;
    steps[(if decode as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int })
            as usize]
        .targetstart = bmp.offset(firsthalf as isize);
    steps[(if decode as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int })
            as usize]
        .targetlen = secondhalf;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut base: SHA_State = SHA_State {
            h: [0; 5],
            block: [0; 64],
            blkused: 0,
            lenhi: 0,
            lenlo: 0,
        };
        let mut final_0: SHA_State = SHA_State {
            h: [0; 5],
            block: [0; 64],
            blkused: 0,
            lenhi: 0,
            lenlo: 0,
        };
        let mut digest: [libc::c_uchar; 20] = [0; 20];
        let mut numberbuf: [libc::c_char; 80] = [0; 80];
        let mut digestpos: libc::c_int = 20 as libc::c_int;
        let mut counter: libc::c_int = 0 as libc::c_int;
        SHA_Init(&mut base);
        SHA_Bytes(
            &mut base,
            steps[i as usize].seedstart as *const libc::c_void,
            steps[i as usize].seedlen,
        );
        j = 0 as libc::c_int;
        while j < steps[i as usize].targetlen {
            if digestpos >= 20 as libc::c_int {
                let fresh0 = counter;
                counter = counter + 1;
                sprintf(
                    numberbuf.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    fresh0,
                );
                final_0 = base;
                SHA_Bytes(
                    &mut final_0,
                    numberbuf.as_mut_ptr() as *const libc::c_void,
                    strlen(numberbuf.as_mut_ptr()) as libc::c_int,
                );
                SHA_Final(&mut final_0, digest.as_mut_ptr());
                digestpos = 0 as libc::c_int;
            }
            let fresh1 = digestpos;
            digestpos = digestpos + 1;
            let ref mut fresh2 = *(steps[i as usize].targetstart).offset(j as isize);
            *fresh2 = (*fresh2 as libc::c_int ^ digest[fresh1 as usize] as libc::c_int)
                as libc::c_uchar;
            j += 1;
            j;
        }
        if bits % 8 as libc::c_int != 0 {
            let ref mut fresh3 = *bmp.offset((bits / 8 as libc::c_int) as isize);
            *fresh3 = (*fresh3 as libc::c_int
                & (0xff as libc::c_int
                    & 0xff00 as libc::c_int >> bits % 8 as libc::c_int))
                as libc::c_uchar;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bin2hex(
    mut in_0: *const libc::c_uchar,
    mut inlen: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = smalloc(
        ((inlen * 2 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = ret;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < inlen * 2 as libc::c_int {
        let mut v: libc::c_int = *in_0.offset((i / 2 as libc::c_int) as isize)
            as libc::c_int;
        if i % 2 as libc::c_int == 0 as libc::c_int {
            v >>= 4 as libc::c_int;
        }
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"0123456789abcdef\0"))[(v & 0xf as libc::c_int) as usize];
        i += 1;
        i;
    }
    *p = '\0' as i32 as libc::c_char;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn hex2bin(
    mut in_0: *const libc::c_char,
    mut outlen: libc::c_int,
) -> *mut libc::c_uchar {
    let mut ret: *mut libc::c_uchar = smalloc(
        (outlen as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        (outlen as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < outlen * 2 as libc::c_int {
        let mut c: libc::c_int = *in_0.offset(i as isize) as libc::c_int;
        let mut v: libc::c_int = 0;
        if c != 0 as libc::c_int {} else {
            __assert_fail(
                b"c != 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"unsigned char *hex2bin(const char *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_8228: {
            if c != 0 as libc::c_int {} else {
                __assert_fail(
                    b"c != 0\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                    174 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 42],
                        &[libc::c_char; 42],
                    >(b"unsigned char *hex2bin(const char *, int)\0"))
                        .as_ptr(),
                );
            }
        };
        if c >= '0' as i32 && c <= '9' as i32 {
            v = c - '0' as i32;
        } else if c >= 'a' as i32 && c <= 'f' as i32 {
            v = c - 'a' as i32 + 10 as libc::c_int;
        } else if c >= 'A' as i32 && c <= 'F' as i32 {
            v = c - 'A' as i32 + 10 as libc::c_int;
        } else {
            v = 0 as libc::c_int;
        }
        let ref mut fresh5 = *ret.offset((i / 2 as libc::c_int) as isize);
        *fresh5 = (*fresh5 as libc::c_int
            | v << 4 as libc::c_int * (1 as libc::c_int - i % 2 as libc::c_int))
            as libc::c_uchar;
        i += 1;
        i;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fgetline(mut fp: *mut FILE) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = smalloc(
        (512 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut size: libc::c_int = 512 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    while !(fgets(ret.offset(len as isize), size - len, fp)).is_null() {
        len = (len as libc::c_ulong).wrapping_add(strlen(ret.offset(len as isize)))
            as libc::c_int as libc::c_int;
        if *ret.offset((len - 1 as libc::c_int) as isize) as libc::c_int == '\n' as i32 {
            break;
        }
        size = len + 512 as libc::c_int;
        ret = srealloc(
            ret as *mut libc::c_void,
            (size as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    if len == 0 as libc::c_int {
        sfree(ret as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    *ret.offset(len as isize) = '\0' as i32 as libc::c_char;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn getenv_bool(
    mut name: *const libc::c_char,
    mut dflt: libc::c_int,
) -> libc::c_int {
    let mut env: *mut libc::c_char = getenv(name);
    if env.is_null() {
        return dflt;
    }
    if !(strchr(
        b"yYtT\0" as *const u8 as *const libc::c_char,
        *env.offset(0 as libc::c_int as isize) as libc::c_int,
    ))
        .is_null()
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn colour_distance(
    mut a: *const libc::c_float,
    mut b: *const libc::c_float,
) -> libc::c_float {
    return __tg_sqrt(
        (*a.offset(0 as libc::c_int as isize) - *b.offset(0 as libc::c_int as isize))
            * (*a.offset(0 as libc::c_int as isize)
                - *b.offset(0 as libc::c_int as isize))
            + (*a.offset(1 as libc::c_int as isize)
                - *b.offset(1 as libc::c_int as isize))
                * (*a.offset(1 as libc::c_int as isize)
                    - *b.offset(1 as libc::c_int as isize))
            + (*a.offset(2 as libc::c_int as isize)
                - *b.offset(2 as libc::c_int as isize))
                * (*a.offset(2 as libc::c_int as isize)
                    - *b.offset(2 as libc::c_int as isize)),
    );
}
#[no_mangle]
pub unsafe extern "C" fn colour_mix(
    mut src1: *const libc::c_float,
    mut src2: *const libc::c_float,
    mut p: libc::c_float,
    mut dst: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        *dst
            .offset(
                i as isize,
            ) = *src1.offset(i as isize) * (1.0f32 - p) + *src2.offset(i as isize) * p;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn game_mkhighlight_specific(
    mut fe: *mut frontend,
    mut ret: *mut libc::c_float,
    mut background: libc::c_int,
    mut highlight: libc::c_int,
    mut lowlight: libc::c_int,
) {
    static mut black: [libc::c_float; 3] = [0.0f32, 0.0f32, 0.0f32];
    static mut white: [libc::c_float; 3] = [1.0f32, 1.0f32, 1.0f32];
    let mut db: libc::c_float = 0.;
    let mut dw: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let k: libc::c_float = (__tg_sqrt_0(3 as libc::c_int as libc::c_double)
        / 6.0f32 as libc::c_double) as libc::c_float;
    if lowlight >= 0 as libc::c_int {
        db = colour_distance(
            &mut *ret.offset((background * 3 as libc::c_int) as isize)
                as *mut libc::c_float as *const libc::c_float,
            black.as_ptr(),
        );
        if db < k {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                *ret
                    .offset(
                        (lowlight * 3 as libc::c_int + i) as isize,
                    ) = black[i as usize];
                i += 1;
                i;
            }
            if db == 0.0f32 {
                colour_mix(
                    black.as_ptr(),
                    white.as_ptr(),
                    (k as libc::c_double
                        / __tg_sqrt_0(3 as libc::c_int as libc::c_double))
                        as libc::c_float,
                    &mut *ret.offset((background * 3 as libc::c_int) as isize),
                );
            } else {
                colour_mix(
                    black.as_ptr(),
                    &mut *ret.offset((background * 3 as libc::c_int) as isize)
                        as *mut libc::c_float as *const libc::c_float,
                    k / db,
                    &mut *ret.offset((background * 3 as libc::c_int) as isize),
                );
            }
        } else {
            colour_mix(
                &mut *ret.offset((background * 3 as libc::c_int) as isize)
                    as *mut libc::c_float as *const libc::c_float,
                black.as_ptr(),
                k / db,
                &mut *ret.offset((lowlight * 3 as libc::c_int) as isize),
            );
        }
    }
    if highlight >= 0 as libc::c_int {
        dw = colour_distance(
            &mut *ret.offset((background * 3 as libc::c_int) as isize)
                as *mut libc::c_float as *const libc::c_float,
            white.as_ptr(),
        );
        if dw < k {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                *ret
                    .offset(
                        (highlight * 3 as libc::c_int + i) as isize,
                    ) = white[i as usize];
                i += 1;
                i;
            }
            if dw == 0.0f32 {
                colour_mix(
                    white.as_ptr(),
                    black.as_ptr(),
                    (k as libc::c_double
                        / __tg_sqrt_0(3 as libc::c_int as libc::c_double))
                        as libc::c_float,
                    &mut *ret.offset((background * 3 as libc::c_int) as isize),
                );
            } else {
                colour_mix(
                    white.as_ptr(),
                    &mut *ret.offset((background * 3 as libc::c_int) as isize)
                        as *mut libc::c_float as *const libc::c_float,
                    k / dw,
                    &mut *ret.offset((background * 3 as libc::c_int) as isize),
                );
            }
            if lowlight >= 0 as libc::c_int {
                colour_mix(
                    &mut *ret.offset((background * 3 as libc::c_int) as isize)
                        as *mut libc::c_float as *const libc::c_float,
                    black.as_ptr(),
                    k / db,
                    &mut *ret.offset((lowlight * 3 as libc::c_int) as isize),
                );
            }
        } else {
            colour_mix(
                &mut *ret.offset((background * 3 as libc::c_int) as isize)
                    as *mut libc::c_float as *const libc::c_float,
                white.as_ptr(),
                k / dw,
                &mut *ret.offset((highlight * 3 as libc::c_int) as isize),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn game_mkhighlight(
    mut fe: *mut frontend,
    mut ret: *mut libc::c_float,
    mut background: libc::c_int,
    mut highlight: libc::c_int,
    mut lowlight: libc::c_int,
) {
    frontend_default_colour(
        fe,
        &mut *ret.offset((background * 3 as libc::c_int) as isize),
    );
    game_mkhighlight_specific(fe, ret, background, highlight, lowlight);
}
unsafe extern "C" fn memswap(
    mut av: *mut libc::c_void,
    mut bv: *mut libc::c_void,
    mut size: libc::c_int,
) {
    let mut tmpbuf: [libc::c_char; 512] = [0; 512];
    let mut a: *mut libc::c_char = av as *mut libc::c_char;
    let mut b: *mut libc::c_char = bv as *mut libc::c_char;
    while size > 0 as libc::c_int {
        let mut thislen: libc::c_int = (if (size as libc::c_ulong)
            < ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
        {
            size as libc::c_ulong
        } else {
            ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong
        }) as libc::c_int;
        memcpy(
            tmpbuf.as_mut_ptr() as *mut libc::c_void,
            a as *const libc::c_void,
            thislen as libc::c_ulong,
        );
        memcpy(
            a as *mut libc::c_void,
            b as *const libc::c_void,
            thislen as libc::c_ulong,
        );
        memcpy(
            b as *mut libc::c_void,
            tmpbuf.as_mut_ptr() as *const libc::c_void,
            thislen as libc::c_ulong,
        );
        a = a.offset(thislen as isize);
        b = b.offset(thislen as isize);
        size -= thislen;
    }
}
#[no_mangle]
pub unsafe extern "C" fn shuffle(
    mut array: *mut libc::c_void,
    mut nelts: libc::c_int,
    mut eltsize: libc::c_int,
    mut rs: *mut random_state,
) {
    let mut carray: *mut libc::c_char = array as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    i = nelts;
    loop {
        let fresh6 = i;
        i = i - 1;
        if !(fresh6 > 1 as libc::c_int) {
            break;
        }
        let mut j: libc::c_int = random_upto(rs, (i + 1 as libc::c_int) as libc::c_ulong)
            as libc::c_int;
        if j != i {
            memswap(
                carray.offset((eltsize * i) as isize) as *mut libc::c_void,
                carray.offset((eltsize * j) as isize) as *mut libc::c_void,
                eltsize,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn draw_rect_outline(
    mut dr: *mut drawing,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut colour: libc::c_int,
) {
    let mut x0: libc::c_int = x;
    let mut x1: libc::c_int = x + w - 1 as libc::c_int;
    let mut y0: libc::c_int = y;
    let mut y1: libc::c_int = y + h - 1 as libc::c_int;
    let mut coords: [libc::c_int; 8] = [0; 8];
    coords[0 as libc::c_int as usize] = x0;
    coords[1 as libc::c_int as usize] = y0;
    coords[2 as libc::c_int as usize] = x0;
    coords[3 as libc::c_int as usize] = y1;
    coords[4 as libc::c_int as usize] = x1;
    coords[5 as libc::c_int as usize] = y1;
    coords[6 as libc::c_int as usize] = x1;
    coords[7 as libc::c_int as usize] = y0;
    draw_polygon(dr, coords.as_mut_ptr(), 4 as libc::c_int, -(1 as libc::c_int), colour);
}
#[no_mangle]
pub unsafe extern "C" fn draw_rect_corners(
    mut dr: *mut drawing,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut r: libc::c_int,
    mut col: libc::c_int,
) {
    draw_line(dr, cx - r, cy - r, cx - r, cy - r / 2 as libc::c_int, col);
    draw_line(dr, cx - r, cy - r, cx - r / 2 as libc::c_int, cy - r, col);
    draw_line(dr, cx - r, cy + r, cx - r, cy + r / 2 as libc::c_int, col);
    draw_line(dr, cx - r, cy + r, cx - r / 2 as libc::c_int, cy + r, col);
    draw_line(dr, cx + r, cy - r, cx + r, cy - r / 2 as libc::c_int, col);
    draw_line(dr, cx + r, cy - r, cx + r / 2 as libc::c_int, cy - r, col);
    draw_line(dr, cx + r, cy + r, cx + r, cy + r / 2 as libc::c_int, col);
    draw_line(dr, cx + r, cy + r, cx + r / 2 as libc::c_int, cy + r, col);
}
#[no_mangle]
pub unsafe extern "C" fn move_cursor(
    mut button: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut maxw: libc::c_int,
    mut maxh: libc::c_int,
    mut wrap: bool,
) {
    let mut dx: libc::c_int = 0 as libc::c_int;
    let mut dy: libc::c_int = 0 as libc::c_int;
    match button {
        521 => {
            dy = -(1 as libc::c_int);
        }
        522 => {
            dy = 1 as libc::c_int;
        }
        524 => {
            dx = 1 as libc::c_int;
        }
        523 => {
            dx = -(1 as libc::c_int);
        }
        _ => return,
    }
    if wrap {
        *x = (*x + dx + maxw) % maxw;
        *y = (*y + dy + maxh) % maxh;
    } else {
        *x = if (if *x + dx > 0 as libc::c_int { *x + dx } else { 0 as libc::c_int })
            < maxw - 1 as libc::c_int
        {
            if *x + dx > 0 as libc::c_int { *x + dx } else { 0 as libc::c_int }
        } else {
            maxw - 1 as libc::c_int
        };
        *y = if (if *y + dy > 0 as libc::c_int { *y + dy } else { 0 as libc::c_int })
            < maxh - 1 as libc::c_int
        {
            if *y + dy > 0 as libc::c_int { *y + dy } else { 0 as libc::c_int }
        } else {
            maxh - 1 as libc::c_int
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn c2pos(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
) -> libc::c_int {
    if cy == -(1 as libc::c_int) {
        return cx
    } else if cx == w {
        return w + cy
    } else if cy == h {
        return w + h + (w - cx - 1 as libc::c_int)
    } else if cx == -(1 as libc::c_int) {
        return w + h + w + (h - cy - 1 as libc::c_int)
    }
    if (b"invalid cursor pos!\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"invalid cursor pos!\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
            386 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"int c2pos(int, int, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_9664: {
        if (b"invalid cursor pos!\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"!\"invalid cursor pos!\"\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                386 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"int c2pos(int, int, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn c2diff(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut button: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int = 0 as libc::c_int;
    if button == CURSOR_UP as libc::c_int || button == CURSOR_DOWN as libc::c_int
        || button == CURSOR_RIGHT as libc::c_int || button == CURSOR_LEFT as libc::c_int
    {} else {
        __assert_fail(
            b"IS_CURSOR_MOVE(button)\0" as *const u8 as *const libc::c_char,
            b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int c2diff(int, int, int, int, int)\0"))
                .as_ptr(),
        );
    }
    'c_10042: {
        if button == CURSOR_UP as libc::c_int || button == CURSOR_DOWN as libc::c_int
            || button == CURSOR_RIGHT as libc::c_int
            || button == CURSOR_LEFT as libc::c_int
        {} else {
            __assert_fail(
                b"IS_CURSOR_MOVE(button)\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                394 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"int c2diff(int, int, int, int, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if cy == -(1 as libc::c_int) {
        diff = if button == CURSOR_RIGHT as libc::c_int {
            1 as libc::c_int
        } else if button == CURSOR_LEFT as libc::c_int {
            -(1 as libc::c_int)
        } else {
            diff
        };
    }
    if cy == h {
        diff = if button == CURSOR_RIGHT as libc::c_int {
            -(1 as libc::c_int)
        } else if button == CURSOR_LEFT as libc::c_int {
            1 as libc::c_int
        } else {
            diff
        };
    }
    if cx == -(1 as libc::c_int) {
        diff = if button == CURSOR_UP as libc::c_int {
            1 as libc::c_int
        } else if button == CURSOR_DOWN as libc::c_int {
            -(1 as libc::c_int)
        } else {
            diff
        };
    }
    if cx == w {
        diff = if button == CURSOR_UP as libc::c_int {
            -(1 as libc::c_int)
        } else if button == CURSOR_DOWN as libc::c_int {
            1 as libc::c_int
        } else {
            diff
        };
    }
    if button == CURSOR_LEFT as libc::c_int && cx == w
        && (cy == 0 as libc::c_int || cy == h - 1 as libc::c_int)
    {
        diff = if cy == 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
    }
    if button == CURSOR_RIGHT as libc::c_int && cx == -(1 as libc::c_int)
        && (cy == 0 as libc::c_int || cy == h - 1 as libc::c_int)
    {
        diff = if cy == 0 as libc::c_int {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    if button == CURSOR_DOWN as libc::c_int && cy == -(1 as libc::c_int)
        && (cx == 0 as libc::c_int || cx == w - 1 as libc::c_int)
    {
        diff = if cx == 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
    }
    if button == CURSOR_UP as libc::c_int && cy == h
        && (cx == 0 as libc::c_int || cx == w - 1 as libc::c_int)
    {
        diff = if cx == 0 as libc::c_int {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn pos2c(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pos: libc::c_int,
    mut cx: *mut libc::c_int,
    mut cy: *mut libc::c_int,
) {
    let mut max: libc::c_int = w + h + w + h;
    pos = (pos + max) % max;
    if pos < w {
        *cx = pos;
        *cy = -(1 as libc::c_int);
        return;
    }
    pos -= w;
    if pos < h {
        *cx = w;
        *cy = pos;
        return;
    }
    pos -= h;
    if pos < w {
        *cx = w - pos - 1 as libc::c_int;
        *cy = h;
        return;
    }
    pos -= w;
    if pos < h {
        *cx = -(1 as libc::c_int);
        *cy = h - pos - 1 as libc::c_int;
        return;
    }
    if (b"invalid pos, huh?\0" as *const u8 as *const libc::c_char).is_null() {} else {
        __assert_fail(
            b"!\"invalid pos, huh?\"\0" as *const u8 as *const libc::c_char,
            b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void pos2c(int, int, int, int *, int *)\0"))
                .as_ptr(),
        );
    }
    'c_10137: {
        if (b"invalid pos, huh?\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"!\"invalid pos, huh?\"\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                440 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void pos2c(int, int, int, int *, int *)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn draw_text_outline(
    mut dr: *mut drawing,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut fonttype: libc::c_int,
    mut fontsize: libc::c_int,
    mut align: libc::c_int,
    mut text_colour: libc::c_int,
    mut outline_colour: libc::c_int,
    mut text: *const libc::c_char,
) {
    if outline_colour > -(1 as libc::c_int) {
        draw_text(
            dr,
            x - 1 as libc::c_int,
            y,
            fonttype,
            fontsize,
            align,
            outline_colour,
            text,
        );
        draw_text(
            dr,
            x + 1 as libc::c_int,
            y,
            fonttype,
            fontsize,
            align,
            outline_colour,
            text,
        );
        draw_text(
            dr,
            x,
            y - 1 as libc::c_int,
            fonttype,
            fontsize,
            align,
            outline_colour,
            text,
        );
        draw_text(
            dr,
            x,
            y + 1 as libc::c_int,
            fonttype,
            fontsize,
            align,
            outline_colour,
            text,
        );
    }
    draw_text(dr, x, y, fonttype, fontsize, align, text_colour, text);
}
#[no_mangle]
pub unsafe extern "C" fn copy_left_justified(
    mut buf: *mut libc::c_char,
    mut sz: size_t,
    mut str: *const libc::c_char,
) {
    let mut len: size_t = strlen(str);
    if sz > 0 as libc::c_int as size_t {} else {
        __assert_fail(
            b"sz > 0\0" as *const u8 as *const libc::c_char,
            b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void copy_left_justified(char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_10529: {
        if sz > 0 as libc::c_int as size_t {} else {
            __assert_fail(
                b"sz > 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                461 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void copy_left_justified(char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        buf as *mut libc::c_void,
        ' ' as i32,
        sz.wrapping_sub(1 as libc::c_int as size_t),
    );
    if len <= sz.wrapping_sub(1 as libc::c_int as size_t) {} else {
        __assert_fail(
            b"len <= sz - 1\0" as *const u8 as *const libc::c_char,
            b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
            463 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void copy_left_justified(char *, size_t, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_10471: {
        if len <= sz.wrapping_sub(1 as libc::c_int as size_t) {} else {
            __assert_fail(
                b"len <= sz - 1\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                463 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[libc::c_char; 55],
                >(b"void copy_left_justified(char *, size_t, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    memcpy(buf as *mut libc::c_void, str as *const libc::c_void, len);
    *buf
        .offset(
            sz.wrapping_sub(1 as libc::c_int as size_t) as isize,
        ) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn button2label(mut button: libc::c_int) -> *mut libc::c_char {
    if 'A' as i32 <= button && button <= 'Z' as i32
        || 'a' as i32 <= button && button <= 'z' as i32
        || '0' as i32 <= button && button <= '9' as i32
    {
        let mut str: [libc::c_char; 2] = [0; 2];
        str[0 as libc::c_int as usize] = button as libc::c_char;
        str[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        return dupstr(str.as_mut_ptr());
    }
    match button {
        521 => return dupstr(b"Up\0" as *const u8 as *const libc::c_char),
        522 => return dupstr(b"Down\0" as *const u8 as *const libc::c_char),
        523 => return dupstr(b"Left\0" as *const u8 as *const libc::c_char),
        524 => return dupstr(b"Right\0" as *const u8 as *const libc::c_char),
        525 => return dupstr(b"Select\0" as *const u8 as *const libc::c_char),
        8 => return dupstr(b"Clear\0" as *const u8 as *const libc::c_char),
        _ => {
            fatal(b"unknown generic key\0" as *const u8 as *const libc::c_char);
        }
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn make_prefs_path(
    mut dir: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut game: *const game,
    mut suffix: *const libc::c_char,
) -> *mut libc::c_char {
    let mut dirlen: size_t = strlen(dir);
    let mut seplen: size_t = strlen(sep);
    let mut gamelen: size_t = strlen((*game).name);
    let mut suffixlen: size_t = strlen(suffix);
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    if dir.is_null() || sep.is_null() || game.is_null() || suffix.is_null() {
        return 0 as *mut libc::c_char;
    }
    path = smalloc(
        dirlen
            .wrapping_add(seplen)
            .wrapping_add(gamelen)
            .wrapping_add(suffixlen)
            .wrapping_add(1 as libc::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    p = path;
    memcpy(p as *mut libc::c_void, dir as *const libc::c_void, dirlen);
    p = p.offset(dirlen as isize);
    memcpy(p as *mut libc::c_void, sep as *const libc::c_void, seplen);
    p = p.offset(seplen as isize);
    q = (*game).name;
    while *q != 0 {
        if *q as libc::c_int != ' ' as i32 {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = tolower(*q as libc::c_uchar as libc::c_int) as libc::c_char;
        }
        q = q.offset(1);
        q;
    }
    memcpy(p as *mut libc::c_void, suffix as *const libc::c_void, suffixlen);
    p = p.offset(suffixlen as isize);
    *p = '\0' as i32 as libc::c_char;
    return path;
}
#[no_mangle]
pub unsafe extern "C" fn n_times_root_k(
    mut n_signed: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_uint = 0;
    let mut r: libc::c_uint = 0;
    let mut m: libc::c_uint = 0;
    let mut sign: libc::c_int = if n_signed < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    let mut n: libc::c_uint = (n_signed * sign) as libc::c_uint;
    let mut bitpos: libc::c_uint = 0;
    m = 0 as libc::c_int as libc::c_uint;
    r = m;
    x = r;
    bitpos = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint)
        & !((2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint) >> 1 as libc::c_int);
    while bitpos != 0 {
        let mut a: libc::c_uint = 0;
        let mut b: libc::c_uint = (if n & bitpos != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        if x.wrapping_mul(x).wrapping_add(r)
            == (k as libc::c_uint).wrapping_mul(m).wrapping_mul(m)
        {} else {
            __assert_fail(
                b"x*x + r == k*m*m\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                596 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"int n_times_root_k(int, int)\0"))
                    .as_ptr(),
            );
        }
        'c_7919: {
            if x.wrapping_mul(x).wrapping_add(r)
                == (k as libc::c_uint).wrapping_mul(m).wrapping_mul(m)
            {} else {
                __assert_fail(
                    b"x*x + r == k*m*m\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                    596 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"int n_times_root_k(int, int)\0"))
                        .as_ptr(),
                );
            }
        };
        if r
            < (2 as libc::c_int as libc::c_uint)
                .wrapping_mul(x)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"r < 2*x+1\0" as *const u8 as *const libc::c_char,
                b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                597 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"int n_times_root_k(int, int)\0"))
                    .as_ptr(),
            );
        }
        'c_7864: {
            if r
                < (2 as libc::c_int as libc::c_uint)
                    .wrapping_mul(x)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
            {} else {
                __assert_fail(
                    b"r < 2*x+1\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/misc.c\0" as *const u8 as *const libc::c_char,
                    597 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"int n_times_root_k(int, int)\0"))
                        .as_ptr(),
                );
            }
        };
        a = 0 as libc::c_int as libc::c_uint;
        loop {
            let mut pos: libc::c_uint = (4 as libc::c_int as libc::c_uint)
                .wrapping_mul(r)
                .wrapping_add(
                    (k as libc::c_uint)
                        .wrapping_mul(b)
                        .wrapping_mul(
                            (4 as libc::c_int as libc::c_uint)
                                .wrapping_mul(m)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        ),
                );
            let mut neg: libc::c_uint = (4 as libc::c_int as libc::c_uint)
                .wrapping_mul(a)
                .wrapping_mul(x)
                .wrapping_add(a.wrapping_mul(a));
            if pos < neg {
                break;
            }
            a = a.wrapping_add(1);
            a;
        }
        a = a.wrapping_sub(1);
        a;
        r = (4 as libc::c_int as libc::c_uint)
            .wrapping_mul(r)
            .wrapping_add(
                b
                    .wrapping_mul(k as libc::c_uint)
                    .wrapping_mul(
                        (4 as libc::c_int as libc::c_uint)
                            .wrapping_mul(m)
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    ),
            )
            .wrapping_sub(
                (4 as libc::c_int as libc::c_uint)
                    .wrapping_mul(a)
                    .wrapping_mul(x)
                    .wrapping_add(a.wrapping_mul(a)),
            );
        m = (2 as libc::c_int as libc::c_uint).wrapping_mul(m).wrapping_add(b);
        x = (2 as libc::c_int as libc::c_uint).wrapping_mul(x).wrapping_add(a);
        bitpos >>= 1 as libc::c_int;
    }
    if r > x {
        x = x.wrapping_add(1);
        x;
    }
    if sign == 1 as libc::c_int {
        return x as libc::c_int
    } else {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int
            + x
                .wrapping_neg()
                .wrapping_sub(
                    (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_uint,
                ) as libc::c_int
    };
}
