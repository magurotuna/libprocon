use cargo_snippet::snippet;

#[snippet("2D")]
pub trait TwoDimensionVectorExt {
    fn adj4(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = (usize, usize)>>;
    fn adj8(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = (usize, usize)>>;
}

#[snippet("2D")]
impl<T> TwoDimensionVectorExt for Vec<Vec<T>> {
    fn adj4(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let height = self.len() as isize;
        assert!(height > 0);
        let width = self[0].len() as isize;
        assert!(width > 0);
        let adj = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(move |&(dy, dx)| {
                let ny = row as isize + dy;
                let nx = col as isize + dx;
                if 0 <= ny && ny < height && 0 <= nx && nx < width {
                    Some((ny as usize, nx as usize))
                } else {
                    None
                }
            });
        Box::new(adj)
    }

    fn adj8(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let height = self.len() as isize;
        assert!(height > 0);
        let width = self[0].len() as isize;
        assert!(width > 0);
        let adj = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ]
        .iter()
        .filter_map(move |&(dy, dx)| {
            let ny = row as isize + dy;
            let nx = col as isize + dx;
            if 0 <= ny && ny < height && 0 <= nx && nx < width {
                Some((ny as usize, nx as usize))
            } else {
                None
            }
        });
        Box::new(adj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adj4() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        // |o|x| |
        // |x| | |
        // | | | |
        let mut expected = [(0, 1), (1, 0)];
        expected.sort();
        let mut actual: Vec<_> = v.adj4(0, 0).collect();
        actual.sort();
        assert_eq!(actual, expected);

        // | |x| |
        // |x|o|x|
        // | |x| |
        let mut expected = [(0, 1), (1, 0), (1, 2), (2, 1)];
        expected.sort();
        let mut actual: Vec<_> = v.adj4(1, 1).collect();
        actual.sort();
        assert_eq!(actual, expected);

        // |x|o|x|
        // | |x| |
        // | | | |
        let mut expected = [(0, 0), (0, 2), (1, 1)];
        expected.sort();
        let mut actual: Vec<_> = v.adj4(0, 1).collect();
        actual.sort();
        assert_eq!(actual, expected);

        // | | |x|
        // | |x|o|
        // | | |x|
        let mut expected = [(0, 2), (1, 1), (2, 2)];
        expected.sort();
        let mut actual: Vec<_> = v.adj4(1, 2).collect();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adj8() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        // |o|x| |
        // |x|x| |
        // | | | |
        let mut expected = [(0, 1), (1, 0), (1, 1)];
        expected.sort();
        let mut actual: Vec<_> = v.adj8(0, 0).collect();
        actual.sort();
        assert_eq!(actual, expected);

        // |x|x|x|
        // |x|o|x|
        // |x|x|x|
        let mut expected = [
            (0, 1),
            (1, 0),
            (1, 2),
            (2, 1),
            (0, 0),
            (0, 2),
            (2, 0),
            (2, 2),
        ];
        expected.sort();
        let mut actual: Vec<_> = v.adj8(1, 1).collect();
        actual.sort();
        assert_eq!(actual, expected);

        // |x|o|x|
        // |x|x|x|
        // | | | |
        let mut expected = [(0, 0), (0, 2), (1, 1), (1, 0), (1, 2)];
        expected.sort();
        let mut actual: Vec<_> = v.adj8(0, 1).collect();
        actual.sort();
        assert_eq!(actual, expected);

        // | |x|x|
        // | |x|o|
        // | |x|x|
        let mut expected = [(0, 2), (1, 1), (2, 2), (0, 1), (2, 1)];
        expected.sort();
        let mut actual: Vec<_> = v.adj8(1, 2).collect();
        actual.sort();
        assert_eq!(actual, expected);
    }
}
