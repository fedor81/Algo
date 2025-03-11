pub fn bubble_sort<T, I>(enumerate: I) -> Vec<T>
where
    T: Ord,
    I: IntoIterator<Item = T>,
{
    let mut vec: Vec<T> = enumerate.into_iter().collect();

    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 - i {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }

    vec
}
