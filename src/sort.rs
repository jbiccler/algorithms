pub fn insertion_sort<T: PartialOrd>(v: &mut [T]) {
    let n = v.len();
    for i in 1..n {
        let mut j = i;
        while v[j] < v[j.saturating_sub(1)] && j > 0 {
            v.swap(j, j - 1);
            j -= 1;
        }
    }
}
pub fn bubble_sort<T: PartialOrd>(v: &mut Vec<T>) {
    // simple in place bubble sort
    if v.is_empty() {
        return;
    }
    for j in 1..v.len() {
        let mut swapped: bool = false;
        for i in 0..(v.len() - j) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            return ();
        };
    }
}
pub fn bubble_sort_recursive<T: PartialOrd>(v: &mut Vec<T>) {
    // simple in place bubble sort but recursive
    if v.is_empty() {
        return;
    }
    let mut swapped: bool = false;
    for i in 0..(v.len() - 1) {
        if v[i] > v[i + 1] {
            v.swap(i, i + 1);
            swapped = true;
        }
    }
    if swapped {
        bubble_sort(v);
    };
}
