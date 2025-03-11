pub fn quick_sort<T: PartialOrd + Clone>(collection: &mut [T], reverse: bool) {
    if collection.len() <= 1 {
        return;
    }

    let pivot_index = collection.len() - 1;
    let pivot = collection[pivot_index].clone();
    let mut partition_index = 0;

    for i in 0..pivot_index {
        if collection[i] <= pivot && !reverse || collection[i] >= pivot && reverse {
            collection.swap(partition_index, i);
            partition_index += 1;
        }
    }

    collection.swap(partition_index, pivot_index);

    quick_sort(&mut collection[..partition_index], reverse);
    quick_sort(&mut collection[partition_index + 1..], reverse);
}

pub fn quick_sort_non_recursive<T: PartialOrd + Clone>(collection: &mut [T], reverse: bool) {
    let mut stack = Vec::new();
    stack.push((0, collection.len()));

    while let Some((start, end)) = stack.pop() {
        if end - start <= 1 {
            continue;
        }

        let pivot_index = end - 1;
        collection.swap(pivot_index, end - 1);

        let mut partition_index = start;
        for i in start..end - 1 {
            if (collection[i] <= collection[end - 1] && !reverse)
                || (collection[i] >= collection[end - 1] && reverse)
            {
                collection.swap(partition_index, i);
                partition_index += 1;
            }
        }

        collection.swap(partition_index, end - 1);

        if partition_index > start {
            stack.push((start, partition_index));
        }
        if partition_index + 1 < end {
            stack.push((partition_index + 1, end));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn quick_sort_tests() {
        quick_sort_test_helper(vec![4, 2, 5, 3, 1]);
        quick_sort_test_helper(vec![3, 3, 3, 1, 2, 5, 3, 1, 32, 5, 213, 33]);
        quick_sort_test_helper(vec!["b", "c", "a", "yyy", "aaa", "oleg", "111"]);
        quick_sort_test_helper(vec![(1, "aa"), (0, "bb"), (15, "00")]);
    }

    fn quick_sort_test_helper<T>(mut numbers: Vec<T>)
    where
        T: Ord + Clone + Debug,
    {
        let mut sort_numbers = numbers.clone();
        sort_numbers.sort();

        quick_sort(&mut numbers, false);
        assert_eq!(numbers, sort_numbers);
    }
}
