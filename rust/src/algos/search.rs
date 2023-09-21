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
pub fn two_cystal_balls(breaks: &[bool]) -> Option<usize> {
    // jump by sqrt of N -> results in algo that is O(sqrt(N))
    if breaks.is_empty() {
        return None;
    }
    let sqrtn = f64::sqrt(breaks.len() as f64) as usize;
    let mut i = usize::min(sqrtn, breaks.len() - 1);
    // deal with array of length 1
    if i == 0 && breaks[i] {
        return Some(0);
    }
    loop {
        // end if end of array is reached and only contained false values
        if i >= breaks.len() - 1 && !breaks[i] {
            break;
        }
        if breaks[i] {
            // first ball broke
            // walk it back from last known non-breaking height
            let prev = i.saturating_sub(sqrtn);
            for j in prev..i {
                if breaks[j] {
                    // second ball breaks at optimal position
                    return Some(j);
                }
            }
            // if for loop finished without returning then i was optimal
            return Some(i);
        }
        i = usize::min(breaks.len() - 1, i + sqrtn);
    }
    None
}
