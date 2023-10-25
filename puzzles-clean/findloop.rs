#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn sfree(p: *mut libc::c_void);
    fn smalloc(size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct findloopstate {
    pub parent: libc::c_int,
    pub child: libc::c_int,
    pub sibling: libc::c_int,
    pub component_root: libc::c_int,
    pub visited: bool,
    pub index: libc::c_int,
    pub minindex: libc::c_int,
    pub maxindex: libc::c_int,
    pub minreachable: libc::c_int,
    pub maxreachable: libc::c_int,
    pub bridge: libc::c_int,
}
pub type neighbour_fn_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn findloop_new_state(
    mut nvertices: libc::c_int,
) -> *mut findloopstate {
    return smalloc(
        ((nvertices + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<findloopstate>() as libc::c_ulong),
    ) as *mut findloopstate;
}
#[no_mangle]
pub unsafe extern "C" fn findloop_free_state(mut state: *mut findloopstate) {
    sfree(state as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn findloop_is_loop_edge(
    mut pv: *mut findloopstate,
    mut u: libc::c_int,
    mut v: libc::c_int,
) -> bool {
    return !((*pv.offset(u as isize)).bridge == v
        || (*pv.offset(v as isize)).bridge == u);
}
unsafe extern "C" fn findloop_is_bridge_oneway(
    mut pv: *mut findloopstate,
    mut u: libc::c_int,
    mut v: libc::c_int,
    mut u_vertices: *mut libc::c_int,
    mut v_vertices: *mut libc::c_int,
) -> bool {
    let mut r: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut below: libc::c_int = 0;
    if (*pv.offset(u as isize)).bridge != v {
        return 0 as libc::c_int != 0;
    }
    r = (*pv.offset(u as isize)).component_root;
    total = (*pv.offset(r as isize)).maxindex - (*pv.offset(r as isize)).minindex
        + 1 as libc::c_int;
    below = (*pv.offset(u as isize)).maxindex - (*pv.offset(u as isize)).minindex
        + 1 as libc::c_int;
    if !u_vertices.is_null() {
        *u_vertices = below;
    }
    if !v_vertices.is_null() {
        *v_vertices = total - below;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn findloop_is_bridge(
    mut pv: *mut findloopstate,
    mut u: libc::c_int,
    mut v: libc::c_int,
    mut u_vertices: *mut libc::c_int,
    mut v_vertices: *mut libc::c_int,
) -> bool {
    return findloop_is_bridge_oneway(pv, u, v, u_vertices, v_vertices) as libc::c_int
        != 0
        || findloop_is_bridge_oneway(pv, v, u, v_vertices, u_vertices) as libc::c_int
            != 0;
}
#[no_mangle]
pub unsafe extern "C" fn findloop_run(
    mut pv: *mut findloopstate,
    mut nvertices: libc::c_int,
    mut neighbour: neighbour_fn_t,
    mut ctx: *mut libc::c_void,
) -> bool {
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut root: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut nbridges: libc::c_int = 0;
    let mut nedges: libc::c_int = 0;
    root = nvertices;
    v = 0 as libc::c_int;
    while v <= nvertices {
        (*pv.offset(v as isize)).parent = root;
        (*pv.offset(v as isize)).child = -(2 as libc::c_int);
        (*pv.offset(v as isize)).sibling = -(1 as libc::c_int);
        (*pv.offset(v as isize)).visited = 0 as libc::c_int != 0;
        v += 1;
        v;
    }
    (*pv.offset(root as isize)).child = -(1 as libc::c_int);
    nedges = 0 as libc::c_int;
    v = 0 as libc::c_int;
    while v < nvertices {
        if (*pv.offset(v as isize)).parent == root {
            (*pv.offset(v as isize)).sibling = (*pv.offset(root as isize)).child;
            (*pv.offset(root as isize)).child = v;
            (*pv.offset(v as isize)).component_root = v;
            u = v;
            loop {
                if !(*pv.offset(u as isize)).visited {
                    (*pv.offset(u as isize)).visited = 1 as libc::c_int != 0;
                    w = neighbour.expect("non-null function pointer")(u, ctx);
                    while w >= 0 as libc::c_int {
                        if (*pv.offset(w as isize)).child == -(2 as libc::c_int) {
                            (*pv.offset(w as isize)).child = -(1 as libc::c_int);
                            (*pv.offset(w as isize))
                                .sibling = (*pv.offset(u as isize)).child;
                            (*pv.offset(w as isize)).parent = u;
                            (*pv.offset(w as isize))
                                .component_root = (*pv.offset(u as isize)).component_root;
                            (*pv.offset(u as isize)).child = w;
                        }
                        if w > u {
                            nedges += 1;
                            nedges;
                        }
                        w = neighbour
                            .expect(
                                "non-null function pointer",
                            )(-(1 as libc::c_int), ctx);
                    }
                    if (*pv.offset(u as isize)).child >= 0 as libc::c_int {
                        u = (*pv.offset(u as isize)).child;
                        continue;
                    }
                }
                if u == v {
                    break;
                }
                if (*pv.offset(u as isize)).sibling >= 0 as libc::c_int {
                    u = (*pv.offset(u as isize)).sibling;
                } else {
                    u = (*pv.offset(u as isize)).parent;
                }
            }
        }
        v += 1;
        v;
    }
    index = 0 as libc::c_int;
    v = 0 as libc::c_int;
    while v < nvertices {
        (*pv.offset(v as isize)).visited = 0 as libc::c_int != 0;
        v += 1;
        v;
    }
    (*pv.offset(root as isize)).visited = 1 as libc::c_int != 0;
    u = (*pv.offset(root as isize)).child;
    loop {
        if !(*pv.offset(u as isize)).visited {
            (*pv.offset(u as isize)).visited = 1 as libc::c_int != 0;
            let ref mut fresh0 = (*pv.offset(u as isize)).index;
            *fresh0 = index;
            (*pv.offset(u as isize)).minindex = *fresh0;
            index += 1;
            index;
            if (*pv.offset(u as isize)).child >= 0 as libc::c_int {
                u = (*pv.offset(u as isize)).child;
                continue;
            }
        }
        if u == root {
            break;
        }
        (*pv.offset(u as isize)).maxindex = index - 1 as libc::c_int;
        if (*pv.offset(u as isize)).sibling >= 0 as libc::c_int {
            u = (*pv.offset(u as isize)).sibling;
        } else {
            u = (*pv.offset(u as isize)).parent;
        }
    }
    v = 0 as libc::c_int;
    while v < nvertices {
        (*pv.offset(v as isize)).bridge = -(1 as libc::c_int);
        v += 1;
        v;
    }
    nbridges = 0 as libc::c_int;
    v = 0 as libc::c_int;
    while v < nvertices {
        (*pv.offset(v as isize)).visited = 0 as libc::c_int != 0;
        v += 1;
        v;
    }
    u = (*pv.offset(root as isize)).child;
    (*pv.offset(root as isize)).visited = 1 as libc::c_int != 0;
    loop {
        if !(*pv.offset(u as isize)).visited {
            (*pv.offset(u as isize)).visited = 1 as libc::c_int != 0;
            let ref mut fresh1 = (*pv.offset(u as isize)).maxreachable;
            *fresh1 = (*pv.offset(u as isize)).minindex;
            (*pv.offset(u as isize)).minreachable = *fresh1;
            w = neighbour.expect("non-null function pointer")(u, ctx);
            while w >= 0 as libc::c_int {
                if w != (*pv.offset(u as isize)).parent {
                    let mut i: libc::c_int = (*pv.offset(w as isize)).index;
                    if (*pv.offset(u as isize)).minreachable > i {
                        (*pv.offset(u as isize)).minreachable = i;
                    }
                    if (*pv.offset(u as isize)).maxreachable < i {
                        (*pv.offset(u as isize)).maxreachable = i;
                    }
                }
                w = neighbour
                    .expect("non-null function pointer")(-(1 as libc::c_int), ctx);
            }
            if (*pv.offset(u as isize)).child >= 0 as libc::c_int {
                u = (*pv.offset(u as isize)).child;
                continue;
            }
        }
        if u == root {
            break;
        }
        v = (*pv.offset(u as isize)).child;
        while v >= 0 as libc::c_int {
            if (*pv.offset(u as isize)).minreachable
                > (*pv.offset(v as isize)).minreachable
            {
                (*pv.offset(u as isize))
                    .minreachable = (*pv.offset(v as isize)).minreachable;
            }
            if (*pv.offset(u as isize)).maxreachable
                < (*pv.offset(v as isize)).maxreachable
            {
                (*pv.offset(u as isize))
                    .maxreachable = (*pv.offset(v as isize)).maxreachable;
            }
            v = (*pv.offset(v as isize)).sibling;
        }
        v = (*pv.offset(u as isize)).parent;
        if v != root {
            if (*pv.offset(u as isize)).minreachable >= (*pv.offset(u as isize)).minindex
                && (*pv.offset(u as isize)).maxreachable
                    <= (*pv.offset(u as isize)).maxindex
            {
                (*pv.offset(u as isize)).bridge = v;
                nbridges += 1;
                nbridges;
            }
        }
        if (*pv.offset(u as isize)).sibling >= 0 as libc::c_int {
            u = (*pv.offset(u as isize)).sibling;
        } else {
            u = (*pv.offset(u as isize)).parent;
        }
    }
    return nbridges < nedges;
}
