use std::{fs::File, io::Write};

fn main() {
    // trait 的动态分派是使用 trait object
    let mut f = File::create("/tmp/test_write_trait").unwrap();
    let w: &mut dyn Write = &mut f;
    w.write_all(b"hello ").unwrap();

    // 无法在 trait object 里调用使用了 Self 的函数
    // let w1 = w.by_ref();
    // w1.write_all(b"world").unwrap();

    // 不允许携带泛型参数，是因为 Rust 里带泛型的类型在编译时会做单态化，而 trait object 是运行时的产物，两者不能兼容。

}