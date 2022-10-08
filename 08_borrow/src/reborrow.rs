fn main() {
    let mut x = 42;

    // 已经存在可变引用
    let r1 = &mut x;
    // reborrow 可以通过
    let r2 = &*r1;
    // &x 不可以在可变引用作用域内使用只读引用
    // let r2 = &x;

    println!("r1: {:p}, r2: {:p}", &r1, &r2);

    *r1 += 1;
}
