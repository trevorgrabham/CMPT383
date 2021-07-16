use std::fmt;

fn gcd(a: i64, b: i64) -> i64 {
    // Euclidean Algorithm
    if b == 0 {
        if a >= 0 {
            a
        }
        else {
            -a
        }
    }
    else {
        gcd(b, a%b)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rational {
    pub n: i64,
    pub d: i64,
}

impl Rational {
    pub fn new(n: i64, d: i64) -> Rational {
        Rational {
            n: n,
            d: d,
        }
    }
    pub fn reduce(&mut self) {
        let divisor = gcd(self.n, self.d);
        self.n = self.n/divisor;
        self.d = self.d/divisor;
    }
}

impl From<i64> for Rational {
    fn from(n: i64) -> Rational {
        Rational {
            n: n,
            d: 1,
        }
    }
}

// ###################################################################
// new functions

impl From<Rational> for f64 {
    fn from(r: Rational) -> f64 {
        r.n as f64/r.d
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}