## date 命令

linux中，我们使用`date`命令可以获得当前时间：

```shell
$ date
2022年 03月 01日 星期二 00:22:20 CST
```

查看date命令调用了哪些东西来获取时间：

```shell
$ readelf -Ws /usr/bin/date | grep -i time
     5: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND localtime@GLIBC_2.2.5 (3)
    10: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND localtime_r@GLIBC_2.2.5 (3)
    14: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND clock_gettime@GLIBC_2.17 (6)
    29: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND gmtime_r@GLIBC_2.2.5 (3)
    46: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND time@GLIBC_2.2.5 (3)
    48: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND settimeofday@GLIBC_2.2.5 (3)
    52: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND clock_settime@GLIBC_2.17 (6)
    53: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND mktime@GLIBC_2.2.5 (3)
    58: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND timegm@GLIBC_2.2.5 (3)
    59: 0000000000000000     0 FUNC    GLOBAL DEFAULT  UND strftime@GLIBC_2.2.5 (3)
```

我们可以发现它调用了 `clock_gettime@GLIBC_2.17 (6)`，我们如果hook住这个调用即可实现改变date命令获取的时间。

我们先导出glibc的api：

```rust
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[link(name = "c")]
extern "C" {
    fn __clock_gettime(clk_id: i32, tp: *mut timespec) -> i32;
}

```
然后公开一个相同名字的 api 然后把返回的时间（sec）加上 `24 * 60 * 60` 即一天 ：

```rust
#[no_mangle]
pub unsafe extern "C" fn clock_gettime(clk_id: i32, tp: *mut timespec) -> i32 {
    let mut t = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let ok = __clock_gettime(clk_id, &mut t);
    *tp = timespec {
        tv_sec: t.tv_sec + 24 * 60 * 60,
        tv_nsec: t.tv_nsec,
    };
    ok
}
```

编译成动态库，我们的 `Cargo.toml` 加上：

```toml
#...
[lib]
crate-type = ["cdylib"]
#...
```

然后使用 `LD_PRELOAD` 将它导入：
```shell
$ LD_PRELOAD=(pwd)/libekko.so date
2022年 03月 02日 星期三 00:35:35 CST
```
发现时间获取的时间刚好加快了一天。

///////////////
TODO...

我们编写一个小程序：
```rust
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn main() {
    loop {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        thread::sleep(Duration::from_millis(1000));
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        println!("start:{} end:{} end-start:{}", start, end, end - start);
    }
}
```

## tips

某单机游戏需要等待5s才能使用发动一次攻击，如果hook住时间就可以。。。