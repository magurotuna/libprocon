#![allow(dead_code)]

use cargo_snippet::snippet;

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
