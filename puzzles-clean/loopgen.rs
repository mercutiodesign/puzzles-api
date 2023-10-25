use ::libc;
extern "C" {
    pub type random_state;
    pub type tree234_Tag;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn shuffle(
        array: *mut libc::c_void,
        nelts: libc::c_int,
        eltsize: libc::c_int,
        rs: *mut random_state,
    );
    fn random_bits(state: *mut random_state, bits: libc::c_int) -> libc::c_ulong;
    fn random_upto(state: *mut random_state, limit: libc::c_ulong) -> libc::c_ulong;
    fn newtree234(cmp: cmpfn234) -> *mut tree234;
    fn freetree234(t: *mut tree234);
    fn add234(t: *mut tree234, e: *mut libc::c_void) -> *mut libc::c_void;
    fn index234(t: *mut tree234, index: libc::c_int) -> *mut libc::c_void;
    fn del234(t: *mut tree234, e: *mut libc::c_void) -> *mut libc::c_void;
    fn count234(t: *mut tree234) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type tree234 = tree234_Tag;
pub type cmpfn234 =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>;
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
pub type face_colour = libc::c_uint;
pub const FACE_BLACK: face_colour = 2;
pub const FACE_GREY: face_colour = 1;
pub const FACE_WHITE: face_colour = 0;
pub type loopgen_bias_fn_t =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct face_score {
    pub white_score: libc::c_int,
    pub black_score: libc::c_int,
    pub random: libc::c_ulong,
}
unsafe extern "C" fn generic_sort_cmpfn(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
    mut offset: size_t,
) -> libc::c_int {
    let mut f1: *mut face_score = v1 as *mut face_score;
    let mut f2: *mut face_score = v2 as *mut face_score;
    let mut r: libc::c_int = 0;
    r = *((f2 as *mut libc::c_char).offset(offset as isize) as *mut libc::c_int)
        - *((f1 as *mut libc::c_char).offset(offset as isize) as *mut libc::c_int);
    if r != 0 {
        return r;
    }
    if (*f1).random < (*f2).random {
        return -(1 as libc::c_int);
    } else if (*f1).random > (*f2).random {
        return 1 as libc::c_int;
    }
    return f1.offset_from(f2) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn white_sort_cmpfn(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
) -> libc::c_int {
    return generic_sort_cmpfn(v1, v2, 0 as libc::c_ulong);
}
unsafe extern "C" fn black_sort_cmpfn(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
) -> libc::c_int {
    return generic_sort_cmpfn(v1, v2, 4 as libc::c_ulong);
}
unsafe extern "C" fn can_colour_face(
    mut g: *mut grid,
    mut board: *mut libc::c_char,
    mut face_index: libc::c_int,
    mut colour: face_colour,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut test_face: *mut grid_face = *((*g).faces).offset(face_index as isize);
    let mut starting_face: *mut grid_face = 0 as *mut grid_face;
    let mut current_face: *mut grid_face = 0 as *mut grid_face;
    let mut starting_dot: *mut grid_dot = 0 as *mut grid_dot;
    let mut transitions: libc::c_int = 0;
    let mut current_state: bool = false;
    let mut s: bool = false;
    let mut found_same_coloured_neighbour: bool = 0 as libc::c_int != 0;
    if *board.offset(face_index as isize) as libc::c_uint != colour as libc::c_uint {
    } else {
        __assert_fail(
            b"board[face_index] != colour\0" as *const u8 as *const libc::c_char,
            b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                b"_Bool can_colour_face(grid *, char *, int, enum face_colour)\0",
            ))
            .as_ptr(),
        );
    }
    'c_8325: {
        if *board.offset(face_index as isize) as libc::c_uint != colour as libc::c_uint {
        } else {
            __assert_fail(
                b"board[face_index] != colour\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                    b"_Bool can_colour_face(grid *, char *, int, enum face_colour)\0",
                ))
                .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < (*test_face).order {
        let mut e: *mut grid_edge = *((*test_face).edges).offset(i as isize);
        let mut f: *mut grid_face = if (*e).face1 == test_face {
            (*e).face2
        } else {
            (*e).face1
        };
        if (if f.is_null() {
            FACE_BLACK as libc::c_int
        } else {
            *board.offset((*f).index as isize) as libc::c_int
        }) as libc::c_uint
            == colour as libc::c_uint
        {
            found_same_coloured_neighbour = 1 as libc::c_int != 0;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if !found_same_coloured_neighbour {
        return 0 as libc::c_int != 0;
    }
    j = 0 as libc::c_int;
    i = j;
    current_face = *((**((*test_face).dots).offset(0 as libc::c_int as isize)).faces)
        .offset(0 as libc::c_int as isize);
    if current_face == test_face {
        j = 1 as libc::c_int;
        current_face = *((**((*test_face).dots).offset(0 as libc::c_int as isize)).faces)
            .offset(1 as libc::c_int as isize);
    }
    transitions = 0 as libc::c_int;
    current_state = (if current_face.is_null() {
        FACE_BLACK as libc::c_int
    } else {
        *board.offset((*current_face).index as isize) as libc::c_int
    }) as libc::c_uint
        == colour as libc::c_uint;
    starting_dot = 0 as *mut grid_dot;
    starting_face = 0 as *mut grid_face;
    loop {
        loop {
            j += 1;
            j;
            if j == (**((*test_face).dots).offset(i as isize)).order {
                j = 0 as libc::c_int;
            }
            if !(*((**((*test_face).dots).offset(i as isize)).faces).offset(j as isize)
                == test_face)
            {
                break;
            }
            i += 1;
            i;
            if i == (*test_face).order {
                i = 0 as libc::c_int;
            }
            j = 0 as libc::c_int;
            while j < (**((*test_face).dots).offset(i as isize)).order {
                if *((**((*test_face).dots).offset(i as isize)).faces).offset(j as isize)
                    == current_face
                {
                    break;
                }
                j += 1;
                j;
            }
            if j != (**((*test_face).dots).offset(i as isize)).order {
            } else {
                __assert_fail(
                    b"j != test_face->dots[i]->order\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                    187 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                        b"_Bool can_colour_face(grid *, char *, int, enum face_colour)\0",
                    ))
                    .as_ptr(),
                );
            }
            'c_8004: {
                if j != (**((*test_face).dots).offset(i as isize)).order {
                } else {
                    __assert_fail(
                        b"j != test_face->dots[i]->order\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                        187 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                            b"_Bool can_colour_face(grid *, char *, int, enum face_colour)\0",
                        ))
                        .as_ptr(),
                    );
                }
            };
        }
        current_face = *((**((*test_face).dots).offset(i as isize)).faces).offset(j as isize);
        s = (if current_face.is_null() {
            FACE_BLACK as libc::c_int
        } else {
            *board.offset((*current_face).index as isize) as libc::c_int
        }) as libc::c_uint
            == colour as libc::c_uint;
        if starting_dot.is_null() {
            starting_dot = *((*test_face).dots).offset(i as isize);
            starting_face = current_face;
            current_state = s;
        } else {
            if s as libc::c_int != current_state as libc::c_int {
                transitions += 1;
                transitions;
                current_state = s;
                if transitions > 2 as libc::c_int {
                    break;
                }
            }
            if *((*test_face).dots).offset(i as isize) == starting_dot
                && current_face == starting_face
            {
                break;
            }
        }
    }
    return if transitions == 2 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
