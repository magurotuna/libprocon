use cargo_snippet::snippet;

#[snippet("INT_TRAIT")]
pub trait Int:
    std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + std::hash::Hash
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Copy
{
    fn zero() -> Self;
    fn one() -> Self;
    fn next(self) -> Self;
    fn prev(self) -> Self;
    fn sqrt_floor(self) -> Self;
}

#[snippet("INT_TRAIT")]
macro_rules! impl_int_for_numerics {
    ( $( $t: ty )* ) => (
        $(
            impl Int for $t {
                fn zero() -> Self {
                    0
                }
                fn one() -> Self {
                    1
                }
                fn next(self) -> Self {
                    self + Self::one()
                }
                fn prev(self) -> Self {
                    self - Self::one()
                }
                fn sqrt_floor(self) -> Self {
                    if self < Self::zero() {
                        return Self::zero();
                    }
                    let two = Self::one().next();
                    let mut ok = Self::zero();
                    let mut ng = self.next();
                    while ng - ok > 1 {
                        let mid = (ng + ok) / two;
                        if mid * mid <= self {
                            ok = mid;
                        } else {
                            ng = mid;
                        }
                    }
                    ok
                }
            }
        )*
    )
}

#[snippet("INT_TRAIT")]
impl_int_for_numerics!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

#[snippet("GCD", include = "INT_TRAIT")]
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Int,
{
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet("LCM", include = "GCD", include = "INT_TRAIT")]
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Int,
{
    a / gcd(a, b) * b
}

#[snippet("PRIME", include = "INT_TRAIT")]
pub trait Prime<T: Int> {
    fn lower_primes(&self) -> Vec<T>;
    fn factorize(&self) -> std::collections::HashMap<T, usize>;
}

#[snippet("PRIME", include = "INT_TRAIT")]
impl<T> Prime<T> for T
where
    T: Int,
{
    /// エラトステネスの篩を用いてself以下の素数を求める
    /// 計算量: O(n log log n)
    fn lower_primes(&self) -> Vec<T> {
        let &this = self;
        let mut v = Vec::new();
        if this <= T::one() {
            return v;
        }
        let mut deque = std::collections::VecDeque::new();
        let mut t = T::one().next(); // 2, which is the first prime number

        // prepare `2, 3, 4, ..., this` sequence
        while t <= this {
            deque.push_back(t);
            t = t.next();
        }

        let mut p = match deque.pop_front() {
            Some(x) => x,
            None => return v,
        };
        v.push(p);
        while p * p <= this {
            deque = deque
                .iter()
                .filter(|&&x| x % p != T::zero())
                .copied()
                .collect();
            p = match deque.pop_front() {
                Some(x) => x,
                None => return v,
            };
            v.push(p);
        }
        for n in deque {
            v.push(n);
        }
        v
    }

    /// エラトステネスの篩を用いてselfを素因数分解する
    fn factorize(&self) -> std::collections::HashMap<T, usize> {
        let mut ret = std::collections::HashMap::new();
        let primes = self.sqrt_floor().lower_primes();

        let mut tmp = *self;
        for prime in primes {
            while tmp % prime == T::zero() {
                tmp = tmp / prime;
                *ret.entry(prime).or_insert(0) += 1;
            }
        }
        if tmp > T::one() {
            *ret.entry(tmp).or_insert(0) += 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_sqrt_floor() {
        let tests = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (8, 2),
            (9, 3),
            (99, 9),
            (100, 10),
            (-1, 0),
        ];
        for test in tests.iter() {
            assert_eq!(test.0.sqrt_floor(), test.1);
        }
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(3, 7), 1);
        assert_eq!(gcd(10, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(14, 21), 42);
    }

    #[test]
    fn test_lower_primes() {
        assert_eq!(10_usize.lower_primes(), vec![2_usize, 3, 5, 7]);
        assert_eq!(15_usize.lower_primes(), vec![2_usize, 3, 5, 7, 11, 13]);
        assert_eq!(1_usize.lower_primes(), vec![]);
        assert_eq!(2_usize.lower_primes(), vec![2_usize]);
    }

    #[test]
    fn test_factorize() {
        let mut result_10 = HashMap::new();
        result_10.insert(2_usize, 1_usize);
        result_10.insert(5_usize, 1_usize);
        assert_eq!(10_usize.factorize(), result_10);

        let mut result_12 = HashMap::new();
        result_12.insert(2_usize, 2_usize);
        result_12.insert(3_usize, 1_usize);
        assert_eq!(12_usize.factorize(), result_12);

        let result_1 = HashMap::new();
        assert_eq!(1_usize.factorize(), result_1);

        let result_0 = HashMap::new();
        assert_eq!(0_usize.factorize(), result_0);

        let mut result_99991 = HashMap::new();
        result_99991.insert(99991_usize, 1_usize);
        assert_eq!(99991_usize.factorize(), result_99991);

        let mut result_2013 = HashMap::new();
        result_2013.insert(3_usize, 1_usize);
        result_2013.insert(11_usize, 1_usize);
        result_2013.insert(61_usize, 1_usize);
        assert_eq!(2013_usize.factorize(), result_2013);
    }
}
