// Mostly referred to: https://github.com/rust-num/num-rational

use crate::integer::{gcd, Int};
use cargo_snippet::snippet;

#[snippet("RATIONAL")]
#[derive(Clone, Copy)]
pub struct Rational<T: Int> {
    /// Numerator
    numerator: T,
    /// Denomitor
    denomitor: T,
}

impl<T> std::fmt::Display for Rational<T>
where
    T: Int + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.numerator, self.denomitor)
    }
}

impl<T> std::fmt::Debug for Rational<T>
where
    T: Int + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.numerator, self.denomitor)
    }
}

impl<T: Int> Rational<T> {
    pub fn new(numerator: T, denomitor: T) -> Rational<T> {
        let mut r = Rational {
            numerator,
            denomitor,
        };
        r.reduce();
        r
    }

    pub fn from_integer(t: T) -> Rational<T> {
        Rational::new(t, T::one())
    }

    pub fn to_integer(&self) -> T {
        self.truncate().numerator
    }

    /// Round value towards zero.
    pub fn truncate(&self) -> Rational<T> {
        Rational::from_integer(self.numerator / self.denomitor)
    }

    /// Round value towards minus infinity.
    pub fn floor(&self) -> Rational<T> {
        if *self < T::zero() {
            Rational::from_integer((self.numerator - self.denomitor + T::one()) / self.denomitor)
        } else {
            Rational::from_integer(self.numerator / self.denomitor)
        }
    }

    /// Round value towards plus infinity.
    pub fn ceil(&self) -> Rational<T> {
        if *self < T::zero() {
            Rational::from_integer(self.numerator / self.denomitor)
        } else {
            Rational::from_integer((self.numerator + self.denomitor - T::one()) / self.denomitor)
        }
    }

    /// Puts self into lowest terms, with denomitor > 0.
    fn reduce(&mut self) {
        assert!(!self.denomitor.is_zero());

        if self.numerator.is_zero() {
            self.denomitor = T::one();
            return;
        }
        if self.numerator == self.denomitor {
            self.numerator = T::one();
            self.denomitor = T::one();
            return;
        }

        let g = gcd(self.numerator, self.denomitor);
        self.numerator = self.numerator / g;
        self.denomitor = self.denomitor / g;

        // keep denomitor positive
        if self.denomitor < T::zero() {
            self.numerator = T::zero() - self.numerator;
            self.denomitor = T::zero() - self.denomitor;
        }
    }
}

impl<T> From<T> for Rational<T>
where
    T: Int,
{
    fn from(x: T) -> Self {
        Rational::from_integer(x)
    }
}

impl<T> From<(T, T)> for Rational<T>
where
    T: Int,
{
    fn from((numerator, denomitor): (T, T)) -> Self {
        Rational::new(numerator, denomitor)
    }
}

impl<T> PartialEq<Self> for Rational<T>
where
    T: Int,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl<T> Eq for Rational<T> where T: Int {}

impl<T> PartialEq<T> for Rational<T>
where
    T: Int,
{
    fn eq(&self, other: &T) -> bool {
        let other = Self::from_integer(*other);
        *self == other
    }
}

macro_rules! impl_partial_eq_with_rational_for_numerics {
    ( $( $t: ty )* ) => {
        $(
            impl PartialEq<Rational<$t>> for $t {
                fn eq(&self, other: &Rational<$t>) -> bool {
                    let r = Rational::from_integer(*self);
                    r == *other
                }
            }
        )*
    }
}
impl_partial_eq_with_rational_for_numerics!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

impl<T> Ord for Rational<T>
where
    T: Int,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.denomitor == other.denomitor {
            return if self.denomitor > T::zero() {
                self.numerator.cmp(&other.numerator)
            } else {
                self.numerator.cmp(&other.numerator).reverse()
            };
        }

        if self.numerator == other.numerator {
            return match self.numerator.cmp(&T::zero()) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                std::cmp::Ordering::Greater => self.denomitor.cmp(&other.denomitor).reverse(),
                std::cmp::Ordering::Less => self.denomitor.cmp(&other.denomitor),
            };
        }

        let self_int = self.numerator.div_floor(&self.denomitor);
        let self_rem = self.numerator.mod_floor(&self.denomitor);
        let other_int = other.numerator.div_floor(&other.denomitor);
        let other_rem = other.numerator.mod_floor(&other.denomitor);
        match self_int.cmp(&other_int) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => match (self_rem.is_zero(), other_rem.is_zero()) {
                (true, true) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                (false, false) => {
                    let self_new = Rational::new(self.denomitor, self_rem);
                    let other_new = Rational::new(other.denomitor, other_rem);
                    self_new.cmp(&other_new).reverse()
                }
            },
        }
    }
}

