use ::libc;
extern "C" {
    pub type random_state;
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
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn domino_layout(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut rs: *mut random_state,
) -> *mut libc::c_int {
    let mut grid: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut grid2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut list: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut wh: libc::c_int = w * h;
    grid = smalloc(
        (wh as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    grid2 = smalloc(
        (wh as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    list = smalloc(
        ((2 as libc::c_int * wh) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    domino_layout_prealloc(w, h, rs, grid, grid2, list);
    sfree(grid2 as *mut libc::c_void);
    sfree(list as *mut libc::c_void);
    return grid;
}
#[no_mangle]
pub unsafe extern "C" fn domino_layout_prealloc(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut rs: *mut random_state,
    mut grid: *mut libc::c_int,
    mut grid2: *mut libc::c_int,
    mut list: *mut libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut wh: libc::c_int = w * h;
    let mut todo: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < wh {
        *grid.offset(i as isize) = i;
        i += 1;
        i;
    }
    k = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < h - 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < w {
            let fresh0 = k;
            k = k + 1;
            *list.offset(fresh0 as isize) = 2 as libc::c_int * (j * w + i);
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    j = 0 as libc::c_int;
    while j < h {
        i = 0 as libc::c_int;
        while i < w - 1 as libc::c_int {
            let fresh1 = k;
            k = k + 1;
            *list.offset(fresh1 as isize) = 2 as libc::c_int * (j * w + i) + 1 as libc::c_int;
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    if k == 2 as libc::c_int * wh - h - w {
    } else {
        __assert_fail(
            b"k == 2*wh - h - w\0" as *const u8 as *const libc::c_char,
            b"/puzzles/laydomino.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 75], &[libc::c_char; 75]>(
                b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3329: {
        if k == 2 as libc::c_int * wh - h - w {
        } else {
            __assert_fail(
                b"k == 2*wh - h - w\0" as *const u8 as *const libc::c_char,
                b"/puzzles/laydomino.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 75], &[libc::c_char; 75]>(
                    b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    shuffle(
        list as *mut libc::c_void,
        k,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        rs,
    );
    i = 0 as libc::c_int;
    while i < k {
        let mut horiz: libc::c_int = 0;
        let mut xy: libc::c_int = 0;
        let mut xy2: libc::c_int = 0;
        horiz = *list.offset(i as isize) % 2 as libc::c_int;
        xy = *list.offset(i as isize) / 2 as libc::c_int;
        xy2 = xy + (if horiz != 0 { 1 as libc::c_int } else { w });
        if *grid.offset(xy as isize) == xy && *grid.offset(xy2 as isize) == xy2 {
            *grid.offset(xy as isize) = xy2;
            *grid.offset(xy2 as isize) = xy;
        }
        i += 1;
        i;
    }
    loop {
        k = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < wh {
            if *grid.offset(j as isize) == j {
                k += 1;
                k;
                i = j;
            }
            j += 1;
            j;
        }
        if k == wh % 2 as libc::c_int {
            break;
        }
        j = 0 as libc::c_int;
        while j < wh {
            *grid2.offset(j as isize) = -(1 as libc::c_int);
            j += 1;
            j;
        }
        *grid2.offset(i as isize) = 0 as libc::c_int;
        done = 0 as libc::c_int;
        todo = 1 as libc::c_int;
        *list.offset(0 as libc::c_int as isize) = i;
        while done < todo {
            let mut d: [libc::c_int; 4] = [0; 4];
            let mut nd: libc::c_int = 0;
            let mut x: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let fresh2 = done;
            done = done + 1;
            i = *list.offset(fresh2 as isize);
            x = i % w;
            y = i / w;
            nd = 0 as libc::c_int;
            if x > 0 as libc::c_int {
                let fresh3 = nd;
                nd = nd + 1;
                d[fresh3 as usize] = i - 1 as libc::c_int;
            }
            if (x + 1 as libc::c_int) < w {
                let fresh4 = nd;
                nd = nd + 1;
                d[fresh4 as usize] = i + 1 as libc::c_int;
            }
            if y > 0 as libc::c_int {
                let fresh5 = nd;
                nd = nd + 1;
                d[fresh5 as usize] = i - w;
            }
            if (y + 1 as libc::c_int) < h {
                let fresh6 = nd;
                nd = nd + 1;
                d[fresh6 as usize] = i + w;
            }
            shuffle(
                d.as_mut_ptr() as *mut libc::c_void,
                nd,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                rs,
            );
            j = 0 as libc::c_int;
            while j < nd {
                k = d[j as usize];
                if *grid.offset(k as isize) == k {
                    *grid2.offset(k as isize) = i;
                    break;
                } else {
                    m = *grid.offset(k as isize);
                    if *grid2.offset(m as isize) < 0 as libc::c_int
                        || *grid2.offset(m as isize) > *grid2.offset(i as isize) + 1 as libc::c_int
                    {
                        *grid2.offset(m as isize) = *grid2.offset(i as isize) + 1 as libc::c_int;
                        *grid2.offset(k as isize) = i;
                        if todo < wh {
                        } else {
                            __assert_fail(
                                b"todo < wh\0" as *const u8 as *const libc::c_char,
                                b"/puzzles/laydomino.c\0" as *const u8
                                    as *const libc::c_char,
                                244 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 75],
                                    &[libc::c_char; 75],
                                >(
                                    b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                        'c_2893: {
                            if todo < wh {
                            } else {
                                __assert_fail(
                                    b"todo < wh\0" as *const u8 as *const libc::c_char,
                                    b"/puzzles/laydomino.c\0" as *const u8
                                        as *const libc::c_char,
                                    244 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 75],
                                        &[libc::c_char; 75],
                                    >(
                                        b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
                                    ))
                                        .as_ptr(),
                                );
                            }
                        };
                        let fresh7 = todo;
                        todo = todo + 1;
                        *list.offset(fresh7 as isize) = m;
                    }
                    j += 1;
                    j;
                }
            }
            if j < nd {
                i = k;
                break;
            } else {
                i = -(1 as libc::c_int);
            }
        }
        if i >= 0 as libc::c_int {
        } else {
            __assert_fail(
                b"i >= 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/laydomino.c\0" as *const u8 as *const libc::c_char,
                264 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 75], &[libc::c_char; 75]>(
                    b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_2800: {
            if i >= 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"i >= 0\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/laydomino.c\0" as *const u8 as *const libc::c_char,
                    264 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        loop {
            j = *grid2.offset(i as isize);
            if j >= 0 as libc::c_int && j < wh {
            } else {
                __assert_fail(
                    b"j >= 0 && j < wh\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/laydomino.c\0" as *const u8 as *const libc::c_char,
                    272 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 75],
                        &[libc::c_char; 75],
                    >(
                        b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_2733: {
                if j >= 0 as libc::c_int && j < wh {
                } else {
                    __assert_fail(
                        b"j >= 0 && j < wh\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/laydomino.c\0" as *const u8 as *const libc::c_char,
                        272 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 75],
                            &[libc::c_char; 75],
                        >(
                            b"void domino_layout_prealloc(int, int, random_state *, int *, int *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            k = *grid.offset(j as isize);
            *grid.offset(i as isize) = j;
            *grid.offset(j as isize) = i;
            if j == k {
                break;
            }
            i = k;
        }
    }
}
