use std::{fs::File, os::unix::prelude::FileExt, path::Path};

// ref https://github.com/kekeimiku/parse_proc_maps
#[derive(Debug)]
pub struct Maps<'a> {
    pub start: usize,
    pub end: usize,
    pub flags: &'a str,
    pub offset: usize,
    pub dev: &'a str,
    pub inode: usize,
    pub pathname: &'a str,
}

impl Maps<'_> {
    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }
    pub fn size(&self) -> usize {
        self.end - self.start
    }
    pub fn is_exec(&self) -> bool {
        &self.flags[2..3] == "x"
    }
    pub fn is_read(&self) -> bool {
        &self.flags[0..1] == "r"
    }
    pub fn is_write(&self) -> bool {
        &self.flags[1..2] == "w"
    }
    pub fn pathname(&self) -> &str {
        self.pathname
    }
}

pub struct MapsIter<'a>(core::str::Lines<'a>);

impl<'a> MapsIter<'a> {
    pub fn new(contents: &'a str) -> Self {
        Self(contents.lines())
    }
}

impl<'a> Iterator for MapsIter<'a> {
    type Item = Maps<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.0.next()?;
        let mut split = line.splitn(6, ' ');
        let mut range_split = split.next()?.split('-');
        let start = usize::from_str_radix(range_split.next()?, 16).unwrap();
        let end = usize::from_str_radix(range_split.next()?, 16).unwrap();
        let flags = split.next()?;
        let offset = usize::from_str_radix(split.next()?, 16).unwrap();
        let dev = split.next()?;
        let inode = split.next()?.parse::<usize>().unwrap();
        let pathname = split.next()?.trim_start();

        Some(Maps { start, end, flags, offset, dev, inode, pathname })
    }
}

fn main() {}

#[link(name = "c")]
extern "C" {
    fn __mprotect(addr: *mut core::ffi::c_void, len: usize, prot: i32) -> i32;
}

pub fn read_bytes(pid: u32, offset: u64, size: usize) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(&Path::new(&format!("/proc/{}/mem", pid)))?;
    let mut buffer = vec![0; size];
    file.read_at(&mut buffer, offset)?;
    Ok(buffer)
}

pub fn write_bytes(pid: u32, offset: u64, buffer: &[u8]) -> Result<usize, std::io::Error> {
    let file = File::open(&Path::new(&format!("/proc/{}/mem", pid)))?;
    file.write_at(buffer, offset)?;
    Ok(buffer.len())
}
