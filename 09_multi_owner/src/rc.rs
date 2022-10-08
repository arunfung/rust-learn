use std::rc::Rc;

fn main() {
    let a = Rc::new(1);
    println!("{:?}", a);
}
