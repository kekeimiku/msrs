#![feature(lang_items)]
#![no_std]
use core::{mem::transmute, panic::PanicInfo, ffi::c_void};
use windows_sys::Win32::{
    Foundation::{BOOL, HINSTANCE, LPARAM, LRESULT, WPARAM},
    System::{SystemServices::DLL_PROCESS_ATTACH, Threading::GetCurrentThreadId},
    UI::WindowsAndMessaging::{
        CallNextHookEx, MessageBoxA, SetWindowsHookExW, HHOOK, MB_OK, MSG, WH_GETMESSAGE,
        WM_COMMAND,
    },
};

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "stdcall" fn _DllMainCRTStartup(
    _h_inst_dll: HINSTANCE,
    call_reason: u32,
    _lpv_reserved: c_void,
) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => unsafe {
            SetWindowsHookExW(
                WH_GETMESSAGE,
                Some(callback),
                _h_inst_dll,
                GetCurrentThreadId(),
            );
        },
        _ => (),
    }

    1
}

unsafe extern "system" fn callback(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let msg = &*(l_param as *const MSG);
    if n_code >= 0 {
        if msg.message == WM_COMMAND && msg.wParam == 97 {
            MessageBoxA(
                0,
                transmute(b"hello tool 1\0".as_ptr()),
                transmute(b"title\0".as_ptr()),
                MB_OK,
            );
        }
    }

    CallNextHookEx(HHOOK::default(), n_code, w_param, l_param)
}

