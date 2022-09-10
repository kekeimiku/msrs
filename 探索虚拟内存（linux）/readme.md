
# TODO

## 预备知识

在计算机领域， 虚拟内存是通过软硬件结合实现的一种内存管理技术， 它将程序所使用的内存地址（虚拟内存地址）映射到计算机的物理内存上（物理内存地址），这使得每个程序看到的内存地址空间都是连续的（或是一些连续地址空间的集合）。操作系统管理虚拟地址空间， 以及虚拟地址空间到物理内存的映射。CPU中的地址转换硬件(通常被称为内存管理单元, MMU)自动将虚拟内存地址转换成物理内存地址。操作系统可以提供比实际物理内存更多的虚拟内存，这一行为是通过操作系统中的软件来实现的。

关于虚拟内存的具体解释可以看 [维基百科-Virtual memory](https://en.wikipedia.org/wiki/Virtual_memory)


## 一个简单的rust程序

```rust
use std::{process, thread, time::Duration};

fn main() {
    let hello = "hello";
    loop {
        thread::sleep(Duration::from_millis(1000));
        println!("{} addr:{:p} pid:{}", hello, hello , process::id());
    }
}
```

这个程序每秒打印 一次 `hello字符串 s的地址和当前程序的pid`

运行结果类似于

```
hello addr:0x7fb5a2956000 pid:7951
hello addr:0x7fb5a2956000 pid:7951
hello addr:0x7fb5a2956000 pid:7951
```

## proc文件系统

`/proc`目录包含一系列文件，我们只关注其中的三个：

`/proc/[pid]/mem `

该文件可用于访问进程内存的页面。

`/proc/[pid]/maps`

该文件包含当前映射的内存区域及其访问权限的文件。

`/proc/[pid]/comm`

该文件内容为当前进程的名称。

读取 `/proc/[pid]maps` 我们可以得到类似如下内容

```
55555613d000-55555613e000 rw-p 00000000 00:00 0                          [heap]
7fb5a2904000-7fb5a2905000 ---p 00000000 00:00 0
7fb5a2905000-7fb5a2907000 rw-p 00000000 00:00 0
7fb5a2907000-7fb5a290c000 r--p 00000000 103:02 3408451                   /home/keke/hello/target/x86_64-unknown-linux-musl/release/hello
7fb5a290c000-7fb5a2956000 r-xp 00005000 103:02 3408451                   /home/keke/hello/target/x86_64-unknown-linux-musl/release/hello
7fb5a2956000-7fb5a2961000 r--p 0004f000 103:02 3408451                   /home/keke/hello/target/x86_64-unknown-linux-musl/release/hello
7fb5a2961000-7fb5a2962000 rw-p 00000000 00:00 0
7fb5a2962000-7fb5a296c000 rw-p 0005a000 103:02 3408451                   /home/keke/hello/target/x86_64-unknown-linux-musl/release/hello
7fb5a296c000-7fb5a296d000 rw-p 00000000 00:00 0
7fffb1513000-7fffb1534000 rw-p 00000000 00:00 0                          [stack]
7fffb1534000-7fffb1538000 r--p 00000000 00:00 0                          [vvar]
7fffb1538000-7fffb153a000 r-xp 00000000 00:00 0                          [vdso]
ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  [vsyscall]
```

我们可以发现 变量 `hello 0x7fb5a2956000` 的地址位于 

```
7fb5a2956000-7fb5a2961000 r--p 0004f000 103:02 3408451
```

也就是说，如果我们读取这个地址的五个字节（`"hello"这个字符串的长度`）,就可以得到"hello"，我们来试一下：

```rust    
    let file = File::open(&Path::new(&format!("/proc/{}/mem", pid)))?;
    let mut buffer = vec![0; 5];
    file.read_at(&mut buffer, 0x7fb5a2956000)?;
    println!("{}", std::str::from_utf8(&buffer)?);
```

运行结果

```
hello
```

当然，我们也可以更改它

## todo...