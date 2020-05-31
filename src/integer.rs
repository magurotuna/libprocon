use cargo_snippet::snippet;

#[snippet("INT_TRAIT")]
#[snippet("INT")]
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
    fn sqrt_floor(self) -> Self {
        if self < Self::zero() {
            return Self::zero();
        }
        let two = Self::one().next();
        let mut ok = Self::zero();
        let mut ng = self.next();
        while ng - ok > Self::one() {
            let mid = (ng + ok) / two;
            if mid * mid <= self {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
    fn is_one(&self) -> bool {
        *self == Self::one()
    }
    fn div_rem(&self, other: &Self) -> (Self, Self) {
        ((*self) / (*other), (*self) % (*other))
    }
    fn div_floor(&self, other: &Self) -> Self {
        // Algorithm from [Daan Leijen. _Division and Modulus for Computer Scientists_,
        // December 2001](http://research.microsoft.com/pubs/151917/divmodnote-letter.pdf)
        let (d, r) = self.div_rem(other);
        if (r > Self::zero() && *other < Self::zero())
            || (r < Self::zero() && *other > Self::zero())
        {
            d.prev()
        } else {
            d
        }
    }
    fn mod_floor(&self, other: &Self) -> Self {
        let r = *self % *other;
        if (r > Self::zero() && *other < Self::zero())
            || (r < Self::zero() && *other > Self::zero())
        {
            r + *other
        } else {
            r
        }
    }
}

#[snippet("INT_TRAIT")]
#[snippet("INT")]
macro_rules! impl_int_for_numerics {
    ( $( $t: ty )* ) => {
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
            }
        )*
    }
}

#[snippet("INT_TRAIT")]
#[snippet("INT")]
impl_int_for_numerics!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

#[snippet("INT")]
#[snippet("GCD")]
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

#[snippet("INT")]
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Int,
{
    a / gcd(a, b) * b
}

#[snippet("INT")]
pub fn divosors<T>(n: T) -> Vec<T>
where
    T: Int,
{
    let mut ret = Vec::new();
    let mut cur = T::one();
    loop {
        if cur * cur > n {
            break;
        }
        if n % cur == T::zero() {
            ret.push(cur);
            if cur * cur != n {
                ret.push(n / cur);
            }
        }
        cur = cur.next();
    }
    ret.sort_unstable();
    ret
}

/// Time complexity: O(n log log n)
#[snippet("INT")]
pub fn lower_primes<T>(n: T) -> Vec<T>
where
    T: Int,
{
    let mut ret = Vec::new();
    if n <= T::one() {
        return ret;
    }
    let mut deque = std::collections::VecDeque::new();
    let mut t = T::one().next(); // 2, which is the first prime number

    // prepare `2, 3, 4, ..., this` sequence
    while t <= n {
        deque.push_back(t);
        t = t.next();
    }

    let mut p = match deque.pop_front() {
        Some(x) => x,
        None => return ret,
    };
    ret.push(p);
    while p * p <= n {
        deque = deque
            .iter()
            .filter(|&&x| x % p != T::zero())
            .copied()
            .collect();
        p = match deque.pop_front() {
            Some(x) => x,
            None => return ret,
        };
        ret.push(p);
    }
    for n in deque {
        ret.push(n);
    }
    ret
}

/// Time complexity: O(sqrt(n))
#[snippet("INT")]
pub fn factorize<T>(n: T) -> std::collections::HashMap<T, usize>
where
    T: Int,
{
    let mut ret = std::collections::HashMap::new();
    if n <= T::one() {
        return ret;
    }
    let mut n = n;
    let mut cur = T::one().next(); // 2
    loop {
        if cur * cur > n {
            break;
        }
        if n % cur != T::zero() {
            cur = cur.next();
            continue;
        }
        let mut exp = 0;
        while n % cur == T::zero() {
            exp += 1;
            n = n / cur;
        }
        ret.insert(cur, exp);
    }
    if n != T::one() {
        ret.insert(n, 1);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use maplit::hashmap;

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
    fn test_divisors() {
        assert_eq!(divosors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divosors(1), vec![1]);
        assert_eq!(divosors(7), vec![1, 7]);
        assert_eq!(divosors(25), vec![1, 5, 25]);
    }

    #[test]
    fn test_lower_primes() {
        assert_eq!(lower_primes(10_usize), vec![2_usize, 3, 5, 7]);
        assert_eq!(lower_primes(15_usize), vec![2_usize, 3, 5, 7, 11, 13]);
        assert!(lower_primes(1_usize).is_empty());
        assert_eq!(lower_primes(2_usize), vec![2_usize]);
        assert_eq!(lower_primes(3_usize), vec![2_usize, 3]);
    }

    #[test]
    fn test_factorize() {
        let result_0: HashMap<usize, usize> = hashmap! {};
        assert_eq!(factorize(0_usize), result_0);

        let result_1: HashMap<usize, usize> = hashmap! {};
        assert_eq!(factorize(1_usize), result_1);

        let result_2 = hashmap! {
            2_usize => 1_usize,
        };
        assert_eq!(factorize(2_usize), result_2);

        let result_10 = hashmap! {
            2_usize => 1_usize,
            5 => 1,
        };
        assert_eq!(factorize(10_usize), result_10);

        let result_12 = hashmap! {
            2_usize => 2_usize,
            3 => 1,
        };
        assert_eq!(factorize(12_usize), result_12);

        let result_99991 = hashmap! {
            99991_usize => 1_usize,
        };
        assert_eq!(factorize(99991_usize), result_99991);

        let result_2013 = hashmap! {
            3_usize => 1_usize,
            11_usize => 1_usize,
            61_usize => 1_usize,
        };
        assert_eq!(factorize(2013_usize), result_2013);
    }
}
