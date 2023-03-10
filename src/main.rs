mod node;

use std::collections::HashMap;
use node::Node;

fn main() {
    let s: Node = Node::new();
    let b = s.has_next(String::from("stam"));
    println!("{}", b)
}

fn update_hash_map(map: &mut HashMap<String, String>) {
    map.insert("hello".to_string(), "world".to_string());
}
