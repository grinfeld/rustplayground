
use std::sync::RwLock;
use crate::linkedlist::LinkedList;

mod safelinkedlist_tests;


pub struct LinkedListThreadSafe<T: Clone> {
    inner: RwLock<LinkedList<T>>
}

impl<T: Clone> LinkedListThreadSafe<T> {
    pub fn new() -> Self {
        LinkedListThreadSafe {
            inner: RwLock::new(LinkedList::new())
        }
    }

    pub fn size(&self) -> usize {
        self.inner.read().unwrap().size()
    }
    
    pub fn is_empty(&self) -> bool {
        self.inner.read().unwrap().is_empty()
    }

    pub fn push_front(&self, value: T) {
        let mut guard = self.inner.write().unwrap();
        guard.push_front(value);
    }

    pub fn remove_first(&self) -> Option<T> {
        let mut guard = self.inner.write().unwrap();
        guard.remove_first()
    }
    
    pub fn peek_head(&self) -> Option<T> {
        self.inner.read().unwrap().get_value()
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        let list_snapshot = {
            let guard = self.inner.read().unwrap();
            // cloning the lock to avoid locking the whole list for a long time
            // "head" is wrapped with Arc, so clone() method won't clone the whole list.
            // Therefor, while it copies inner LinkedList, when it reaches the "head" - it will clone only pointer (Arc)
            // and then it will protect only operations on head and not all other elements in the list.
            guard.clone()
        };
        for v in &list_snapshot {
            f(v);
        }
    }
}