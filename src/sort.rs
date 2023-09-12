pub fn bubble_sort<T: PartialOrd>(v: &mut Vec<T>) {
    // simple in place bubble sort
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
