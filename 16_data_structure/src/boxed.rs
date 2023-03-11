use std::ops::Deref;

fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    println!("cap {:?}", v1.capacity());
    v1.push(5);
    println!("cap should be 8: {}, len: {}", v1.capacity(), v1.len());


    // 从 Vec<T> 转换成 Box<[T]>，此时会丢弃多余的 capacity
    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("cap should be exactly 5: {}, len: {}", v2.capacity(), v2.len());

    assert!(b2.deref() == v2);

    // Box<[T]> 可以更改其内部数据，但无法 push
    b2[0] = 2;
    // b2.push(6);
    println!("b2: {:?}", b2);
}