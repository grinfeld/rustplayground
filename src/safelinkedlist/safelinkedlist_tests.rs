use crate::safelinkedlist::LinkedList;
#[test]
fn test_linkedlist() {
    let mut ll: LinkedList<i32> = LinkedList::new();

    assert!(ll.is_empty());
    assert_eq!(ll.size, 0);


    ll.push_front(5);
    ll.push_front(10);
    assert_eq!(ll.size, 2);
    assert!(!ll.is_empty());

    for v in ll.into_iter() {
        println!("{:?}", v)
    }

    ll.remove_first();
    assert_eq!(ll.size, 1);

    println!("After remove");
    for v in ll.into_iter() {
        println!("{:?}", v)
    }
}