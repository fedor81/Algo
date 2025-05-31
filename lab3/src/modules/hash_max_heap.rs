use std::{collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

pub struct HashMaxHeap<T, P> {
    data: Vec<Rc<T>>,
    hash_map: HashMap<Rc<T>, DictNode<P>>,
}

struct DictNode<P> {
    power: P,
    index: usize,
}

impl<T, P> HashMaxHeap<T, P>
where
    T: Hash + Eq + Display,
    P: Ord,
{
    pub fn new() -> Self {
        Self {
            data: vec![],
            hash_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: T, power: P) {
        let rc_value = Rc::new(key);

        let index = if let Some(node) = self.hash_map.get_mut(&rc_value) {
            node.power = power;
            node.index
        } else {
            let index = self.data.len();
            self.data.push(rc_value.clone());
            self.hash_map.insert(rc_value, DictNode { power, index });
            index
        };

        self.balance(index);
    }

    pub fn change_power<F>(&mut self, key: &T, set_new_power: F)
    where
        F: FnOnce(&P) -> P,
    {
        let node = self
            .hash_map
            .get_mut(key)
            .expect(&format!("Heap does not contain: {}", key));
        let index = node.index;
        node.power = set_new_power(&node.power);
        self.balance(index);
    }

    pub fn contains(&self, key: &T) -> bool {
        self.hash_map.contains_key(key)
    }

    fn balance_regarding_parent(&mut self, mut index: usize) -> usize {
        while let Some(parent) = self.parent(index) {
            let parent = self.hash_map.get(&self.data[parent]).unwrap();
            let current = self.hash_map.get(&self.data[index]).unwrap();

            if parent.power < current.power {
                index = parent.index;
                self.swap(current.index, parent.index);
            } else {
                break;
            }
        }
        index
    }

    fn swap(&mut self, index: usize, other: usize) {
        let node = self.hash_map.get_mut(&self.data[index]).unwrap();
        node.index = other;

        let other_node = self.hash_map.get_mut(&self.data[other]).unwrap();
        other_node.index = index;

        self.data.swap(index, other);
    }

    pub fn balance(&mut self, mut index: usize) -> usize {
        index = self.balance_regarding_children(index);
        index = self.balance_regarding_parent(index);
        index
    }

    fn balance_regarding_children(&mut self, mut index: usize) -> usize {
        while let Some(left) = self.left(index) {
            let current = self.hash_map.get(&self.data[index]).unwrap();
            let left = self.hash_map.get(&self.data[left]).unwrap();
            let mut max = left;

            if let Some(right) = self.right(index) {
                let right = self.hash_map.get(&self.data[right]).unwrap();

                if left.power < right.power {
                    max = right;
                }
            }

            if current.power < max.power {
                index = max.index;
                self.swap(index, max.index);
            } else {
                break;
            }
        }
        index
    }

    pub fn get_at(&self, index: usize) -> Option<&T> {
        self.data.get(index).map(|v| &**v)
    }

    pub fn get_index(&self, value: &T) -> Option<usize> {
        self.hash_map.get(value).map(|node| node.index)
    }

    fn parent(&self, index: usize) -> Option<usize> {
        if index > 0 {
            Some((index - 1) / 2)
        } else {
            None
        }
    }

    fn left(&self, index: usize) -> Option<usize> {
        let result = 2 * index + 1;
        self.check(result)
    }

    fn check(&self, index: usize) -> Option<usize> {
        if index < self.data.len() {
            Some(index)
        } else {
            None
        }
    }

    fn right(&self, index: usize) -> Option<usize> {
        let result = 2 * index + 2;
        self.check(result)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn remove(&mut self, index: usize) -> T {
        todo!("Операция пока не реализована")
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0).map(|v| &**v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap1() {
        let mut heap = HashMaxHeap::new();
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.left(0), None);

        heap.insert(2, 2);
        assert_eq!(heap.peek(), Some(&2));
        assert_eq!(heap.get_index(&2), Some(0));

        heap.insert(8, 8);
        assert_eq!(heap.peek(), Some(&8));
        assert_eq!(heap.get_index(&2), Some(1));
        assert_eq!(heap.get_index(&8), Some(0));

        heap.insert(4, 4);
        assert_eq!(heap.peek(), Some(&8));
        assert_eq!(heap.get_at(2), Some(&4));

        heap.insert(1, 1);
        assert_eq!(heap.peek(), Some(&8));

        heap.insert(5, 5);
        heap.insert(3, 3);
        assert_eq!(heap.peek(), Some(&8));

        heap.insert(7, 7);
        heap.insert(10, 10);
        assert_eq!(heap.peek(), Some(&10));

        heap.insert(6, 6);
        heap.insert(9, 9);

        assert_eq!(heap.left(0), Some(1));
        assert_eq!(heap.right(0), Some(2));
        assert_eq!(heap.right(1), Some(4));
        assert_eq!(heap.right(2), Some(6));

        assert_eq!(heap.peek(), Some(&10));

        heap.change_power(&10, |_power| 0);
        assert_eq!(heap.peek(), Some(&9));

        heap.change_power(&1, |_power| 100);
        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.len(), 10);

        heap.insert(1, 1);
        assert_eq!(heap.peek(), Some(&9));
        assert_eq!(heap.len(), 10);
    }

    #[test]
    fn test_heap2() {
        let mut heap = HashMaxHeap::new();

        heap.insert("abc", 2);
        heap.insert("def", 0);
        heap.insert("ghi", 3);
        heap.insert("jkl", 3);
        heap.insert("mno", 4);
        heap.insert("pqr", 1);

        assert_eq!(*heap.peek().unwrap(), "mno");
        heap.change_power(&"def", |_| 100);
        assert_eq!(*heap.peek().unwrap(), "def");
    }
}
