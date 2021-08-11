fn factors(n: i64) -> Vec<i64>{
    let mut v = Vec::new();
    for i in 1..=n{
        if n%i ==0{
            v.push(i);
        }
    }
    v
}

fn increment(mut v: Vec<i64>) {
    v.iter_mut().for_each(|i| {
        (*i) += 1;
    });
}

fn main() {
    let a = 120_i64;
    let f = factors(a);
    let g = f;
    increment(g);
    println!("{:?}", g);
}
