#[allow(dead_code)]
// 因为 String 类型没有实现 Copy。 因此，Developer 数据结构只能 clone，无法 copy
// Developer 类型在做参数传递时，会执行 Move 语义，而 Language 会 执行 Copy 语义。
// #[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
struct Developer {
    name: String,
    age:u8,
    lang:Language
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Language {
    Rust,
    TypeScript,
    GO,
    PHP
}
fn main() {
    let dev = Developer {
        name: "Try".to_string(),
        age: 18,
        lang: Language::Rust
    };
    let  dev1 = dev.clone();
    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    // 对于 name，也就是 String 类型的 Clone，其堆上的内存也被 Clone 了一份，所以 Clone 是深度拷贝，栈内存和堆内存一起拷贝。
    println!("dev1: {:?}, addr of dev1 name: {:p}", dev1, dev1.name.as_str());
}
