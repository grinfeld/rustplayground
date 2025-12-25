use std::sync::{Arc, Mutex};

// Type alias for a pointer to a node. The list is never null, it's a pointer to a sentinel.
type NodePtr<T> = Arc<Mutex<Node<T>>>;
// A link can be optional, though in the circular list it rarely is.
type Link<T> = Option<NodePtr<T>>;

// The Node struct. `value` is an Option to accommodate the sentinel node.
#[derive(Debug)]
pub struct Node<T> {
    value: Option<T>,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    // Creates a new empty list, represented by a sentinel node.
    // The sentinel's `next` and `prev` point to itself, forming a circular list.
    pub fn new() -> NodePtr<T> {
        let sentinel = Arc::new(Mutex::new(Node {
            value: None,
            prev: None,
            next: None,
        }));
        let mut sentinel_guard = sentinel.lock().unwrap();
        sentinel_guard.prev = Some(sentinel.clone());
        sentinel_guard.next = Some(sentinel.clone());
        drop(sentinel_guard);
        sentinel
    }

    // Private helper to create a new node with a value.
    fn new_with_value(value: T) -> NodePtr<T> {
        Arc::new(Mutex::new(Node {
            value: Some(value),
            prev: None,
            next: None,
        }))
    }

    // Adds a new node to the front of the list (after the sentinel).
    // `self` should be the sentinel node.
    pub fn push_front(self: &Arc<Mutex<Self>>, value: T) {
        let new_node = Self::new_with_value(value);
        let mut sentinel_guard = self.lock().unwrap();
        let old_first = sentinel_guard.next.clone().unwrap(); // In a circular list, this is always Some.

        new_node.lock().unwrap().prev = Some(self.clone());
        new_node.lock().unwrap().next = Some(old_first.clone());

        old_first.lock().unwrap().prev = Some(new_node.clone());
        sentinel_guard.next = Some(new_node);
    }

    // Adds a new node to the end of the list (before the sentinel).
    // `self` should be the sentinel node.
    pub fn push_end(self: &Arc<Mutex<Self>>, value: T) {
        let new_node = Self::new_with_value(value);
        let mut sentinel_guard = self.lock().unwrap();
        let old_last = sentinel_guard.prev.clone().unwrap(); // In a circular list, this is always Some.

        new_node.lock().unwrap().next = Some(self.clone());
        new_node.lock().unwrap().prev = Some(old_last.clone());

        old_last.lock().unwrap().next = Some(new_node.clone());
        sentinel_guard.prev = Some(new_node);
    }
}

// The iterator struct for the list.
pub struct ListIterator<T> {
    current: NodePtr<T>,
    sentinel: NodePtr<T>,
}

impl<T: Clone> Node<T> {
    // Returns an iterator over the values in the list.
    pub fn iter(self: &Arc<Mutex<Self>>) -> ListIterator<T> {
        let sentinel_guard = self.lock().unwrap();
        ListIterator {
            current: sentinel_guard.next.clone().unwrap(),
            sentinel: self.clone(),
        }
    }
}

// Implement the Iterator trait for ListIterator.
impl<T: Clone> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if Arc::ptr_eq(&self.current, &self.sentinel) {
            return None;
        }
        let current_guard = self.current.lock().unwrap();
        let value = current_guard.value.clone();
        let next_node = current_guard.next.clone().unwrap();
        drop(current_guard);

        self.current = next_node;
        value
    }
}
