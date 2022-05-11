#[derive(Debug)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node { val, next: None }
    }
}

fn to_list(vector: Vec<i32>) -> Option<Box<Node>> {	
    vector.iter().fold(None, |cur, &value| {
        let mut new_node = Node::new(value);
        new_node.next = cur;
        Some(Box::new(new_node))
    })
}

fn unbox<Node>(value: Box<Node>) -> Node {
    *value
}

fn main() {
    let _init_node = Node {
        val: 0,
        next: Some(Box::new(Node::new(1))),
    };
    let vector = vec![0, 1, 2, 3];
    for node in to_list(vector) {
	let new_node = unbox(node);
        println!("node val: {:?} next: {:?}", new_node.val, new_node.next);
    }
}
