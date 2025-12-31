
use std::sync::Arc;

mod linkedlist_tests;

#[derive(Debug, Clone)]
struct Node<T: Clone> {
    value: Option<T>,
    next: Option<Arc<Node<T>>>
}

#[derive(Debug, Clone)]
pub struct LinkedList<T: Clone> {
    head: Option<Arc<Node<T>>>,
    size: usize
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            size: 0
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_value(&self) -> Option<T> {
        match &self.head {
            None => None,
            Some(n) => n.value.clone()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_front(&mut self, value: T) -> &mut Self {
        if self.is_empty() {
            self.head = Some(Arc::new(Node {
                value: Some(value),
                next: None
            }));
        } else {
            self.head = Some(Arc::new(Node {
                value: Some(value),
                next: self.head.as_ref().map(|h| Arc::clone(h))
            }));
        }
        self.size = self.size + 1;
        self
    }

    pub fn remove_first(&mut self) -> Option<T> {
        let head_rc = self.head.take()?;
        match Arc::try_unwrap(head_rc) {
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

pub struct Iter<'a, T> where T: Clone {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> where T: Clone {
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

impl<'a, T> IntoIterator for &'a LinkedList<T> where T: Clone {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: self.head.as_deref() }
    }
}