use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher},
};

/// Простая хэш-таблица для хранения чисел
/// Под капотом в ячейках находится VecDeque
pub struct Dict<K, V>
where
    K: Copy + Clone,
    V: Copy + Clone,
{
    buckets: Vec<Option<VecDeque<DictNode<K, V>>>>,
}

#[derive(Debug, Clone, Copy)]
struct DictNode<K, V>
where
    K: Copy + Clone,
    V: Copy + Clone,
{
    key: K,
    value: V,
}

impl<K, V> Dict<K, V>
where
    K: Hash + Eq + Clone + Copy,
    V: Clone + Copy,
{
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buckets = vec![];
        for i in 0..capacity {
            buckets.push(None);
        }
        Self { buckets }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = self.get_bucket(&key);
        match self.buckets[index].as_mut() {
            Some(deque) => {
                for node in deque.iter_mut() {
                    if node.key == key {
                        let old_value = node.value;
                        node.value = value;
                        return Some(old_value);
                    }
                }
                deque.push_back(DictNode::new(key, value));
            }
            None => {
                let mut deque = VecDeque::new();
                deque.push_back(DictNode::new(key, value));
                self.buckets[index] = Some(deque);
            }
        }
        None
    }

    pub fn get(&self, key: K) -> Option<V> {
        let index = self.get_bucket(&key);
        if let Some(deque) = self.buckets[index].as_ref() {
            for node in deque.iter() {
                if node.key == key {
                    return Some(node.value);
                }
            }
        }

        None
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let index = self.get_bucket(&key);
        if let Some(deque) = self.buckets[index].as_mut() {
            for i in 0..deque.len() {
                if deque[i].key == key {
                    return Some(deque.remove(i).unwrap().value);
                }
            }
        }

        None
    }

    fn get_bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }
}

impl<K, V> DictNode<K, V>
where
    K: Copy + Clone,
    V: Copy + Clone,
{
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut dict = Dict::with_capacity(10);
        dict.insert(1, "one");
        dict.insert(2, "two");

        assert_eq!(dict.get(1), Some("one"));
        assert_eq!(dict.get(2), Some("two"));
        assert_eq!(dict.get(3), None);
    }

    #[test]
    fn test_update_existing_key() {
        let mut dict = Dict::with_capacity(10);
        dict.insert(1, "one");
        dict.insert(1, "updated_one");

        assert_eq!(dict.get(1), Some("updated_one"));
    }

    #[test]
    fn test_remove() {
        let mut dict = Dict::with_capacity(10);
        dict.insert(1, "one");
        dict.insert(2, "two");

        assert_eq!(dict.remove(1), Some("one"));
        assert_eq!(dict.get(1), None);
        assert_eq!(dict.get(2), Some("two"));
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut dict = Dict::with_capacity(10);
        dict.insert(1, "one");

        assert_eq!(dict.remove(2), None);
        assert_eq!(dict.get(1), Some("one"));
    }

    #[test]
    fn test_with_string_keys() {
        let mut dict = Dict::with_capacity(10);
        dict.insert(10, 42);
        dict.insert(20, 24);

        assert_eq!(dict.get(10), Some(42));
        assert_eq!(dict.get(20), Some(24));
        assert_eq!(dict.get(100), None);
    }

    #[test]
    fn test_collision_handling() {
        let mut dict = Dict::with_capacity(1); // Force collisions with capacity 1
        dict.insert(1, "one");
        dict.insert(2, "two");
        dict.insert(3, "three");

        assert_eq!(dict.get(1), Some("one"));
        assert_eq!(dict.get(2), Some("two"));
        assert_eq!(dict.get(3), Some("three"));
    }

    #[test]
    fn test_empty_dict() {
        let dict: Dict<i32, &str> = Dict::with_capacity(10);
        assert_eq!(dict.get(1), None);
    }
}
