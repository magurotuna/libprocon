use libprocon::{chmax, chmin, max, min};

#[test]
fn test_min_macro() {
    assert_eq!(0, min!(0, 1, 2, 3, 4, 5, 2, 4, 5));
    assert_eq!(-5, min!(0, 1, 2, 3, 4, -5, 2, 4, 5));
    assert_eq!(10, min!(12542, 2142, 2256, 525, 10, 21, 11));
}

#[test]
fn test_max_macro() {
    assert_eq!(5, max!(0, 1, 2, 3, 4, 5, 2, 4, 5));
    assert_eq!(5, max!(0, 1, 2, 3, 4, -5, 2, 4, 5));
    assert_eq!(12542, max!(12542, 2142, 2256, 525, 10, 21, 11));
}

#[test]
fn test_min_macro_trailing_comma() {
    assert_eq!(0, min!(0, 1, 2, 3, 4, 5, 2, 4, 5,));
    assert_eq!(0, min!(0, 1, 2, 3, 4, 5, 2, 4, 5,,));
}

#[test]
fn test_max_macro_trailing_comma() {
    assert_eq!(5, max!(0, 1, 2, 3, 4, 5, 2, 4, 5,));
    assert_eq!(5, max!(0, 1, 2, 3, 4, 5, 2, 4, 5,,));
}

#[test]
fn test_chmin_macro() {
    let mut ans = 42;
    chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, -5);

    let mut ans = -10;
    chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, -10);
}

#[test]
fn test_chmax_macro() {
    let mut ans = 42;
    chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, 200);

    let mut ans = 201;
    chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, 201);
}

#[test]
fn test_chmin_macro_trailing_comma() {
    let mut ans = 42;
    chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(ans, -5);

    let mut ans = 42;
    chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(ans, -5);
}

#[test]
fn test_chmax_macro_trailing_comma() {
    let mut ans = 42;
    chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(ans, 200);

    let mut ans = 42;
    chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(ans, 200);
}

#[test]
fn test_chmin_macro_indexing() {
    let mut v = vec![1, 2, 3];
    chmin!(v[1], 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(v, vec![1, -5, 3]);

    let mut v = vec![1, 2, 3];
    chmin!(v[2], 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(v, vec![1, 2, -5]);
}

#[test]
fn test_chmax_macro_indexing() {
    let mut v = vec![1, 2, 3];
    chmax!(v[1], 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(v, vec![1, 200, 3]);

    let mut v = vec![1, 2, 3];
    chmax!(v[2], 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(v, vec![1, 2, 200]);
}
