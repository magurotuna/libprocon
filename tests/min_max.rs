use libprocon::{chmax, chmin, max, min};

#[test]
fn test_min_macro() {
    assert_eq!(0, min!(0, 1, 2, 3, 4, 5, 2, 4, 5));
    assert_eq!(-5, min!(0, 1, 2, 3, 4, -5, 2, 4, 5));
    assert_eq!(10, min!(12542, 2142, 2256, 525, 10, 21, 11));
    assert_eq!(0, min!(0));
}

#[test]
fn test_max_macro() {
    assert_eq!(5, max!(0, 1, 2, 3, 4, 5, 2, 4, 5));
    assert_eq!(5, max!(0, 1, 2, 3, 4, -5, 2, 4, 5));
    assert_eq!(12542, max!(12542, 2142, 2256, 525, 10, 21, 11));
    assert_eq!(0, max!(0));
}

#[test]
fn test_min_macro_trailing_comma() {
    assert_eq!(0, min!(0, 1, 2, 3, 4, 5, 2, 4, 5,));
    assert_eq!(0, min!(0, 1, 2, 3, 4, 5, 2, 4, 5,,));
    assert_eq!(0, min!(0,,,));
}

#[test]
fn test_max_macro_trailing_comma() {
    assert_eq!(5, max!(0, 1, 2, 3, 4, 5, 2, 4, 5,));
    assert_eq!(5, max!(0, 1, 2, 3, 4, 5, 2, 4, 5,,));
    assert_eq!(0, max!(0,,,));
}

#[test]
fn test_chmin_macro() {
    let mut ans = 42;
    let changed = chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, -5);
    assert!(changed);

    let mut ans = -10;
    let changed = chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2, -10);
    assert_eq!(ans, -10);
    assert!(!changed);
}

#[test]
fn test_chmax_macro() {
    let mut ans = 42;
    let changed = chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2);
    assert_eq!(ans, 200);
    assert!(changed);

    let mut ans = 201;
    let changed = chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2, 201);
    assert_eq!(ans, 201);
    assert!(!changed);
}

#[test]
fn test_chmin_macro_trailing_comma() {
    let mut ans = 42;
    let changed = chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(ans, -5);
    assert!(changed);

    let mut ans = 42;
    let changed = chmin!(ans, 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(ans, -5);
    assert!(changed);

    let mut ans = 42;
    let changed = chmin!(ans, 42, 50, 43, 100,,,,,,);
    assert_eq!(ans, 42);
    assert!(!changed);
}

#[test]
fn test_chmax_macro_trailing_comma() {
    let mut ans = 42;
    let changed = chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(ans, 200);
    assert!(changed);

    let mut ans = 42;
    let changed = chmax!(ans, 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(ans, 200);
    assert!(changed);

    let mut ans = 42;
    let changed = chmax!(ans, 0, -2, -40, 42, 30);
    assert_eq!(ans, 42);
    assert!(!changed);
}

#[test]
fn test_chmin_macro_indexing() {
    let mut v = vec![1, 2, 3];
    let changed = chmin!(v[1], 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(v, vec![1, -5, 3]);
    assert!(changed);

    let mut v = vec![1, 2, 3];
    let changed = chmin!(v[2], 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(v, vec![1, 2, -5]);
    assert!(changed);

    let mut v = vec![1, 2, 3];
    let changed = chmin!(v[0], 10, 200, 3000);
    assert_eq!(v, vec![1, 2, 3]);
    assert!(!changed);
}

#[test]
fn test_chmax_macro_indexing() {
    let mut v = vec![1, 2, 3];
    let changed = chmax!(v[1], 100, 0, -5, 100 * 2, 100 / 2,);
    assert_eq!(v, vec![1, 200, 3]);
    assert!(changed);

    let mut v = vec![1, 2, 3];
    let changed = chmax!(v[2], 100, 0, -5, 100 * 2, 100 / 2,,);
    assert_eq!(v, vec![1, 2, 200]);
    assert!(changed);

    let mut v = vec![1, 2, 3];
    let changed = chmax!(v[0], -100, 0, -5,,,);
    assert_eq!(v, vec![1, 2, 3]);
    assert!(!changed);
}
