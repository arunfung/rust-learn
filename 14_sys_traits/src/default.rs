use std::fmt;

// struct 可以 derive Default，但我们需要所有字段都实现了 Default
#[derive(Clone, Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

// enum 不能 derive Default
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

// 手工实现 Default
impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    pub fn new(name: &str) -> Self {
        // 用 ..Default::default() 为剩余字段使用缺省值
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} developer",
            self.name, self.age, self.lang
        )
    }
}

fn main() {
    // 使用 T::default()
    let dev1 = Developer::default();
    // 使用 Default::default()，但此时类型无法通过上下文推断，需要提供类型
    let dev2: Developer = Default::default();
    // 使用 T::new
    let dev3 = Developer::new("Tyr");
    // Debug 是为开发者调试打印数据结构所设计的，而 Display 是给用户显示数据 结构所设计的。
    // 这也是为什么 Debug trait 的实现可以通过派生宏直接生成，而 Display 必须手工实现。
    // 在使用的时候，Debug 用 {:?} 来打印，Display 用 {} 打印。
    println!("dev1: {}\ndev2: {}\ndev3: {:?}", dev1, dev2, dev3);

    use std::sync::{Arc, Mutex};
    let shared = Arc::new(Mutex::new(1));
    let mut g = shared.lock().unwrap();
    *g += 1;
    println!("shared{:?}", shared);
}