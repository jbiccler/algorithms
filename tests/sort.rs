#![feature(test)]
extern crate test;
use algos::algos::sort::*;
use test::Bencher;
#[bench]
fn bubble_int(b: &mut Bencher) {
    b.iter(|| {
        let mut v = vec![4, 3, 1, 3, 5, 6];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 3, 4, 5, 6]);
    })
}
#[bench]
fn bubble_float(b: &mut Bencher) {
    b.iter(|| {
        let mut v = vec![4., 3., 1., 3., 5., 6.];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1., 3., 3., 4., 5., 6.]);
    })
}
#[bench]
fn bubble_unit(b: &mut Bencher) {
    b.iter(|| {
        let mut v: Vec<i32> = vec![];
        bubble_sort(&mut v);
        assert_eq!(v, Vec::<i32>::new());
    })
}
#[bench]
fn bubble_single(b: &mut Bencher) {
    b.iter(|| {
        let mut v: Vec<i32> = vec![1];
        bubble_sort(&mut v);
        assert_eq!(v, Vec::<i32>::from([1]));
    })
}

#[bench]
fn bubble_int_recursive(b: &mut Bencher) {
    b.iter(|| {
        let mut v = vec![4, 3, 1, 3, 5, 6];
        bubble_sort_recursive(&mut v);
        assert_eq!(v, vec![1, 3, 3, 4, 5, 6]);
    })
}
#[bench]
fn bubble_float_recursive(b: &mut Bencher) {
    b.iter(|| {
        let mut v = vec![4., 3., 1., 3., 5., 6.];
        bubble_sort_recursive(&mut v);
        assert_eq!(v, vec![1., 3., 3., 4., 5., 6.]);
    })
}
#[bench]
fn bubble_unit_recursive(b: &mut Bencher) {
    b.iter(|| {
        let mut v: Vec<i32> = vec![];
        bubble_sort_recursive(&mut v);
        assert_eq!(v, Vec::<i32>::new());
    })
}
#[bench]
fn bubble_single_recursive(b: &mut Bencher) {
    b.iter(|| {
        let mut v: Vec<i32> = vec![1];
        bubble_sort_recursive(&mut v);
        assert_eq!(v, Vec::<i32>::from([1]));
    })
}
#[test]
fn test_insertion_sort() {
    let mut v = vec![35, 5, 3, 2, 1, 5, 3, 2];
    insertion_sort(&mut v);
    assert_eq!(v, vec![1, 2, 2, 3, 3, 5, 5, 35]);
    assert_eq!(Vec::<i32>::new(), Vec::<i32>::new());
}

#[test]
fn test_merge_sort() {
    let mut v = vec![35, 5, 3, 2, 1, 5, 3, 2];
    merge_sort(&mut v);
    assert_eq!(v, vec![1, 2, 2, 3, 3, 5, 5, 35]);
}
#[test]
fn test_quick_sort() {
    let mut v = vec![35, 5, 3, 2, 1, 5, 3, 2];
    quick_sort(&mut v);
    assert_eq!(v, vec![1, 2, 2, 3, 3, 5, 5, 35]);
}
