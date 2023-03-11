fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    println!("cap {:?}", v1.capacity());
    v1.push(5);
    println!("cap should be 8: {}, len: {}", v1.capacity(), v1.len());
}