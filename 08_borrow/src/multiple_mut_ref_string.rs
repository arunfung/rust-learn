fn main() {
    let mut arr = vec![String::from("a"), String::from("b")]; // cache the last item
    let last = &arr[0];
// consume previously stored last item
    println!("last: {:?}", last);
    arr.push(String::from("c"));
    println!("last: {:?}", arr);
}