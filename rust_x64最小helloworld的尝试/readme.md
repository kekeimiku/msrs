### asm版：327字节
```rust
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
            in("rdx") h.len() as u64
        );
        //exit
        asm!(
            "syscall",
            in("rax") 60
        );
    };
}
```

```shell
cargo build --release --target x86_64-unknown-linux-gnu
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld", "-C", "link-arg=-nostdlib", "-C", "link-arg=-static", "-C", "link-arg=-Wl,--build-id=none"]
llvm-strip --strip-sections hello
ll rust-helloworld
327 byte     rust-helloworld
```

### libc版：552字节
```rust
#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::panicinfo) -> ! {
    loop {}
}

#[link(name = "c")]
extern "c" {
    fn write(fd: i32, buf: *const i8, count: usize) -> isize;
}

#[no_mangle]
fn main() -> isize {
    unsafe { write(1, b"hello, world!\n" as *const u8 as *const i8, 14) };
    0
}
```
