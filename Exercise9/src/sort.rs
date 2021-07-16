use rand::Rng;

const INSERTION_SORT_CUTOFF: usize = 0;

/// Sort v[left..=right] in-place with insertion sort.
pub fn insertion_sort<T: Copy + Ord>(v: &mut Vec<T>, left: usize, right: usize) {
    for i in (left + 1)..=right {
        let x = v[i];
        let mut j = i;
        while j >= left + 1 && v[j - 1] > x {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = x;
    }
}

/// Partition v[left..=right] around a random pivot, in-place. Returns the position of the pivot.
/// i.e. after "let p = partition(v, left, right)", everything in v[left..p] is <= v[p] and
/// everything in v[p+1..=right] is >= v[p].
pub fn partition<T: Copy + Ord>(v: &mut Vec<T>, left: usize, right: usize) -> usize {
    let mut temp: T;

    // Choose random pivot
    let mut rng = rand::thread_rng();
    let pivot_index: usize = rng.gen_range(left..=right);
    let pivot = v[pivot_index];

    // Move pivot at the end of the range to match standard Lumuto algorithm
    temp = v[pivot_index];
    v[pivot_index] = v[right];
    v[right] = temp;

    // Move smaller elements before the pivot
    let mut pos = left;
    for i in left..right {
        if v[i] < pivot {
            temp = v[pos];
            v[pos] = v[i];
            v[i] = temp;
            pos += 1;
        }
    }

    // Put pivot into position
    temp = v[right];
    v[right] = v[pos];
    v[pos] = temp;

    pos
}

pub fn quicksort_partial<T: Copy + Ord>(v: &mut Vec<T>, left: usize, right: usize) {
    if right-left < INSERTION_SORT_CUTOFF {
        insertion_sort(v, left, right);
    }
    if left < right {
        let p = partition(v, left, right);
        quicksort_partial(v, left, p-1);
        quicksort_partial(v, p+1, right);
    }
}

pub fn quicksort<T: Copy + Ord>(v: &mut Vec<T>) {
    quicksort_partial(v, 0, v.len()-1);
}


