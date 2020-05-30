use cargo_snippet::snippet;

#[snippet("BINARY_SEARCH")]
pub trait BinarySearchExt {
    type Item;
    fn lower_bound(&self, value: &Self::Item) -> usize
    where
        Self::Item: Ord;
    fn upper_bound(&self, value: &Self::Item) -> usize
    where
        Self::Item: Ord;
    fn lower_bound_by<P: Fn(&Self::Item) -> bool>(&self, predicate: P) -> usize;
}

#[snippet("BINARY_SEARCH")]
impl<T> BinarySearchExt for [T] {
    type Item = T;

    /// Given a ascending-sorted array, find the minimum index `i`
    /// such that the i-th value in the array is greater than or equal to `value`.
    fn lower_bound(&self, value: &Self::Item) -> usize
    where
        Self::Item: Ord,
    {
        let mut ok = self.len() as i64;
        let mut ng = -1_i64;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self[mid as usize] >= *value {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    /// Given a ascending-sorted array, find the minimum index `i`
    /// such that the i-th value in the array is greater than `value`.
    fn upper_bound(&self, value: &Self::Item) -> usize
    where
        Self::Item: Ord,
    {
        let mut ok = self.len() as i64;
        let mut ng = -1_i64;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self[mid as usize] > *value {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    /// Given a array, find the minimum index `i`
    /// such that we get `true` when passing `i` to `predicate`.
    /// NOTE: The given array must have monotonicity, which means that
    /// there is at most only one point where the boundary between
    /// satisfying the predicate and not satisfying it.
    fn lower_bound_by<P: Fn(&Self::Item) -> bool>(&self, predicate: P) -> usize {
        let mut ok = self.len() as i64;
        let mut ng = -1_i64;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if predicate(&self[mid as usize]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let v = vec![2, 2, 5, 5, 9];
        assert_eq!(v.lower_bound(&1), 0);
        assert_eq!(v.lower_bound(&2), 0);
        assert_eq!(v.lower_bound(&3), 2);
        assert_eq!(v.lower_bound(&5), 2);
        assert_eq!(v.lower_bound(&8), 4);
        assert_eq!(v.lower_bound(&9), 4);
        assert_eq!(v.lower_bound(&10), 5);

        // The number of elements greater than or equal to `4`.
        assert_eq!(v.len() - v.lower_bound(&4), 3);
        // In case of `9`
        assert_eq!(v.len() - v.lower_bound(&9), 1);
    }

    #[test]
    fn test_upper_bound() {
        let v = vec![2, 2, 5, 5, 9];
        assert_eq!(v.upper_bound(&1), 0);
        assert_eq!(v.upper_bound(&2), 2);
        assert_eq!(v.upper_bound(&3), 2);
        assert_eq!(v.upper_bound(&5), 4);
        assert_eq!(v.upper_bound(&8), 4);
        assert_eq!(v.upper_bound(&9), 5);
        assert_eq!(v.upper_bound(&10), 5);

        // The number of elements greater than `4`.
        assert_eq!(v.len() - v.upper_bound(&4), 3);
        // In case of `5`
        assert_eq!(v.len() - v.upper_bound(&5), 1);
    }

    #[test]
    fn test_lower_bound_by() {
        let v = vec![
            "abc",
            "def",
            "ghi",
            "THIS_IS_LONG_STRING",
            "THIS_IS_ALSO_LONG_STRING",
        ];
        assert_eq!(v.lower_bound_by(|s| s.len() >= 2), 0);
        assert_eq!(v.lower_bound_by(|s| s.len() >= 3), 0);
        assert_eq!(v.lower_bound_by(|s| s.len() >= 4), 3);
        assert_eq!(v.lower_bound_by(|s| s.len() >= 19), 3);
        assert_eq!(v.lower_bound_by(|s| s.len() >= 20), 4);
        assert_eq!(v.lower_bound_by(|s| s.len() >= 24), 4);
        assert_eq!(v.lower_bound_by(|s| s.len() >= 25), 5);
    }
}
