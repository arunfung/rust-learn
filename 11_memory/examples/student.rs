use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: String,
    age: u8,
    scores: HashMap<String, u8>,
}

fn main() {
    let student = Student {
        name: "Student".into(),
        age: 18,
        scores: HashMap::from_iter(IntoIterator::into_iter(
            [("Math".into(), 90), ("Math".into(), 85)]
        )),
    };
    println!("student: {:?}", student);
}