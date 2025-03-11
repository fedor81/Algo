use std::collections::VecDeque;

pub fn merge_sort<T: PartialOrd + Clone>(collection: &Vec<T>, reverse: bool) -> Vec<T> {
    let mut subarrays: VecDeque<Vec<&T>> = collection.iter().map(|i| vec![i]).collect();

    while subarrays.len() > 1 {
        let subarray1 = subarrays.pop_front().unwrap();
        let subarray2 = subarrays.pop_front().unwrap();

        let merged = merge(&subarray1, &subarray2, reverse);
        subarrays.push_back(merged);
    }

    subarrays
        .pop_front()
        .unwrap()
        .iter()
        .map(|i| (*i).clone())
        .collect::<Vec<T>>()
}

fn merge<'a, T>(sequence1: &[&'a T], sequence2: &[&'a T], reverse: bool) -> Vec<&'a T>
where
    T: PartialOrd,
{
    let mut result = Vec::with_capacity(sequence1.len() + sequence2.len());
    let mut pointer1 = 0;
    let mut pointer2 = 0;

    while pointer1 < sequence1.len() && pointer2 < sequence2.len() {
        if (*sequence1[pointer1] < *sequence2[pointer2]) ^ reverse {
            result.push(sequence1[pointer1]);
            pointer1 += 1;
        } else {
            result.push(sequence2[pointer2]);
            pointer2 += 1;
        }
    }

    result.extend_from_slice(&sequence1[pointer1..]);
    result.extend_from_slice(&sequence2[pointer2..]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn merge_sort_tests() {
        merge_sort_test_helper(vec![4, 2, 5, 3, 1]);
        merge_sort_test_helper(vec![3, 3, 3, 1, 2, 5, 3, 1, 32, 5, 213, 33]);
        merge_sort_test_helper(vec!["b", "c", "a", "yyy", "aaa", "oleg", "111"]);
        merge_sort_test_helper(vec![(1, "aa"), (0, "bb"), (15, "00")]);
        merge_sort_test_helper(vec![1000000; 1000000]);
    }

    fn merge_sort_test_helper<T>(mut numbers: Vec<T>)
    where
        T: Ord + Clone + Debug,
    {
        let mut excepted = numbers.clone();
        excepted.sort();

        let result = merge_sort(&mut numbers, false);
        assert_eq!(result, excepted);
    }
}
