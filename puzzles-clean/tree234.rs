use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn sfree(p: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tree234_Tag {
    pub root: *mut node234,
    pub cmp: cmpfn234,
}
pub type cmpfn234 = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
pub type node234 = node234_Tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node234_Tag {
    pub parent: *mut node234,
    pub kids: [*mut node234; 4],
    pub counts: [libc::c_int; 4],
    pub elems: [*mut libc::c_void; 3],
}
pub type tree234 = tree234_Tag;
pub type copyfn234 = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
>;
pub type C2RustUnnamed = libc::c_uint;
pub const REL234_GE: C2RustUnnamed = 4;
pub const REL234_GT: C2RustUnnamed = 3;
pub const REL234_LE: C2RustUnnamed = 2;
pub const REL234_LT: C2RustUnnamed = 1;
pub const REL234_EQ: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn newtree234(mut cmp: cmpfn234) -> *mut tree234 {
    let mut ret: *mut tree234 = smalloc(
        ::core::mem::size_of::<tree234>() as libc::c_ulong,
    ) as *mut tree234;
    (*ret).root = 0 as *mut node234;
    (*ret).cmp = cmp;
    return ret;
}
unsafe extern "C" fn freenode234(mut n: *mut node234) {
    if n.is_null() {
        return;
    }
    freenode234((*n).kids[0 as libc::c_int as usize]);
    freenode234((*n).kids[1 as libc::c_int as usize]);
    freenode234((*n).kids[2 as libc::c_int as usize]);
    freenode234((*n).kids[3 as libc::c_int as usize]);
    sfree(n as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn freetree234(mut t: *mut tree234) {
    freenode234((*t).root);
    sfree(t as *mut libc::c_void);
}
unsafe extern "C" fn countnode234(mut n: *mut node234) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if n.is_null() {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        count += (*n).counts[i as usize];
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if !((*n).elems[i as usize]).is_null() {
            count += 1;
            count;
        }
        i += 1;
        i;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn count234(mut t: *mut tree234) -> libc::c_int {
    if !((*t).root).is_null() {
        return countnode234((*t).root)
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn add234_insert(
    mut left: *mut node234,
    mut e: *mut libc::c_void,
    mut right: *mut node234,
    mut root: *mut *mut node234,
    mut n: *mut node234,
    mut ki: libc::c_int,
) -> libc::c_int {
    let mut lcount: libc::c_int = 0;
    let mut rcount: libc::c_int = 0;
    lcount = countnode234(left);
    rcount = countnode234(right);
    while !n.is_null() {
        if ((*n).elems[1 as libc::c_int as usize]).is_null() {
            if ki == 0 as libc::c_int {
                (*n)
                    .kids[2 as libc::c_int
                    as usize] = (*n).kids[1 as libc::c_int as usize];
                (*n)
                    .counts[2 as libc::c_int
                    as usize] = (*n).counts[1 as libc::c_int as usize];
                (*n)
                    .elems[1 as libc::c_int
                    as usize] = (*n).elems[0 as libc::c_int as usize];
                (*n).kids[1 as libc::c_int as usize] = right;
                (*n).counts[1 as libc::c_int as usize] = rcount;
                (*n).elems[0 as libc::c_int as usize] = e;
                (*n).kids[0 as libc::c_int as usize] = left;
                (*n).counts[0 as libc::c_int as usize] = lcount;
            } else {
                (*n).kids[2 as libc::c_int as usize] = right;
                (*n).counts[2 as libc::c_int as usize] = rcount;
                (*n).elems[1 as libc::c_int as usize] = e;
                (*n).kids[1 as libc::c_int as usize] = left;
                (*n).counts[1 as libc::c_int as usize] = lcount;
            }
            if !((*n).kids[0 as libc::c_int as usize]).is_null() {
                (*(*n).kids[0 as libc::c_int as usize]).parent = n;
            }
            if !((*n).kids[1 as libc::c_int as usize]).is_null() {
                (*(*n).kids[1 as libc::c_int as usize]).parent = n;
            }
            if !((*n).kids[2 as libc::c_int as usize]).is_null() {
                (*(*n).kids[2 as libc::c_int as usize]).parent = n;
            }
            break;
        } else if ((*n).elems[2 as libc::c_int as usize]).is_null() {
            if ki == 0 as libc::c_int {
                (*n)
                    .kids[3 as libc::c_int
                    as usize] = (*n).kids[2 as libc::c_int as usize];
                (*n)
                    .counts[3 as libc::c_int
                    as usize] = (*n).counts[2 as libc::c_int as usize];
                (*n)
                    .elems[2 as libc::c_int
                    as usize] = (*n).elems[1 as libc::c_int as usize];
                (*n)
                    .kids[2 as libc::c_int
                    as usize] = (*n).kids[1 as libc::c_int as usize];
                (*n)
                    .counts[2 as libc::c_int
                    as usize] = (*n).counts[1 as libc::c_int as usize];
                (*n)
                    .elems[1 as libc::c_int
                    as usize] = (*n).elems[0 as libc::c_int as usize];
                (*n).kids[1 as libc::c_int as usize] = right;
                (*n).counts[1 as libc::c_int as usize] = rcount;
                (*n).elems[0 as libc::c_int as usize] = e;
                (*n).kids[0 as libc::c_int as usize] = left;
                (*n).counts[0 as libc::c_int as usize] = lcount;
            } else if ki == 1 as libc::c_int {
                (*n)
                    .kids[3 as libc::c_int
                    as usize] = (*n).kids[2 as libc::c_int as usize];
                (*n)
                    .counts[3 as libc::c_int
                    as usize] = (*n).counts[2 as libc::c_int as usize];
                (*n)
                    .elems[2 as libc::c_int
                    as usize] = (*n).elems[1 as libc::c_int as usize];
                (*n).kids[2 as libc::c_int as usize] = right;
                (*n).counts[2 as libc::c_int as usize] = rcount;
                (*n).elems[1 as libc::c_int as usize] = e;
                (*n).kids[1 as libc::c_int as usize] = left;
                (*n).counts[1 as libc::c_int as usize] = lcount;
            } else {
                (*n).kids[3 as libc::c_int as usize] = right;
                (*n).counts[3 as libc::c_int as usize] = rcount;
                (*n).elems[2 as libc::c_int as usize] = e;
                (*n).kids[2 as libc::c_int as usize] = left;
                (*n).counts[2 as libc::c_int as usize] = lcount;
            }
            if !((*n).kids[0 as libc::c_int as usize]).is_null() {
                (*(*n).kids[0 as libc::c_int as usize]).parent = n;
            }
            if !((*n).kids[1 as libc::c_int as usize]).is_null() {
                (*(*n).kids[1 as libc::c_int as usize]).parent = n;
            }
            if !((*n).kids[2 as libc::c_int as usize]).is_null() {
                (*(*n).kids[2 as libc::c_int as usize]).parent = n;
            }
            if !((*n).kids[3 as libc::c_int as usize]).is_null() {
                (*(*n).kids[3 as libc::c_int as usize]).parent = n;
            }
            break;
        } else {
            let mut m: *mut node234 = smalloc(
                ::core::mem::size_of::<node234>() as libc::c_ulong,
            ) as *mut node234;
            (*m).parent = (*n).parent;
            if ki == 0 as libc::c_int {
                (*m).kids[0 as libc::c_int as usize] = left;
                (*m).counts[0 as libc::c_int as usize] = lcount;
                (*m).elems[0 as libc::c_int as usize] = e;
                (*m).kids[1 as libc::c_int as usize] = right;
                (*m).counts[1 as libc::c_int as usize] = rcount;
                (*m)
                    .elems[1 as libc::c_int
                    as usize] = (*n).elems[0 as libc::c_int as usize];
                (*m)
                    .kids[2 as libc::c_int
                    as usize] = (*n).kids[1 as libc::c_int as usize];
                (*m)
                    .counts[2 as libc::c_int
                    as usize] = (*n).counts[1 as libc::c_int as usize];
                e = (*n).elems[1 as libc::c_int as usize];
                (*n)
                    .kids[0 as libc::c_int
                    as usize] = (*n).kids[2 as libc::c_int as usize];
                (*n)
                    .counts[0 as libc::c_int
                    as usize] = (*n).counts[2 as libc::c_int as usize];
                (*n)
                    .elems[0 as libc::c_int
                    as usize] = (*n).elems[2 as libc::c_int as usize];
                (*n)
                    .kids[1 as libc::c_int
                    as usize] = (*n).kids[3 as libc::c_int as usize];
                (*n)
                    .counts[1 as libc::c_int
                    as usize] = (*n).counts[3 as libc::c_int as usize];
            } else if ki == 1 as libc::c_int {
                (*m)
                    .kids[0 as libc::c_int
                    as usize] = (*n).kids[0 as libc::c_int as usize];
                (*m)
                    .counts[0 as libc::c_int
                    as usize] = (*n).counts[0 as libc::c_int as usize];
                (*m)
                    .elems[0 as libc::c_int
                    as usize] = (*n).elems[0 as libc::c_int as usize];
                (*m).kids[1 as libc::c_int as usize] = left;
                (*m).counts[1 as libc::c_int as usize] = lcount;
                (*m).elems[1 as libc::c_int as usize] = e;
                (*m).kids[2 as libc::c_int as usize] = right;
                (*m).counts[2 as libc::c_int as usize] = rcount;
                e = (*n).elems[1 as libc::c_int as usize];
                (*n)
                    .kids[0 as libc::c_int
                    as usize] = (*n).kids[2 as libc::c_int as usize];
                (*n)
                    .counts[0 as libc::c_int
                    as usize] = (*n).counts[2 as libc::c_int as usize];
                (*n)
                    .elems[0 as libc::c_int
                    as usize] = (*n).elems[2 as libc::c_int as usize];
                (*n)
                    .kids[1 as libc::c_int
                    as usize] = (*n).kids[3 as libc::c_int as usize];
                (*n)
                    .counts[1 as libc::c_int
                    as usize] = (*n).counts[3 as libc::c_int as usize];
            } else if ki == 2 as libc::c_int {
                (*m)
                    .kids[0 as libc::c_int
                    as usize] = (*n).kids[0 as libc::c_int as usize];
                (*m)
                    .counts[0 as libc::c_int
                    as usize] = (*n).counts[0 as libc::c_int as usize];
                (*m)
                    .elems[0 as libc::c_int
                    as usize] = (*n).elems[0 as libc::c_int as usize];
                (*m)
                    .kids[1 as libc::c_int
                    as usize] = (*n).kids[1 as libc::c_int as usize];
                (*m)
                    .counts[1 as libc::c_int
                    as usize] = (*n).counts[1 as libc::c_int as usize];
                (*m)
                    .elems[1 as libc::c_int
                    as usize] = (*n).elems[1 as libc::c_int as usize];
                (*m).kids[2 as libc::c_int as usize] = left;
                (*m).counts[2 as libc::c_int as usize] = lcount;
                (*n).kids[0 as libc::c_int as usize] = right;
                (*n).counts[0 as libc::c_int as usize] = rcount;
                (*n)
                    .elems[0 as libc::c_int
                    as usize] = (*n).elems[2 as libc::c_int as usize];
                (*n)
                    .kids[1 as libc::c_int
                    as usize] = (*n).kids[3 as libc::c_int as usize];
                (*n)
                    .counts[1 as libc::c_int
                    as usize] = (*n).counts[3 as libc::c_int as usize];
            } else {
                (*m)
                    .kids[0 as libc::c_int
                    as usize] = (*n).kids[0 as libc::c_int as usize];
                (*m)
                    .counts[0 as libc::c_int
                    as usize] = (*n).counts[0 as libc::c_int as usize];
                (*m)
                    .elems[0 as libc::c_int
                    as usize] = (*n).elems[0 as libc::c_int as usize];
                (*m)
                    .kids[1 as libc::c_int
                    as usize] = (*n).kids[1 as libc::c_int as usize];
                (*m)
                    .counts[1 as libc::c_int
                    as usize] = (*n).counts[1 as libc::c_int as usize];
                (*m)
                    .elems[1 as libc::c_int
                    as usize] = (*n).elems[1 as libc::c_int as usize];
                (*m)
                    .kids[2 as libc::c_int
                    as usize] = (*n).kids[2 as libc::c_int as usize];
                (*m)
                    .counts[2 as libc::c_int
                    as usize] = (*n).counts[2 as libc::c_int as usize];
                (*n).kids[0 as libc::c_int as usize] = left;
                (*n).counts[0 as libc::c_int as usize] = lcount;
                (*n).elems[0 as libc::c_int as usize] = e;
                (*n).kids[1 as libc::c_int as usize] = right;
                (*n).counts[1 as libc::c_int as usize] = rcount;
                e = (*n).elems[2 as libc::c_int as usize];
            }
            (*n).kids[2 as libc::c_int as usize] = 0 as *mut node234;
            (*n).kids[3 as libc::c_int as usize] = (*n).kids[2 as libc::c_int as usize];
            (*m).kids[3 as libc::c_int as usize] = (*n).kids[3 as libc::c_int as usize];
            (*n).counts[2 as libc::c_int as usize] = 0 as libc::c_int;
            (*n)
                .counts[3 as libc::c_int
                as usize] = (*n).counts[2 as libc::c_int as usize];
            (*m)
                .counts[3 as libc::c_int
                as usize] = (*n).counts[3 as libc::c_int as usize];
            (*n).elems[1 as libc::c_int as usize] = 0 as *mut libc::c_void;
            (*n)
                .elems[2 as libc::c_int
                as usize] = (*n).elems[1 as libc::c_int as usize];
            (*m)
                .elems[2 as libc::c_int
                as usize] = (*n).elems[2 as libc::c_int as usize];
            if !((*m).kids[0 as libc::c_int as usize]).is_null() {
                (*(*m).kids[0 as libc::c_int as usize]).parent = m;
            }
            if !((*m).kids[1 as libc::c_int as usize]).is_null() {
                (*(*m).kids[1 as libc::c_int as usize]).parent = m;
            }
            if !((*m).kids[2 as libc::c_int as usize]).is_null() {
                (*(*m).kids[2 as libc::c_int as usize]).parent = m;
            }
            if !((*n).kids[0 as libc::c_int as usize]).is_null() {
                (*(*n).kids[0 as libc::c_int as usize]).parent = n;
            }
            if !((*n).kids[1 as libc::c_int as usize]).is_null() {
                (*(*n).kids[1 as libc::c_int as usize]).parent = n;
            }
            left = m;
            lcount = countnode234(left);
            right = n;
            rcount = countnode234(right);
            if !((*n).parent).is_null() {
                ki = if (*(*n).parent).kids[0 as libc::c_int as usize] == n {
                    0 as libc::c_int
                } else if (*(*n).parent).kids[1 as libc::c_int as usize] == n {
                    1 as libc::c_int
                } else if (*(*n).parent).kids[2 as libc::c_int as usize] == n {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                };
            }
            n = (*n).parent;
        }
    }
    if !n.is_null() {
        while !((*n).parent).is_null() {
            let mut count: libc::c_int = countnode234(n);
            let mut childnum: libc::c_int = 0;
            childnum = if (*(*n).parent).kids[0 as libc::c_int as usize] == n {
                0 as libc::c_int
            } else if (*(*n).parent).kids[1 as libc::c_int as usize] == n {
                1 as libc::c_int
            } else if (*(*n).parent).kids[2 as libc::c_int as usize] == n {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            (*(*n).parent).counts[childnum as usize] = count;
            n = (*n).parent;
        }
        return 0 as libc::c_int;
    } else {
        *root = smalloc(::core::mem::size_of::<node234>() as libc::c_ulong)
            as *mut node234;
        (**root).kids[0 as libc::c_int as usize] = left;
        (**root).counts[0 as libc::c_int as usize] = lcount;
        (**root).elems[0 as libc::c_int as usize] = e;
        (**root).kids[1 as libc::c_int as usize] = right;
        (**root).counts[1 as libc::c_int as usize] = rcount;
        (**root).elems[1 as libc::c_int as usize] = 0 as *mut libc::c_void;
        (**root).kids[2 as libc::c_int as usize] = 0 as *mut node234;
        (**root).counts[2 as libc::c_int as usize] = 0 as libc::c_int;
        (**root).elems[2 as libc::c_int as usize] = 0 as *mut libc::c_void;
        (**root).kids[3 as libc::c_int as usize] = 0 as *mut node234;
        (**root).counts[3 as libc::c_int as usize] = 0 as libc::c_int;
        (**root).parent = 0 as *mut node234;
        if !((**root).kids[0 as libc::c_int as usize]).is_null() {
            (*(**root).kids[0 as libc::c_int as usize]).parent = *root;
        }
        if !((**root).kids[1 as libc::c_int as usize]).is_null() {
            (*(**root).kids[1 as libc::c_int as usize]).parent = *root;
        }
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn add234_internal(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    let mut n: *mut node234 = 0 as *mut node234;
    let mut ki: libc::c_int = 0;
    let mut orig_e: *mut libc::c_void = e;
    let mut c: libc::c_int = 0;
    if ((*t).root).is_null() {
        (*t)
            .root = smalloc(::core::mem::size_of::<node234>() as libc::c_ulong)
            as *mut node234;
        (*(*t).root).elems[2 as libc::c_int as usize] = 0 as *mut libc::c_void;
        (*(*t).root)
            .elems[1 as libc::c_int
            as usize] = (*(*t).root).elems[2 as libc::c_int as usize];
        (*(*t).root).kids[1 as libc::c_int as usize] = 0 as *mut node234;
        (*(*t).root)
            .kids[0 as libc::c_int
            as usize] = (*(*t).root).kids[1 as libc::c_int as usize];
        (*(*t).root).kids[3 as libc::c_int as usize] = 0 as *mut node234;
        (*(*t).root)
            .kids[2 as libc::c_int
            as usize] = (*(*t).root).kids[3 as libc::c_int as usize];
        (*(*t).root).counts[1 as libc::c_int as usize] = 0 as libc::c_int;
        (*(*t).root)
            .counts[0 as libc::c_int
            as usize] = (*(*t).root).counts[1 as libc::c_int as usize];
        (*(*t).root).counts[3 as libc::c_int as usize] = 0 as libc::c_int;
        (*(*t).root)
            .counts[2 as libc::c_int
            as usize] = (*(*t).root).counts[3 as libc::c_int as usize];
        (*(*t).root).parent = 0 as *mut node234;
        (*(*t).root).elems[0 as libc::c_int as usize] = e;
        return orig_e;
    }
    n = (*t).root;
    loop {
        if index >= 0 as libc::c_int {
            if ((*n).kids[0 as libc::c_int as usize]).is_null() {
                ki = index;
            } else if index <= (*n).counts[0 as libc::c_int as usize] {
                ki = 0 as libc::c_int;
            } else {
                index -= (*n).counts[0 as libc::c_int as usize] + 1 as libc::c_int;
                if index <= (*n).counts[1 as libc::c_int as usize] {
                    ki = 1 as libc::c_int;
                } else {
                    index -= (*n).counts[1 as libc::c_int as usize] + 1 as libc::c_int;
                    if index <= (*n).counts[2 as libc::c_int as usize] {
                        ki = 2 as libc::c_int;
                    } else {
                        index
                            -= (*n).counts[2 as libc::c_int as usize] + 1 as libc::c_int;
                        if index <= (*n).counts[3 as libc::c_int as usize] {
                            ki = 3 as libc::c_int;
                        } else {
                            return 0 as *mut libc::c_void
                        }
                    }
                }
            }
        } else {
            c = ((*t).cmp)
                .expect(
                    "non-null function pointer",
                )(e, (*n).elems[0 as libc::c_int as usize]);
            if c < 0 as libc::c_int {
                ki = 0 as libc::c_int;
            } else if c == 0 as libc::c_int {
                return (*n).elems[0 as libc::c_int as usize]
            } else if ((*n).elems[1 as libc::c_int as usize]).is_null()
                || {
                    c = ((*t).cmp)
                        .expect(
                            "non-null function pointer",
                        )(e, (*n).elems[1 as libc::c_int as usize]);
                    c < 0 as libc::c_int
                }
            {
                ki = 1 as libc::c_int;
            } else if c == 0 as libc::c_int {
                return (*n).elems[1 as libc::c_int as usize]
            } else if ((*n).elems[2 as libc::c_int as usize]).is_null()
                || {
                    c = ((*t).cmp)
                        .expect(
                            "non-null function pointer",
                        )(e, (*n).elems[2 as libc::c_int as usize]);
                    c < 0 as libc::c_int
                }
            {
                ki = 2 as libc::c_int;
            } else if c == 0 as libc::c_int {
                return (*n).elems[2 as libc::c_int as usize]
            } else {
                ki = 3 as libc::c_int;
            }
        }
        if ((*n).kids[ki as usize]).is_null() {
            break;
        }
        n = (*n).kids[ki as usize];
        if n.is_null() {
            break;
        }
    }
    add234_insert(0 as *mut node234, e, 0 as *mut node234, &mut (*t).root, n, ki);
    return orig_e;
}
#[no_mangle]
pub unsafe extern "C" fn add234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
) -> *mut libc::c_void {
    if ((*t).cmp).is_none() {
        return 0 as *mut libc::c_void;
    }
    return add234_internal(t, e, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn addpos234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    if index < 0 as libc::c_int || ((*t).cmp).is_some() {
        return 0 as *mut libc::c_void;
    }
    return add234_internal(t, e, index);
}
#[no_mangle]
pub unsafe extern "C" fn index234(
    mut t: *mut tree234,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    let mut n: *mut node234 = 0 as *mut node234;
    if ((*t).root).is_null() {
        return 0 as *mut libc::c_void;
    }
    if index < 0 as libc::c_int || index >= countnode234((*t).root) {
        return 0 as *mut libc::c_void;
    }
    n = (*t).root;
    while !n.is_null() {
        if index < (*n).counts[0 as libc::c_int as usize] {
            n = (*n).kids[0 as libc::c_int as usize];
        } else {
            index -= (*n).counts[0 as libc::c_int as usize] + 1 as libc::c_int;
            if index < 0 as libc::c_int {
                return (*n).elems[0 as libc::c_int as usize]
            } else if index < (*n).counts[1 as libc::c_int as usize] {
                n = (*n).kids[1 as libc::c_int as usize];
            } else {
                index -= (*n).counts[1 as libc::c_int as usize] + 1 as libc::c_int;
                if index < 0 as libc::c_int {
                    return (*n).elems[1 as libc::c_int as usize]
                } else if index < (*n).counts[2 as libc::c_int as usize] {
                    n = (*n).kids[2 as libc::c_int as usize];
                } else {
                    index -= (*n).counts[2 as libc::c_int as usize] + 1 as libc::c_int;
                    if index < 0 as libc::c_int {
                        return (*n).elems[2 as libc::c_int as usize]
                    } else {
                        n = (*n).kids[3 as libc::c_int as usize];
                    }
                }
            }
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn findrelpos234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
    mut cmp: cmpfn234,
    mut relation: libc::c_int,
    mut index: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut n: *mut node234 = 0 as *mut node234;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut ecount: libc::c_int = 0;
    let mut kcount: libc::c_int = 0;
    let mut cmpret: libc::c_int = 0;
    if ((*t).root).is_null() {
        return 0 as *mut libc::c_void;
    }
    if cmp.is_none() {
        cmp = (*t).cmp;
    }
    n = (*t).root;
    idx = 0 as libc::c_int;
    ecount = -(1 as libc::c_int);
    cmpret = 0 as libc::c_int;
    if e.is_null() {
        if relation == REL234_LT as libc::c_int || relation == REL234_GT as libc::c_int
        {} else {
            __assert_fail(
                b"relation == REL234_LT || relation == REL234_GT\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                463 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[libc::c_char; 61],
                >(b"void *findrelpos234(tree234 *, void *, cmpfn234, int, int *)\0"))
                    .as_ptr(),
            );
        }
        'c_4879: {
            if relation == REL234_LT as libc::c_int
                || relation == REL234_GT as libc::c_int
            {} else {
                __assert_fail(
                    b"relation == REL234_LT || relation == REL234_GT\0" as *const u8
                        as *const libc::c_char,
                    b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                    463 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 61],
                        &[libc::c_char; 61],
                    >(b"void *findrelpos234(tree234 *, void *, cmpfn234, int, int *)\0"))
                        .as_ptr(),
                );
            }
        };
        if relation == REL234_LT as libc::c_int {
            cmpret = 1 as libc::c_int;
        } else if relation == REL234_GT as libc::c_int {
            cmpret = -(1 as libc::c_int);
        }
    }
    loop {
        kcount = 0 as libc::c_int;
        while kcount < 4 as libc::c_int {
            if kcount >= 3 as libc::c_int || ((*n).elems[kcount as usize]).is_null()
                || {
                    c = (if cmpret != 0 {
                        cmpret
                    } else {
                        cmp
                            .expect(
                                "non-null function pointer",
                            )(e, (*n).elems[kcount as usize])
                    });
                    c < 0 as libc::c_int
                }
            {
                break;
            }
            if !((*n).kids[kcount as usize]).is_null() {
                idx += (*n).counts[kcount as usize];
            }
            if c == 0 as libc::c_int {
                ecount = kcount;
                break;
            } else {
                idx += 1;
                idx;
                kcount += 1;
                kcount;
            }
        }
        if ecount >= 0 as libc::c_int {
            break;
        }
        if ((*n).kids[kcount as usize]).is_null() {
            break;
        }
        n = (*n).kids[kcount as usize];
    }
    if ecount >= 0 as libc::c_int {
        if relation != REL234_LT as libc::c_int && relation != REL234_GT as libc::c_int {
            if !index.is_null() {
                *index = idx;
            }
            return (*n).elems[ecount as usize];
        }
        if relation == REL234_LT as libc::c_int {
            idx -= 1;
            idx;
        } else {
            idx += 1;
            idx;
        }
    } else {
        if relation == REL234_EQ as libc::c_int {
            return 0 as *mut libc::c_void;
        }
        if relation == REL234_LT as libc::c_int || relation == REL234_LE as libc::c_int {
            idx -= 1;
            idx;
        }
    }
    ret = index234(t, idx);
    if !ret.is_null() && !index.is_null() {
        *index = idx;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn find234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
    mut cmp: cmpfn234,
) -> *mut libc::c_void {
    return findrelpos234(t, e, cmp, REL234_EQ as libc::c_int, 0 as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn findrel234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
    mut cmp: cmpfn234,
    mut relation: libc::c_int,
) -> *mut libc::c_void {
    return findrelpos234(t, e, cmp, relation, 0 as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn findpos234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
    mut cmp: cmpfn234,
    mut index: *mut libc::c_int,
) -> *mut libc::c_void {
    return findrelpos234(t, e, cmp, REL234_EQ as libc::c_int, index);
}
unsafe extern "C" fn trans234_subtree_right(
    mut n: *mut node234,
    mut ki: libc::c_int,
    mut k: *mut libc::c_int,
    mut index: *mut libc::c_int,
) {
    let mut src: *mut node234 = 0 as *mut node234;
    let mut dest: *mut node234 = 0 as *mut node234;
    let mut i: libc::c_int = 0;
    let mut srclen: libc::c_int = 0;
    let mut adjust: libc::c_int = 0;
    src = (*n).kids[ki as usize];
    dest = (*n).kids[(ki + 1 as libc::c_int) as usize];
    (*dest).kids[3 as libc::c_int as usize] = (*dest).kids[2 as libc::c_int as usize];
    (*dest)
        .counts[3 as libc::c_int as usize] = (*dest).counts[2 as libc::c_int as usize];
    (*dest).elems[2 as libc::c_int as usize] = (*dest).elems[1 as libc::c_int as usize];
    (*dest).kids[2 as libc::c_int as usize] = (*dest).kids[1 as libc::c_int as usize];
    (*dest)
        .counts[2 as libc::c_int as usize] = (*dest).counts[1 as libc::c_int as usize];
    (*dest).elems[1 as libc::c_int as usize] = (*dest).elems[0 as libc::c_int as usize];
    (*dest).kids[1 as libc::c_int as usize] = (*dest).kids[0 as libc::c_int as usize];
    (*dest)
        .counts[1 as libc::c_int as usize] = (*dest).counts[0 as libc::c_int as usize];
    i = if !((*src).elems[2 as libc::c_int as usize]).is_null() {
        2 as libc::c_int
    } else if !((*src).elems[1 as libc::c_int as usize]).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*dest).elems[0 as libc::c_int as usize] = (*n).elems[ki as usize];
    (*n).elems[ki as usize] = (*src).elems[i as usize];
    (*src).elems[i as usize] = 0 as *mut libc::c_void;
    (*dest)
        .kids[0 as libc::c_int as usize] = (*src).kids[(i + 1 as libc::c_int) as usize];
    (*dest)
        .counts[0 as libc::c_int
        as usize] = (*src).counts[(i + 1 as libc::c_int) as usize];
    (*src).kids[(i + 1 as libc::c_int) as usize] = 0 as *mut node234;
    (*src).counts[(i + 1 as libc::c_int) as usize] = 0 as libc::c_int;
    if !((*dest).kids[0 as libc::c_int as usize]).is_null() {
        (*(*dest).kids[0 as libc::c_int as usize]).parent = dest;
    }
    adjust = (*dest).counts[0 as libc::c_int as usize] + 1 as libc::c_int;
    (*n).counts[ki as usize] -= adjust;
    (*n).counts[(ki + 1 as libc::c_int) as usize] += adjust;
    srclen = (*n).counts[ki as usize];
    if !k.is_null() {
        if *k == ki && *index > srclen {
            *index -= srclen + 1 as libc::c_int;
            *k += 1;
            *k;
        } else if *k == ki + 1 as libc::c_int {
            *index += adjust;
        }
    }
}
unsafe extern "C" fn trans234_subtree_left(
    mut n: *mut node234,
    mut ki: libc::c_int,
    mut k: *mut libc::c_int,
    mut index: *mut libc::c_int,
) {
    let mut src: *mut node234 = 0 as *mut node234;
    let mut dest: *mut node234 = 0 as *mut node234;
    let mut i: libc::c_int = 0;
    let mut adjust: libc::c_int = 0;
    src = (*n).kids[ki as usize];
    dest = (*n).kids[(ki - 1 as libc::c_int) as usize];
    i = if !((*dest).elems[1 as libc::c_int as usize]).is_null() {
        2 as libc::c_int
    } else if !((*dest).elems[0 as libc::c_int as usize]).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*dest).elems[i as usize] = (*n).elems[(ki - 1 as libc::c_int) as usize];
    (*n)
        .elems[(ki - 1 as libc::c_int)
        as usize] = (*src).elems[0 as libc::c_int as usize];
    (*dest)
        .kids[(i + 1 as libc::c_int) as usize] = (*src).kids[0 as libc::c_int as usize];
    (*dest)
        .counts[(i + 1 as libc::c_int)
        as usize] = (*src).counts[0 as libc::c_int as usize];
    if !((*dest).kids[(i + 1 as libc::c_int) as usize]).is_null() {
        (*(*dest).kids[(i + 1 as libc::c_int) as usize]).parent = dest;
    }
    (*src).kids[0 as libc::c_int as usize] = (*src).kids[1 as libc::c_int as usize];
    (*src).counts[0 as libc::c_int as usize] = (*src).counts[1 as libc::c_int as usize];
    (*src).elems[0 as libc::c_int as usize] = (*src).elems[1 as libc::c_int as usize];
    (*src).kids[1 as libc::c_int as usize] = (*src).kids[2 as libc::c_int as usize];
    (*src).counts[1 as libc::c_int as usize] = (*src).counts[2 as libc::c_int as usize];
    (*src).elems[1 as libc::c_int as usize] = (*src).elems[2 as libc::c_int as usize];
    (*src).kids[2 as libc::c_int as usize] = (*src).kids[3 as libc::c_int as usize];
    (*src).counts[2 as libc::c_int as usize] = (*src).counts[3 as libc::c_int as usize];
    (*src).elems[2 as libc::c_int as usize] = 0 as *mut libc::c_void;
    (*src).kids[3 as libc::c_int as usize] = 0 as *mut node234;
    (*src).counts[3 as libc::c_int as usize] = 0 as libc::c_int;
    adjust = (*dest).counts[(i + 1 as libc::c_int) as usize] + 1 as libc::c_int;
    (*n).counts[ki as usize] -= adjust;
    (*n).counts[(ki - 1 as libc::c_int) as usize] += adjust;
    if !k.is_null() {
        if *k == ki {
            *index -= adjust;
            if *index < 0 as libc::c_int {
                *index
                    += (*n).counts[(ki - 1 as libc::c_int) as usize] + 1 as libc::c_int;
                *k -= 1;
                *k;
            }
        }
    }
}
unsafe extern "C" fn trans234_subtree_merge(
    mut n: *mut node234,
    mut ki: libc::c_int,
    mut k: *mut libc::c_int,
    mut index: *mut libc::c_int,
) {
    let mut left: *mut node234 = 0 as *mut node234;
    let mut right: *mut node234 = 0 as *mut node234;
    let mut i: libc::c_int = 0;
    let mut leftlen: libc::c_int = 0;
    let mut rightlen: libc::c_int = 0;
    let mut lsize: libc::c_int = 0;
    let mut rsize: libc::c_int = 0;
    left = (*n).kids[ki as usize];
    leftlen = (*n).counts[ki as usize];
    right = (*n).kids[(ki + 1 as libc::c_int) as usize];
    rightlen = (*n).counts[(ki + 1 as libc::c_int) as usize];
    if ((*left).elems[2 as libc::c_int as usize]).is_null()
        && ((*right).elems[2 as libc::c_int as usize]).is_null()
    {} else {
        __assert_fail(
            b"!left->elems[2] && !right->elems[2]\0" as *const u8 as *const libc::c_char,
            b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
            801 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void trans234_subtree_merge(node234 *, int, int *, int *)\0"))
                .as_ptr(),
        );
    }
    'c_5751: {
        if ((*left).elems[2 as libc::c_int as usize]).is_null()
            && ((*right).elems[2 as libc::c_int as usize]).is_null()
        {} else {
            __assert_fail(
                b"!left->elems[2] && !right->elems[2]\0" as *const u8
                    as *const libc::c_char,
                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                801 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 58],
                    &[libc::c_char; 58],
                >(b"void trans234_subtree_merge(node234 *, int, int *, int *)\0"))
                    .as_ptr(),
            );
        }
    };
    lsize = if !((*left).elems[1 as libc::c_int as usize]).is_null() {
        2 as libc::c_int
    } else if !((*left).elems[0 as libc::c_int as usize]).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    rsize = if !((*right).elems[1 as libc::c_int as usize]).is_null() {
        2 as libc::c_int
    } else if !((*right).elems[0 as libc::c_int as usize]).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*left).elems[lsize as usize] = (*n).elems[ki as usize];
    i = 0 as libc::c_int;
    while i < rsize + 1 as libc::c_int {
        (*left)
            .kids[(lsize + 1 as libc::c_int + i) as usize] = (*right).kids[i as usize];
        (*left)
            .counts[(lsize + 1 as libc::c_int + i)
            as usize] = (*right).counts[i as usize];
        if !((*left).kids[(lsize + 1 as libc::c_int + i) as usize]).is_null() {
            (*(*left).kids[(lsize + 1 as libc::c_int + i) as usize]).parent = left;
        }
        if i < rsize {
            (*left)
                .elems[(lsize + 1 as libc::c_int + i)
                as usize] = (*right).elems[i as usize];
        }
        i += 1;
        i;
    }
    (*n).counts[ki as usize] += rightlen + 1 as libc::c_int;
    sfree(right as *mut libc::c_void);
    i = ki + 1 as libc::c_int;
    while i < 3 as libc::c_int {
        (*n).kids[i as usize] = (*n).kids[(i + 1 as libc::c_int) as usize];
        (*n).counts[i as usize] = (*n).counts[(i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    i = ki;
    while i < 2 as libc::c_int {
        (*n).elems[i as usize] = (*n).elems[(i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    (*n).kids[3 as libc::c_int as usize] = 0 as *mut node234;
    (*n).counts[3 as libc::c_int as usize] = 0 as libc::c_int;
    (*n).elems[2 as libc::c_int as usize] = 0 as *mut libc::c_void;
    if !k.is_null() {
        if *k == ki + 1 as libc::c_int {
            *k -= 1;
            *k;
            *index += leftlen + 1 as libc::c_int;
        } else if *k > ki + 1 as libc::c_int {
            *k -= 1;
            *k;
        }
    }
}
unsafe extern "C" fn delpos234_internal(
    mut t: *mut tree234,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    let mut n: *mut node234 = 0 as *mut node234;
    let mut retval: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ki: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    retval = 0 as *mut libc::c_void;
    n = (*t).root;
    loop {
        let mut sub: *mut node234 = 0 as *mut node234;
        if index <= (*n).counts[0 as libc::c_int as usize] {
            ki = 0 as libc::c_int;
        } else {
            index -= (*n).counts[0 as libc::c_int as usize] + 1 as libc::c_int;
            if index <= (*n).counts[1 as libc::c_int as usize] {
                ki = 1 as libc::c_int;
            } else {
                index -= (*n).counts[1 as libc::c_int as usize] + 1 as libc::c_int;
                if index <= (*n).counts[2 as libc::c_int as usize] {
                    ki = 2 as libc::c_int;
                } else {
                    index -= (*n).counts[2 as libc::c_int as usize] + 1 as libc::c_int;
                    if index <= (*n).counts[3 as libc::c_int as usize] {
                        ki = 3 as libc::c_int;
                    } else {
                        __assert_fail(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                            892 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 41],
                                &[libc::c_char; 41],
                            >(b"void *delpos234_internal(tree234 *, int)\0"))
                                .as_ptr(),
                        );
                        'c_6939: {
                            __assert_fail(
                                b"0\0" as *const u8 as *const libc::c_char,
                                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                                892 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 41],
                                    &[libc::c_char; 41],
                                >(b"void *delpos234_internal(tree234 *, int)\0"))
                                    .as_ptr(),
                            );
                        };
                    }
                }
            }
        }
        if ((*n).kids[0 as libc::c_int as usize]).is_null() {
            break;
        }
        if index == (*n).counts[ki as usize] {
            let mut m: *mut node234 = 0 as *mut node234;
            if !((*n).elems[ki as usize]).is_null() {} else {
                __assert_fail(
                    b"n->elems[ki]\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                    908 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"void *delpos234_internal(tree234 *, int)\0"))
                        .as_ptr(),
                );
            }
            'c_6863: {
                if !((*n).elems[ki as usize]).is_null() {} else {
                    __assert_fail(
                        b"n->elems[ki]\0" as *const u8 as *const libc::c_char,
                        b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                        908 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"void *delpos234_internal(tree234 *, int)\0"))
                            .as_ptr(),
                    );
                }
            };
            ki += 1;
            ki;
            index = 0 as libc::c_int;
            m = (*n).kids[ki as usize];
            while !((*m).kids[0 as libc::c_int as usize]).is_null() {
                m = (*m).kids[0 as libc::c_int as usize];
            }
            retval = (*n).elems[(ki - 1 as libc::c_int) as usize];
            (*n)
                .elems[(ki - 1 as libc::c_int)
                as usize] = (*m).elems[0 as libc::c_int as usize];
        }
        sub = (*n).kids[ki as usize];
        if ((*sub).elems[1 as libc::c_int as usize]).is_null() {
            if ki > 0 as libc::c_int
                && !((*(*n).kids[(ki - 1 as libc::c_int) as usize])
                    .elems[1 as libc::c_int as usize])
                    .is_null()
            {
                trans234_subtree_right(n, ki - 1 as libc::c_int, &mut ki, &mut index);
            } else if ki < 3 as libc::c_int
                && !((*n).kids[(ki + 1 as libc::c_int) as usize]).is_null()
                && !((*(*n).kids[(ki + 1 as libc::c_int) as usize])
                    .elems[1 as libc::c_int as usize])
                    .is_null()
            {
                trans234_subtree_left(n, ki + 1 as libc::c_int, &mut ki, &mut index);
            } else {
                trans234_subtree_merge(
                    n,
                    if ki > 0 as libc::c_int { ki - 1 as libc::c_int } else { ki },
                    &mut ki,
                    &mut index,
                );
                sub = (*n).kids[ki as usize];
                if ((*n).elems[0 as libc::c_int as usize]).is_null() {
                    (*t).root = sub;
                    (*sub).parent = 0 as *mut node234;
                    sfree(n as *mut libc::c_void);
                    n = 0 as *mut node234;
                }
            }
        }
        if !n.is_null() {
            (*n).counts[ki as usize] -= 1;
            (*n).counts[ki as usize];
        }
        n = sub;
    }
    if ((*n).kids[0 as libc::c_int as usize]).is_null() {} else {
        __assert_fail(
            b"!n->kids[0]\0" as *const u8 as *const libc::c_char,
            b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
            972 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void *delpos234_internal(tree234 *, int)\0"))
                .as_ptr(),
        );
    }
    'c_5196: {
        if ((*n).kids[0 as libc::c_int as usize]).is_null() {} else {
            __assert_fail(
                b"!n->kids[0]\0" as *const u8 as *const libc::c_char,
                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                972 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void *delpos234_internal(tree234 *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if retval.is_null() {
        retval = (*n).elems[ki as usize];
    }
    i = ki;
    while i < 2 as libc::c_int
        && !((*n).elems[(i + 1 as libc::c_int) as usize]).is_null()
    {
        (*n).elems[i as usize] = (*n).elems[(i + 1 as libc::c_int) as usize];
        i += 1;
        i;
    }
    (*n).elems[i as usize] = 0 as *mut libc::c_void;
    if ((*n).elems[0 as libc::c_int as usize]).is_null() {
        if n == (*t).root {} else {
            __assert_fail(
                b"n == t->root\0" as *const u8 as *const libc::c_char,
                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                987 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void *delpos234_internal(tree234 *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_5077: {
            if n == (*t).root {} else {
                __assert_fail(
                    b"n == t->root\0" as *const u8 as *const libc::c_char,
                    b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                    987 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"void *delpos234_internal(tree234 *, int)\0"))
                        .as_ptr(),
                );
            }
        };
        sfree(n as *mut libc::c_void);
        (*t).root = 0 as *mut node234;
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn delpos234(
    mut t: *mut tree234,
    mut index: libc::c_int,
) -> *mut libc::c_void {
    if index < 0 as libc::c_int || index >= countnode234((*t).root) {
        return 0 as *mut libc::c_void;
    }
    return delpos234_internal(t, index);
}
#[no_mangle]
pub unsafe extern "C" fn del234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut index: libc::c_int = 0;
    if (findrelpos234(t, e, None, REL234_EQ as libc::c_int, &mut index)).is_null() {
        return 0 as *mut libc::c_void;
    }
    return delpos234_internal(t, index);
}
unsafe extern "C" fn join234_internal(
    mut left: *mut node234,
    mut sep: *mut libc::c_void,
    mut right: *mut node234,
    mut height: *mut libc::c_int,
) -> *mut node234 {
    let mut root: *mut node234 = 0 as *mut node234;
    let mut node: *mut node234 = 0 as *mut node234;
    let mut relht: libc::c_int = *height;
    let mut ki: libc::c_int = 0;
    if relht == 0 as libc::c_int {
        let mut newroot: *mut node234 = 0 as *mut node234;
        newroot = smalloc(::core::mem::size_of::<node234>() as libc::c_ulong)
            as *mut node234;
        (*newroot).kids[0 as libc::c_int as usize] = left;
        (*newroot).counts[0 as libc::c_int as usize] = countnode234(left);
        (*newroot).elems[0 as libc::c_int as usize] = sep;
        (*newroot).kids[1 as libc::c_int as usize] = right;
        (*newroot).counts[1 as libc::c_int as usize] = countnode234(right);
        (*newroot).elems[1 as libc::c_int as usize] = 0 as *mut libc::c_void;
        (*newroot).kids[2 as libc::c_int as usize] = 0 as *mut node234;
        (*newroot).counts[2 as libc::c_int as usize] = 0 as libc::c_int;
        (*newroot).elems[2 as libc::c_int as usize] = 0 as *mut libc::c_void;
        (*newroot).kids[3 as libc::c_int as usize] = 0 as *mut node234;
        (*newroot).counts[3 as libc::c_int as usize] = 0 as libc::c_int;
        (*newroot).parent = 0 as *mut node234;
        if !left.is_null() {
            (*left).parent = newroot;
        }
        if !right.is_null() {
            (*right).parent = newroot;
        }
        *height = 1 as libc::c_int;
        return newroot;
    }
    if relht < 0 as libc::c_int {
        root = right;
        node = root;
        loop {
            relht += 1;
            if !(relht < 0 as libc::c_int) {
                break;
            }
            node = (*node).kids[0 as libc::c_int as usize];
        }
        ki = 0 as libc::c_int;
        right = (*node).kids[ki as usize];
    } else {
        root = left;
        node = root;
        loop {
            relht -= 1;
            if !(relht > 0 as libc::c_int) {
                break;
            }
            if !((*node).elems[2 as libc::c_int as usize]).is_null() {
                node = (*node).kids[3 as libc::c_int as usize];
            } else if !((*node).elems[1 as libc::c_int as usize]).is_null() {
                node = (*node).kids[2 as libc::c_int as usize];
            } else {
                node = (*node).kids[1 as libc::c_int as usize];
            }
        }
        if !((*node).elems[2 as libc::c_int as usize]).is_null() {
            ki = 3 as libc::c_int;
        } else if !((*node).elems[1 as libc::c_int as usize]).is_null() {
            ki = 2 as libc::c_int;
        } else {
            ki = 1 as libc::c_int;
        }
        left = (*node).kids[ki as usize];
    }
    *height = add234_insert(left, sep, right, &mut root, node, ki);
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn height234(mut t: *mut tree234) -> libc::c_int {
    let mut level: libc::c_int = 0 as libc::c_int;
    let mut n: *mut node234 = (*t).root;
    while !n.is_null() {
        level += 1;
        level;
        n = (*n).kids[0 as libc::c_int as usize];
    }
    return level;
}
#[no_mangle]
pub unsafe extern "C" fn join234(
    mut t1: *mut tree234,
    mut t2: *mut tree234,
) -> *mut tree234 {
    let mut size2: libc::c_int = countnode234((*t2).root);
    if size2 > 0 as libc::c_int {
        let mut element: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut relht: libc::c_int = 0;
        if ((*t1).cmp).is_some() {
            element = index234(t2, 0 as libc::c_int);
            element = findrelpos234(
                t1,
                element,
                None,
                REL234_GE as libc::c_int,
                0 as *mut libc::c_int,
            );
            if !element.is_null() {
                return 0 as *mut tree234;
            }
        }
        element = delpos234(t2, 0 as libc::c_int);
        relht = height234(t1) - height234(t2);
        (*t1).root = join234_internal((*t1).root, element, (*t2).root, &mut relht);
        (*t2).root = 0 as *mut node234;
    }
    return t1;
}
#[no_mangle]
pub unsafe extern "C" fn join234r(
    mut t1: *mut tree234,
    mut t2: *mut tree234,
) -> *mut tree234 {
    let mut size1: libc::c_int = countnode234((*t1).root);
    if size1 > 0 as libc::c_int {
        let mut element: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut relht: libc::c_int = 0;
        if ((*t2).cmp).is_some() {
            element = index234(t1, size1 - 1 as libc::c_int);
            element = findrelpos234(
                t2,
                element,
                None,
                REL234_LE as libc::c_int,
                0 as *mut libc::c_int,
            );
            if !element.is_null() {
                return 0 as *mut tree234;
            }
        }
        element = delpos234(t1, size1 - 1 as libc::c_int);
        relht = height234(t1) - height234(t2);
        (*t2).root = join234_internal((*t1).root, element, (*t2).root, &mut relht);
        (*t1).root = 0 as *mut node234;
    }
    return t2;
}
unsafe extern "C" fn split234_internal(
    mut t: *mut tree234,
    mut index: libc::c_int,
) -> *mut node234 {
    let mut halves: [*mut node234; 2] = [0 as *mut node234, 0 as *mut node234];
    let mut n: *mut node234 = 0 as *mut node234;
    let mut sib: *mut node234 = 0 as *mut node234;
    let mut sub: *mut node234 = 0 as *mut node234;
    let mut lparent: *mut node234 = 0 as *mut node234;
    let mut rparent: *mut node234 = 0 as *mut node234;
    let mut ki: libc::c_int = 0;
    let mut pki: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut lcount: libc::c_int = 0;
    let mut rcount: libc::c_int = 0;
    n = (*t).root;
    if index == 0 as libc::c_int {
        return 0 as *mut node234;
    }
    if index == countnode234((*t).root) {
        let mut ret: *mut node234 = (*t).root;
        (*t).root = 0 as *mut node234;
        return ret;
    }
    halves[1 as libc::c_int as usize] = 0 as *mut node234;
    halves[0 as libc::c_int as usize] = halves[1 as libc::c_int as usize];
    rparent = 0 as *mut node234;
    lparent = rparent;
    pki = -(1 as libc::c_int);
    while !n.is_null() {
        lcount = index;
        rcount = countnode234(n) - lcount;
        if index <= (*n).counts[0 as libc::c_int as usize] {
            ki = 0 as libc::c_int;
        } else {
            index -= (*n).counts[0 as libc::c_int as usize] + 1 as libc::c_int;
            if index <= (*n).counts[1 as libc::c_int as usize] {
                ki = 1 as libc::c_int;
            } else {
                index -= (*n).counts[1 as libc::c_int as usize] + 1 as libc::c_int;
                if index <= (*n).counts[2 as libc::c_int as usize] {
                    ki = 2 as libc::c_int;
                } else {
                    index -= (*n).counts[2 as libc::c_int as usize] + 1 as libc::c_int;
                    ki = 3 as libc::c_int;
                }
            }
        }
        sub = (*n).kids[ki as usize];
        sib = smalloc(::core::mem::size_of::<node234>() as libc::c_ulong)
            as *mut node234;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if i + ki < 3 as libc::c_int && !((*n).elems[(i + ki) as usize]).is_null() {
                (*sib).elems[i as usize] = (*n).elems[(i + ki) as usize];
                (*sib)
                    .kids[(i + 1 as libc::c_int)
                    as usize] = (*n).kids[(i + ki + 1 as libc::c_int) as usize];
                if !((*sib).kids[(i + 1 as libc::c_int) as usize]).is_null() {
                    (*(*sib).kids[(i + 1 as libc::c_int) as usize]).parent = sib;
                }
                (*sib)
                    .counts[(i + 1 as libc::c_int)
                    as usize] = (*n).counts[(i + ki + 1 as libc::c_int) as usize];
                (*n).elems[(i + ki) as usize] = 0 as *mut libc::c_void;
                (*n).kids[(i + ki + 1 as libc::c_int) as usize] = 0 as *mut node234;
                (*n).counts[(i + ki + 1 as libc::c_int) as usize] = 0 as libc::c_int;
            } else {
                (*sib).elems[i as usize] = 0 as *mut libc::c_void;
                (*sib).kids[(i + 1 as libc::c_int) as usize] = 0 as *mut node234;
                (*sib).counts[(i + 1 as libc::c_int) as usize] = 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        if !lparent.is_null() {
            (*lparent).kids[pki as usize] = n;
            (*lparent).counts[pki as usize] = lcount;
            (*n).parent = lparent;
            (*rparent).kids[0 as libc::c_int as usize] = sib;
            (*rparent).counts[0 as libc::c_int as usize] = rcount;
            (*sib).parent = rparent;
        } else {
            halves[0 as libc::c_int as usize] = n;
            (*n).parent = 0 as *mut node234;
            halves[1 as libc::c_int as usize] = sib;
            (*sib).parent = 0 as *mut node234;
        }
        lparent = n;
        rparent = sib;
        pki = ki;
        n = sub;
    }
    if !(halves[0 as libc::c_int as usize]).is_null() {} else {
        __assert_fail(
            b"halves[0] != NULL\0" as *const u8 as *const libc::c_char,
            b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
            1269 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"node234 *split234_internal(tree234 *, int)\0"))
                .as_ptr(),
        );
    }
    'c_7722: {
        if !(halves[0 as libc::c_int as usize]).is_null() {} else {
            __assert_fail(
                b"halves[0] != NULL\0" as *const u8 as *const libc::c_char,
                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                1269 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"node234 *split234_internal(tree234 *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(halves[1 as libc::c_int as usize]).is_null() {} else {
        __assert_fail(
            b"halves[1] != NULL\0" as *const u8 as *const libc::c_char,
            b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
            1270 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"node234 *split234_internal(tree234 *, int)\0"))
                .as_ptr(),
        );
    }
    'c_7673: {
        if !(halves[1 as libc::c_int as usize]).is_null() {} else {
            __assert_fail(
                b"halves[1] != NULL\0" as *const u8 as *const libc::c_char,
                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                1270 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"node234 *split234_internal(tree234 *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*rparent).counts[0 as libc::c_int as usize] = 0 as libc::c_int;
    (*lparent).counts[pki as usize] = (*rparent).counts[0 as libc::c_int as usize];
    (*rparent).kids[0 as libc::c_int as usize] = 0 as *mut node234;
    (*lparent).kids[pki as usize] = (*rparent).kids[0 as libc::c_int as usize];
    half = 0 as libc::c_int;
    while half < 2 as libc::c_int {
        while !(halves[half as usize]).is_null()
            && ((*halves[half as usize]).elems[0 as libc::c_int as usize]).is_null()
        {
            halves[half
                as usize] = (*halves[half as usize]).kids[0 as libc::c_int as usize];
            sfree((*halves[half as usize]).parent as *mut libc::c_void);
            (*halves[half as usize]).parent = 0 as *mut node234;
        }
        n = halves[half as usize];
        while !n.is_null() {
            let mut toward: Option::<
                unsafe extern "C" fn(
                    *mut node234,
                    libc::c_int,
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> (),
            > = None;
            let mut ni: libc::c_int = 0;
            let mut merge: libc::c_int = 0;
            if half == 1 as libc::c_int {
                ki = 0 as libc::c_int;
                ni = 1 as libc::c_int;
                merge = 0 as libc::c_int;
                toward = Some(
                    trans234_subtree_left
                        as unsafe extern "C" fn(
                            *mut node234,
                            libc::c_int,
                            *mut libc::c_int,
                            *mut libc::c_int,
                        ) -> (),
                );
            } else {
                ki = if !((*n).kids[3 as libc::c_int as usize]).is_null() {
                    3 as libc::c_int
                } else if !((*n).kids[2 as libc::c_int as usize]).is_null() {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                ni = ki - 1 as libc::c_int;
                merge = ni;
                toward = Some(
                    trans234_subtree_right
                        as unsafe extern "C" fn(
                            *mut node234,
                            libc::c_int,
                            *mut libc::c_int,
                            *mut libc::c_int,
                        ) -> (),
                );
            }
            sub = (*n).kids[ki as usize];
            if !sub.is_null() && ((*sub).elems[1 as libc::c_int as usize]).is_null() {
                let mut undersized: bool = ((*sub).elems[0 as libc::c_int as usize])
                    .is_null();
                if ((*(*n).kids[ni as usize]).elems[1 as libc::c_int as usize]).is_null()
                    || undersized as libc::c_int != 0
                        && ((*(*n).kids[ni as usize]).elems[2 as libc::c_int as usize])
                            .is_null()
                {
                    trans234_subtree_merge(
                        n,
                        merge,
                        0 as *mut libc::c_int,
                        0 as *mut libc::c_int,
                    );
                    sub = (*n).kids[merge as usize];
                    if ((*n).elems[0 as libc::c_int as usize]).is_null() {
                        if ((*n).parent).is_null() {} else {
                            __assert_fail(
                                b"!n->parent\0" as *const u8 as *const libc::c_char,
                                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                                1351 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<
                                    &[u8; 43],
                                    &[libc::c_char; 43],
                                >(b"node234 *split234_internal(tree234 *, int)\0"))
                                    .as_ptr(),
                            );
                        }
                        'c_7363: {
                            if ((*n).parent).is_null() {} else {
                                __assert_fail(
                                    b"!n->parent\0" as *const u8 as *const libc::c_char,
                                    b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                                    1351 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<
                                        &[u8; 43],
                                        &[libc::c_char; 43],
                                    >(b"node234 *split234_internal(tree234 *, int)\0"))
                                        .as_ptr(),
                                );
                            }
                        };
                        halves[half as usize] = sub;
                        (*halves[half as usize]).parent = 0 as *mut node234;
                        sfree(n as *mut libc::c_void);
                    }
                } else {
                    toward
                        .expect(
                            "non-null function pointer",
                        )(n, ni, 0 as *mut libc::c_int, 0 as *mut libc::c_int);
                    if undersized {
                        toward
                            .expect(
                                "non-null function pointer",
                            )(n, ni, 0 as *mut libc::c_int, 0 as *mut libc::c_int);
                    }
                }
            }
            n = sub;
        }
        half += 1;
        half;
    }
    (*t).root = halves[1 as libc::c_int as usize];
    return halves[0 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn splitpos234(
    mut t: *mut tree234,
    mut index: libc::c_int,
    mut before: bool,
) -> *mut tree234 {
    let mut ret: *mut tree234 = 0 as *mut tree234;
    let mut n: *mut node234 = 0 as *mut node234;
    let mut count: libc::c_int = 0;
    count = countnode234((*t).root);
    if index < 0 as libc::c_int || index > count {
        return 0 as *mut tree234;
    }
    ret = newtree234((*t).cmp);
    n = split234_internal(t, index);
    if before {
        (*ret).root = n;
    } else {
        (*ret).root = (*t).root;
        (*t).root = n;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn split234(
    mut t: *mut tree234,
    mut e: *mut libc::c_void,
    mut cmp: cmpfn234,
    mut rel: libc::c_int,
) -> *mut tree234 {
    let mut before: bool = false;
    let mut index: libc::c_int = 0;
    if rel != REL234_EQ as libc::c_int {} else {
        __assert_fail(
            b"rel != REL234_EQ\0" as *const u8 as *const libc::c_char,
            b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
            1398 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 52],
                &[libc::c_char; 52],
            >(b"tree234 *split234(tree234 *, void *, cmpfn234, int)\0"))
                .as_ptr(),
        );
    }
    'c_8421: {
        if rel != REL234_EQ as libc::c_int {} else {
            __assert_fail(
                b"rel != REL234_EQ\0" as *const u8 as *const libc::c_char,
                b"/puzzles/tree234.c\0" as *const u8 as *const libc::c_char,
                1398 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 52],
                    &[libc::c_char; 52],
                >(b"tree234 *split234(tree234 *, void *, cmpfn234, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if rel == REL234_GT as libc::c_int || rel == REL234_GE as libc::c_int {
        before = 1 as libc::c_int != 0;
        rel = if rel == REL234_GT as libc::c_int {
            REL234_LE as libc::c_int
        } else {
            REL234_LT as libc::c_int
        };
    } else {
        before = 0 as libc::c_int != 0;
    }
    if (findrelpos234(t, e, cmp, rel, &mut index)).is_null() {
        index = 0 as libc::c_int;
    }
    return splitpos234(t, index + 1 as libc::c_int, before);
}
unsafe extern "C" fn copynode234(
    mut n: *mut node234,
    mut copyfn: copyfn234,
    mut copyfnstate: *mut libc::c_void,
) -> *mut node234 {
    let mut i: libc::c_int = 0;
    let mut n2: *mut node234 = smalloc(
        ::core::mem::size_of::<node234>() as libc::c_ulong,
    ) as *mut node234;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if !((*n).elems[i as usize]).is_null() && copyfn.is_some() {
            (*n2)
                .elems[i
                as usize] = copyfn
                .expect(
                    "non-null function pointer",
                )(copyfnstate, (*n).elems[i as usize]);
        } else {
            (*n2).elems[i as usize] = (*n).elems[i as usize];
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !((*n).kids[i as usize]).is_null() {
            (*n2)
                .kids[i
                as usize] = copynode234((*n).kids[i as usize], copyfn, copyfnstate);
            (*(*n2).kids[i as usize]).parent = n2;
        } else {
            (*n2).kids[i as usize] = 0 as *mut node234;
        }
        (*n2).counts[i as usize] = (*n).counts[i as usize];
        i += 1;
        i;
    }
    return n2;
}
#[no_mangle]
pub unsafe extern "C" fn copytree234(
    mut t: *mut tree234,
    mut copyfn: copyfn234,
    mut copyfnstate: *mut libc::c_void,
) -> *mut tree234 {
    let mut t2: *mut tree234 = 0 as *mut tree234;
    t2 = newtree234((*t).cmp);
    if !((*t).root).is_null() {
        (*t2).root = copynode234((*t).root, copyfn, copyfnstate);
        (*(*t2).root).parent = 0 as *mut node234;
    } else {
        (*t2).root = 0 as *mut node234;
    }
    return t2;
}
