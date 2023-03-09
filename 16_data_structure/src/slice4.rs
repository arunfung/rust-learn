fn main() {
    let s = String::from("hello");
    // &String 会被解引用成 &str
    print_slice(&s);
    // &s[..] 和 s.as_str() 一样，都会得到 &str
    print_slice(s.as_str());
    print_slice(&s[..]);
}

fn print_slice(s: &str) {
    println!("{:?}", s);
}