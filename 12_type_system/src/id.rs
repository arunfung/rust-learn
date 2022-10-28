fn id<T>(x : T) -> T {
    x
}

fn main() {
    let int = id(12);
    let string = id("hello world");
    println!("{}, {}", int, string);
}