/*
    The prealloc version of the function, ran faster on average in all of the tests

    Over all of the tests, the highest time for the prealloc version was still lower than the 
    lowest time for the standard version. This was over 100 runs for each test, and over all 3 tests
*/
pub fn hailstone(n: u64) -> u64 {
    if n % 2 == 0 { n/2 } else { 3*n + 1 }
}

pub fn hailstone_sequence_append(n: u64) -> Vec<u64> {
    let mut return_val: Vec<u64> = Vec::new();
    let mut result = n;
    return_val.push(n);
    while result != 1 {
        result = hailstone(result);
        return_val.push(result);
    }
    return return_val;
}

pub fn hailstone_sequence_prealloc(n: u64) -> Vec<u64> {
    let mut m = n;
    let mut count = 1;
    while m != 1 {
        m = hailstone(m);
        count += 1;
    }
    let mut return_val: Vec<u64> = Vec::with_capacity(count);
    m = n;
    return_val.push(n);
    while m != 1 {
        m = hailstone(m);
        return_val.push(m);
    }
    return return_val;
}
