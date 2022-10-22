#[derive(Default)]
struct Align1 {
    a: u8,
    b: usize,
    c: u32,
}

#[derive(Default)]
struct Align2 {
    a: u8,
}

fn main() {
    let s1 = "a";
    let s2 = "aaaa";
    let s3 = "hello";
    let a = Align1::default();
    let b = Align2::default();
    println!("{:p}", s1);
    println!("{:p}", s2);
    println!("{:p}", s3);
    println!("Align1.a: {:p}", &a.a);
    println!("Align1.b: {:p}", &a.b);
    println!("Align1.c: {:p}", &a.c);
    println!("Align2.a: {:p}", &b.a);
}