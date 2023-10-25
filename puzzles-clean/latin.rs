use ::libc;
extern "C" {
    pub type random_state;
    pub type tree234_Tag;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
    fn shuffle(
        array: *mut libc::c_void,
        nelts: libc::c_int,
        eltsize: libc::c_int,
        rs: *mut random_state,
    );
    fn newtree234(cmp: cmpfn234) -> *mut tree234;
    fn freetree234(t: *mut tree234);
    fn add234(t: *mut tree234, e: *mut libc::c_void) -> *mut libc::c_void;
    fn index234(t: *mut tree234, index: libc::c_int) -> *mut libc::c_void;
    fn find234(t: *mut tree234, e: *mut libc::c_void, cmp: cmpfn234) -> *mut libc::c_void;
    fn count234(t: *mut tree234) -> libc::c_int;
    fn matching_with_scratch(
        scratch: *mut libc::c_void,
        nl: libc::c_int,
        nr: libc::c_int,
        adjlists: *mut *mut libc::c_int,
        adjsizes: *mut libc::c_int,
        rs: *mut random_state,
        outl: *mut libc::c_int,
        outr: *mut libc::c_int,
    ) -> libc::c_int;
    fn matching_scratch_size(nl: libc::c_int, nr: libc::c_int) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type tree234 = tree234_Tag;
pub type cmpfn234 =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int>;
pub type digit = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latin_solver {
    pub o: libc::c_int,
    pub cube: *mut libc::c_uchar,
    pub grid: *mut digit,
    pub row: *mut libc::c_uchar,
    pub col: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latin_solver_scratch {
    pub grid: *mut libc::c_uchar,
    pub rowidx: *mut libc::c_uchar,
    pub colidx: *mut libc::c_uchar,
    pub set: *mut libc::c_uchar,
    pub neighbours: *mut libc::c_int,
    pub bfsqueue: *mut libc::c_int,
}
pub type usersolver_t =
    Option<unsafe extern "C" fn(*mut latin_solver, *mut libc::c_void) -> libc::c_int>;
pub type validator_t = Option<unsafe extern "C" fn(*mut latin_solver, *mut libc::c_void) -> bool>;
pub type ctxnew_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>;
pub type ctxfree_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type C2RustUnnamed = libc::c_uint;
pub const diff_unfinished: C2RustUnnamed = 12;
pub const diff_ambiguous: C2RustUnnamed = 11;
pub const diff_impossible: C2RustUnnamed = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lcparams {
    pub elt: digit,
    pub count: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_place(
    mut solver: *mut latin_solver,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut n: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut o: libc::c_int = (*solver).o;
    if n <= o {
    } else {
        __assert_fail(
            b"n <= o\0" as *const u8 as *const libc::c_char,
            b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
            38 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                b"void latin_solver_place(struct latin_solver *, int, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3577: {
        if n <= o {
        } else {
            __assert_fail(
                b"n <= o\0" as *const u8 as *const libc::c_char,
                b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                    b"void latin_solver_place(struct latin_solver *, int, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if *((*solver).cube)
        .offset(((x * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize)
        != 0
    {
    } else {
        __assert_fail(
            b"cube(x,y,n)\0" as *const u8 as *const libc::c_char,
            b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                b"void latin_solver_place(struct latin_solver *, int, int, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_3479: {
        if *((*solver).cube)
            .offset(((x * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize)
            != 0
        {
        } else {
            __assert_fail(
                b"cube(x,y,n)\0" as *const u8 as *const libc::c_char,
                b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                39 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 62], &[libc::c_char; 62]>(
                    b"void latin_solver_place(struct latin_solver *, int, int, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    i = 1 as libc::c_int;
    while i <= o {
        if i != n {
            *((*solver).cube)
                .offset(((x * (*solver).o + y) * (*solver).o + i - 1 as libc::c_int) as isize) =
                0 as libc::c_int as libc::c_uchar;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < o {
        if i != y {
            *((*solver).cube)
                .offset(((x * (*solver).o + i) * (*solver).o + n - 1 as libc::c_int) as isize) =
                0 as libc::c_int as libc::c_uchar;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < o {
        if i != x {
            *((*solver).cube)
                .offset(((i * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize) =
                0 as libc::c_int as libc::c_uchar;
        }
        i += 1;
        i;
    }
    *((*solver).grid).offset((y * o + x) as isize) = n as digit;
    let ref mut fresh0 = *((*solver).col).offset((x * o + n - 1 as libc::c_int) as isize);
    *fresh0 = 1 as libc::c_int as libc::c_uchar;
    *((*solver).row).offset((y * o + n - 1 as libc::c_int) as isize) = *fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_elim(
    mut solver: *mut latin_solver,
    mut start: libc::c_int,
    mut step: libc::c_int,
) -> libc::c_int {
    let mut o: libc::c_int = (*solver).o;
    let mut fpos: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    m = 0 as libc::c_int;
    fpos = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < o {
        if *((*solver).cube).offset((start + i * step) as isize) != 0 {
            fpos = start + i * step;
            m += 1;
            m;
        }
        i += 1;
        i;
    }
    if m == 1 as libc::c_int {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        if fpos >= 0 as libc::c_int {
        } else {
            __assert_fail(
                b"fpos >= 0\0" as *const u8 as *const libc::c_char,
                b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                    b"int latin_solver_elim(struct latin_solver *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
        'c_3717: {
            if fpos >= 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"fpos >= 0\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                    100 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 55], &[libc::c_char; 55]>(
                        b"int latin_solver_elim(struct latin_solver *, int, int)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        n = 1 as libc::c_int + fpos % o;
        y = fpos / o;
        x = y / o;
        y %= o;
        if *((*solver).grid).offset((y * o + x) as isize) == 0 {
            latin_solver_place(solver, x, y, n);
            return 1 as libc::c_int;
        }
    } else if m == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_set(
    mut solver: *mut latin_solver,
    mut scratch: *mut latin_solver_scratch,
    mut start: libc::c_int,
    mut step1: libc::c_int,
    mut step2: libc::c_int,
) -> libc::c_int {
    let mut o: libc::c_int = (*solver).o;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut grid: *mut libc::c_uchar = (*scratch).grid;
    let mut rowidx: *mut libc::c_uchar = (*scratch).rowidx;
    let mut colidx: *mut libc::c_uchar = (*scratch).colidx;
    let mut set: *mut libc::c_uchar = (*scratch).set;
    memset(
        rowidx as *mut libc::c_void,
        1 as libc::c_int,
        o as libc::c_ulong,
    );
    memset(
        colidx as *mut libc::c_void,
        1 as libc::c_int,
        o as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < o {
        let mut count_0: libc::c_int = 0 as libc::c_int;
        let mut first: libc::c_int = -(1 as libc::c_int);
        j = 0 as libc::c_int;
        while j < o {
            if *((*solver).cube).offset((start + i * step1 + j * step2) as isize) != 0 {
                first = j;
                count_0 += 1;
                count_0;
            }
            j += 1;
            j;
        }
        if count_0 == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if count_0 == 1 as libc::c_int {
            let ref mut fresh1 = *colidx.offset(first as isize);
            *fresh1 = 0 as libc::c_int as libc::c_uchar;
            *rowidx.offset(i as isize) = *fresh1;
        }
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    i = j;
    while i < o {
        if *rowidx.offset(i as isize) != 0 {
            let fresh2 = j;
            j = j + 1;
            *rowidx.offset(fresh2 as isize) = i as libc::c_uchar;
        }
        i += 1;
        i;
    }
    n = j;
    j = 0 as libc::c_int;
    i = j;
    while i < o {
        if *colidx.offset(i as isize) != 0 {
            let fresh3 = j;
            j = j + 1;
            *colidx.offset(fresh3 as isize) = i as libc::c_uchar;
        }
        i += 1;
        i;
    }
    if n == j {
    } else {
        __assert_fail(
            b"n == j\0" as *const u8 as *const libc::c_char,
            b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"int latin_solver_set(struct latin_solver *, struct latin_solver_scratch *, int, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4261: {
        if n == j {
        } else {
            __assert_fail(
                b"n == j\0" as *const u8 as *const libc::c_char,
                b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                197 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 90],
                    &[libc::c_char; 90],
                >(
                    b"int latin_solver_set(struct latin_solver *, struct latin_solver_scratch *, int, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < n {
        j = 0 as libc::c_int;
        while j < n {
            *grid.offset((i * o + j) as isize) = *((*solver).cube).offset(
                (start
                    + *rowidx.offset(i as isize) as libc::c_int * step1
                    + *colidx.offset(j as isize) as libc::c_int * step2) as isize,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    memset(
        set as *mut libc::c_void,
        0 as libc::c_int,
        n as libc::c_ulong,
    );
    count = 0 as libc::c_int;
    loop {
        if count > 1 as libc::c_int && count < n - 1 as libc::c_int {
            let mut rows: libc::c_int = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < n {
                let mut ok: bool = 1 as libc::c_int != 0;
                j = 0 as libc::c_int;
                while j < n {
                    if *set.offset(j as isize) as libc::c_int != 0
                        && *grid.offset((i * o + j) as isize) as libc::c_int != 0
                    {
                        ok = 0 as libc::c_int != 0;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
                if ok {
                    rows += 1;
                    rows;
                }
                i += 1;
                i;
            }
            if rows > n - count {
                return -(1 as libc::c_int);
            }
            if rows >= n - count {
                let mut progress: bool = 0 as libc::c_int != 0;
                i = 0 as libc::c_int;
                while i < n {
                    let mut ok_0: bool = 1 as libc::c_int != 0;
                    j = 0 as libc::c_int;
                    while j < n {
                        if *set.offset(j as isize) as libc::c_int != 0
                            && *grid.offset((i * o + j) as isize) as libc::c_int != 0
                        {
                            ok_0 = 0 as libc::c_int != 0;
                            break;
                        } else {
                            j += 1;
                            j;
                        }
                    }
                    if !ok_0 {
                        j = 0 as libc::c_int;
                        while j < n {
                            if *set.offset(j as isize) == 0
                                && *grid.offset((i * o + j) as isize) as libc::c_int != 0
                            {
                                let mut fpos: libc::c_int = start
                                    + *rowidx.offset(i as isize) as libc::c_int * step1
                                    + *colidx.offset(j as isize) as libc::c_int * step2;
                                progress = 1 as libc::c_int != 0;
                                *((*solver).cube).offset(fpos as isize) =
                                    0 as libc::c_int as libc::c_uchar;
                            }
                            j += 1;
                            j;
                        }
                    }
                    i += 1;
                    i;
                }
                if progress {
                    return 1 as libc::c_int;
                }
            }
        }
        i = n;
        while i > 0 as libc::c_int
            && *set.offset((i - 1 as libc::c_int) as isize) as libc::c_int != 0
        {
            i -= 1;
            *set.offset(i as isize) = 0 as libc::c_int as libc::c_uchar;
            count -= 1;
            count;
        }
        if !(i > 0 as libc::c_int) {
            break;
        }
        i -= 1;
        *set.offset(i as isize) = 1 as libc::c_int as libc::c_uchar;
        count += 1;
        count;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_forcing(
    mut solver: *mut latin_solver,
    mut scratch: *mut latin_solver_scratch,
) -> libc::c_int {
    let mut o: libc::c_int = (*solver).o;
    let mut bfsqueue: *mut libc::c_int = (*scratch).bfsqueue;
    let mut number: *mut libc::c_uchar = (*scratch).grid;
    let mut neighbours: *mut libc::c_int = (*scratch).neighbours;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < o {
        x = 0 as libc::c_int;
        while x < o {
            let mut count: libc::c_int = 0;
            let mut t: libc::c_int = 0;
            let mut n: libc::c_int = 0;
            t = 0 as libc::c_int;
            count = t;
            n = 1 as libc::c_int;
            while n <= o {
                if *((*solver).cube)
                    .offset(((x * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize)
                    != 0
                {
                    count += 1;
                    count;
                    t += n;
                }
                n += 1;
                n;
            }
            if !(count != 2 as libc::c_int) {
                n = 1 as libc::c_int;
                while n <= o {
                    if *((*solver).cube).offset(
                        ((x * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize,
                    ) != 0
                    {
                        let mut orign: libc::c_int = 0;
                        let mut currn: libc::c_int = 0;
                        let mut head: libc::c_int = 0;
                        let mut tail: libc::c_int = 0;
                        orign = n;
                        memset(
                            number as *mut libc::c_void,
                            o + 1 as libc::c_int,
                            (o * o) as libc::c_ulong,
                        );
                        tail = 0 as libc::c_int;
                        head = tail;
                        let fresh4 = tail;
                        tail = tail + 1;
                        *bfsqueue.offset(fresh4 as isize) = y * o + x;
                        *number.offset((y * o + x) as isize) = (t - n) as libc::c_uchar;
                        while head < tail {
                            let mut xx: libc::c_int = 0;
                            let mut yy: libc::c_int = 0;
                            let mut nneighbours: libc::c_int = 0;
                            let mut xt: libc::c_int = 0;
                            let mut yt: libc::c_int = 0;
                            let mut i: libc::c_int = 0;
                            let fresh5 = head;
                            head = head + 1;
                            xx = *bfsqueue.offset(fresh5 as isize);
                            yy = xx / o;
                            xx %= o;
                            currn = *number.offset((yy * o + xx) as isize) as libc::c_int;
                            nneighbours = 0 as libc::c_int;
                            yt = 0 as libc::c_int;
                            while yt < o {
                                let fresh6 = nneighbours;
                                nneighbours = nneighbours + 1;
                                *neighbours.offset(fresh6 as isize) = yt * o + xx;
                                yt += 1;
                                yt;
                            }
                            xt = 0 as libc::c_int;
                            while xt < o {
                                let fresh7 = nneighbours;
                                nneighbours = nneighbours + 1;
                                *neighbours.offset(fresh7 as isize) = yy * o + xt;
                                xt += 1;
                                xt;
                            }
                            i = 0 as libc::c_int;
                            while i < nneighbours {
                                let mut cc: libc::c_int = 0;
                                let mut tt: libc::c_int = 0;
                                let mut nn: libc::c_int = 0;
                                xt = *neighbours.offset(i as isize) % o;
                                yt = *neighbours.offset(i as isize) / o;
                                if !(*number.offset((yt * o + xt) as isize) as libc::c_int <= o) {
                                    if !(*((*solver).cube).offset(
                                        ((xt * (*solver).o + yt) * (*solver).o + currn
                                            - 1 as libc::c_int)
                                            as isize,
                                    ) == 0)
                                    {
                                        if !(xt == xx && yt == yy) {
                                            tt = 0 as libc::c_int;
                                            cc = tt;
                                            nn = 1 as libc::c_int;
                                            while nn <= o {
                                                if *((*solver).cube).offset(
                                                    ((xt * (*solver).o + yt) * (*solver).o + nn
                                                        - 1 as libc::c_int)
                                                        as isize,
                                                ) != 0
                                                {
                                                    cc += 1;
                                                    cc;
                                                    tt += nn;
                                                }
                                                nn += 1;
                                                nn;
                                            }
                                            if cc == 2 as libc::c_int {
                                                let fresh8 = tail;
                                                tail = tail + 1;
                                                *bfsqueue.offset(fresh8 as isize) = yt * o + xt;
                                                *number.offset((yt * o + xt) as isize) =
                                                    (tt - currn) as libc::c_uchar;
                                            }
                                            if currn == orign && (xt == x || yt == y) {
                                                *((*solver).cube).offset(
                                                    ((xt * (*solver).o + yt) * (*solver).o + orign
                                                        - 1 as libc::c_int)
                                                        as isize,
                                                ) = 0 as libc::c_int as libc::c_uchar;
                                                return 1 as libc::c_int;
                                            }
                                        }
                                    }
                                }
                                i += 1;
                                i;
                            }
                        }
                    }
                    n += 1;
                    n;
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_new_scratch(
    mut solver: *mut latin_solver,
) -> *mut latin_solver_scratch {
    let mut scratch: *mut latin_solver_scratch =
        smalloc(::core::mem::size_of::<latin_solver_scratch>() as libc::c_ulong)
            as *mut latin_solver_scratch;
    let mut o: libc::c_int = (*solver).o;
    (*scratch).grid = smalloc(
        ((o * o) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    (*scratch).rowidx = smalloc(
        (o as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    (*scratch).colidx = smalloc(
        (o as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    (*scratch).set = smalloc(
        (o as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    (*scratch).neighbours = smalloc(
        ((3 as libc::c_int * o) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*scratch).bfsqueue = smalloc(
        ((o * o) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    return scratch;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_free_scratch(mut scratch: *mut latin_solver_scratch) {
    sfree((*scratch).bfsqueue as *mut libc::c_void);
    sfree((*scratch).neighbours as *mut libc::c_void);
    sfree((*scratch).set as *mut libc::c_void);
    sfree((*scratch).colidx as *mut libc::c_void);
    sfree((*scratch).rowidx as *mut libc::c_void);
    sfree((*scratch).grid as *mut libc::c_void);
    sfree(scratch as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_alloc(
    mut solver: *mut latin_solver,
    mut grid: *mut digit,
    mut o: libc::c_int,
) -> bool {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    (*solver).o = o;
    (*solver).cube = smalloc(
        ((o * o * o) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    (*solver).grid = grid;
    memset(
        (*solver).cube as *mut libc::c_void,
        1 as libc::c_int,
        (o * o * o) as libc::c_ulong,
    );
    (*solver).row = smalloc(
        ((o * o) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    (*solver).col = smalloc(
        ((o * o) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong),
    ) as *mut libc::c_uchar;
    memset(
        (*solver).row as *mut libc::c_void,
        0 as libc::c_int,
        (o * o) as libc::c_ulong,
    );
    memset(
        (*solver).col as *mut libc::c_void,
        0 as libc::c_int,
        (o * o) as libc::c_ulong,
    );
    x = 0 as libc::c_int;
    while x < o {
        y = 0 as libc::c_int;
        while y < o {
            let mut n: libc::c_int = *grid.offset((y * o + x) as isize) as libc::c_int;
            if n != 0 {
                if *((*solver).cube)
                    .offset(((x * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize)
                    != 0
                {
                    latin_solver_place(solver, x, y, n);
                } else {
                    return 0 as libc::c_int != 0;
                }
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_free(mut solver: *mut latin_solver) {
    sfree((*solver).cube as *mut libc::c_void);
    sfree((*solver).row as *mut libc::c_void);
    sfree((*solver).col as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_diff_simple(mut solver: *mut latin_solver) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut o: libc::c_int = (*solver).o;
    y = 0 as libc::c_int;
    while y < o {
        n = 1 as libc::c_int;
        while n <= o {
            if *((*solver).row).offset((y * o + n - 1 as libc::c_int) as isize) == 0 {
                ret = latin_solver_elim(
                    solver,
                    (0 as libc::c_int * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int,
                    o * o,
                );
                if ret != 0 as libc::c_int {
                    return ret;
                }
            }
            n += 1;
            n;
        }
        y += 1;
        y;
    }
    x = 0 as libc::c_int;
    while x < o {
        n = 1 as libc::c_int;
        while n <= o {
            if *((*solver).col).offset((x * o + n - 1 as libc::c_int) as isize) == 0 {
                ret = latin_solver_elim(
                    solver,
                    (x * (*solver).o + 0 as libc::c_int) * (*solver).o + n - 1 as libc::c_int,
                    o,
                );
                if ret != 0 as libc::c_int {
                    return ret;
                }
            }
            n += 1;
            n;
        }
        x += 1;
        x;
    }
    x = 0 as libc::c_int;
    while x < o {
        y = 0 as libc::c_int;
        while y < o {
            if *((*solver).grid).offset((y * o + x) as isize) == 0 {
                ret = latin_solver_elim(
                    solver,
                    (x * (*solver).o + y) * (*solver).o + 1 as libc::c_int - 1 as libc::c_int,
                    1 as libc::c_int,
                );
                if ret != 0 as libc::c_int {
                    return ret;
                }
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_diff_set(
    mut solver: *mut latin_solver,
    mut scratch: *mut latin_solver_scratch,
    mut extreme: bool,
) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut o: libc::c_int = (*solver).o;
    if !extreme {
        y = 0 as libc::c_int;
        while y < o {
            ret = latin_solver_set(
                solver,
                scratch,
                (0 as libc::c_int * (*solver).o + y) * (*solver).o + 1 as libc::c_int
                    - 1 as libc::c_int,
                o * o,
                1 as libc::c_int,
            );
            if ret != 0 as libc::c_int {
                return ret;
            }
            y += 1;
            y;
        }
        x = 0 as libc::c_int;
        while x < o {
            ret = latin_solver_set(
                solver,
                scratch,
                (x * (*solver).o + 0 as libc::c_int) * (*solver).o + 1 as libc::c_int
                    - 1 as libc::c_int,
                o,
                1 as libc::c_int,
            );
            if ret != 0 as libc::c_int {
                return ret;
            }
            x += 1;
            x;
        }
    } else {
        n = 1 as libc::c_int;
        while n <= o {
            ret = latin_solver_set(
                solver,
                scratch,
                (0 as libc::c_int * (*solver).o + 0 as libc::c_int) * (*solver).o + n
                    - 1 as libc::c_int,
                o * o,
                o,
            );
            if ret != 0 as libc::c_int {
                return ret;
            }
            n += 1;
            n;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn latin_solver_recurse(
    mut solver: *mut latin_solver,
    mut diff_simple: libc::c_int,
    mut diff_set_0: libc::c_int,
    mut diff_set_1: libc::c_int,
    mut diff_forcing: libc::c_int,
    mut diff_recursive: libc::c_int,
    mut usersolvers: *const usersolver_t,
    mut valid: validator_t,
    mut ctx: *mut libc::c_void,
    mut ctxnew: ctxnew_t,
    mut ctxfree: ctxfree_t,
) -> libc::c_int {
    let mut best: libc::c_int = 0;
    let mut bestcount: libc::c_int = 0;
    let mut o: libc::c_int = (*solver).o;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    best = -(1 as libc::c_int);
    bestcount = o + 1 as libc::c_int;
    y = 0 as libc::c_int;
    while y < o {
        x = 0 as libc::c_int;
        while x < o {
            if *((*solver).grid).offset((y * o + x) as isize) == 0 {
                let mut count: libc::c_int = 0;
                count = 0 as libc::c_int;
                n = 1 as libc::c_int;
                while n <= o {
                    if *((*solver).cube).offset(
                        ((x * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize,
                    ) != 0
                    {
                        count += 1;
                        count;
                    }
                    n += 1;
                    n;
                }
                if count > 1 as libc::c_int {
                } else {
                    __assert_fail(
                        b"count > 1\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                        753 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 137],
                            &[libc::c_char; 137],
                        >(
                            b"int latin_solver_recurse(struct latin_solver *, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_6869: {
                    if count > 1 as libc::c_int {
                    } else {
                        __assert_fail(
                            b"count > 1\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                            753 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 137],
                                &[libc::c_char; 137],
                            >(
                                b"int latin_solver_recurse(struct latin_solver *, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                if count < bestcount {
                    bestcount = count;
                    best = y * o + x;
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    if best == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    } else {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut list: *mut digit = 0 as *mut digit;
        let mut ingrid: *mut digit = 0 as *mut digit;
        let mut outgrid: *mut digit = 0 as *mut digit;
        let mut diff: libc::c_int = diff_impossible as libc::c_int;
        y = best / o;
        x = best % o;
        list = smalloc(
            (o as libc::c_ulong).wrapping_mul(::core::mem::size_of::<digit>() as libc::c_ulong),
        ) as *mut digit;
        ingrid = smalloc(
            ((o * o) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<digit>() as libc::c_ulong),
        ) as *mut digit;
        outgrid = smalloc(
            ((o * o) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<digit>() as libc::c_ulong),
        ) as *mut digit;
        memcpy(
            ingrid as *mut libc::c_void,
            (*solver).grid as *const libc::c_void,
            (o * o) as libc::c_ulong,
        );
        j = 0 as libc::c_int;
        n = 1 as libc::c_int;
        while n <= o {
            if *((*solver).cube)
                .offset(((x * (*solver).o + y) * (*solver).o + n - 1 as libc::c_int) as isize)
                != 0
            {
                let fresh9 = j;
                j = j + 1;
                *list.offset(fresh9 as isize) = n as digit;
            }
            n += 1;
            n;
        }
        i = 0 as libc::c_int;
        while i < j {
            let mut ret: libc::c_int = 0;
            let mut newctx: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut subsolver: latin_solver = latin_solver {
                o: 0,
                cube: 0 as *mut libc::c_uchar,
                grid: 0 as *mut digit,
                row: 0 as *mut libc::c_uchar,
                col: 0 as *mut libc::c_uchar,
            };
            memcpy(
                outgrid as *mut libc::c_void,
                ingrid as *const libc::c_void,
                (o * o) as libc::c_ulong,
            );
            *outgrid.offset((y * o + x) as isize) = *list.offset(i as isize);
            if ctxnew.is_some() {
                newctx = ctxnew.expect("non-null function pointer")(ctx);
            } else {
                newctx = ctx;
            }
            if latin_solver_alloc(&mut subsolver, outgrid, o) {
                ret = latin_solver_top(
                    &mut subsolver,
                    diff_recursive,
                    diff_simple,
                    diff_set_0,
                    diff_set_1,
                    diff_forcing,
                    diff_recursive,
                    usersolvers,
                    valid,
                    newctx,
                    ctxnew,
                    ctxfree,
                );
            } else {
                ret = diff_impossible as libc::c_int;
            }
            latin_solver_free(&mut subsolver);
            if ctxnew.is_some() {
                ctxfree.expect("non-null function pointer")(newctx);
            }
            if ret != diff_unfinished as libc::c_int {
            } else {
                __assert_fail(
                    b"ret != diff_unfinished\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                    847 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 137],
                        &[libc::c_char; 137],
                    >(
                        b"int latin_solver_recurse(struct latin_solver *, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_6532: {
                if ret != diff_unfinished as libc::c_int {
                } else {
                    __assert_fail(
                        b"ret != diff_unfinished\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                        847 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 137],
                            &[libc::c_char; 137],
                        >(
                            b"int latin_solver_recurse(struct latin_solver *, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            if diff == diff_impossible as libc::c_int && ret != diff_impossible as libc::c_int {
                memcpy(
                    (*solver).grid as *mut libc::c_void,
                    outgrid as *const libc::c_void,
                    (o * o) as libc::c_ulong,
                );
            }
            if ret == diff_ambiguous as libc::c_int {
                diff = diff_ambiguous as libc::c_int;
            } else if !(ret == diff_impossible as libc::c_int) {
                if diff == diff_impossible as libc::c_int {
                    diff = diff_recursive;
                } else {
                    diff = diff_ambiguous as libc::c_int;
                }
            }
            if diff == diff_ambiguous as libc::c_int {
                break;
            }
            i += 1;
            i;
        }
        sfree(outgrid as *mut libc::c_void);
        sfree(ingrid as *mut libc::c_void);
        sfree(list as *mut libc::c_void);
        if diff == diff_impossible as libc::c_int {
            return -(1 as libc::c_int);
        } else if diff == diff_ambiguous as libc::c_int {
            return 2 as libc::c_int;
        } else {
            if diff == diff_recursive {
            } else {
                __assert_fail(
                    b"diff == diff_recursive\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                    885 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 137],
                        &[libc::c_char; 137],
                    >(
                        b"int latin_solver_recurse(struct latin_solver *, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_6375: {
                if diff == diff_recursive {
                } else {
                    __assert_fail(
                        b"diff == diff_recursive\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                        885 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 137],
                            &[libc::c_char; 137],
                        >(
                            b"int latin_solver_recurse(struct latin_solver *, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn latin_solver_top(
    mut solver: *mut latin_solver,
    mut maxdiff: libc::c_int,
    mut diff_simple: libc::c_int,
    mut diff_set_0: libc::c_int,
    mut diff_set_1: libc::c_int,
    mut diff_forcing: libc::c_int,
    mut diff_recursive: libc::c_int,
    mut usersolvers: *const usersolver_t,
    mut valid: validator_t,
    mut ctx: *mut libc::c_void,
    mut ctxnew: ctxnew_t,
    mut ctxfree: ctxfree_t,
) -> libc::c_int {
    let mut scratch: *mut latin_solver_scratch = latin_solver_new_scratch(solver);
    let mut ret: libc::c_int = 0;
    let mut diff: libc::c_int = diff_simple;
    if maxdiff <= diff_recursive {
    } else {
        __assert_fail(
            b"maxdiff <= diff_recursive\0" as *const u8 as *const libc::c_char,
            b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
            900 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 138],
                &[libc::c_char; 138],
            >(
                b"int latin_solver_top(struct latin_solver *, int, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7201: {
        if maxdiff <= diff_recursive {
        } else {
            __assert_fail(
                b"maxdiff <= diff_recursive\0" as *const u8 as *const libc::c_char,
                b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                900 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 138],
                    &[libc::c_char; 138],
                >(
                    b"int latin_solver_top(struct latin_solver *, int, int, int, int, int, int, const usersolver_t *, validator_t, void *, ctxnew_t, ctxfree_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut i: libc::c_int = 0;
    '_cont: loop {
        latin_solver_debug((*solver).cube, (*solver).o);
        i = 0 as libc::c_int;
        while i <= maxdiff {
            if (*usersolvers.offset(i as isize)).is_some() {
                ret = (*usersolvers.offset(i as isize)).expect("non-null function pointer")(
                    solver, ctx,
                );
            } else {
                ret = 0 as libc::c_int;
            }
            if ret == 0 as libc::c_int && i == diff_simple {
                ret = latin_solver_diff_simple(solver);
            }
            if ret == 0 as libc::c_int && i == diff_set_0 {
                ret = latin_solver_diff_set(solver, scratch, 0 as libc::c_int != 0);
            }
            if ret == 0 as libc::c_int && i == diff_set_1 {
                ret = latin_solver_diff_set(solver, scratch, 1 as libc::c_int != 0);
            }
            if ret == 0 as libc::c_int && i == diff_forcing {
                ret = latin_solver_forcing(solver, scratch);
            }
            if ret < 0 as libc::c_int {
                diff = diff_impossible as libc::c_int;
                break '_cont;
            } else if ret > 0 as libc::c_int {
                diff = if diff > i { diff } else { i };
                continue '_cont;
            } else {
                i += 1;
                i;
            }
        }
        if maxdiff == diff_recursive {
            let mut nsol: libc::c_int = latin_solver_recurse(
                solver,
                diff_simple,
                diff_set_0,
                diff_set_1,
                diff_forcing,
                diff_recursive,
                usersolvers,
                valid,
                ctx,
                ctxnew,
                ctxfree,
            );
            if nsol < 0 as libc::c_int {
                diff = diff_impossible as libc::c_int;
            } else if nsol == 1 as libc::c_int {
                diff = diff_recursive;
            } else if nsol > 1 as libc::c_int {
                diff = diff_ambiguous as libc::c_int;
            }
        } else {
            let mut x: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let mut o: libc::c_int = (*solver).o;
            y = 0 as libc::c_int;
            while y < o {
                x = 0 as libc::c_int;
                while x < o {
                    if *((*solver).grid).offset((y * o + x) as isize) == 0 {
                        diff = diff_unfinished as libc::c_int;
                    }
                    x += 1;
                    x;
                }
                y += 1;
                y;
            }
        }
        break;
    }
    if diff != diff_impossible as libc::c_int
        && diff != diff_unfinished as libc::c_int
        && diff != diff_ambiguous as libc::c_int
        && valid.is_some()
        && !valid.expect("non-null function pointer")(solver, ctx)
    {
        diff = diff_impossible as libc::c_int;
    }
    latin_solver_free_scratch(scratch);
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_main(
    mut solver: *mut latin_solver,
    mut maxdiff: libc::c_int,
    mut diff_simple: libc::c_int,
    mut diff_set_0: libc::c_int,
    mut diff_set_1: libc::c_int,
    mut diff_forcing: libc::c_int,
    mut diff_recursive: libc::c_int,
    mut usersolvers: *const usersolver_t,
    mut valid: validator_t,
    mut ctx: *mut libc::c_void,
    mut ctxnew: ctxnew_t,
    mut ctxfree: ctxfree_t,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    diff = latin_solver_top(
        solver,
        maxdiff,
        diff_simple,
        diff_set_0,
        diff_set_1,
        diff_forcing,
        diff_recursive,
        usersolvers,
        valid,
        ctx,
        ctxnew,
        ctxfree,
    );
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver(
    mut grid: *mut digit,
    mut o: libc::c_int,
    mut maxdiff: libc::c_int,
    mut diff_simple: libc::c_int,
    mut diff_set_0: libc::c_int,
    mut diff_set_1: libc::c_int,
    mut diff_forcing: libc::c_int,
    mut diff_recursive: libc::c_int,
    mut usersolvers: *const usersolver_t,
    mut valid: validator_t,
    mut ctx: *mut libc::c_void,
    mut ctxnew: ctxnew_t,
    mut ctxfree: ctxfree_t,
) -> libc::c_int {
    let mut solver: latin_solver = latin_solver {
        o: 0,
        cube: 0 as *mut libc::c_uchar,
        grid: 0 as *mut digit,
        row: 0 as *mut libc::c_uchar,
        col: 0 as *mut libc::c_uchar,
    };
    let mut diff: libc::c_int = 0;
    if latin_solver_alloc(&mut solver, grid, o) {
        diff = latin_solver_main(
            &mut solver,
            maxdiff,
            diff_simple,
            diff_set_0,
            diff_set_1,
            diff_forcing,
            diff_recursive,
            usersolvers,
            valid,
            ctx,
            ctxnew,
            ctxfree,
        );
    } else {
        diff = diff_impossible as libc::c_int;
    }
    latin_solver_free(&mut solver);
    return diff;
}
#[no_mangle]
pub unsafe extern "C" fn latin_solver_debug(mut cube: *mut libc::c_uchar, mut o: libc::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn latin_debug(mut sq: *mut digit, mut o: libc::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn latin_generate(
    mut o: libc::c_int,
    mut rs: *mut random_state,
) -> *mut digit {
    let mut sq: *mut digit = 0 as *mut digit;
    let mut adjdata: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut adjsizes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut matching: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut adjlists: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut scratch: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut row: *mut digit = 0 as *mut digit;
    sq = smalloc(
        ((o * o) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<digit>() as libc::c_ulong),
    ) as *mut digit;
    row = smalloc(
        (o as libc::c_ulong).wrapping_mul(::core::mem::size_of::<digit>() as libc::c_ulong),
    ) as *mut digit;
    i = 0 as libc::c_int;
    while i < o {
        *row.offset(i as isize) = i as digit;
        i += 1;
        i;
    }
    shuffle(
        row as *mut libc::c_void,
        i,
        ::core::mem::size_of::<digit>() as libc::c_ulong as libc::c_int,
        rs,
    );
    scratch = smalloc(matching_scratch_size(o, o));
    adjdata = smalloc(
        ((o * o) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    adjlists = smalloc(
        (o as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    adjsizes = smalloc(
        (o as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    matching = smalloc(
        (o as libc::c_ulong).wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < o {
        j = 0 as libc::c_int;
        while j < o {
            let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
            let mut adj: *mut libc::c_int = adjdata.offset((j * o) as isize);
            k = 0 as libc::c_int;
            while k < o {
                *adj.offset(k as isize) = 1 as libc::c_int;
                k += 1;
                k;
            }
            k = 0 as libc::c_int;
            while k < i {
                *adj.offset(
                    (*sq.offset((*row.offset(k as isize) as libc::c_int * o + j) as isize)
                        as libc::c_int
                        - 1 as libc::c_int) as isize,
                ) = 0 as libc::c_int;
                k += 1;
                k;
            }
            p = adj;
            let ref mut fresh10 = *adjlists.offset(j as isize);
            *fresh10 = p;
            k = 0 as libc::c_int;
            while k < o {
                if *adj.offset(k as isize) != 0 {
                    let fresh11 = p;
                    p = p.offset(1);
                    *fresh11 = k;
                }
                k += 1;
                k;
            }
            *adjsizes.offset(j as isize) =
                p.offset_from(*adjlists.offset(j as isize)) as libc::c_long as libc::c_int;
            j += 1;
            j;
        }
        j = matching_with_scratch(
            scratch,
            o,
            o,
            adjlists,
            adjsizes,
            rs,
            matching,
            0 as *mut libc::c_int,
        );
        if j == o {
        } else {
            __assert_fail(
                b"j == o\0" as *const u8 as *const libc::c_char,
                b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                1208 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                    b"digit *latin_generate(int, random_state *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_7369: {
            if j == o {
            } else {
                __assert_fail(
                    b"j == o\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                    1208 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 43], &[libc::c_char; 43]>(
                        b"digit *latin_generate(int, random_state *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        j = 0 as libc::c_int;
        while j < o {
            *sq.offset((*row.offset(i as isize) as libc::c_int * o + j) as isize) =
                (*matching.offset(j as isize) + 1 as libc::c_int) as digit;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    sfree(matching as *mut libc::c_void);
    sfree(adjlists as *mut libc::c_void);
    sfree(adjsizes as *mut libc::c_void);
    sfree(adjdata as *mut libc::c_void);
    sfree(scratch);
    sfree(row as *mut libc::c_void);
    return sq;
}
#[no_mangle]
pub unsafe extern "C" fn latin_generate_rect(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut rs: *mut random_state,
) -> *mut digit {
    let mut o: libc::c_int = if w > h { w } else { h };
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut latin: *mut digit = 0 as *mut digit;
    let mut latin_rect: *mut digit = 0 as *mut digit;
    latin = latin_generate(o, rs);
    latin_rect = smalloc(
        ((w * h) as libc::c_ulong).wrapping_mul(::core::mem::size_of::<digit>() as libc::c_ulong),
    ) as *mut digit;
    x = 0 as libc::c_int;
    while x < w {
        y = 0 as libc::c_int;
        while y < h {
            *latin_rect.offset((y * w + x) as isize) = *latin.offset((y * o + x) as isize);
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    sfree(latin as *mut libc::c_void);
    return latin_rect;
}
unsafe extern "C" fn latin_check_cmp(
    mut v1: *mut libc::c_void,
    mut v2: *mut libc::c_void,
) -> libc::c_int {
    let mut lc1: *mut lcparams = v1 as *mut lcparams;
    let mut lc2: *mut lcparams = v2 as *mut lcparams;
    if ((*lc1).elt as libc::c_int) < (*lc2).elt as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*lc1).elt as libc::c_int > (*lc2).elt as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn latin_check(mut sq: *mut digit, mut order: libc::c_int) -> bool {
    let mut dict: *mut tree234 = newtree234(Some(
        latin_check_cmp
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    ));
    let mut c: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ret: bool = 0 as libc::c_int != 0;
    let mut lcp: *mut lcparams = 0 as *mut lcparams;
    let mut lc: lcparams = lcparams { elt: 0, count: 0 };
    let mut aret: *mut lcparams = 0 as *mut lcparams;
    c = 0 as libc::c_int;
    while c < order {
        r = 0 as libc::c_int;
        while r < order {
            lc.elt = *sq.offset((r * order + c) as isize);
            lc.count = 0 as libc::c_int;
            lcp =
                find234(dict, &mut lc as *mut lcparams as *mut libc::c_void, None) as *mut lcparams;
            if lcp.is_null() {
                lcp = smalloc(::core::mem::size_of::<lcparams>() as libc::c_ulong) as *mut lcparams;
                (*lcp).elt = *sq.offset((r * order + c) as isize);
                (*lcp).count = 1 as libc::c_int;
                aret = add234(dict, lcp as *mut libc::c_void) as *mut lcparams;
                if aret == lcp {
                } else {
                    __assert_fail(
                        b"aret == lcp\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                        1292 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            b"_Bool latin_check(digit *, int)\0",
                        ))
                        .as_ptr(),
                    );
                }
                'c_8026: {
                    if aret == lcp {
                    } else {
                        __assert_fail(
                            b"aret == lcp\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/latin.c\0" as *const u8 as *const libc::c_char,
                            1292 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                                b"_Bool latin_check(digit *, int)\0",
                            ))
                            .as_ptr(),
                        );
                    }
                };
            } else {
                (*lcp).count += 1;
                (*lcp).count;
            }
            r += 1;
            r;
        }
        c += 1;
        c;
    }
    if count234(dict) != order {
        ret = 1 as libc::c_int != 0;
    } else {
        c = 0 as libc::c_int;
        loop {
            lcp = index234(dict, c) as *mut lcparams;
            if lcp.is_null() {
                break;
            }
            if (*lcp).count != order {
                ret = 1 as libc::c_int != 0;
            }
            c += 1;
            c;
        }
    }
    c = 0 as libc::c_int;
    loop {
        lcp = index234(dict, c) as *mut lcparams;
        if lcp.is_null() {
            break;
        }
        sfree(lcp as *mut libc::c_void);
        c += 1;
        c;
    }
    freetree234(dict);
    return ret;
}
