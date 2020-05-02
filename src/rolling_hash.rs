use cargo_snippet::snippet;

#[snippet("ROLLING_HASH")]
#[derive(Debug)]
pub struct RHMod(u64);

#[snippet("ROLLING_HASH")]
#[derive(Debug)]
pub struct RHBase(u64);

#[snippet("ROLLING_HASH")]
#[derive(Debug)]
struct RHInner {
    hash: u64,
    power: u64,
}

#[snippet("ROLLING_HASH")]
impl RHInner {
    fn new(hash: u64, power: u64) -> RHInner {
        RHInner {
            #[allow(clippy::redundant_field_names)]
            hash: hash,
            #[allow(clippy::redundant_field_names)]
            power: power,
        }
    }
}

#[snippet("ROLLING_HASH")]
#[derive(Debug)]
pub struct RollingHash {
    hash_pow_list: Vec<(RHMod, Vec<RHInner>)>,
}

#[snippet("ROLLING_HASH")]
impl RollingHash {
    pub fn new(target: &[char]) -> RollingHash {
        RollingHash::with_base_mod(
            target,
            &[
                // https://competitive12.blogspot.com/2019/06/blog-post_26.html
                (RHBase(2315961251), RHMod(4294966367)),
                (RHBase(1692999586), RHMod(4294959359)),
            ],
        )
    }

    fn with_base_mod(target: &[char], base_mod: &[(RHBase, RHMod)]) -> RollingHash {
        let hp_list = base_mod
            .iter()
            .map(|&(RHBase(base), RHMod(modulo))| {
                let mut hp = Vec::with_capacity(target.len() + 1);
                hp.push(RHInner::new(0, 1));

                for (i, &c) in target.iter().enumerate() {
                    let RHInner { hash, power } = hp[i];
                    let next_hash = (hash + c as u64) * base % modulo;
                    let next_power = power * base % modulo;
                    hp.push(RHInner::new(next_hash, next_power));
                }

                (RHMod(modulo), hp)
            })
            .collect();

        RollingHash {
            hash_pow_list: hp_list,
        }
    }

    // get the hash between [left, right)
    pub fn get(&self, left: usize, right: usize) -> u64 {
        self.hash_pow_list
            .iter()
            .map(|&(RHMod(modulo), ref hp)| {
                (hp[right].hash + modulo - hp[left].hash * hp[right - left].power % modulo) % modulo
            })
            .fold(0, |a, b| a ^ b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash() {
        let target: Vec<char> = "abcabcddd".chars().collect();
        let rh = RollingHash::new(&target);
        assert_eq!(rh.get(0, 3), rh.get(3, 6));
        assert_eq!(rh.get(1, 3), rh.get(4, 6));
        assert_eq!(rh.get(6, 7), rh.get(7, 8));
        assert_eq!(rh.get(7, 8), rh.get(8, 9));
        assert_ne!(rh.get(0, 4), rh.get(3, 7));
        assert_ne!(rh.get(0, 3), rh.get(4, 7));

        let target: Vec<char> = "strangeorange".chars().collect();
        let rh = RollingHash::new(&target);
        assert_eq!(rh.get(2, 7), rh.get(8, 13));
        assert_ne!(rh.get(1, 7), rh.get(7, 13));
        assert_ne!(rh.get(0, 7), rh.get(6, 13));
    }
}
