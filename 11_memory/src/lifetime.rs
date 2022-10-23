use std::str::Chars;

// 错误，没有标注生命周期，就算标注返回的也是drop的string
// fn lifetime1() -> &str {
//     let name = "Tyr".to_string();
//     &name[1..]
// }

// 错误，同样没有标注生命周期，并且引用的参数也是具有所有权的，函数结束就会drop，无法引用
// fn lifetime2(name: String) -> &str {
//     &name[1..]
// }

// 正确，为什么?
fn lifetime3(name: &str) -> Chars {
    name.chars()
}

fn main() {
    // let l1 = lifetime1();
    // lifetime2("lifetime2".into());
    let l3 = lifetime3("lifetime3");
    println!("{:?}", l3);
}