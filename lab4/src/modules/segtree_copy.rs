use std::{
    fmt::Debug,
    ops::{Bound, Range, RangeBounds},
};

/// Простое дерево отрезков предназначенное для чисел или других копируемых типов
#[derive(Debug)]
pub struct SegTree<T, F>
where
    T: Copy + Default + Debug + Ord,
    F: Fn(T, T) -> T,
{
    data: Vec<T>,
    size: usize,
    f: F,
}

impl<T, F> SegTree<T, F>
where
    T: Copy + Default + Debug + Ord,
    F: Fn(T, T) -> T,
{
    pub fn from_vec(elements: &[T], sum: F) -> Self {
        if elements.is_empty() {
            panic!("elements must not be empty");
        }

        let size = elements.len();
        let mut data = vec![T::default(); 2 * size];
        data[size..(2 * size)].copy_from_slice(elements);

        for i in (1..size).rev() {
            data[i] = sum(
                data.get(2 * i).map(|&value| value).unwrap_or_default(),
                data.get(2 * i + 1).map(|&value| value).unwrap_or_default(),
            )
        }

        Self { data, size, f: sum }
    }

    pub fn update(&mut self, index: usize, value: T) -> Result<(), &str> {
        let mut node = self.size + index;

        if self.size <= index {
            return Err("index out of bounds");
        } else if self.data[node] == value {
            return Ok(());
        }

        self.data[node] = value;

        while node > 0 {
            node /= 2;
            self.data[node] = (self.f)(self.data[2 * node], self.data[2 * node + 1])
        }

        Ok(())
    }

    fn convert_bounds<R>(&self, range: R) -> (usize, usize)
    where
        R: RangeBounds<usize>,
    {
        (
            match range.start_bound() {
                Bound::Included(&s) => s,
                Bound::Excluded(&s) => s + 1,
                Bound::Unbounded => 0,
            },
            match range.end_bound() {
                Bound::Included(&e) => e + 1,
                Bound::Excluded(&e) => e,
                Bound::Unbounded => self.size,
            },
        )
    }

    pub fn query<R>(&self, range: R) -> T
    where
        R: RangeBounds<usize>,
    {
        let (start, end) = self.convert_bounds(range);
        if start >= self.size || end > self.size {
            panic!("index out of range");
        }

        let mut left = self.size + start;
        let mut right = self.size + end;
        let mut result = T::default();

        while left < right {
            if left % 2 == 1 {
                result = (self.f)(result, self.data[left]);
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                result = (self.f)(result, self.data[right]);
            }
            left /= 2;
            right /= 2;
        }

        result
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seg_tree() {
        let vec = vec![1, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut tree = SegTree::from_vec(&vec, |a, b| a + b);
        assert_eq!(tree.query(0..vec.len()), 38);
        assert_eq!(tree.query(1..4), 5);
        assert_eq!(tree.query(4..7), 4);
        assert_eq!(tree.query(6..9), -3);
        assert_eq!(tree.query(9..vec.len()), 37);
        assert_eq!(tree.update(5, 10), Ok(()));
        assert_eq!(tree.update(14, -8), Ok(()));
        assert_eq!(tree.query(4..7), 19);
        assert_eq!(tree.update(15, 100), Err("index out of bounds"));
        assert_eq!(tree.query(5..5), 0);
    }
}
