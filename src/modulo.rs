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
