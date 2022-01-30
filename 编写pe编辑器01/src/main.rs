use std::{
    env, fs,
    mem::{self, size_of},
};
use windows_sys::Win32::System::{
    Diagnostics::Debug::{IMAGE_FILE_HEADER, IMAGE_OPTIONAL_HEADER32},
    SystemServices::IMAGE_DOS_HEADER,
};

fn main() {
    let args = env::args().nth(1).unwrap();

    let file = fs::read(args).unwrap();

    let dos_header =
        unsafe { &*mem::transmute::<*const u8, *const IMAGE_DOS_HEADER>(file.as_ptr()) };

    println!("{:x}", dos_header.e_magic);

    let file_header = unsafe {
        &*mem::transmute::<*const u8, *const IMAGE_FILE_HEADER>(
            file.as_ptr().offset((dos_header.e_lfanew + 4) as isize),
        )
    };

    println!("{:x}", file_header.NumberOfSections);

    let option_header =
        unsafe {
            &*mem::transmute::<*const u8, *const IMAGE_OPTIONAL_HEADER32>(file.as_ptr().offset(
                (dos_header.e_lfanew + 4) as isize + size_of::<IMAGE_FILE_HEADER>() as isize,
            ))
        };

    println!("{:x}", option_header.SizeOfCode);
}
