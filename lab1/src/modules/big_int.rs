use std::{
    cmp::{max, Ordering},
    fmt::Display,
    ops::{Add, Sub},
    rc::Rc,
};

#[derive(Debug, Eq)]
pub struct BigInt {
    digits: Rc<Vec<i8>>,
    negative: bool,
}

impl BigInt {
    pub fn from(mut n: i64) -> Self {
        let mut digits = Vec::new();
        let negative = n < 0;
        n = n.abs();

        while n > 0 {
            digits.push((n % 10) as i8);
            n /= 10;
        }

        Self {
            digits: Rc::new(digits),
            negative,
        }
    }

    pub fn zero() -> Self {
        Self {
            digits: Rc::new(Vec::new()),
            negative: false,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let negative = s.starts_with('-');
        let s = if negative { &s[1..] } else { s };
        let digits: Vec<i8> = s
            .trim()
            .chars()
            .rev()
            .map(|c| (c as u8 - b'0') as i8)
            .collect();

        Self {
            negative,
            digits: Rc::new(digits),
        }
    }

    fn remove_zeros(digits: &mut Vec<i8>) {
        while digits[digits.len() - 1] == 0 && digits.len() > 1 {
            digits.pop();
        }
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self {
            digits: Rc::new(vec![0]),
            negative: false,
        }
    }
}

impl Clone for BigInt {
    fn clone(&self) -> Self {
        Self {
            digits: self.digits.clone(),
            negative: self.negative,
        }
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl Add for &BigInt {
    type Output = BigInt;

    fn add(self, other: Self) -> Self::Output {
        if self.negative == other.negative {
            let mut digits = Vec::new();
            let mut number_transfer = 0;

            for i in 0..(max(self.digits.len(), other.digits.len())) {
                let self_digit = *self.digits.get(i).unwrap_or(&0);
                let other_digit = *other.digits.get(i).unwrap_or(&0);
                let sum = self_digit + other_digit + number_transfer;

                number_transfer = sum / 10;
                digits.push(sum % 10);
            }

            if number_transfer != 0 {
                digits.push(number_transfer);
            }

            BigInt {
                digits: digits.into(),
                negative: self.negative,
            }
        } else if self.negative {
            other
                - &BigInt {
                    digits: Rc::clone(&self.digits),
                    negative: false,
                }
        } else {
            self - &BigInt {
                digits: Rc::clone(&other.digits),
                negative: false,
            }
        }
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl Sub for &BigInt {
    type Output = BigInt;

    fn sub(mut self, mut other: Self) -> Self::Output {
        if self.negative == other.negative {
            let mut negative = false;

            match self.partial_cmp(other) {
                Some(Ordering::Less) => {
                    negative = true;
                    (self, other) = (other, self);
                }
                Some(Ordering::Equal) => return BigInt::from_str("0"),
                _ => {}
            }

            let mut digits = Vec::new();
            let mut borrow_number = 0;

            for i in 0..max(self.digits.len(), other.digits.len()) {
                let self_digit = *self.digits.get(i).unwrap_or(&0);
                let other_digit = *other.digits.get(i).unwrap_or(&0);
                let difference = self_digit - other_digit - borrow_number;

                if difference < 0 {
                    borrow_number = 1;
                    digits.push(difference + 10);
                } else {
                    borrow_number = 0;
                    digits.push(difference);
                }
            }

            BigInt::remove_zeros(&mut digits);

            BigInt {
                digits: digits.into(),
                negative,
            }
        } else {
            &BigInt {
                digits: Rc::clone(&other.digits),
                negative: false,
            } + self
        }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.negative { "-" } else { "" },
            self.digits
                .iter()
                .rev()
                .map(|digit| (*digit as u8 + b'0') as char)
                .collect::<String>()
        )
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.digits == other.digits && self.negative == other.negative
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.negative != other.negative {
            if self.negative {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            let (current, other) = if self.negative {
                (other, self)
            } else {
                (self, other)
            };
            for i in (0..max(current.digits.len(), other.digits.len())).rev() {
                let self_digit = *current.digits.get(i).unwrap_or(&0);
                let other_digit = *other.digits.get(i).unwrap_or(&0);

                if self_digit < other_digit {
                    return Some(Ordering::Less);
                } else if self_digit > other_digit {
                    return Some(Ordering::Greater);
                }
            }
            Some(Ordering::Equal)
        }
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        let n = BigInt::from_str("-12345");
        assert_eq!(*n.digits, vec![5, 4, 3, 2, 1]);
        assert_eq!(n.negative, true);
    }

    #[test]
    fn add_test() {
        let n1 = BigInt::from_str("12345");
        let n2 = BigInt::from_str("54321");
        let n3 = BigInt::from_str("-54321");
        let n4 = BigInt::from_str("-12345");

        assert_eq!((&n1 + &n2).to_string(), "66666");
        assert_eq!((&n1 + &n4).to_string(), "0");
        assert_eq!((&n2 + &n3).to_string(), "0");
        assert_eq!((&n3 + &n2).to_string(), "0");
        assert_eq!((&n3 + &n4).to_string(), "-66666");
        assert_eq!((&n1 + &n3).to_string(), "-41976");
        assert_eq!((&n3 + &n1).to_string(), "-41976");
    }

    #[test]
    fn cmp_test() {
        let n1 = BigInt::from_str("12345");
        let n2 = BigInt::from_str("54321");
        let n3 = BigInt::from_str("-54321");
        let n4 = BigInt::from_str("-12345");
        let n5 = BigInt::from_str("0");
        let n6 = BigInt::from_str("99999999999999");

        assert!(n1 > n4);
        assert!(n1 < n2);
        assert!(n1 > n3);
        assert!(n2 > n3);
        assert!(n3 < n2);
        assert!(n4 < n1);
        assert!(n5 < n6);
        assert!(n4 < n6);
        assert!(n5 > n3);
    }

    #[test]
    fn sub_test() {
        let n1 = BigInt::from_str("9999999999999999999999999");
        let n2 = BigInt::from_str("333333");
        let n3 = BigInt::from_str("-9999999999999999999999999");
        let n4 = BigInt::from_str("1111111999999999999999991");

        assert_eq!((&n1 - &n2).to_string(), "9999999999999999999666666");
        assert_eq!((&n3 - &n4).to_string(), "-8888888000000000000000008");

        let n1 = BigInt::from_str("12345");
        let n3 = BigInt::from_str("54321");

        assert_eq!((&n1 - &n3).to_string(), "-41976");
    }

    #[test]
    fn large_number_test() {
        let n1 = BigInt {
            digits: Rc::new(vec![8; 1000]),
            negative: false,
        };
        let n2 = BigInt {
            digits: Rc::new(vec![1; 1000]),
            negative: false,
        };

        assert_eq!((n1 + n2).to_string(), "9".repeat(1000));
    }
}
