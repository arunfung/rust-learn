fn main() {
    let mut arr = vec![1, 2, 3];
    // cache the last item
    // 只读引用
    let last = arr.last();

    // 1、下标取值 i32实现了copy trait
    // let last = arr[0];

    // 2、先使用后push
    // println!("last: {:?}", last);

    // 可变引用
    // arr.push(4);
    // consume previously stored last item
    println!("last: {:?}", last);
    println!("last: {:?}", arr);
}