impl<T> PartialOrd<Self> for Rational<T>
where
    T: Int,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialOrd<T> for Rational<T>
where
    T: Int,
{
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        let other = Self::from_integer(*other);
        Some(self.cmp(&other))
    }
}

macro_rules! impl_partial_ord_with_rational_for_numerics {
    ( $( $t: ty )* ) => {
        $(
            impl PartialOrd<Rational<$t>> for $t {
                fn partial_cmp(&self, other: &Rational<$t>) -> Option<std::cmp::Ordering> {
                    let r = Rational::from_integer(*self);
                    Some(r.cmp(other))
                }
            }
        )*
    }
}
impl_partial_ord_with_rational_for_numerics!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

impl<T> std::ops::Add<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let denom_gcd = gcd(self.denomitor, rhs.denomitor);
        let denom_lcm = (self.denomitor.mul(rhs.denomitor)).div(denom_gcd);
        let self_mul = (rhs.denomitor).div(denom_gcd);
        let rhs_mul = (self.denomitor).div(denom_gcd);
        let numer = self.numerator.mul(self_mul).add(rhs.numerator.mul(rhs_mul));
        Rational::new(numer, denom_lcm)
    }
}

impl<T> std::ops::Add<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = Self::from_integer(rhs);
        self.add(rhs)
    }
}

impl<T> std::ops::AddAssign<Self> for Rational<T>
where
    T: Int,
{
    fn add_assign(&mut self, other: Self) {
        let add = std::ops::Add::<Self>::add(*self, other);
        self.numerator = add.numerator;
        self.denomitor = add.denomitor;
    }
}

impl<T> std::ops::AddAssign<T> for Rational<T>
where
    T: Int,
{
    fn add_assign(&mut self, other: T) {
        let add = std::ops::Add::<T>::add(*self, other);
        self.numerator = add.numerator;
        self.denomitor = add.denomitor;
    }
}

impl<T> std::ops::Sub<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let denom_gcd = gcd(self.denomitor, rhs.denomitor);
        let denom_lcm = self.denomitor.mul(rhs.denomitor).div(denom_gcd);
        let self_mul = rhs.denomitor.div(denom_gcd);
        let rhs_mul = self.denomitor.div(denom_gcd);
        let numer = self.numerator.mul(self_mul).sub(rhs.numerator.mul(rhs_mul));
        Rational::new(numer, denom_lcm)
    }
}

impl<T> std::ops::Sub<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = Self::from_integer(rhs);
        self - rhs
    }
}

impl<T> std::ops::SubAssign<Self> for Rational<T>
where
    T: Int,
{
    fn sub_assign(&mut self, other: Self) {
        let sub = *self - other;
        self.numerator = sub.numerator;
        self.denomitor = sub.denomitor;
    }
}

impl<T> std::ops::SubAssign<T> for Rational<T>
where
    T: Int,
{
    fn sub_assign(&mut self, other: T) {
        let sub = *self - other;
        self.numerator = sub.numerator;
        self.denomitor = sub.denomitor;
    }
}

impl<T> std::ops::Mul<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let gcd1 = gcd(self.numerator, rhs.denomitor);
        let gcd2 = gcd(self.denomitor, rhs.numerator);
        let num1 = self.numerator.div(gcd1);
        let den1 = self.denomitor.div(gcd2);
        let num2 = rhs.numerator.div(gcd2);
        let den2 = rhs.denomitor.div(gcd1);
        Self::new(num1 * num2, den1 * den2)
    }
}

impl<T> std::ops::Mul<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = Self::from_integer(rhs);
        self * rhs
    }
}

impl<T> std::ops::MulAssign<Self> for Rational<T>
where
    T: Int,
{
    fn mul_assign(&mut self, rhs: Self) {
        let mul = *self * rhs;
        self.numerator = mul.numerator;
        self.denomitor = mul.denomitor;
    }
}

impl<T> std::ops::MulAssign<T> for Rational<T>
where
    T: Int,
{
    fn mul_assign(&mut self, rhs: T) {
        let mul = *self * rhs;
        self.numerator = mul.numerator;
        self.denomitor = mul.denomitor;
    }
}

