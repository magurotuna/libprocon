use cargo_snippet::snippet;

#[snippet("GCD")]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet("LCM", include = "GCD")]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet("PRIME")]
pub trait Prime {
    fn lower_primes(&self) -> Vec<usize>;
    fn factorize(&self) -> std::collections::HashMap<usize, usize>;
}

#[snippet("PRIME")]
impl Prime for usize {
    /// エラトステネスの篩を用いてself以下の素数を求める
    /// 計算量: O(n log log n)
    fn lower_primes(&self) -> Vec<usize> {
        let &this = self;
        let mut v = Vec::new();
        if this < 2 {
            return v;
        }
        let mut deque = (2..(this + 1)).collect::<std::collections::VecDeque<usize>>();

        let mut p = match deque.pop_front() {
            Some(x) => x,
            None => return v,
        };
        v.push(p);
        while p as f64 <= (this as f64).sqrt() {
            deque = deque.iter().filter(|&&x| x % p != 0).map(|x| *x).collect();
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
    fn factorize(&self) -> std::collections::HashMap<usize, usize> {
        let mut ret = std::collections::HashMap::new();
        let primes = ((*self as f64).sqrt() as usize).lower_primes();

        let mut tmp = *self;
        for prime in primes {
            while tmp % prime == 0 {
                tmp /= prime;
                *ret.entry(prime).or_insert(0) += 1;
            }
        }
        if tmp > 1 {
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
