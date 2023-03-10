fn main() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = String::from("hello");
    let s1 = &arr[..2];
    let s2 = &vec[..2];
    // &str 本身就是一个特殊的 slice
    let s3 = &s[..2];
    println!("s1: {:?}, s2: {:?}, s3: {:?}", s1, s2, s3);
}