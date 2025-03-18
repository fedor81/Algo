pub fn heap_sort<T: Ord>(collection: &mut Vec<T>, reverse: bool) {
    heap_sort_cmp(collection, |a, b| if reverse { a > b } else { a < b });
}

pub fn heap_sort_cmp<F, T>(collection: &mut Vec<T>, cmp: F)
where
    F: Fn(&T, &T) -> bool,
    T: Ord,
{
    if collection.len() <= 1 {
        return;
    }

    for i in (0..=(collection.len() / 2 - 1)).rev() {
        heapify(collection, collection.len(), i, &cmp); // Построение кучи
    }

    for i in (0..=(collection.len() - 1)).rev() {
        collection.swap(0, i);
        heapify(collection, i, 0, &cmp);
    }
}

fn heapify<T, F>(collection: &mut Vec<T>, len: usize, index: usize, cmp: F)
where
    F: Fn(&T, &T) -> bool,
    T: Ord,
{
    let mut largest = index; // Сравниваем три элемента
    let left = 2 * index + 1;
    let right = 2 * index + 2;

    if left < len && cmp(&collection[largest], &collection[left]) {
        largest = left;
    }

    if right < len && cmp(&collection[largest], &collection[right]) {
        largest = right;
    }

    if largest != index {
        collection.swap(index, largest);
        heapify(collection, len, largest, cmp); // Повторяем для подмножетва
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn test_heap_sort() {
        test_helper(vec![
            5, 3, 1, 78, 3, 9, 3, 1, 32, 214, 0, 1, 2, 3, 545, 1, 19,
        ]);
        test_helper(vec![
            "abc", "dgfg", "gf", "sge", "grsets", "ADFg", "Gy", "Gg", "gfd", "eer",
        ]);
        test_helper(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        test_helper(vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        test_helper(vec![1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1]);
        test_helper(vec![0]);
        test_helper(vec![10; 100000]);
    }

    fn test_helper<T: Clone + Ord + Debug>(collection: Vec<T>) {
        let mut collection = collection;
        let mut expected = collection.clone();

        heap_sort(&mut collection, false);
        expected.sort();

        assert_eq!(collection, expected);
    }
}
