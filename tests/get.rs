use libprocon::get;
use std::io::{BufRead, BufReader};

macro_rules! test_get {
    ([$src:expr] $tt:tt) => {
        {
            let bytes = $src.as_bytes();
            let mut reader = BufReader::new(bytes);
            get!(@inner [&mut reader] $tt)
        }
    };
}

#[test]
fn test_get_macro() {
    assert_eq!(test_get!(["42"] usize), 42);
    assert_eq!(test_get!(["42"] usize1), 41);
    assert_eq!(test_get!(["a"] char), 'a');
    assert_eq!(test_get!(["1 2 3 4"][i32]), vec![1_i32, 2, 3, 4]);
    assert_eq!(test_get!(["foobar"] String), "foobar".to_string());
    assert_eq!(
        test_get!(["foobar"] chars),
        "foobar".chars().collect::<Vec<_>>()
    );
    assert_eq!(
        test_get!(["foobar"] chars),
        "foobar".chars().collect::<Vec<_>>()
    );
    assert_eq!(test_get!(["1\n2\n3\n4"] [u32; 4]), vec![1_u32, 2, 3, 4]);
    assert_eq!(
        test_get!(["1\n2\n3\n4\n5\n6\n"] [u32; 4]),
        vec![1_u32, 2, 3, 4]
    );
    assert_eq!(
        test_get!(["foo\nbar"] [String; 2]),
        vec!["foo".to_string(), "bar".to_string()]
    );
    assert_eq!(
        test_get!(["foo\nbar"] [chars; 2]),
        vec![vec!['f', 'o', 'o'], vec!['b', 'a', 'r']]
    );
    assert_eq!(
        test_get!(["foo\nbar"] [[chars]; 2]),
        vec![vec!['f', 'o', 'o'], vec!['b', 'a', 'r']]
    );
    assert_eq!(
        test_get!(["3 2 1"](isize, i32, usize1)),
        (3_isize, 2_i32, 0_usize)
    );
    assert_eq!(
        test_get!(["3 2 1\n30 20 10\n"] [(usize1, usize1, u64); 2]),
        vec![(2_usize, 1_usize, 1_u64), (29, 19, 10)]
    );
    assert_eq!(
        test_get!(["3 2\n"] [(usize1, usize); 1]),
        vec![(2_usize, 2_usize)]
    );
    assert_eq!(
        test_get!(["3 2 1\n30 20 10\n"] [[usize]; 2]),
        vec![vec![3, 2, 1], vec![30, 20, 10]]
    );
}
