#![no_std]
#![no_main]

use core::arch::asm;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn _start() {
    let h: &str = "Hello, World!\n";
    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") 1,
            in("rsi") h.as_ptr() as u64,
            in("rdx") h.len() as u64,
            options(nostack)
        );
        //exit(0)
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") 0,
            options(noreturn)
        );
    };
}
