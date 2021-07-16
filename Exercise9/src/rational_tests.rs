#[cfg(test)]
use crate::rational::Rational;

#[test]
fn rational_test() {
    let mut r1 = Rational::new(2,4);
    assert_eq!(r1.display(), "2/4");
    r1.reduce();
    assert_eq!(r1, Rational::new(1,2));
    let f: f64 = r1.into();
    assert_eq!(f, 0.5);
}