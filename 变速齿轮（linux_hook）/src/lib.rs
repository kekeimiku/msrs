#![feature(lang_items)]
#![no_std]
#![allow(non_camel_case_types)]

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link(name = "c")]
extern "C" {
    fn __gettimeofday(tp: *mut timeval, tz: *mut timezone) -> i32;
    fn __clock_gettime(clk_id: i32, tp: *mut timespec) -> i32;
}

pub enum timezone {}

#[repr(C)]
#[derive(Default)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Default)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clk_id: i32, tp: *mut timespec) -> i32 {
    let mut t = timespec {
        ..Default::default()
    };

    let ok = __clock_gettime(clk_id, &mut t);

    *tp = timespec {
        tv_sec: t.tv_sec * 5,
        tv_nsec: t.tv_nsec * 5,
    };

    ok
}

#[no_mangle]
pub unsafe extern "C" fn gettimeofday(tv: *mut timeval, tz: *mut timezone) -> i32 {
    let mut t = timeval {
        ..Default::default()
    };

    let ok = __gettimeofday(&mut t, tz);

    *tv = timeval {
        tv_sec: t.tv_sec * 5,
        tv_usec: t.tv_usec * 5,
    };

    ok
}
