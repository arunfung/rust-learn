use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    id: usize,
    // 使用 Rc<RefCell<T>> 让节点可以被修改
    downstream: Option<Arc<Mutex<Node>>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Arc<Mutex<Node>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Arc<Mutex<Node>>> {
        self.downstream.as_ref().cloned()
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);

    node3.update_downstream(Arc::new(Mutex::new(node4)));
    node1.update_downstream(Arc::new(Mutex::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1: {:?}, node2: {:?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    // 多线程获得锁后，来修改 downstream
    node3.lock().unwrap().downstream = Some(Arc::new(Mutex::new(node5)));

    println!("node1: {:?}, node2: {:?}", node1, node2);
}