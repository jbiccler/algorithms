pub mod sort;

use algos::sort::bubble_sort;

fn main() {
    let mut v = vec![3, 4, 5, 1, 223, 3, 4];
    dbg!("{}", &v);
    bubble_sort(&mut v);
    dbg!("{}", &v);
}
