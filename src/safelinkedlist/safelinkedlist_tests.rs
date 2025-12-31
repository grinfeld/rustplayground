use crate::safelinkedlist::LinkedListThreadSafe;
use std::sync::Arc;

#[test]
fn test_linkedlist() {
    let mut ll: LinkedListThreadSafe<i32> = LinkedListThreadSafe::new();

    assert!(ll.is_empty());
    assert_eq!(ll.size(), 0);


    ll.push_front(5);
    ll.push_front(10);
    assert_eq!(ll.size(), 2);
    assert!(!ll.is_empty());

    // for v in ll.into_iter() {
    //     println!("{:?}", v)
    // }

    ll.remove_first();
    assert_eq!(ll.size(), 1);

    /*println!("After remove");
    for v in ll.into_iter() {
        println!("{:?}", v)
    }*/
}

#[test]
fn test_linkedlist_threads() {
    let ll: LinkedListThreadSafe<i32> = LinkedListThreadSafe::new();
    let shared_list = Arc::new(ll);

    for i in 0..10 {
        let mut list_clone = Arc::clone(&shared_list);
        let handle = std::thread::spawn(move || {
            list_clone.push_front(i * 10);
        });
        handle.join().unwrap();
        print!("{:?}", shared_list.peek_head().unwrap());
        assert_eq!(shared_list.size(), 10);
    }


}