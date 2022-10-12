fn main() {
    let s1 = String::from("Lindsey");
    let s2 = String::from("Rosie");

    let result = max(&s1, &s2);

    println!("bigger one: {}", result);

    let result = get_max(&s1);
    println!("bigger two: {}", result);
}

fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    println!("bigger one: {:?}", s1 > s2);
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
