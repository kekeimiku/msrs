use windows_sys::Win32::{
    Graphics::Gdi::{BitBlt, GetDC, ReleaseDC, SRCCOPY},
    UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN},
};

fn main() {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        let (mut x, mut y, mut w, mut h);

        loop {
            let hdc = GetDC(0);

            x = fastrand::i32(1..3000) % (width * 5 / 4) - width / 4;
            y = fastrand::i32(1..3000) % (height * 5 / 4) - height / 4;
            w = fastrand::i32(1..3000) % width / 2;
            h = fastrand::i32(1..3000) % height / 2;

            BitBlt(
                hdc,
                x + fastrand::i32(1..3000) % 3 - 1,
                y + fastrand::i32(1..3000) % 5,
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
