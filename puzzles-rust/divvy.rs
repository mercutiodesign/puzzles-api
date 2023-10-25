use ::libc;
extern "C" {
    pub type random_state;
    pub type DSF;
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
    fn dsf_new(size: libc::c_int) -> *mut DSF;
    fn dsf_free(dsf: *mut DSF);
    fn dsf_canonify(dsf: *mut DSF, n: libc::c_int) -> libc::c_int;
    fn dsf_equivalent(dsf: *mut DSF, n1: libc::c_int, n2: libc::c_int) -> bool;
    fn dsf_merge(dsf: *mut DSF, n1: libc::c_int, n2: libc::c_int);
    fn random_upto(state: *mut random_state, limit: libc::c_ulong) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
unsafe extern "C" fn addremcommon(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut own: *mut libc::c_int,
    mut val: libc::c_int,
) -> bool {
    let mut neighbours: [libc::c_int; 8] = [0; 8];
    let mut dir: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    dir = 0 as libc::c_int;
    while dir < 8 as libc::c_int {
        let mut dx: libc::c_int = if dir & 3 as libc::c_int == 2 as libc::c_int {
            0 as libc::c_int
        } else if dir > 2 as libc::c_int && dir < 6 as libc::c_int {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        let mut dy: libc::c_int = if dir & 3 as libc::c_int == 0 as libc::c_int {
            0 as libc::c_int
        } else if dir < 4 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
        let mut sx: libc::c_int = x + dx;
        let mut sy: libc::c_int = y + dy;
        if sx < 0 as libc::c_int || sx >= w || sy < 0 as libc::c_int || sy >= h {
            neighbours[dir as usize] = -(1 as libc::c_int);
        } else {
            neighbours[dir as usize] = *own.offset((sy * w + sx) as isize);
        }
        dir += 1;
        dir;
    }
    if neighbours[0 as libc::c_int as usize] != val
        && neighbours[2 as libc::c_int as usize] != val
        && neighbours[4 as libc::c_int as usize] != val
        && neighbours[6 as libc::c_int as usize] != val
    {
        return 0 as libc::c_int != 0;
    }
    count = 0 as libc::c_int;
    dir = 0 as libc::c_int;
    while dir < 8 as libc::c_int {
        let mut next: libc::c_int = dir + 1 as libc::c_int & 7 as libc::c_int;
        let mut gotthis: bool = neighbours[dir as usize] == val;
        let mut gotnext: bool = neighbours[next as usize] == val;
        if gotthis as libc::c_int != gotnext as libc::c_int {
            count += 1;
            count;
        }
        dir += 1;
        dir;
    }
    return count == 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn divvy_rectangle_attempt(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut k: libc::c_int,
    mut rs: *mut random_state,
) -> *mut DSF {
    let mut current_block: u64;
    let mut order: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut queue: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut own: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sizes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut addable: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut retdsf: *mut DSF = 0 as *mut DSF;
    let mut tmpdsf: *mut DSF = 0 as *mut DSF;
    let mut removable: *mut bool = 0 as *mut bool;
    let mut wh: libc::c_int = w * h;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut qhead: libc::c_int = 0;
    let mut qtail: libc::c_int = 0;
    n = wh / k;
    if wh == k * n {
    } else {
        __assert_fail(
            b"wh == k*n\0" as *const u8 as *const libc::c_char,
            b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_4697: {
        if wh == k * n {
        } else {
            __assert_fail(
                b"wh == k*n\0" as *const u8 as *const libc::c_char,
                b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                272 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                    b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    order = smalloc(
        (wh as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    tmp = smalloc(
        (wh as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    own = smalloc(
        (wh as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    sizes = smalloc(
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    queue = smalloc(
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    addable = smalloc(
        ((wh * 4 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    removable = smalloc(
        (wh as libc::c_ulong).wrapping_mul(::core::mem::size_of::<bool>() as libc::c_ulong),
    ) as *mut bool;
    tmpdsf = 0 as *mut DSF;
    retdsf = tmpdsf;
    i = 0 as libc::c_int;
    while i < wh {
        *order.offset(i as isize) = i;
        i += 1;
        i;
    }
    shuffle(
        order as *mut libc::c_void,
        wh,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
        rs,
    );
    i = 0 as libc::c_int;
    while i < wh {
        *own.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < n {
        *own.offset(*order.offset(i as isize) as isize) = i;
        *sizes.offset(i as isize) = 1 as libc::c_int;
        i += 1;
        i;
    }
    loop {
        y = 0 as libc::c_int;
        while y < h {
            x = 0 as libc::c_int;
            while x < w {
                let mut yx: libc::c_int = y * w + x;
                let mut curr: libc::c_int = *own.offset(yx as isize);
                let mut dir: libc::c_int = 0;
                if curr < 0 as libc::c_int {
                    *removable.offset(yx as isize) = 0 as libc::c_int != 0;
                } else if *sizes.offset(curr as isize) == 1 as libc::c_int {
                    *removable.offset(yx as isize) = 1 as libc::c_int != 0;
                } else {
                    *removable.offset(yx as isize) = addremcommon(w, h, x, y, own, curr);
                }
                dir = 0 as libc::c_int;
                while dir < 4 as libc::c_int {
                    let mut dx: libc::c_int = if dir == 0 as libc::c_int {
                        -(1 as libc::c_int)
                    } else if dir == 1 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    let mut dy: libc::c_int = if dir == 2 as libc::c_int {
                        -(1 as libc::c_int)
                    } else if dir == 3 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                    let mut sx: libc::c_int = x + dx;
                    let mut sy: libc::c_int = y + dy;
                    let mut syx: libc::c_int = sy * w + sx;
                    *addable.offset((yx * 4 as libc::c_int + dir) as isize) = -(1 as libc::c_int);
                    if !(sx < 0 as libc::c_int || sx >= w || sy < 0 as libc::c_int || sy >= h) {
                        if !(*own.offset(syx as isize) < 0 as libc::c_int) {
                            if !(*own.offset(syx as isize) == *own.offset(yx as isize)) {
                                if addremcommon(w, h, x, y, own, *own.offset(syx as isize)) {
                                    *addable.offset((yx * 4 as libc::c_int + dir) as isize) =
                                        *own.offset(syx as isize);
                                }
                            }
                        }
                    }
                    dir += 1;
                    dir;
                }
                x += 1;
                x;
            }
            y += 1;
            y;
        }
        j = 0 as libc::c_int;
        i = j;
        while i < n {
            if *sizes.offset(i as isize) < k {
                let fresh0 = j;
                j = j + 1;
                *tmp.offset(fresh0 as isize) = i;
            }
            i += 1;
            i;
        }
        if j == 0 as libc::c_int {
            current_block = 8937240710477387595;
            break;
        }
        j = *tmp.offset(random_upto(rs, j as libc::c_ulong) as isize);
        if wh >= 2 as libc::c_int * n {
        } else {
            __assert_fail(
                b"wh >= 2*n\0" as *const u8 as *const libc::c_char,
                b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                406 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                    b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_4133: {
            if wh >= 2 as libc::c_int * n {
            } else {
                __assert_fail(
                    b"wh >= 2*n\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                    406 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                        b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        i = 0 as libc::c_int;
        while i < n {
            let ref mut fresh1 = *tmp.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
            *fresh1 = -(1 as libc::c_int);
            *tmp.offset((2 as libc::c_int * i) as isize) = *fresh1;
            i += 1;
            i;
        }
        qtail = 0 as libc::c_int;
        qhead = qtail;
        let fresh2 = qtail;
        qtail = qtail + 1;
        *queue.offset(fresh2 as isize) = j;
        let ref mut fresh3 = *tmp.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
        *fresh3 = -(2 as libc::c_int);
        *tmp.offset((2 as libc::c_int * j) as isize) = *fresh3;
        while qhead < qtail {
            let mut tmpsq: libc::c_int = 0;
            j = *queue.offset(qhead as isize);
            tmpsq = *tmp.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
            if tmpsq >= 0 as libc::c_int {
                if *own.offset(tmpsq as isize) == j {
                } else {
                    __assert_fail(
                        b"own[tmpsq] == j\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                        427 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                            b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                        ))
                        .as_ptr(),
                    );
                }
                'c_3994: {
                    if *own.offset(tmpsq as isize) == j {
                    } else {
                        __assert_fail(
                            b"own[tmpsq] == j\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                            427 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                                b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                };
                *own.offset(tmpsq as isize) = -(3 as libc::c_int);
            }
            i = 0 as libc::c_int;
            while i < wh {
                let mut dir_0: libc::c_int = 0;
                if !(*own.offset(*order.offset(i as isize) as isize) != -(1 as libc::c_int)) {
                    if *sizes.offset(j as isize) == 1 as libc::c_int && tmpsq >= 0 as libc::c_int {
                        break;
                    }
                    dir_0 = 0 as libc::c_int;
                    while dir_0 < 4 as libc::c_int {
                        if *addable
                            .offset((*order.offset(i as isize) * 4 as libc::c_int + dir_0) as isize)
                            == j
                        {
                            if addremcommon(
                                w,
                                h,
                                *order.offset(i as isize) % w,
                                *order.offset(i as isize) / w,
                                own,
                                j,
                            ) {
                                break;
                            }
                        }
                        dir_0 += 1;
                        dir_0;
                    }
                    if !(dir_0 == 4 as libc::c_int) {
                        break;
                    }
                }
                i += 1;
                i;
            }
            if i < wh {
                i = *order.offset(i as isize);
                if tmpsq >= 0 as libc::c_int {
                    *own.offset(tmpsq as isize) = j;
                }
                loop {
                    *own.offset(i as isize) = j;
                    if *tmp.offset((2 as libc::c_int * j) as isize) == -(2 as libc::c_int) {
                        break;
                    }
                    i = *tmp.offset((2 as libc::c_int * j + 1 as libc::c_int) as isize);
                    j = *tmp.offset((2 as libc::c_int * j) as isize);
                }
                let ref mut fresh4 = *sizes.offset(j as isize);
                *fresh4 += 1;
                *fresh4;
                break;
            } else {
                i = 0 as libc::c_int;
                while i < wh {
                    let mut dir_1: libc::c_int = 0;
                    let mut nj: libc::c_int = 0;
                    nj = *own.offset(*order.offset(i as isize) as isize);
                    if !(nj < 0 as libc::c_int
                        || *tmp.offset((2 as libc::c_int * nj) as isize) != -(1 as libc::c_int))
                    {
                        if *removable.offset(*order.offset(i as isize) as isize) {
                            dir_1 = 0 as libc::c_int;
                            while dir_1 < 4 as libc::c_int {
                                if *addable.offset(
                                    (*order.offset(i as isize) * 4 as libc::c_int + dir_1) as isize,
                                ) == j
                                {
                                    if addremcommon(
                                        w,
                                        h,
                                        *order.offset(i as isize) % w,
                                        *order.offset(i as isize) / w,
                                        own,
                                        j,
                                    ) {
                                        if qtail < n {
                                        } else {
                                            __assert_fail(
                                                b"qtail < n\0" as *const u8 as *const libc::c_char,
                                                b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                                                553 as libc::c_int as libc::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 60],
                                                    &[libc::c_char; 60],
                                                >(
                                                    b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                        'c_3390: {
                                            if qtail < n {
                                            } else {
                                                __assert_fail(
                                                    b"qtail < n\0" as *const u8 as *const libc::c_char,
                                                    b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                                                    553 as libc::c_int as libc::c_uint,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 60],
                                                        &[libc::c_char; 60],
                                                    >(
                                                        b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                                                    ))
                                                        .as_ptr(),
                                                );
                                            }
                                        };
                                        let fresh5 = qtail;
                                        qtail = qtail + 1;
                                        *queue.offset(fresh5 as isize) = nj;
                                        *tmp.offset((2 as libc::c_int * nj) as isize) = j;
                                        *tmp.offset(
                                            (2 as libc::c_int * nj + 1 as libc::c_int) as isize,
                                        ) = *order.offset(i as isize);
                                        break;
                                    }
                                }
                                dir_1 += 1;
                                dir_1;
                            }
                        }
                    }
                    i += 1;
                    i;
                }
                if tmpsq >= 0 as libc::c_int {
                    *own.offset(tmpsq as isize) = j;
                }
                qhead += 1;
                qhead;
            }
        }
        if !(qhead == qtail) {
            continue;
        }
        retdsf = 0 as *mut DSF;
        current_block = 10191365816945047851;
        break;
    }
    match current_block {
        8937240710477387595 => {
            i = 0 as libc::c_int;
            while i < wh {
                if *own.offset(i as isize) >= 0 as libc::c_int && *own.offset(i as isize) < n {
                } else {
                    __assert_fail(
                        b"own[i] >= 0 && own[i] < n\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                        611 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                            b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                        ))
                        .as_ptr(),
                    );
                }
                'c_3203: {
                    if *own.offset(i as isize) >= 0 as libc::c_int && *own.offset(i as isize) < n {
                    } else {
                        __assert_fail(
                            b"own[i] >= 0 && own[i] < n\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                            611 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                                b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                };
                *tmp.offset(*own.offset(i as isize) as isize) = i;
                i += 1;
                i;
            }
            retdsf = dsf_new(wh);
            i = 0 as libc::c_int;
            while i < wh {
                dsf_merge(retdsf, i, *tmp.offset(*own.offset(i as isize) as isize));
                i += 1;
                i;
            }
            tmpdsf = dsf_new(wh);
            y = 0 as libc::c_int;
            while y < h {
                x = 0 as libc::c_int;
                while (x + 1 as libc::c_int) < w {
                    if *own.offset((y * w + x) as isize)
                        == *own.offset((y * w + (x + 1 as libc::c_int)) as isize)
                    {
                        dsf_merge(tmpdsf, y * w + x, y * w + (x + 1 as libc::c_int));
                    }
                    x += 1;
                    x;
                }
                y += 1;
                y;
            }
            x = 0 as libc::c_int;
            while x < w {
                y = 0 as libc::c_int;
                while (y + 1 as libc::c_int) < h {
                    if *own.offset((y * w + x) as isize)
                        == *own.offset(((y + 1 as libc::c_int) * w + x) as isize)
                    {
                        dsf_merge(tmpdsf, y * w + x, (y + 1 as libc::c_int) * w + x);
                    }
                    y += 1;
                    y;
                }
                x += 1;
                x;
            }
            i = 0 as libc::c_int;
            while i < wh {
                j = dsf_canonify(retdsf, i);
                if dsf_equivalent(tmpdsf, j, i) {
                } else {
                    __assert_fail(
                        b"dsf_equivalent(tmpdsf, j, i)\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                        635 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                            b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                        ))
                        .as_ptr(),
                    );
                }
                'c_2900: {
                    if dsf_equivalent(tmpdsf, j, i) {
                    } else {
                        __assert_fail(
                            b"dsf_equivalent(tmpdsf, j, i)\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/divvy.c\0" as *const u8 as *const libc::c_char,
                            635 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 60], &[libc::c_char; 60]>(
                                b"DSF *divvy_rectangle_attempt(int, int, int, random_state *)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                };
                i += 1;
                i;
            }
        }
        _ => {}
    }
    sfree(order as *mut libc::c_void);
    sfree(tmp as *mut libc::c_void);
    dsf_free(tmpdsf);
    sfree(own as *mut libc::c_void);
    sfree(sizes as *mut libc::c_void);
    sfree(queue as *mut libc::c_void);
    sfree(addable as *mut libc::c_void);
    sfree(removable as *mut libc::c_void);
    return retdsf;
}
#[no_mangle]
pub unsafe extern "C" fn divvy_rectangle(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut k: libc::c_int,
    mut rs: *mut random_state,
) -> *mut DSF {
    let mut ret: *mut DSF = 0 as *mut DSF;
    loop {
        ret = divvy_rectangle_attempt(w, h, k, rs);
        if !ret.is_null() {
            break;
        }
    }
    return ret;
}
