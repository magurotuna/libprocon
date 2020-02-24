use cargo_snippet::snippet;

// ref: https://github.com/hatoo/competitive-rust-snippets/blob/master/src/modulo.rs

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

/// 累乗のmod
#[snippet("MOD_POW")]
pub fn mod_pow(x: u64, n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}

/// mod m での a の逆元を求める
/// m と a が互いに素でなければならないことに注意
/// cf. [「1000000007 で割ったあまり」の求め方を総特集！ 〜 逆元から離散対数まで 〜 - Qiita](https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a)
#[snippet("MOD_INV")]
pub fn mod_inv(a: u64, m: u64) -> u64 {
    use std::mem::swap;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        u -= t * v;
        swap(&mut u, &mut v);
    }
    u %= m as i64;
    if u < 0 {
        u += m as i64;
    }
    u as u64
}

/// 二項係数を mod のもとで求める
/// cf. [よくやる二項係数 (nCk mod. p)、逆元 (a^-1 mod. p) の求め方 - けんちょんの競プロ精進記録](http://drken1215.hatenablog.com/entry/2018/06/08/210000)
#[snippet("COMBINATION")]
pub struct Comb {
    max_size: usize,
    modulo: usize,
    factorical_table: Vec<usize>,
    factorical_inverse_table: Vec<usize>,
    inverse_table: Vec<usize>,
}

#[snippet("COMBINATION")]
impl Comb {
    pub fn new(max_size: usize, modulo: usize) -> Self {
        let max_size = std::cmp::max(10, max_size);

        // 10^7 までしか実用的な速度で計算できない
        assert!(max_size <= 10_000_000);

        let mut factorical_table = vec![0; max_size];
        let mut factorical_inverse_table = vec![0; max_size];
        let mut inverse_table = vec![0; max_size];
        factorical_table[0] = 1;
        factorical_table[1] = 1;
        factorical_inverse_table[0] = 1;
        factorical_inverse_table[1] = 1;
        inverse_table[1] = 1;
        for i in 2..max_size {
            factorical_table[i] = factorical_table[i - 1] * i % modulo;
            inverse_table[i] = modulo - inverse_table[modulo % i] * (modulo / i) % modulo;
            factorical_inverse_table[i] =
                factorical_inverse_table[i - 1] * inverse_table[i] % modulo;
        }
        Self {
            max_size,
            modulo,
            factorical_table,
            factorical_inverse_table,
            inverse_table,
        }
    }

    pub fn calc(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.factorical_table[n]
                * (self.factorical_inverse_table[k] * self.factorical_inverse_table[n - k]
                    % self.modulo)
                % self.modulo
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_mod_pow() {
        let m = 1_000_000_007;
        let x = 1234;
        let mut t = 1;
        for i in 0..1000 {
            assert_eq!(mod_pow(x, i, m), t);
            t = t * x % m;
        }
    }

    #[test]
    fn test_mod_inv() {
        let tests = [
            (1, 13, 1),
            (2, 13, 7),
            (3, 13, 9),
            (4, 13, 10),
            (5, 13, 8),
            (6, 13, 11),
        ];

        for test in tests.iter() {
            assert_eq!(test.2, mod_inv(test.0, test.1));
        }
    }

    #[test]
    fn test_comb() {
        let modulo = 1_000_000_007;
        let comb = Comb::new(1_000_000, modulo);
        let tests = [
            (2, 1, 2),
            (20, 15, 15504),
            (20, 5, 15504),
            (25, 15, 3268760),
            (50, 10, 272278100),
            (666666, 333333, 151840682),
            (10, 9999, 0),
        ];

        for test in tests.iter() {
            assert_eq!(test.2, comb.calc(test.0, test.1));
        }
    }
}
