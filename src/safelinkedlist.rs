
use std::sync::Mutex;
use crate::linkedlist::LinkedList;

mod safelinkedlist_tests;


pub struct LinkedListThreadSafe<T: Clone> {
    inner: Mutex<LinkedList<T>>
}

impl<T: Clone> LinkedListThreadSafe<T> {
    pub fn new() -> Self {
        LinkedListThreadSafe {
            inner: Mutex::new(LinkedList::new())
        }
    }

    pub fn size(&self) -> usize {
        self.inner.lock().unwrap().size()
    }
    
    pub fn is_empty(&self) -> bool {
        self.inner.lock().unwrap().is_empty()
    }

    pub fn push_front(&self, value: T) {
        let mut guard = self.inner.lock().unwrap();
        guard.push_front(value);
    }

    pub fn remove_first(&self) -> Option<T> {
        let mut guard = self.inner.lock().unwrap();
        guard.remove_first()
    }
    
    pub fn peek_head(&self) -> Option<T> {
        self.inner.lock().unwrap().get_value()
    }
}
/*
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

impl<'a, T> IntoIterator for &'a LinkedListState<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let guard = self.locker.lock().unwrap();
        Iter { current: self.head.as_deref() }
    }
}
*/