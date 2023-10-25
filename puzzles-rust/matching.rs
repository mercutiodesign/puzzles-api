use ::libc;
extern "C" {
    pub type random_state;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn shuffle(
        array: *mut libc::c_void,
        nelts: libc::c_int,
        eltsize: libc::c_int,
        rs: *mut random_state,
    );
    fn random_upto(state: *mut random_state, limit: libc::c_ulong) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scratch {
    pub LtoR: *mut libc::c_int,
    pub RtoL: *mut libc::c_int,
    pub Llayer: *mut libc::c_int,
    pub Rlayer: *mut libc::c_int,
    pub Lqueue: *mut libc::c_int,
    pub Rqueue: *mut libc::c_int,
    pub augpath: *mut libc::c_int,
    pub dfsstate: *mut libc::c_int,
    pub Lorder: *mut libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn matching_scratch_size(mut nl: libc::c_int, mut nr: libc::c_int) -> size_t {
    let mut n: size_t = 0;
    let mut nmin: libc::c_int = if nl < nr { nl } else { nr };
    n = (::core::mem::size_of::<scratch>() as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    n = n.wrapping_add(nl as size_t);
    n = n.wrapping_add(nr as size_t);
    n = n.wrapping_add(nl as size_t);
    n = n.wrapping_add(nr as size_t);
    n = n.wrapping_add(nl as size_t);
    n = n.wrapping_add(nr as size_t);
    n = n.wrapping_add((2 as libc::c_int * nmin) as size_t);
    n = n.wrapping_add(nmin as size_t);
    n = n.wrapping_add(nl as size_t);
    return n.wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn matching_with_scratch(
    mut scratchv: *mut libc::c_void,
    mut nl: libc::c_int,
    mut nr: libc::c_int,
    mut adjlists: *mut *mut libc::c_int,
    mut adjsizes: *mut libc::c_int,
    mut rs: *mut random_state,
    mut outl: *mut libc::c_int,
    mut outr: *mut libc::c_int,
) -> libc::c_int {
    let mut s: *mut scratch = scratchv as *mut scratch;
    let mut L: libc::c_int = 0;
    let mut R: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut libc::c_int = scratchv as *mut libc::c_int;
    let mut nmin: libc::c_int = if nl < nr { nl } else { nr };
    p = p.offset(
        (::core::mem::size_of::<scratch>() as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) as isize,
    );
    (*s).LtoR = p;
    p = p.offset(nl as isize);
    (*s).RtoL = p;
    p = p.offset(nr as isize);
    (*s).Llayer = p;
    p = p.offset(nl as isize);
    (*s).Rlayer = p;
    p = p.offset(nr as isize);
    (*s).Lqueue = p;
    p = p.offset(nl as isize);
    (*s).Rqueue = p;
    p = p.offset(nr as isize);
    (*s).augpath = p;
    p = p.offset((2 as libc::c_int * nmin) as isize);
    (*s).dfsstate = p;
    p = p.offset(nmin as isize);
    (*s).Lorder = p;
    p = p.offset(nl as isize);
    L = 0 as libc::c_int;
    while L < nl {
        *((*s).LtoR).offset(L as isize) = -(1 as libc::c_int);
        L += 1;
        L;
    }
    R = 0 as libc::c_int;
    while R < nr {
        *((*s).RtoL).offset(R as isize) = -(1 as libc::c_int);
        R += 1;
        R;
    }
    's_101: loop {
        let mut Lqs: libc::c_int = 0;
        let mut Rqs: libc::c_int = 0;
        let mut layer: libc::c_int = 0;
        let mut target_layer: libc::c_int = 0;
        L = 0 as libc::c_int;
        while L < nl {
            *((*s).Llayer).offset(L as isize) = -(1 as libc::c_int);
            L += 1;
            L;
        }
        R = 0 as libc::c_int;
        while R < nr {
            *((*s).Rlayer).offset(R as isize) = -(1 as libc::c_int);
            R += 1;
            R;
        }
        Lqs = 0 as libc::c_int;
        L = 0 as libc::c_int;
        while L < nl {
            if *((*s).LtoR).offset(L as isize) == -(1 as libc::c_int) {
                *((*s).Llayer).offset(L as isize) = 0 as libc::c_int;
                let fresh0 = Lqs;
                Lqs = Lqs + 1;
                *((*s).Lqueue).offset(fresh0 as isize) = L;
            }
            L += 1;
            L;
        }
        layer = 0 as libc::c_int;
        loop {
            let mut found_free_R_vertex: bool = 0 as libc::c_int != 0;
            Rqs = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < Lqs {
                L = *((*s).Lqueue).offset(i as isize);
                if *((*s).Llayer).offset(L as isize) == layer {
                } else {
                    __assert_fail(
                        b"s->Llayer[L] == layer\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/matching.c\0" as *const u8 as *const libc::c_char,
                        143 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 89],
                            &[libc::c_char; 89],
                        >(
                            b"int matching_with_scratch(void *, int, int, int **, int *, random_state *, int *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3728: {
                    if *((*s).Llayer).offset(L as isize) == layer {
                    } else {
                        __assert_fail(
                            b"s->Llayer[L] == layer\0" as *const u8
                                as *const libc::c_char,
                            b"/puzzles/matching.c\0" as *const u8 as *const libc::c_char,
                            143 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 89],
                                &[libc::c_char; 89],
                            >(
                                b"int matching_with_scratch(void *, int, int, int **, int *, random_state *, int *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                j = 0 as libc::c_int;
                while j < *adjsizes.offset(L as isize) {
                    R = *(*adjlists.offset(L as isize)).offset(j as isize);
                    if R != *((*s).LtoR).offset(L as isize)
                        && *((*s).Rlayer).offset(R as isize) == -(1 as libc::c_int)
                    {
                        *((*s).Rlayer).offset(R as isize) = layer + 1 as libc::c_int;
                        let fresh1 = Rqs;
                        Rqs = Rqs + 1;
                        *((*s).Rqueue).offset(fresh1 as isize) = R;
                        if *((*s).RtoL).offset(R as isize) == -(1 as libc::c_int) {
                            found_free_R_vertex = 1 as libc::c_int != 0;
                        }
                    }
                    j += 1;
                    j;
                }
                i += 1;
                i;
            }
            layer += 1;
            layer;
            if found_free_R_vertex {
                break;
            }
            if Rqs == 0 as libc::c_int {
                break 's_101;
            }
            Lqs = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < Rqs {
                R = *((*s).Rqueue).offset(j as isize);
                if *((*s).Rlayer).offset(R as isize) == layer {
                } else {
                    __assert_fail(
                        b"s->Rlayer[R] == layer\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/matching.c\0" as *const u8 as *const libc::c_char,
                        166 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 89],
                            &[libc::c_char; 89],
                        >(
                            b"int matching_with_scratch(void *, int, int, int **, int *, random_state *, int *, int *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_3534: {
                    if *((*s).Rlayer).offset(R as isize) == layer {
                    } else {
                        __assert_fail(
                            b"s->Rlayer[R] == layer\0" as *const u8
                                as *const libc::c_char,
                            b"/puzzles/matching.c\0" as *const u8 as *const libc::c_char,
                            166 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 89],
                                &[libc::c_char; 89],
                            >(
                                b"int matching_with_scratch(void *, int, int, int **, int *, random_state *, int *, int *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                L = *((*s).RtoL).offset(R as isize);
                if L != -(1 as libc::c_int)
                    && *((*s).Llayer).offset(L as isize) == -(1 as libc::c_int)
                {
                    *((*s).Llayer).offset(L as isize) = layer + 1 as libc::c_int;
                    let fresh2 = Lqs;
                    Lqs = Lqs + 1;
                    *((*s).Lqueue).offset(fresh2 as isize) = L;
                }
                j += 1;
                j;
            }
            layer += 1;
            layer;
            if Lqs == 0 as libc::c_int {
                break 's_101;
            }
        }
        target_layer = layer;
        R = 0 as libc::c_int;
        while R < nr {
            if *((*s).Rlayer).offset(R as isize) == target_layer
                && *((*s).RtoL).offset(R as isize) != -(1 as libc::c_int)
            {
                *((*s).Rlayer).offset(R as isize) = -(1 as libc::c_int);
            }
            R += 1;
            R;
        }
        L = 0 as libc::c_int;
        while L < nl {
            *((*s).Lorder).offset(L as isize) = L;
            L += 1;
            L;
        }
        if !rs.is_null() {
            shuffle(
                (*s).Lorder as *mut libc::c_void,
                nl,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
                rs,
            );
        }
        *((*s).dfsstate).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        i = 0 as libc::c_int;
        loop {
            if i == 0 as libc::c_int {
                if *((*s).dfsstate).offset(i as isize) == nl {
                    break;
                }
                let ref mut fresh3 = *((*s).dfsstate).offset(i as isize);
                let fresh4 = *fresh3;
                *fresh3 = *fresh3 + 1;
                L = *((*s).Lorder).offset(fresh4 as isize);
                if *((*s).Llayer).offset(L as isize) != 2 as libc::c_int * i {
                    continue;
                }
            } else {
                L = *((*s).augpath).offset((2 as libc::c_int * i - 2 as libc::c_int) as isize);
                let ref mut fresh5 = *((*s).dfsstate).offset(i as isize);
                let fresh6 = *fresh5;
                *fresh5 = *fresh5 + 1;
                j = fresh6;
                if j == *adjsizes.offset(L as isize) {
                    i -= 1;
                    i;
                    continue;
                } else {
                    if !rs.is_null() && *adjsizes.offset(L as isize) - j > 1 as libc::c_int {
                        let mut which: libc::c_int = (j as libc::c_ulong).wrapping_add(random_upto(
                            rs,
                            (*adjsizes.offset(L as isize) - j) as libc::c_ulong,
                        )) as libc::c_int;
                        let mut tmp: libc::c_int =
                            *(*adjlists.offset(L as isize)).offset(which as isize);
                        *(*adjlists.offset(L as isize)).offset(which as isize) =
                            *(*adjlists.offset(L as isize)).offset(j as isize);
                        *(*adjlists.offset(L as isize)).offset(j as isize) = tmp;
                    }
                    R = *(*adjlists.offset(L as isize)).offset(j as isize);
                    if *((*s).Rlayer).offset(R as isize) != 2 as libc::c_int * i - 1 as libc::c_int
                    {
                        continue;
                    }
                    *((*s).augpath).offset((2 as libc::c_int * i - 1 as libc::c_int) as isize) = R;
                    *((*s).Rlayer).offset(R as isize) = -(1 as libc::c_int);
                    if 2 as libc::c_int * i - 1 as libc::c_int == target_layer {
                        j = 0 as libc::c_int;
                        while j < 2 as libc::c_int * i {
                            *((*s).LtoR).offset(*((*s).augpath).offset(j as isize) as isize) =
                                *((*s).augpath).offset((j + 1 as libc::c_int) as isize);
                            *((*s).RtoL)
                                .offset(*((*s).augpath).offset((j + 1 as libc::c_int) as isize)
                                    as isize) = *((*s).augpath).offset(j as isize);
                            j += 2 as libc::c_int;
                        }
                        i = 0 as libc::c_int;
                        continue;
                    } else {
                        L = *((*s).RtoL).offset(R as isize);
                        if *((*s).Llayer).offset(L as isize) != 2 as libc::c_int * i {
                            continue;
                        }
                    }
                }
            }
            *((*s).augpath).offset((2 as libc::c_int * i) as isize) = L;
            *((*s).Llayer).offset(L as isize) = -(1 as libc::c_int);
            i += 1;
            i;
            *((*s).dfsstate).offset(i as isize) = 0 as libc::c_int;
        }
    }
    if !outl.is_null() {
        i = 0 as libc::c_int;
        while i < nl {
            *outl.offset(i as isize) = *((*s).LtoR).offset(i as isize);
            i += 1;
            i;
        }
    }
    if !outr.is_null() {
        j = 0 as libc::c_int;
        while j < nr {
            *outr.offset(j as isize) = *((*s).RtoL).offset(j as isize);
            j += 1;
            j;
        }
    }
    j = 0 as libc::c_int;
    i = j;
    while i < nl {
        if *((*s).LtoR).offset(i as isize) != -(1 as libc::c_int) {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn matching(
    mut nl: libc::c_int,
    mut nr: libc::c_int,
    mut adjlists: *mut *mut libc::c_int,
    mut adjsizes: *mut libc::c_int,
    mut rs: *mut random_state,
    mut outl: *mut libc::c_int,
    mut outr: *mut libc::c_int,
) -> libc::c_int {
    let mut scratch: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    size = matching_scratch_size(nl, nr) as libc::c_int;
    scratch = malloc(size as libc::c_ulong);
    if scratch.is_null() {
        return -(1 as libc::c_int);
    }
    ret = matching_with_scratch(scratch, nl, nr, adjlists, adjsizes, rs, outl, outr);
    free(scratch);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn matching_witness(
    mut scratchv: *mut libc::c_void,
    mut nl: libc::c_int,
    mut nr: libc::c_int,
    mut witness: *mut libc::c_int,
) {
    let mut s: *mut scratch = scratchv as *mut scratch;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nl {
        *witness.offset(i as isize) =
            (*((*s).Llayer).offset(i as isize) == -(1 as libc::c_int)) as libc::c_int;
        i += 1;
        i;
    }
    j = 0 as libc::c_int;
    while j < nr {
        *witness.offset((nl + j) as isize) =
            (*((*s).Rlayer).offset(j as isize) == -(1 as libc::c_int)) as libc::c_int;
        j += 1;
        j;
    }
}
