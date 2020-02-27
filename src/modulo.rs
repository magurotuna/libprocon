use cargo_snippet::snippet;

/// 累乗のmod
/// cf. https://github.com/hatoo/competitive-rust-snippets/blob/master/src/modulo.rs
#[snippet("MOD_INT")]
pub fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
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
#[snippet("MOD_INT")]
pub fn mod_inv(a: i64, m: i64) -> i64 {
    use std::mem::swap;
    let mut a = a;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        swap(&mut a, &mut b);
        u -= t * v;
        swap(&mut u, &mut v);
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    u
}

#[snippet("MOD_INT")]
#[derive(Debug, Clone, Copy)]
pub struct ModInt {
    value: i64,
    modulo: i64,
}

#[snippet("MOD_INT")]
impl ModInt {
    pub fn new(value: i64, modulo: i64) -> Self {
        let r = value % modulo;
        Self {
            value: if r < 0 { r + modulo } else { r },
            modulo,
        }
    }
}

#[snippet("MOD_INT")]
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[snippet("MOD_INT")]
macro_rules! impl_mod_int {
    ( $( $t: ty )* ) => (
        $(
            impl std::cmp::PartialEq<$t> for ModInt {
                fn eq(&self, other: &$t) -> bool {
                    self.value == (*other as i64)
                }
            }

            impl std::ops::Add<$t> for ModInt {
                type Output = Self;

                fn add(self, other: $t) -> Self {
                    Self::new(self.value + (other as i64), self.modulo)
                }
            }

            impl std::ops::Sub<$t> for ModInt {
                type Output = Self;

                fn sub(self, other: $t) -> Self {
                    Self::new(self.value - (other as i64), self.modulo)
                }
            }

            impl std::ops::Mul<$t> for ModInt {
                type Output = Self;

                fn mul(self, other: $t) -> Self {
                    Self::new(self.value * (other as i64), self.modulo)
                }
            }

            impl std::ops::Div<$t> for ModInt {
                type Output = Self;

                fn div(self, other: $t) -> Self {
                    let inv = mod_inv(other as i64, self.modulo);
                    Self::new(self.value * inv, self.modulo)
                }
            }
        )*
    )
}

#[snippet("MOD_INT")]
impl_mod_int!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_mod_int() {
        let value = 43;
        let modulo = 13;
        let mint = ModInt::new(value, modulo);
        assert_eq!(mint, 4);
        assert_eq!(mint + 10, 1);
        assert_eq!(mint * 10, 1);
        assert_eq!(mint / 8, 7);
    }
}
