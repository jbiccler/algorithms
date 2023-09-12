pub mod sort;

#[cfg(test)]
mod tests {
    use crate::sort::bubble_sort;

    #[test]
    fn bubble_int() {
        let mut v = vec![4, 3, 1, 3, 5, 6];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 3, 4, 5, 6]);
    }
    #[test]
    fn bubble_float() {
        let mut v = vec![4., 3., 1., 3., 5., 6.];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1., 3., 3., 4., 5., 6.]);
    }
    #[test]
    fn bubble_unit() {
        let mut v: Vec<i32> = vec![];
        bubble_sort(&mut v);
        assert_eq!(v, Vec::<i32>::new());
    }
    #[test]
    fn bubble_single() {
        let mut v: Vec<i32> = vec![1];
        bubble_sort(&mut v);
        assert_eq!(v, Vec::<i32>::from([1]));
    }
}
