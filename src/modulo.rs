// ref: https://github.com/hatoo/competitive-rust-snippets/blob/master/src/modulo.rs

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// 累乗のmod
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
