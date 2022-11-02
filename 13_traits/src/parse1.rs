use std::f64;
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for f64 {
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            // 取第一个 match，将其捕获的 digits 换成 f64
            captures.get(0).map_or(0.0, |s| s.as_str().parse().unwrap_or(0.0))
        } else {
            0.0
        }
    }
}

#[test]
fn parse_should_work() {
    // assert_eq!(u8::parse("123abcd"), 123);
    // assert_eq!(u8::parse("123abcd"), 123);
    assert_eq!(f64::parse("123.4abcd"), 123.4);
}
fn main() {
println!("result: {}", f64::parse("255.1 hello world"))
}