use std::cell::Ref;
use std::collections::HashMap;

pub struct Node {
    categories: Vec<String>,
    nexts: Option<HashMap<String, Node>>
}

impl Node {
    pub fn new() -> Self {
        let categories: Vec<String> = Vec::new();
        let nexts: Option<HashMap<String, Node>> = None;
        Node {
            categories,  nexts
        }
    }

    fn new_params(categories: Vec<String>, next_values: HashMap<String, Node>) -> Self {
        let nexts: Option<HashMap<String, Node>> = Some(next_values);
        Node {
            categories,
            nexts
        }
    }

    pub fn categories_copy(self) -> Vec<String> {
        let copy: Vec<String> = self.categories.to_vec();
        copy
    }

    pub fn has_next(self, word: String) -> bool {
        match self.nexts {
            Some(map) => map.contains_key(word.as_str()),
            None => false
        }
    }

    pub fn get_next(self, word: String) -> Option<&'static Node> {
        match self.nexts {
            Some(map) => {
                if map.contains_key(word.as_str()) {
                        map.get(word.as_str())
                } else {
                    return None
                }
            },
            None => None
        }
    }

    pub fn add(self, category: String, word: String) {
        let exists = self.has_next(word);
    }
}