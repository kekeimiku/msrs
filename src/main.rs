use std::{thread, time::Duration};

use rand::Rng;
use windows_sys::Win32::{
    Graphics::Gdi::{BitBlt, GetDC, ReleaseDC, SRCCOPY},
    UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN},
};

fn main() {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        let (mut x, mut y, mut w, mut h);
        let mut rng = rand::thread_rng();

        loop {
            thread::sleep(Duration::from_millis(10));
            let hdc = GetDC(0);

            x = rng.gen::<i32>() % (width * 5 / 4) - width / 4;
            y = rng.gen::<i32>() % (height * 5 / 4) - height / 4;
            w = rng.gen::<i32>() % width / 2;
            h = rng.gen::<i32>() % height / 2;

            BitBlt(
                hdc,
                x + rng.gen::<i32>() % 3 - 1,
                y + rng.gen::<i32>() % 5,
                w,
                h,
                hdc,
                x,
                y,
                SRCCOPY,
            );

            ReleaseDC(0,hdc);
        }
    }
}
