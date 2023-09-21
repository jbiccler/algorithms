use algos::algos::search::*;

#[test]
fn linear_search_double() {
    let v = vec![4, 3, 1, 3, 5, 6];
    assert_eq!(linear_search(v, &3), Some(1));
}
#[test]
fn linear_search_empty() {
    let v = vec![4, 3, 1, 3, 5, 6];
    assert_eq!(linear_search(v, &99), None);
}
#[test]
fn binary_search_double() {
    let mut v = vec![4, 3, 1, 3, 5, 6];
    v.sort();
    assert_eq!(binary_search(&v, &3), Some(1));
}
#[test]
fn binary_search_empty() {
    let mut v = vec![4, 3, 1, 3, 5, 6];
    v.sort();
    assert_eq!(binary_search(&v, &99), None);
}
#[test]
fn two_cystal_balls_on_empty() {
    let v = vec![];
    assert_eq!(two_cystal_balls(&v), None);
}
#[test]
fn two_cystal_balls_invalid() {
    let v = vec![false; 100];
    assert_eq!(two_cystal_balls(&v), None);
}
#[test]
fn two_cystal_balls_on_sqrt() {
    let mut v = vec![false; 60];
    v.append(&mut vec![true; 40]);
    assert_eq!(two_cystal_balls(&v).unwrap(), 60);
}
#[test]
fn two_cystal_balls_not_on_sqrt() {
    let mut v = vec![false; 55];
    v.append(&mut vec![true; 45]);
    assert_eq!(two_cystal_balls(&v).unwrap(), 55);
}
#[test]
fn two_cystal_balls_first() {
    let v = vec![true; 100];
    assert_eq!(two_cystal_balls(&v).unwrap(), 0);
}
#[test]
fn two_cystal_balls_last() {
    let mut v = vec![false; 99];
    v.append(&mut vec![true; 1]);
    assert_eq!(two_cystal_balls(&v).unwrap(), 99);
}
#[test]
fn two_cystal_balls_single_false() {
    let v = vec![false; 1];
    assert_eq!(two_cystal_balls(&v), None);
}
#[test]
fn two_cystal_balls_single_true() {
    let v = vec![true; 1];
    assert_eq!(two_cystal_balls(&v).unwrap(), 0);
}
