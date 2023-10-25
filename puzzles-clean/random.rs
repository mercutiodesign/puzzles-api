use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sfree(p: *mut libc::c_void);
    fn smalloc(size: size_t) -> *mut libc::c_void;
    fn dupstr(s: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_state {
    pub seedbuf: [libc::c_uchar; 40],
    pub databuf: [libc::c_uchar; 20],
    pub pos: libc::c_int,
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
unsafe extern "C" fn SHA_Core_Init(mut h: *mut uint32) {
    *h.offset(0 as libc::c_int as isize) = 0x67452301 as libc::c_int as uint32;
    *h.offset(1 as libc::c_int as isize) = 0xefcdab89 as libc::c_uint;
    *h.offset(2 as libc::c_int as isize) = 0x98badcfe as libc::c_uint;
    *h.offset(3 as libc::c_int as isize) = 0x10325476 as libc::c_int as uint32;
    *h.offset(4 as libc::c_int as isize) = 0xc3d2e1f0 as libc::c_uint;
}
unsafe extern "C" fn SHATransform(mut digest: *mut uint32, mut block: *mut uint32) {
    let mut w: [uint32; 80] = [0; 80];
    let mut a: uint32 = 0;
    let mut b: uint32 = 0;
    let mut c: uint32 = 0;
    let mut d: uint32 = 0;
    let mut e: uint32 = 0;
    let mut t: libc::c_int = 0;
    t = 0 as libc::c_int;
    while t < 16 as libc::c_int {
        w[t as usize] = *block.offset(t as isize);
        t += 1;
        t;
    }
    t = 16 as libc::c_int;
    while t < 80 as libc::c_int {
        let mut tmp: uint32 = w[(t - 3 as libc::c_int) as usize]
            ^ w[(t - 8 as libc::c_int) as usize] ^ w[(t - 14 as libc::c_int) as usize]
            ^ w[(t - 16 as libc::c_int) as usize];
        w[t
            as usize] = tmp << 1 as libc::c_int
            | tmp >> 32 as libc::c_int - 1 as libc::c_int;
        t += 1;
        t;
    }
    a = *digest.offset(0 as libc::c_int as isize);
    b = *digest.offset(1 as libc::c_int as isize);
    c = *digest.offset(2 as libc::c_int as isize);
    d = *digest.offset(3 as libc::c_int as isize);
    e = *digest.offset(4 as libc::c_int as isize);
    t = 0 as libc::c_int;
    while t < 20 as libc::c_int {
        let mut tmp_0: uint32 = (a << 5 as libc::c_int
            | a >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(b & c | d & !b)
            .wrapping_add(e)
            .wrapping_add(w[t as usize])
            .wrapping_add(0x5a827999 as libc::c_int as uint32);
        e = d;
        d = c;
        c = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        b = a;
        a = tmp_0;
        t += 1;
        t;
    }
    t = 20 as libc::c_int;
    while t < 40 as libc::c_int {
        let mut tmp_1: uint32 = (a << 5 as libc::c_int
            | a >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(w[t as usize])
            .wrapping_add(0x6ed9eba1 as libc::c_int as uint32);
        e = d;
        d = c;
        c = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        b = a;
        a = tmp_1;
        t += 1;
        t;
    }
    t = 40 as libc::c_int;
    while t < 60 as libc::c_int {
        let mut tmp_2: uint32 = (a << 5 as libc::c_int
            | a >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(b & c | b & d | c & d)
            .wrapping_add(e)
            .wrapping_add(w[t as usize])
            .wrapping_add(0x8f1bbcdc as libc::c_uint);
        e = d;
        d = c;
        c = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        b = a;
        a = tmp_2;
        t += 1;
        t;
    }
    t = 60 as libc::c_int;
    while t < 80 as libc::c_int {
        let mut tmp_3: uint32 = (a << 5 as libc::c_int
            | a >> 32 as libc::c_int - 5 as libc::c_int)
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(w[t as usize])
            .wrapping_add(0xca62c1d6 as libc::c_uint);
        e = d;
        d = c;
        c = b << 30 as libc::c_int | b >> 32 as libc::c_int - 30 as libc::c_int;
        b = a;
        a = tmp_3;
        t += 1;
        t;
    }
    let ref mut fresh0 = *digest.offset(0 as libc::c_int as isize);
    *fresh0 = (*fresh0).wrapping_add(a);
    let ref mut fresh1 = *digest.offset(1 as libc::c_int as isize);
    *fresh1 = (*fresh1).wrapping_add(b);
    let ref mut fresh2 = *digest.offset(2 as libc::c_int as isize);
    *fresh2 = (*fresh2).wrapping_add(c);
    let ref mut fresh3 = *digest.offset(3 as libc::c_int as isize);
    *fresh3 = (*fresh3).wrapping_add(d);
    let ref mut fresh4 = *digest.offset(4 as libc::c_int as isize);
    *fresh4 = (*fresh4).wrapping_add(e);
}
#[no_mangle]
pub unsafe extern "C" fn SHA_Init(mut s: *mut SHA_State) {
    SHA_Core_Init(((*s).h).as_mut_ptr());
    (*s).blkused = 0 as libc::c_int;
    (*s).lenlo = 0 as libc::c_int as uint32;
    (*s).lenhi = (*s).lenlo;
}
#[no_mangle]
pub unsafe extern "C" fn SHA_Bytes(
    mut s: *mut SHA_State,
    mut p: *const libc::c_void,
    mut len: libc::c_int,
) {
    let mut q: *const libc::c_uchar = p as *const libc::c_uchar;
    let mut wordblock: [uint32; 16] = [0; 16];
    let mut lenw: uint32 = len as uint32;
    let mut i: libc::c_int = 0;
    (*s).lenlo = ((*s).lenlo).wrapping_add(lenw);
    (*s).lenhi = ((*s).lenhi).wrapping_add(((*s).lenlo < lenw) as libc::c_int as uint32);
    if (*s).blkused != 0 && (*s).blkused + len < 64 as libc::c_int {
        memcpy(
            ((*s).block).as_mut_ptr().offset((*s).blkused as isize) as *mut libc::c_void,
            q as *const libc::c_void,
            len as libc::c_ulong,
        );
        (*s).blkused += len;
    } else {
        while (*s).blkused + len >= 64 as libc::c_int {
            memcpy(
                ((*s).block).as_mut_ptr().offset((*s).blkused as isize)
                    as *mut libc::c_void,
                q as *const libc::c_void,
                (64 as libc::c_int - (*s).blkused) as libc::c_ulong,
            );
            q = q.offset((64 as libc::c_int - (*s).blkused) as isize);
            len -= 64 as libc::c_int - (*s).blkused;
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                wordblock[i
                    as usize] = ((*s)
                    .block[(i * 4 as libc::c_int + 0 as libc::c_int) as usize] as uint32)
                    << 24 as libc::c_int
                    | ((*s).block[(i * 4 as libc::c_int + 1 as libc::c_int) as usize]
                        as uint32) << 16 as libc::c_int
                    | ((*s).block[(i * 4 as libc::c_int + 2 as libc::c_int) as usize]
                        as uint32) << 8 as libc::c_int
                    | ((*s).block[(i * 4 as libc::c_int + 3 as libc::c_int) as usize]
                        as uint32) << 0 as libc::c_int;
                i += 1;
                i;
            }
            SHATransform(((*s).h).as_mut_ptr(), wordblock.as_mut_ptr());
            (*s).blkused = 0 as libc::c_int;
        }
        memcpy(
            ((*s).block).as_mut_ptr() as *mut libc::c_void,
            q as *const libc::c_void,
            len as libc::c_ulong,
        );
        (*s).blkused = len;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SHA_Final(
    mut s: *mut SHA_State,
    mut output: *mut libc::c_uchar,
) {
    let mut i: libc::c_int = 0;
    let mut pad: libc::c_int = 0;
    let mut c: [libc::c_uchar; 64] = [0; 64];
    let mut lenhi: uint32 = 0;
    let mut lenlo: uint32 = 0;
    if (*s).blkused >= 56 as libc::c_int {
        pad = 56 as libc::c_int + 64 as libc::c_int - (*s).blkused;
    } else {
        pad = 56 as libc::c_int - (*s).blkused;
    }
    lenhi = (*s).lenhi << 3 as libc::c_int
        | (*s).lenlo >> 32 as libc::c_int - 3 as libc::c_int;
    lenlo = (*s).lenlo << 3 as libc::c_int;
    memset(c.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int, pad as libc::c_ulong);
    c[0 as libc::c_int as usize] = 0x80 as libc::c_int as libc::c_uchar;
    SHA_Bytes(s, &mut c as *mut [libc::c_uchar; 64] as *const libc::c_void, pad);
    c[0 as libc::c_int
        as usize] = (lenhi >> 24 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    c[1 as libc::c_int
        as usize] = (lenhi >> 16 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    c[2 as libc::c_int
        as usize] = (lenhi >> 8 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    c[3 as libc::c_int
        as usize] = (lenhi >> 0 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    c[4 as libc::c_int
        as usize] = (lenlo >> 24 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    c[5 as libc::c_int
        as usize] = (lenlo >> 16 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    c[6 as libc::c_int
        as usize] = (lenlo >> 8 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    c[7 as libc::c_int
        as usize] = (lenlo >> 0 as libc::c_int & 0xff as libc::c_int as uint32)
        as libc::c_uchar;
    SHA_Bytes(
        s,
        &mut c as *mut [libc::c_uchar; 64] as *const libc::c_void,
        8 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        *output
            .offset(
                (i * 4 as libc::c_int) as isize,
            ) = ((*s).h[i as usize] >> 24 as libc::c_int & 0xff as libc::c_int as uint32)
            as libc::c_uchar;
        *output
            .offset(
                (i * 4 as libc::c_int + 1 as libc::c_int) as isize,
            ) = ((*s).h[i as usize] >> 16 as libc::c_int & 0xff as libc::c_int as uint32)
            as libc::c_uchar;
        *output
            .offset(
                (i * 4 as libc::c_int + 2 as libc::c_int) as isize,
            ) = ((*s).h[i as usize] >> 8 as libc::c_int & 0xff as libc::c_int as uint32)
            as libc::c_uchar;
        *output
            .offset(
                (i * 4 as libc::c_int + 3 as libc::c_int) as isize,
            ) = ((*s).h[i as usize] & 0xff as libc::c_int as uint32) as libc::c_uchar;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SHA_Simple(
    mut p: *const libc::c_void,
    mut len: libc::c_int,
    mut output: *mut libc::c_uchar,
) {
    let mut s: SHA_State = SHA_State {
        h: [0; 5],
        block: [0; 64],
        blkused: 0,
        lenhi: 0,
        lenlo: 0,
    };
    SHA_Init(&mut s);
    SHA_Bytes(&mut s, p, len);
    SHA_Final(&mut s, output);
}
#[no_mangle]
pub unsafe extern "C" fn random_new(
    mut seed: *const libc::c_char,
    mut len: libc::c_int,
) -> *mut random_state {
    let mut state: *mut random_state = 0 as *mut random_state;
    state = smalloc(::core::mem::size_of::<random_state>() as libc::c_ulong)
        as *mut random_state;
    SHA_Simple(seed as *const libc::c_void, len, ((*state).seedbuf).as_mut_ptr());
    SHA_Simple(
        ((*state).seedbuf).as_mut_ptr() as *const libc::c_void,
        20 as libc::c_int,
        ((*state).seedbuf).as_mut_ptr().offset(20 as libc::c_int as isize),
    );
    SHA_Simple(
        ((*state).seedbuf).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int,
        ((*state).databuf).as_mut_ptr(),
    );
    (*state).pos = 0 as libc::c_int;
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn random_copy(
    mut tocopy: *mut random_state,
) -> *mut random_state {
    let mut result: *mut random_state = 0 as *mut random_state;
    result = smalloc(::core::mem::size_of::<random_state>() as libc::c_ulong)
        as *mut random_state;
    memcpy(
        ((*result).seedbuf).as_mut_ptr() as *mut libc::c_void,
        ((*tocopy).seedbuf).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong,
    );
    memcpy(
        ((*result).databuf).as_mut_ptr() as *mut libc::c_void,
        ((*tocopy).databuf).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
    );
    (*result).pos = (*tocopy).pos;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn random_bits(
    mut state: *mut random_state,
    mut bits: libc::c_int,
) -> libc::c_ulong {
    let mut ret: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < bits {
        if (*state).pos >= 20 as libc::c_int {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < 20 as libc::c_int {
                if (*state).seedbuf[i as usize] as libc::c_int != 0xff as libc::c_int {
                    (*state)
                        .seedbuf[i
                        as usize] = ((*state).seedbuf[i as usize]).wrapping_add(1);
                    (*state).seedbuf[i as usize];
                    break;
                } else {
                    (*state).seedbuf[i as usize] = 0 as libc::c_int as libc::c_uchar;
                    i += 1;
                    i;
                }
            }
            SHA_Simple(
                ((*state).seedbuf).as_mut_ptr() as *const libc::c_void,
                40 as libc::c_int,
                ((*state).databuf).as_mut_ptr(),
            );
            (*state).pos = 0 as libc::c_int;
        }
        let fresh5 = (*state).pos;
        (*state).pos = (*state).pos + 1;
        ret = ret << 8 as libc::c_int
            | (*state).databuf[fresh5 as usize] as libc::c_ulong;
        n += 8 as libc::c_int;
    }
    ret
        &= ((1 as libc::c_ulong) << bits - 1 as libc::c_int)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn random_upto(
    mut state: *mut random_state,
    mut limit: libc::c_ulong,
) -> libc::c_ulong {
    let mut bits: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_ulong = 0;
    let mut divisor: libc::c_ulong = 0;
    let mut data: libc::c_ulong = 0;
    while limit >> bits != 0 as libc::c_int as libc::c_ulong {
        bits += 1;
        bits;
    }
    bits += 3 as libc::c_int;
    if bits < 32 as libc::c_int {} else {
        __assert_fail(
            b"bits < 32\0" as *const u8 as *const libc::c_char,
            b"/puzzles/random.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[libc::c_char; 57],
            >(b"unsigned long random_upto(random_state *, unsigned long)\0"))
                .as_ptr(),
        );
    }
    'c_4612: {
        if bits < 32 as libc::c_int {} else {
            __assert_fail(
                b"bits < 32\0" as *const u8 as *const libc::c_char,
                b"/puzzles/random.c\0" as *const u8 as *const libc::c_char,
                275 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[libc::c_char; 57],
                >(b"unsigned long random_upto(random_state *, unsigned long)\0"))
                    .as_ptr(),
            );
        }
    };
    max = ((1 as libc::c_long) << bits) as libc::c_ulong;
    divisor = max.wrapping_div(limit);
    max = limit.wrapping_mul(divisor);
    loop {
        data = random_bits(state, bits);
        if !(data >= max) {
            break;
        }
    }
    return data.wrapping_div(divisor);
}
#[no_mangle]
pub unsafe extern "C" fn random_free(mut state: *mut random_state) {
    sfree(state as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn random_state_encode(
    mut state: *mut random_state,
) -> *mut libc::c_char {
    let mut retbuf: [libc::c_char; 256] = [0; 256];
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
    {
        len
            += sprintf(
                retbuf.as_mut_ptr().offset(len as isize),
                b"%02x\0" as *const u8 as *const libc::c_char,
                (*state).seedbuf[i as usize] as libc::c_int,
            );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
    {
        len
            += sprintf(
                retbuf.as_mut_ptr().offset(len as isize),
                b"%02x\0" as *const u8 as *const libc::c_char,
                (*state).databuf[i as usize] as libc::c_int,
            );
        i += 1;
        i;
    }
    len
        += sprintf(
            retbuf.as_mut_ptr().offset(len as isize),
            b"%02x\0" as *const u8 as *const libc::c_char,
            (*state).pos,
        );
    return dupstr(retbuf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn random_state_decode(
    mut input: *const libc::c_char,
) -> *mut random_state {
    let mut state: *mut random_state = 0 as *mut random_state;
    let mut pos: libc::c_int = 0;
    let mut byte: libc::c_int = 0;
    let mut digits: libc::c_int = 0;
    state = smalloc(::core::mem::size_of::<random_state>() as libc::c_ulong)
        as *mut random_state;
    memset(
        ((*state).seedbuf).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong,
    );
    memset(
        ((*state).databuf).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
    );
    (*state).pos = 0 as libc::c_int;
    digits = 0 as libc::c_int;
    byte = digits;
    pos = 0 as libc::c_int;
    while *input != 0 {
        let fresh6 = input;
        input = input.offset(1);
        let mut v: libc::c_int = *fresh6 as libc::c_int;
        if v >= '0' as i32 && v <= '9' as i32 {
            v = v - '0' as i32;
        } else if v >= 'A' as i32 && v <= 'F' as i32 {
            v = v - 'A' as i32 + 10 as libc::c_int;
        } else if v >= 'a' as i32 && v <= 'f' as i32 {
            v = v - 'a' as i32 + 10 as libc::c_int;
        } else {
            v = 0 as libc::c_int;
        }
        byte = byte << 4 as libc::c_int | v;
        digits += 1;
        digits;
        if digits == 2 as libc::c_int {
            if (pos as libc::c_ulong)
                < (::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    )
            {
                let fresh7 = pos;
                pos = pos + 1;
                (*state).seedbuf[fresh7 as usize] = byte as libc::c_uchar;
            } else if (pos as libc::c_ulong)
                < (::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                            ),
                    )
            {
                let fresh8 = pos;
                pos = pos + 1;
                (*state)
                    .databuf[(fresh8 as libc::c_ulong)
                    .wrapping_sub(
                        (::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                            ),
                    ) as usize] = byte as libc::c_uchar;
            } else if pos as libc::c_ulong
                == (::core::mem::size_of::<[libc::c_uchar; 40]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    )
                    .wrapping_add(
                        (::core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                            ),
                    )
                && byte as libc::c_ulong
                    <= (::core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        )
            {
                (*state).pos = byte;
            }
            digits = 0 as libc::c_int;
            byte = digits;
        }
    }
    return state;
}
