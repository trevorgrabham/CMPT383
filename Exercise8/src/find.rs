pub fn find_elt<T: Eq>(values: &Vec<T>, elt: T) -> Option<usize> {
    for i in 0..values.len() {
        if values[i] == elt {
            Some(i)
        }
    }
    None
}
