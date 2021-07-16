#[cfg(test)]
use crate::sort::quicksort;
use rand::Rng;

const VAL_RANGE: i64 = 10000;

pub fn random_vec(n: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    (0..n)
        .into_iter()
        .map(|_| rng.gen_range(-VAL_RANGE..VAL_RANGE))
        .collect()
}

#[test]
fn sort_test() {
    fn assert_is_sorted<T: Ord + std::fmt::Debug>(vec: &Vec<T>) {
        for w in vec.windows(2) {
            assert!(w[0] <= w[1], "found {:?} and {:?} out of order", w[0], w[1]);
        }
    }

    let mut v1 = Vec::from([1]);
    quicksort(&mut v1);
    assert_eq!(v1, Vec::from([1]));

    let mut v2 = Vec::from([1, 6, 2, 4, 8, 2, 1, 4]);
    quicksort(&mut v2);
    assert_eq!(v2, Vec::from([1, 1, 2, 2, 4, 4, 6, 8]));

    let mut v3: Vec<char> = "hello world".chars().collect();
    quicksort(&mut v3);
    assert_eq!(v3, " dehllloorw".chars().collect::<Vec<char>>());

    let mut v_rand = random_vec(10);
    quicksort(&mut v_rand);
    assert_is_sorted(&v_rand);

    v_rand = random_vec(100);
    quicksort(&mut v_rand);
    assert_is_sorted(&v_rand);

    v_rand = random_vec(1000);
    quicksort(&mut v_rand);
    assert_is_sorted(&v_rand);
}
