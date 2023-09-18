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

pub fn merge_sort<T: PartialOrd + Clone + Copy>(v: &mut [T]) {
    let n = v.len();
    if n <= 1 {
        return;
    } else {
        let mid = n / 2;
        let (left, right) = v.split_at_mut(mid);

        merge_sort(left);
        merge_sort(right);

        let mut merged = Vec::with_capacity(n);
        let mut l: usize = 0;
        let mut r: usize = 0;

        while l < left.len() && r < right.len() {
            if left[l] <= right[r] {
                merged.push(left[l]);
                l += 1;
            } else {
                merged.push(right[r]);
                r += 1;
            }
        }
        while l < left.len() {
            merged.push(left[l]);
            l += 1;
        }
        while r < right.len() {
            merged.push(right[r]);
            r += 1;
        }
        v.copy_from_slice(&merged);
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
