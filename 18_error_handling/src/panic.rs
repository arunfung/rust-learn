use std::io::Read;
use std::{fs::File, io, panic};
use thiserror::Error;

fn main() {
    let _result = read_file("hello.txt");
    let result = panic::catch_unwind(|| {
        println!("hello");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
    println!("panic captured: {:#?}", result);
}

// 可以极大避免遗忘错误的显示处理, 但如果我们并不关心错误，只需要传递错误，那么就需要使用 ? 操作符
fn read_file(path: &str) -> Result<String, DataStoreError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Rust 还为 Option 和 Result 提供了大量的辅助函数，如 map / map_err / and_then
// Ok(data)
//     .and_then(validate)
//     .and_then(process)
//     .map(transform)
//     .and_then(store)
//     .map_error()

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
