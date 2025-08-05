use std::fmt::{self, write};
pub fn main() {
    println!("Hello world");
    let a = Node {
        next: None,
        val: 10,
    };
    let b = Node {
        next: Some(Box::new(a)),
        val: 1,
    };
    println!("{}", b.next.unwrap());
}

struct Node {
    pub next: Option<Box<Node>>,
    val: i32,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let next_str = match &self.next {
            Some(node) => node.val.to_string(),
            None => String::from("None"),
        };
        write!(f, "Next_val={}, val={}", next_str, self.val)
    }
}
