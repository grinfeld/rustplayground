
use std::sync::Mutex;
use std::rc::Rc;

mod safelinkedlist_tests;

#[derive(Debug)]
struct Node<T> {
    value: Option<T>,
    next: Option<Rc<Node<T>>>
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
    size: usize,
    locker: Mutex<bool>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
            locker: Mutex::new(false)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_front(&mut self, value: T)  {
        let guard = self.locker.lock().unwrap();
        if self.is_empty() {
            self.head = Some(Rc::new(Node {
                value: Some(value),
                next: None
            }));
        } else {
            self.head = Some(Rc::new(Node {
                value: Some(value),
                next: Some(Rc::clone(&self.head.take().unwrap()))
            }));
        }
        self.size = self.size + 1;
    }

    pub fn remove_first(&mut self) -> Option<T> {
        let guard = self.locker.lock().unwrap();
        let head_rc = self.head.take()?;
        match Rc::try_unwrap(head_rc) {
            Ok(node) => {
                self.head = node.next;
                self.size -= 1;
                node.value
            }
            Err(_) => {
                panic!("Some one holding the node");
            }
        }
    }
}

pub struct Iter<'a, T>  {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>  {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(n) => {
                self.current = n.next.as_deref();
                n.value.as_ref()
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: self.head.as_deref() }
    }
}