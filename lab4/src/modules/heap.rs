pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.balance_regarding_parent(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.len() {
            0 | 1 => self.data.pop(),
            _ => {
                let last_index = self.data.len() - 1;
                self.data.swap(0, last_index);

                let value = self.data.pop();
                self.balance_regarding_children(0);

                value
            }
        }
    }

    fn balance_regarding_parent(&mut self, mut index: usize) -> usize {
        while let Some(parent) = self.parent(index) {
            if self.data[index] < self.data[parent] {
                self.data.swap(parent, index);
                index = parent;
            } else {
                break;
            }
        }
        index
    }

    fn balance_regarding_children(&mut self, mut index: usize) -> usize {
        while let Some(left) = self.left(index) {
            let mut best = left;

            if let Some(right) = self.right(index) {
                if self.data[right] < self.data[left] {
                    best = right;
                }
            }

            if self.data[best] < self.data[index] {
                self.data.swap(index, best);
                index = best;
            } else {
                break;
            }
        }
        index
    }

    fn parent(&self, index: usize) -> Option<usize> {
        if index > 0 {
            Some((index - 1) / 2)
        } else {
            None
        }
    }

    fn left(&self, index: usize) -> Option<usize> {
        self.check(2 * index + 1)
    }

    fn check(&self, index: usize) -> Option<usize> {
        if index < self.data.len() {
            Some(index)
        } else {
            None
        }
    }

    fn right(&self, index: usize) -> Option<usize> {
        self.check(2 * index + 2)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn peek(&self) -> Option<&T> {
        todo!("Операция пока не реализована")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_heap() {
        let heap: MinHeap<i32> = MinHeap::new();
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_push_single() {
        let mut heap = MinHeap::new();
        heap.push(5);
        assert_eq!(heap.len(), 1);
    }

    #[test]
    fn test_push_multiple() {
        let mut heap = MinHeap::new();
        heap.push(5);
        heap.push(3);
        heap.push(7);
        assert_eq!(heap.len(), 3);
    }

    #[test]
    fn test_pop_empty() {
        let mut heap: MinHeap<i32> = MinHeap::new();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_pop_single() {
        let mut heap = MinHeap::new();
        heap.push(5);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn test_heap_order() {
        let mut heap = MinHeap::new();
        // Insert in random order
        heap.push(5);
        heap.push(2);
        heap.push(8);
        heap.push(1);
        heap.push(3);

        // Should come out in ascending order
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(8));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_heap_with_duplicates() {
        let mut heap = MinHeap::new();
        heap.push(2);
        heap.push(2);
        heap.push(1);
        heap.push(3);
        heap.push(2);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
    }

    #[test]
    fn test_parent_calculation() {
        let heap: MinHeap<i32> = MinHeap::new();
        assert_eq!(heap.parent(0), None);
        assert_eq!(heap.parent(1), Some(0));
        assert_eq!(heap.parent(2), Some(0));
        assert_eq!(heap.parent(3), Some(1));
        assert_eq!(heap.parent(4), Some(1));
    }

    #[test]
    fn test_children_calculation() {
        let mut heap = MinHeap::new();
        // Add some elements to test children calculation
        for i in 0..5 {
            heap.push(i);
        }

        assert_eq!(heap.left(0), Some(1));
        assert_eq!(heap.right(0), Some(2));
        assert_eq!(heap.left(1), Some(3));
        assert_eq!(heap.right(1), Some(4));
        // Test for non-existent children
        assert_eq!(heap.left(4), None);
        assert_eq!(heap.right(4), None);
    }
}
