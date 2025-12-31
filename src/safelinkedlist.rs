
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

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&T),
    {
        let list_snapshot = {
            let guard = self.inner.lock().unwrap();
            // cloning the lock to avoid locking the whole list for a long time
            guard.clone()
        };
        for v in &list_snapshot {
            f(v);
        }
    }
}