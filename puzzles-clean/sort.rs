use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type arraysort_cmpfn_t = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        *const libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
unsafe extern "C" fn memswap(
    mut av: *mut libc::c_void,
    mut bv: *mut libc::c_void,
    mut size: size_t,
) {
    let mut t: [libc::c_char; 4096] = [0; 4096];
    let mut a: *mut libc::c_char = av as *mut libc::c_char;
    let mut b: *mut libc::c_char = bv as *mut libc::c_char;
    while size > 0 as libc::c_int as size_t {
        let mut thissize: size_t = if size
            < ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        {
            size
        } else {
            ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        };
        memcpy(t.as_mut_ptr() as *mut libc::c_void, a as *const libc::c_void, thissize);
        memcpy(a as *mut libc::c_void, b as *const libc::c_void, thissize);
        memcpy(b as *mut libc::c_void, t.as_mut_ptr() as *const libc::c_void, thissize);
        size = size.wrapping_sub(thissize);
        a = a.offset(thissize as isize);
        b = b.offset(thissize as isize);
    }
}
unsafe extern "C" fn downheap(
    mut array: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmp: arraysort_cmpfn_t,
    mut ctx: *mut libc::c_void,
    mut i: size_t,
) {
    while (2 as libc::c_int as size_t * i).wrapping_add(1 as libc::c_int as size_t)
        < nmemb
    {
        let mut j: size_t = i;
        if cmp
            .expect(
                "non-null function pointer",
            )(
            (array as *mut libc::c_char).offset((size * j) as isize)
                as *const libc::c_void,
            (array as *mut libc::c_char)
                .offset(
                    (size
                        * (2 as libc::c_int as size_t * i)
                            .wrapping_add(1 as libc::c_int as size_t)) as isize,
                ) as *const libc::c_void,
            ctx,
        ) < 0 as libc::c_int
        {
            j = (2 as libc::c_int as size_t * i)
                .wrapping_add(1 as libc::c_int as size_t);
        }
        if (2 as libc::c_int as size_t * i).wrapping_add(2 as libc::c_int as size_t)
            < nmemb
            && cmp
                .expect(
                    "non-null function pointer",
                )(
                (array as *mut libc::c_char).offset((size * j) as isize)
                    as *const libc::c_void,
                (array as *mut libc::c_char)
                    .offset(
                        (size
                            * (2 as libc::c_int as size_t * i)
                                .wrapping_add(2 as libc::c_int as size_t)) as isize,
                    ) as *const libc::c_void,
                ctx,
            ) < 0 as libc::c_int
        {
            j = (2 as libc::c_int as size_t * i)
                .wrapping_add(2 as libc::c_int as size_t);
        }
        if j == i {
            return;
        }
        memswap(
            (array as *mut libc::c_char).offset((size * j) as isize)
                as *mut libc::c_void,
            (array as *mut libc::c_char).offset((size * i) as isize)
                as *mut libc::c_void,
            size,
        );
        i = j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn arraysort_fn(
    mut array: *mut libc::c_void,
    mut nmemb: size_t,
    mut size: size_t,
    mut cmp: arraysort_cmpfn_t,
    mut ctx: *mut libc::c_void,
) {
    let mut i: size_t = 0;
    if nmemb < 2 as libc::c_int as size_t {
        return;
    }
    i = (nmemb
        .wrapping_sub(1 as libc::c_int as size_t)
        .wrapping_sub(1 as libc::c_int as size_t) / 2 as libc::c_int as size_t)
        .wrapping_add(1 as libc::c_int as size_t);
    loop {
        let fresh0 = i;
        i = i.wrapping_sub(1);
        if !(fresh0 > 0 as libc::c_int as size_t) {
            break;
        }
        downheap(array, nmemb, size, cmp, ctx, i);
    }
    i = nmemb.wrapping_sub(1 as libc::c_int as size_t);
    while i > 0 as libc::c_int as size_t {
        memswap(
            (array as *mut libc::c_char)
                .offset((size * 0 as libc::c_int as size_t) as isize)
                as *mut libc::c_void,
            (array as *mut libc::c_char).offset((size * i) as isize)
                as *mut libc::c_void,
            size,
        );
        downheap(array, i, size, cmp, ctx, 0 as libc::c_int as size_t);
        i = i.wrapping_sub(1);
        i;
    }
}
