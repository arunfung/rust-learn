use std::net::SocketAddr;

fn main() {
    // 在泛型函数后使用 ::<T> 来强制使用类型 T，这种写法被称为 turbofish。
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("addr: {:?}, port: {:?}", addr.ip(), addr.port());
}