impl<T> std::ops::Div<Self> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inv = Self::new(rhs.denomitor, rhs.numerator);
        std::ops::Mul::<Rational<T>>::mul(self, rhs_inv)
    }
}

impl<T> std::ops::Div<T> for Rational<T>
where
    T: Int,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs_inv = Self::new(T::one(), rhs);
        std::ops::Mul::<Rational<T>>::mul(self, rhs_inv)
    }
}

impl<T> std::ops::DivAssign<Self> for Rational<T>
where
    T: Int,
{
    fn div_assign(&mut self, rhs: Self) {
        let div = *self / rhs;
        self.numerator = div.numerator;
        self.denomitor = div.denomitor;
    }
}

impl<T> std::ops::DivAssign<T> for Rational<T>
where
    T: Int,
{
    fn div_assign(&mut self, rhs: T) {
        let div = *self / rhs;
        self.numerator = div.numerator;
        self.denomitor = div.denomitor;
    }
}

macro_rules! impl_ops_for_numerics {
    ( $( $t: ty )* ) => {
        $(
            impl std::ops::Add<Rational<$t>> for $t {
                type Output = Rational<$t>;
                fn add(self, rhs: Rational<$t>) -> Self::Output {
                    Rational::from_integer(self).add(rhs)
                }
            }
            impl std::ops::Sub<Rational<$t>> for $t {
                type Output = Rational<$t>;
                fn sub(self, rhs: Rational<$t>) -> Self::Output {
                    Rational::from_integer(self).sub(rhs)
                }
            }
            impl std::ops::Mul<Rational<$t>> for $t {
                type Output = Rational<$t>;
                fn mul(self, rhs: Rational<$t>) -> Self::Output {
                    Rational::from_integer(self).mul(rhs)
                }
            }
            impl std::ops::Div<Rational<$t>> for $t {
                type Output = Rational<$t>;
                fn div(self, rhs: Rational<$t>) -> Self::Output {
                    Rational::from_integer(self).div(rhs)
                }
            }
        )*
    }
}
impl_ops_for_numerics!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_positive_eq() {
        let r1 = Rational::new(2, 3);
        let r2 = Rational::new(4, 6);
        let r3 = Rational::new(-2, -3);
        let r4 = Rational::new(2, 4);

        assert_eq!(r1, r2);
        assert_eq!(r1, r3);
        assert_ne!(r1, r4);
    }

    #[test]
    fn new_negative_eq() {
        let r1 = Rational::new(-2, 3);
        let r2 = Rational::new(2, -3);
        let r3 = Rational::new(-6, 9);
        let r4 = Rational::new(6, -9);
        let r5 = Rational::new(-2, -3);

        assert_eq!(r1, r2);
        assert_eq!(r1, r3);
        assert_eq!(r1, r4);
        assert_ne!(r1, r5);
    }

    #[test]
    fn new_zero_eq() {
        let r1 = Rational::new(0, 3);
        let r2 = Rational::new(0, 9);
        let r3 = Rational::new(0, 1000);
        let r4 = Rational::new(0, -5);
        let r5 = Rational::new(-0, 5);
        assert_eq!(r1, r2);
        assert_eq!(r1, r3);
        assert_eq!(r1, r4);
        assert_eq!(r1, r5);
    }

    #[test]
    fn from_integer_eq() {
        let new = Rational::new(7, 1);
        let from_integer = Rational::from_integer(7);
        assert_eq!(new, from_integer);

        let new = Rational::new(-7, 1);
        let from_integer = Rational::from_integer(-7);
        assert_eq!(new, from_integer);

        let new = Rational::new(7, -1);
        let from_integer = Rational::from_integer(-7);
        assert_eq!(new, from_integer);
    }

    #[test]
    fn to_integer() {
        assert_eq!(1, Rational::new(4, 3).to_integer());
        assert_eq!(1, Rational::new(5, 3).to_integer());
        assert_eq!(1, Rational::new(3, 3).to_integer());
        assert_eq!(2, Rational::new(6, 3).to_integer());
        assert_eq!(-1, Rational::new(-4, 3).to_integer());
        assert_eq!(-1, Rational::new(-3, 3).to_integer());
        assert_eq!(-2, Rational::new(-6, 3).to_integer());
        assert_eq!(0, Rational::new(0, 3).to_integer());
        assert_eq!(0, Rational::new(1, 3).to_integer());
        assert_eq!(0, Rational::new(-1, 3).to_integer());
    }

    #[test]
    fn cmp_with_self() {
        assert!(Rational::new(3, 2) < Rational::new(5, 2));
        assert!(Rational::new(3, 2) < Rational::new(8, 5));
        assert!(Rational::new(3, 2) > Rational::new(1, 2));
        assert!(Rational::new(3, 2) > Rational::new(7, 5));
        assert!(Rational::new(-3, 2) < Rational::new(1, 2));
        assert!(Rational::new(-3, 2) < Rational::new(0, 2));
        assert!(Rational::new(-3, 2) < Rational::new(-7, 5));
    }

    #[test]
    fn cmp_with_int() {
        assert!(Rational::new(3, 2) < 2);
        assert!(1 < Rational::new(3, 2));
        assert!(-1 < Rational::new(0, 2));
        assert!(-1 < Rational::new(-1, 2));
        assert!(Rational::new(-7, 2) < -3);
    }

    #[test]
    fn truncate() {
        assert_eq!(1, Rational::new(4, 3).truncate());
        assert_eq!(1, Rational::new(5, 3).truncate());
        assert_eq!(1, Rational::new(3, 3).truncate());
        assert_eq!(2, Rational::new(6, 3).truncate());
        assert_eq!(-1, Rational::new(-4, 3).truncate());
        assert_eq!(-1, Rational::new(-3, 3).truncate());
        assert_eq!(-2, Rational::new(-6, 3).truncate());
        assert_eq!(0, Rational::new(0, 3).truncate());
        assert_eq!(0, Rational::new(1, 3).truncate());
        assert_eq!(0, Rational::new(-1, 3).truncate());
    }

    #[test]
    fn floor() {
        assert_eq!(1, Rational::new(7, 4).floor());
        assert_eq!(1, Rational::new(4, 4).floor());
        assert_eq!(0, Rational::new(3, 4).floor());
        assert_eq!(-1, Rational::new(-3, 4).floor());
        assert_eq!(-4, Rational::new(-15, 4).floor());
    }

    #[test]
    fn ceil() {
        assert_eq!(1, Rational::new(3, 4).ceil());
        assert_eq!(3, Rational::new(9, 4).ceil());
        assert_eq!(1, Rational::new(4, 4).ceil());
        assert_eq!(0, Rational::new(0, 4).ceil());
        assert_eq!(0, Rational::new(-3, 4).ceil());
        assert_eq!(-1, Rational::new(-4, 4).ceil());
        assert_eq!(-1, Rational::new(-5, 4).ceil());
        assert_eq!(-3, Rational::new(-13, 4).ceil());
    }

    #[test]
    fn add_with_self() {
        assert_eq!(
            Rational::new(3, 4) + Rational::new(1, 4),
            Rational::from_integer(1)
        );
        assert_eq!(
            Rational::new(1, 2) + Rational::new(1, 3),
            Rational::new(5, 6)
        );
        assert_eq!(
            Rational::new(-1, 2) + Rational::new(1, 3),
            Rational::new(-1, 6)
        );
    }

    #[test]
    fn add_with_int() {
        assert_eq!(Rational::new(3, 4) + 1, Rational::new(7, 4));
        assert_eq!(-2 + Rational::new(-1, 2), Rational::new(-5, 2));
    }

    #[test]
    fn add_assign_with_self() {
        let mut r = Rational::new(3, 4);
        r += Rational::new(1, 4);
        assert_eq!(r, Rational::from_integer(1));

        let mut r = Rational::new(1, 2);
        r += Rational::new(1, 3);
        assert_eq!(r, Rational::new(5, 6));

        let mut r = Rational::new(-1, 2);
        r += Rational::new(1, 3);
        assert_eq!(r, Rational::new(-1, 6));
    }

    #[test]
    fn sub_with_self() {
        assert_eq!(
            Rational::new(3, 4) - Rational::new(1, 4),
            Rational::new(1, 2)
        );
        assert_eq!(
            Rational::new(1, 2) - Rational::new(1, 3),
            Rational::new(1, 6)
        );
        assert_eq!(
            Rational::new(-1, 2) - Rational::new(1, 3),
            Rational::new(-5, 6)
        );
    }

    #[test]
    fn sub_with_int() {
        assert_eq!(Rational::new(3, 4) - 1, Rational::new(-1, 4));
        assert_eq!(2 - Rational::new(-1, 2), Rational::new(5, 2));
    }

    #[test]
    fn sub_assign_with_self() {
        let mut r = Rational::new(3, 4);
        r -= Rational::new(1, 4);
        assert_eq!(r, Rational::new(1, 2));

        let mut r = Rational::new(1, 2);
        r -= Rational::new(1, 3);
        assert_eq!(r, Rational::new(1, 6));

        let mut r = Rational::new(-1, 2);
        r -= Rational::new(1, 3);
        assert_eq!(r, Rational::new(-5, 6));
    }

    #[test]
    fn mul_with_self() {
        assert_eq!(
            Rational::new(1, 2) * Rational::new(3, 4),
            Rational::new(3, 8)
        );
        assert_eq!(
            Rational::new(3, 4) * Rational::new(5, 6),
            Rational::new(5, 8)
        );
        assert_eq!(
            Rational::new(0, 1) * Rational::new(1234, 9999),
            Rational::from_integer(0)
        );
        assert_eq!(
            Rational::new(-1, 2) * Rational::new(3, 4),
            Rational::new(-3, 8)
        );
        assert_eq!(
            Rational::new(-3, 4) * Rational::new(5, 6),
            Rational::new(-5, 8)
        );
    }

    #[test]
    fn mul_with_int() {
        assert_eq!(Rational::new(3, 4) * 3, Rational::new(9, 4));
        assert_eq!(3 * Rational::new(-1, 2), Rational::new(-3, 2));
    }

    #[test]
    fn mul_may_overflow() {
        assert_eq!(
            Rational::<i32>::new(7, 10_i32.pow(9)) * Rational::<i32>::new(10_i32.pow(5), 63),
            Rational::new(1, 9 * 10_i32.pow(4))
        );

        assert_eq!(
            Rational::<i32>::new(7, 10_i32.pow(9)) * Rational::<i32>::new(-10_i32.pow(5), 63),
            Rational::new(-1, 9 * 10_i32.pow(4))
        );
    }

    #[test]
    fn mul_assign_with_self() {
        let mut r = Rational::new(1, 2);
        r *= Rational::new(3, 4);
        assert_eq!(r, Rational::new(3, 8));

        let mut r = Rational::new(3, 4);
        r *= Rational::new(5, 6);
        assert_eq!(r, Rational::new(5, 8));

        let mut r = Rational::new(0, 1);
        r *= Rational::new(1234, 9999);
        assert_eq!(r, Rational::from_integer(0));

        let mut r = Rational::new(-1, 2);
        r *= Rational::new(3, 4);
        assert_eq!(r, Rational::new(-3, 8));

        let mut r = Rational::new(-3, 4);
        r *= Rational::new(5, 6);
        assert_eq!(r, Rational::new(-5, 8));
    }

    #[test]
    fn div_with_self() {
        assert_eq!(
            Rational::new(1, 2) / Rational::new(3, 4),
            Rational::new(2, 3)
        );
        assert_eq!(
            Rational::new(3, 4) / Rational::new(5, 6),
            Rational::new(9, 10)
        );
        assert_eq!(
            Rational::new(0, 1) / Rational::new(1234, 9999),
            Rational::from_integer(0)
        );
        assert_eq!(
            Rational::new(-1, 2) / Rational::new(3, 4),
            Rational::new(-2, 3)
        );
        assert_eq!(
            Rational::new(-3, 4) / Rational::new(5, 6),
            Rational::new(-9, 10)
        );
    }

    #[test]
    fn div_with_int() {
        assert_eq!(Rational::new(3, 4) / 3, Rational::new(1, 4));
        assert_eq!(3 / Rational::new(-5, 2), Rational::new(-6, 5));
    }

    #[test]
    fn div_assign_with_self() {
        let mut r = Rational::new(1, 2);
        r /= Rational::new(3, 4);
        assert_eq!(r, Rational::new(2, 3));

        let mut r = Rational::new(3, 4);
        r /= Rational::new(5, 6);
        assert_eq!(r, Rational::new(9, 10));

        let mut r = Rational::new(0, 1);
        r /= Rational::new(1234, 9999);
        assert_eq!(r, Rational::from_integer(0));

        let mut r = Rational::new(-1, 2);
        r /= Rational::new(3, 4);
        assert_eq!(r, Rational::new(-2, 3));

        let mut r = Rational::new(-3, 4);
        r /= Rational::new(5, 6);
        assert_eq!(r, Rational::new(-9, 10));
    }
}
