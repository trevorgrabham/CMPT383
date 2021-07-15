use crate::find::find_elt;
use crate::hailstone::{hailstone, hailstone_sequence_append, hailstone_sequence_prealloc};
use crate::rational::Rational;

#[cfg(test)]
#[test]
fn hailstone_tests() {
    assert_eq!(hailstone(17), 52);
    assert_eq!(hailstone(18), 9);
    assert_eq!(hailstone(1), 4);
}

#[test]
fn sequence_tests() {
    assert_eq!(
        hailstone_sequence_append(5),
        Vec::<u64>::from([5, 16, 8, 4, 2, 1])
    );
    assert_eq!(
        hailstone_sequence_prealloc(5),
        Vec::<u64>::from([5, 16, 8, 4, 2, 1])
    );
    assert_eq!(hailstone_sequence_append(1), Vec::<u64>::from([1]));
    assert_eq!(hailstone_sequence_prealloc(1), Vec::<u64>::from([1]));
}

#[test]
fn find_test() {
    let v1 = Vec::from([4, 5, 2, 8, 7, 3, 1]);
    assert_eq!(find_elt(&v1, 8), Some(3));
    assert_eq!(find_elt(&v1, 6), None);
    let v2 = "Hello World!".chars().collect();
    assert_eq!(find_elt(&v2, 'o'), Some(4));
    assert_eq!(find_elt(&v2, 'q'), None);
}

#[test]
fn rational_test() {
    let mut r = Rational::new(6, 8);
    assert_eq!(format!("{:?}", r), "Rational { n: 6, d: 8 }");
    r.reduce();
    let four1 = Rational::from(4);
    let four2 = Rational::new(4, 1);
    assert_eq!(four1, four2);
}