unsafe extern "C" fn face_num_neighbours(
    mut g: *mut grid,
    mut board: *mut libc::c_char,
    mut face: *mut grid_face,
    mut colour: face_colour,
) -> libc::c_int {
    let mut colour_count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut f: *mut grid_face = 0 as *mut grid_face;
    let mut e: *mut grid_edge = 0 as *mut grid_edge;
    i = 0 as libc::c_int;
    while i < (*face).order {
        e = *((*face).edges).offset(i as isize);
        f = if (*e).face1 == face {
            (*e).face2
        } else {
            (*e).face1
        };
        if (if f.is_null() {
            FACE_BLACK as libc::c_int
        } else {
            *board.offset((*f).index as isize) as libc::c_int
        }) as libc::c_uint
            == colour as libc::c_uint
        {
            colour_count += 1;
            colour_count;
        }
        i += 1;
        i;
    }
    return colour_count;
}
unsafe extern "C" fn face_score(
    mut g: *mut grid,
    mut board: *mut libc::c_char,
    mut face: *mut grid_face,
    mut colour: face_colour,
) -> libc::c_int {
    return -face_num_neighbours(g, board, face, colour);
}
#[no_mangle]
pub unsafe extern "C" fn generate_loop(
    mut g: *mut grid,
    mut board: *mut libc::c_char,
    mut rs: *mut random_state,
    mut bias: loopgen_bias_fn_t,
    mut biasctx: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num_faces: libc::c_int = (*g).num_faces;
    let mut face_scores: *mut face_score = 0 as *mut face_score;
    let mut fs: *mut face_score = 0 as *mut face_score;
    let mut cur_face: *mut grid_face = 0 as *mut grid_face;
    let mut lightable_faces_sorted: *mut tree234 = 0 as *mut tree234;
    let mut darkable_faces_sorted: *mut tree234 = 0 as *mut tree234;
    let mut face_list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut do_random_pass: bool = false;
    memset(
        board as *mut libc::c_void,
        FACE_GREY as libc::c_int,
        num_faces as libc::c_ulong,
    );
    face_scores = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<face_score>() as libc::c_ulong),
    ) as *mut face_score;
    i = 0 as libc::c_int;
    while i < num_faces {
        (*face_scores.offset(i as isize)).random = random_bits(rs, 31 as libc::c_int);
        let ref mut fresh0 = (*face_scores.offset(i as isize)).white_score;
        *fresh0 = 0 as libc::c_int;
        (*face_scores.offset(i as isize)).black_score = *fresh0;
        i += 1;
        i;
    }
    i = random_upto(rs, num_faces as libc::c_ulong) as libc::c_int;
    *board.offset(i as isize) = FACE_WHITE as libc::c_int as libc::c_char;
    lightable_faces_sorted = newtree234(Some(
        white_sort_cmpfn
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    ));
    darkable_faces_sorted = newtree234(Some(
        black_sort_cmpfn
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    ));
    i = 0 as libc::c_int;
    while i < num_faces {
        let mut f: *mut grid_face = *((*g).faces).offset(i as isize);
        let mut fs_0: *mut face_score = face_scores.offset(i as isize);
        if !(*board.offset(i as isize) as libc::c_int != FACE_GREY as libc::c_int) {
            if can_colour_face(g, board, i, FACE_BLACK) {
                (*fs_0).black_score = face_score(g, board, f, FACE_BLACK);
                add234(darkable_faces_sorted, fs_0 as *mut libc::c_void);
            }
            if can_colour_face(g, board, i, FACE_WHITE) {
                (*fs_0).white_score = face_score(g, board, f, FACE_WHITE);
                add234(lightable_faces_sorted, fs_0 as *mut libc::c_void);
            }
        }
        i += 1;
        i;
    }
    loop {
        let mut colour: face_colour = FACE_WHITE;
        let mut faces_to_pick: *mut tree234 = 0 as *mut tree234;
        let mut c_lightable: libc::c_int = count234(lightable_faces_sorted);
        let mut c_darkable: libc::c_int = count234(darkable_faces_sorted);
        if c_lightable == 0 as libc::c_int && c_darkable == 0 as libc::c_int {
            break;
        }
        if c_lightable != 0 as libc::c_int && c_darkable != 0 as libc::c_int {
        } else {
            __assert_fail(
                b"c_lightable != 0 && c_darkable != 0\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                378 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9090: {
            if c_lightable != 0 as libc::c_int && c_darkable != 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"c_lightable != 0 && c_darkable != 0\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                    378 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        colour = (if random_upto(rs, 2 as libc::c_int as libc::c_ulong) != 0 {
            FACE_WHITE as libc::c_int
        } else {
            FACE_BLACK as libc::c_int
        }) as face_colour;
        if colour as libc::c_uint == FACE_WHITE as libc::c_int as libc::c_uint {
            faces_to_pick = lightable_faces_sorted;
        } else {
            faces_to_pick = darkable_faces_sorted;
        }
        if bias.is_some() {
            let mut j_0: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            let mut best: *mut face_score = 0 as *mut face_score;
            let mut score: libc::c_int = 0;
            let mut bestscore: libc::c_int = 0 as libc::c_int;
            j_0 = 0 as libc::c_int;
            loop {
                fs = index234(faces_to_pick, j_0) as *mut face_score;
                if fs.is_null() {
                    break;
                }
                if !fs.is_null() {
                } else {
                    __assert_fail(
                        b"fs\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                        403 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 78],
                            &[libc::c_char; 78],
                        >(
                            b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_9008: {
                    if !fs.is_null() {
                    } else {
                        __assert_fail(
                            b"fs\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                            403 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 78],
                                &[libc::c_char; 78],
                            >(
                                b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                k = fs.offset_from(face_scores) as libc::c_long as libc::c_int;
                if *board.offset(k as isize) as libc::c_int == FACE_GREY as libc::c_int {
                } else {
                    __assert_fail(
                        b"board[k] == FACE_GREY\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                        405 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 78],
                            &[libc::c_char; 78],
                        >(
                            b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_8954: {
                    if *board.offset(k as isize) as libc::c_int == FACE_GREY as libc::c_int {
                    } else {
                        __assert_fail(
                            b"board[k] == FACE_GREY\0" as *const u8
                                as *const libc::c_char,
                            b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                            405 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 78],
                                &[libc::c_char; 78],
                            >(
                                b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                *board.offset(k as isize) = colour as libc::c_char;
                score = bias.expect("non-null function pointer")(biasctx, board, k);
                *board.offset(k as isize) = FACE_GREY as libc::c_int as libc::c_char;
                bias.expect("non-null function pointer")(biasctx, board, k);
                if best.is_null() || score > bestscore {
                    bestscore = score;
                    best = fs;
                }
                j_0 += 1;
                j_0;
            }
            fs = best;
        } else {
            fs = index234(faces_to_pick, 0 as libc::c_int) as *mut face_score;
        }
        if !fs.is_null() {
        } else {
            __assert_fail(
                b"fs\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                420 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8819: {
            if !fs.is_null() {
            } else {
                __assert_fail(
                    b"fs\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                    420 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        i = fs.offset_from(face_scores) as libc::c_long as libc::c_int;
        if *board.offset(i as isize) as libc::c_int == FACE_GREY as libc::c_int {
        } else {
            __assert_fail(
                b"board[i] == FACE_GREY\0" as *const u8 as *const libc::c_char,
                b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                422 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[libc::c_char; 78],
                >(
                    b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_8763: {
            if *board.offset(i as isize) as libc::c_int == FACE_GREY as libc::c_int {
            } else {
                __assert_fail(
                    b"board[i] == FACE_GREY\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/loopgen.c\0" as *const u8 as *const libc::c_char,
                    422 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 78],
                        &[libc::c_char; 78],
                    >(
                        b"void generate_loop(grid *, char *, random_state *, loopgen_bias_fn_t, void *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *board.offset(i as isize) = colour as libc::c_char;
        if bias.is_some() {
            bias.expect("non-null function pointer")(biasctx, board, i);
        }
        del234(lightable_faces_sorted, fs as *mut libc::c_void);
        del234(darkable_faces_sorted, fs as *mut libc::c_void);
        cur_face = *((*g).faces).offset(i as isize);
        i = 0 as libc::c_int;
        while i < (*cur_face).order {
            let mut d: *mut grid_dot = *((*cur_face).dots).offset(i as isize);
            j = 0 as libc::c_int;
            while j < (*d).order {
                let mut f_0: *mut grid_face = *((*d).faces).offset(j as isize);
                let mut fi: libc::c_int = 0;
                if !f_0.is_null() {
                    if !(f_0 == cur_face) {
                        if !((if f_0.is_null() {
                            FACE_BLACK as libc::c_int
                        } else {
                            *board.offset((*f_0).index as isize) as libc::c_int
                        }) != FACE_GREY as libc::c_int)
                        {
                            fi = (*f_0).index;
                            fs = face_scores.offset(fi as isize);
                            del234(lightable_faces_sorted, fs as *mut libc::c_void);
                            if can_colour_face(g, board, fi, FACE_WHITE) {
                                (*fs).white_score = face_score(g, board, f_0, FACE_WHITE);
                                add234(lightable_faces_sorted, fs as *mut libc::c_void);
                            }
                            del234(darkable_faces_sorted, fs as *mut libc::c_void);
                            if can_colour_face(g, board, fi, FACE_BLACK) {
                                (*fs).black_score = face_score(g, board, f_0, FACE_BLACK);
                                add234(darkable_faces_sorted, fs as *mut libc::c_void);
                            }
                        }
                    }
                }
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    freetree234(lightable_faces_sorted);
    freetree234(darkable_faces_sorted);
    sfree(face_scores as *mut libc::c_void);
    face_list = smalloc(
        (num_faces as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < num_faces {
        *face_list.offset(i as isize) = i;
        i += 1;
        i;
    }
    shuffle(
        face_list as *mut libc::c_void,
        num_faces,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        rs,
    );
    do_random_pass = 0 as libc::c_int != 0;
    loop {
        let mut flipped: bool = 0 as libc::c_int != 0;
        i = 0 as libc::c_int;
        while i < num_faces {
            let mut j_1: libc::c_int = *face_list.offset(i as isize);
            let mut opp: face_colour =
                (if *board.offset(j_1 as isize) as libc::c_int == FACE_WHITE as libc::c_int {
                    FACE_BLACK as libc::c_int
                } else {
                    FACE_WHITE as libc::c_int
                }) as face_colour;
            if can_colour_face(g, board, j_1, opp) {
                let mut face: *mut grid_face = *((*g).faces).offset(j_1 as isize);
                if do_random_pass {
                    if random_upto(rs, 10 as libc::c_int as libc::c_ulong) == 0 {
                        *board.offset(j_1 as isize) = opp as libc::c_char;
                    }
                } else if face_num_neighbours(g, board, face, opp) == 1 as libc::c_int {
                    *board.offset(j_1 as isize) = opp as libc::c_char;
                    flipped = 1 as libc::c_int != 0;
                }
            }
            i += 1;
            i;
        }
        if do_random_pass {
            break;
        }
        if !flipped {
            do_random_pass = 1 as libc::c_int != 0;
        }
    }
    sfree(face_list as *mut libc::c_void);
}
