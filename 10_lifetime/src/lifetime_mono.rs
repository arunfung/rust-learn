trait Print {
    fn print(self);
}

impl Print for &'static str {
    fn print(self) {
        println!("'static str: {}", self);
    }
}

fn main() {
    let s = "hello, world!";
    s.print();
    // print_str(s);
}