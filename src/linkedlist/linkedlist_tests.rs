use crate::linkedlist::LinkedList;

#[test]
fn test_linkedlist() {
    let mut ll: LinkedList<i32> = LinkedList::new();

    assert!(ll.is_empty());
    assert_eq!(ll.size, 0);


    let l1 = ll.push_front(5).push_front(10);
    assert_eq!(l1.size, 2);
    assert!(!l1.is_empty());

    for v in l1.into_iter() {
        println!("{:?}", v)
    }

    l1.remove_first();
    assert_eq!(l1.size, 1);

    println!("After remove");
    for v in l1.into_iter() {
        println!("{:?}", v)
    }
}