fn solve(need_sum: usize) -> usize {
    const MAX_DIGITS_LEN: usize = 9;
    const MAX_SUM: usize = 81;

    if need_sum > MAX_SUM {
        panic!("Too big sum");
    }

    // По индексу будем определять количество цифр в числе + 1
    // Во внутреннем векторе по индексу будем хранить количество чисел с данной суммой + 1
    let mut sum_numbers_count = vec![vec![0usize; MAX_SUM]; MAX_DIGITS_LEN];

    // 1, 2, 3, 4, 5, 6, 7, 8, 9
    for i in 0..9 {
        sum_numbers_count[0][i] = 1;
    }

    for i in 1..MAX_DIGITS_LEN {
        for curr_sum in 0..MAX_SUM {
            for digit in 0..10 {
                let sum = curr_sum + digit;
                if sum >= need_sum {
                    break;
                }
                sum_numbers_count[i][sum] += sum_numbers_count[i - 1][curr_sum];
            }
        }
    }

    let result = sum_numbers_count.iter().map(|x| x[need_sum - 1]).sum();

    // Миллиард не учитывается в массиве, поэтому делаем руками
    if need_sum == 1 { result + 1 } else { result }
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test() {
        assert_eq!(solve(1), 10);
        assert_eq!(solve(2), 45); // Либо две еденицы, либо одна двойка. Расставить две еденицы на 9 мест можно 36-ю способами, расставить одну двойку можно 9-ю способоами. Итого 45
        assert_eq!(solve(81), 1); // 999_999_999
        assert_eq!(solve(3), 165);
    }
}
