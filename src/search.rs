pub fn linear_search<T>(arr: T, elem: &T::Item) -> Option<usize>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    for (i, e) in arr.into_iter().enumerate() {
        if e == *elem {
            return Some(i);
        }
    }
    None
}

// Assuming sorted array
pub fn binary_search<T>(arr: &[T], elem: &T) -> Option<usize>
where
    T: Ord,
{
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    loop {
        // midpoint
        let m: usize = lo + (hi - lo) / 2;
        let val = &arr[m];
        if val == elem {
            return Some(m);
        } else if val < elem {
            lo = m + 1;
        } else if val > elem {
            hi = m;
        }
        if lo >= hi {
            break;
        }
    }
    None
}
