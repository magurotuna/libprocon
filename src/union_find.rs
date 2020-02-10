#[derive(Debug, Clone)]
pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    pub fn root(&mut self, x: usize) -> Option<usize> {
        if x > self.parent.len() {
            None
        } else if self.parent[x] == x {
            Some(x)
        } else {
            let px = self.parent[x];
            let root = self.root(px).unwrap();
            self.parent[x] = root;
            Some(root)
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = match self.root(x) {
            None => return,
            Some(val) => val,
        };
        let y_root = match self.root(y) {
            None => return,
            Some(val) => val,
        };
        if x_root == y_root {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x_root] = y_root;
        } else {
            self.parent[y_root] = x_root;
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        let x_root = match self.root(x) {
            None => return false,
            Some(val) => val,
        };
        let y_root = match self.root(y) {
            None => return false,
            Some(val) => val,
        };

        x_root == y_root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disjoint_set() {
        let mut s = DisjointSet::new(10);

        s.unite(0, 1);
        assert!(s.same(0, 1));
        assert!(!s.same(0, 2));

        s.unite(2, 3);
        assert!(s.same(2, 3));
        assert!(!s.same(0, 2));

        s.unite(1, 3);
        assert!(s.same(0, 2));
        assert!(s.same(1, 2));
        assert!(s.same(0, 3));
        assert!(!s.same(0, 4));

        assert!(!s.same(11, 12)); // out of range
    }
}
