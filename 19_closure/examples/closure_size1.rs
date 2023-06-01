fn main() {
    let name = String::from("Tyr");
    let data = vec!["Rust", "Elixir", "Javascript"];
    let v = &data[..];
    let i = 1u8;
    let c = move || {
        // i: u8, // 1字节 基本类型使用值捕获方式（value capturing）
        println!("i: {:?}", i);
        // v: &[&str], // (ptr|len)=16字节 捕获了切片引用
        println!("v: {:?}", v);
        // name: String, // (ptr|cap|len)=24字节 移动所有权的字符串
        println!("name: {:?}", name.clone());
    };
    c();
    println!("size of c: {}", std::mem::size_of_val(&c));

    // 请问在这里，还能访问 i 么？为什么？
    println!("i: {:?}", i);
    // 请问在这里，还能访问 name 么？为什么？
    // println!("name: {:?}", name);
}
