use std::ops::Deref;

mod node_tests;

type NodeElem<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct  Node<T> where T: Clone {
    value: Option<Box<T>>,
    next: NodeElem<T>,
}

impl<T> Node<T> where T: Clone {

    pub fn new(value: T) -> Node<T> {
        Node {
            value: Some(Box::new(value)), next: None
        }
    }

    pub fn has_next(&self) -> bool {
        (&self.next).is_some()
    }

    pub fn push_front(mut self, value: T) -> Self {
        let new_node = Node {
            value: Some(self.value.unwrap().clone()),
            next: self.next
        };
        self.value = Some(Box::new(value));
        self.next = Some(Box::new(new_node));
        self
    }

    pub fn get_value(&self) -> Option<T> {
        match &self.value {
            None => None,
            Some(val) => Some(val.clone().deref().clone())
        }
    }

    pub fn remove_first(&mut self) -> Option<T> {
        let c = self.value.take();
        if !self.has_next() {
            self.value = None;
            self.next = None;
        } else {
            let next = self.next.as_ref().unwrap().clone();
            self.value = next.value;
            self.next = next.next;
        }

        match c {
            None => None,
            Some(vl) => {
                Some(vl.deref().clone())
            }
        }
    }
}

pub struct NodeIter<'a, T> where T: Clone, {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for NodeIter<'a, T> where T: Clone {
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current?;
        self.current = node.next.as_deref();
        Some(node.value.as_deref())
    }
}

impl<T> Node<T> where T: Clone {
    pub fn iter(&self) -> NodeIter<'_, T> {
        NodeIter { current: Some(self) }
    }
}

impl<'a, T> IntoIterator for &'a Node<T> where T: Clone {
    type Item = Option<&'a T>;
    type IntoIter = NodeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// #[derive(Debug, Clone)]
// pub struct NonEmpty<T> where T: Clone {
//     value: Option<T>,
//     next: Option<Box<NonEmpty<T>>>,
// }
//
// pub struct NodeIter<'a, T> where T: Clone {
//     current: Option<&'a NonEmpty<T>>
// }
//
// impl<'a, T> Iterator for NodeIter<'a, T> where T: Clone {
//     type Item = Option<&'a T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let node: &NonEmpty<T> = self.current?;
//         self.current = node.next.as_deref();
//         Some(node.value.as_ref())
//     }
// }
//
// impl<'a, T> IntoIterator for &'a NonEmpty<T> where T: Clone {
//     type Item = Option<&'a T>;
//     type IntoIter = NodeIter<'a, T>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }
//
// impl<T> NonEmpty<T> where T: Clone {
//     pub fn new(value: T) -> NonEmpty<T> {
//         NonEmpty {
//             value: Some(value),
//             next: None,
//         }
//     }
//
//     pub fn push(mut self, value: T) -> Box<NonEmpty<T>> {
//         let new_node = NonEmpty {
//             value: Some(value),
//             next: None,
//         };
//         self.next = Some(Box::new(new_node));
//         Box::clone(&self.next.unwrap())
//     }
//
//     pub fn iter(&self) -> NodeIter<'_, T> {
//         NodeIter {
//             current: Some(self)
//         }
//     }
// }