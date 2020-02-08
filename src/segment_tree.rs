#[derive(Debug)]
pub struct SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    values: Vec<T>,
    n_leaves: usize,
    identity_elem: T,
    func: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(init_value: T, size: usize, func: F) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            values: vec![init_value; 2 * n - 1],
            n_leaves: n,
            identity_elem: init_value,
            func,
        }
    }

    pub fn update(&mut self, leaf_number: usize, replace_value: T) {
        let mut node_number = leaf_number + self.n_leaves - 1;
        self.values[node_number] = replace_value;
        while node_number > 0 {
            node_number = (node_number - 1) / 2;
            self.values[node_number] = (self.func)(
                self.values[node_number * 2 + 1],
                self.values[node_number * 2 + 2],
            );
        }
    }

    pub fn query(&self, begin: usize, end: usize) -> T {
        self.internal_query(begin, end, 0, 0, self.n_leaves)
    }

    fn internal_query(
        &self,
        begin: usize,
        end: usize,
        node_number: usize,
        left: usize,
        right: usize,
    ) -> T {
        if right <= begin || end <= left {
            self.identity_elem
        } else if begin <= left && right <= end {
            self.values[node_number]
        } else {
            let c1 = self.internal_query(begin, end, 2 * node_number + 1, left, (left + right) / 2);
            let c2 =
                self.internal_query(begin, end, 2 * node_number + 2, (left + right) / 2, right);
            (self.func)(c1, c2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_tree_min() {
        let ident: usize = 1 << 60;
        let mut st = SegmentTree::new(ident, 4, std::cmp::min);
        st.update(0, 2);
        st.update(1, 4);
        st.update(2, 3);
        st.update(3, 0);
        assert_eq!(2, st.query(0, 3));
    }

    #[test]
    fn test_segment_tree_max() {
        let ident: usize = 0;
        let mut st = SegmentTree::new(ident, 4, std::cmp::max);
        st.update(0, 2);
        st.update(1, 4);
        st.update(2, 3);
        st.update(3, 0);
        assert_eq!(4, st.query(0, 3));
    }

    #[test]
    fn test_segment_tree_sum() {
        let ident: usize = 0;
        let mut st = SegmentTree::new(ident, 4, |x, y| x + y);
        st.update(0, 2);
        st.update(1, 4);
        st.update(2, 3);
        st.update(3, 0);
        assert_eq!(9, st.query(0, 3));
        assert_eq!(6, st.query(0, 2));
        assert_eq!(7, st.query(1, 3));
        assert_eq!(7, st.query(1, 4));
    }
}
