use crate::node::Node;

#[test]
fn test_node_creation() {
    let n = Node::new(5);

    assert_eq!(n.get_value().unwrap(), 5);

    assert!(!n.has_next());

    let n1 = n.push_front(10);

    assert!(n1.has_next());

    let mut v: Vec<i32> = vec![];

    for c in n1.iter() {
        v.push(*c.unwrap())
    }

    assert_eq!(v, [10,5]);

    let mut n2 = n1.push_front(15);
    let mut v: Vec<i32> = vec![];
    for c in n2.iter() {
        v.push(*c.unwrap())
    }

    assert_eq!(v, [15, 10, 5]);

    let c = n2.remove_first();
    assert_eq!(c.unwrap(), 15);
    let mut v: Vec<i32> = vec![];
    for c in n2.iter() {
        v.push(*c.unwrap())
    }

    assert_eq!(v, [10, 5]);

    let c = n2.remove_first();
    assert_eq!(c.unwrap(), 10);
    let c = n2.remove_first();
    assert_eq!(c.unwrap(), 5);

    let c = n2.remove_first();
    assert_eq!(c, None);
}