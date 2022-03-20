```shell
cargo build --release --target x86_64-unknown-linux-gnu
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld", "-C", "link-arg=-nostdlib", "-C", "link-arg=-static"]
llvm-strip --strip-sections hello
ll rust-helloworld
407 byte     rust-helloworld
```