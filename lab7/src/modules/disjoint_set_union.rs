use rand;
use std::{collections::HashMap, hash::Hash};

/// Система непересекающихся множеств для Copy-типов
pub struct DisjointSetUnion<T>
where
    T: Hash + Eq + Copy,
{
    parents: HashMap<T, T>,
}

impl<T> DisjointSetUnion<T>
where
    T: Hash + Eq + Copy,
{
    pub fn new() -> Self {
        Self {
            parents: HashMap::new(),
        }
    }

    /// Создать новое множество из одного элемента
    pub fn add(&mut self, v: T) {
        self.parents.insert(v, v);
    }

    /// Найти идентификатор множества
    pub fn find(&self, v: T) -> Option<T> {
        if !self.parents.contains_key(&v) {
            return None;
        }

        let mut root = v;
        while self.parents[&root] != root {
            root = self.parents[&root];
        }
        Some(root)
    }

    pub fn find_or_add(&mut self, v: T) -> T {
        match self.find(v) {
            Some(root) => root,
            None => {
                self.add(v);
                v
            }
        }
    }

    /// Объеденить множества
    pub fn unite(&mut self, a: T, b: T) {
        let root_a = self.find(a).expect("a not in set");
        let root_b = self.find(b).expect("b not in set");

        // Случайным образом выбираем корень
        if rand::random_bool(0.5) {
            self.parents.insert(root_a, root_b);
        } else {
            self.parents.insert(root_b, root_a);
        }
    }
}
