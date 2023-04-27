use std::collections::BTreeMap;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

fn main() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/etc/passwd", 0x1), 12);
    map.insert(Name::new("/etc/hosts", 0x1), 4);
    map.insert(Name::new("/home/tchen", 0x0), 28);

    for item in map.iter() {
        println!("{:?}", item)
    }
}
