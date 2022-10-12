fn main() {
    let s1 = "Hello world";

    println!("first word of s1: {}", first(&s1));
}

// 多余的 lifetime 标注会提醒你不需要
// fn first<'a>(s: &'a str) -> &'a str {
fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos],
    }
}
