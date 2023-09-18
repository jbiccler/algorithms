#![feature(test)]
extern crate test;
pub mod arrays;
pub mod nodes;
pub mod search;
pub mod singly_linked_list;
pub mod sort;

#[cfg(test)]
mod tests {
    use crate::arrays::RingBuffer;
    use crate::nodes::DoublyLinkedList;
    use crate::search::*;
    use crate::singly_linked_list::SinglyLinkedList;
    use crate::sort::merge_sort;
    use crate::sort::*;
    use std::vec;
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
    #[test]
    fn doubly_linked_list_push_pop_front() {
        let mut list = DoublyLinkedList::new();
        let n: usize = 11;
        for i in 1..n {
            list.push_front(i);
        }
        for i in (1..n).rev() {
            dbg!(&i);
            assert_eq!(list.pop_front(), Some(i));
        }
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn queue_dequeue() {
        let mut q: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let n = 10;
        for i in 0..n {
            q.push_back(i);
        }
        dbg!(&q);
        for i in 0..n {
            let popped = q.pop_front();
            dbg!(&popped);
            assert_eq!(popped, Some(i));
        }
        dbg!(&q);
    }
    #[test]
    fn queue_peek() {
        let mut q: SinglyLinkedList<i32> = SinglyLinkedList::new();
        // check empty queue
        assert_eq!(q.peek(), None);
        let n = 10;
        for i in 0..n {
            q.push_back(i);
        }
        for i in 0..n {
            assert_eq!(q.peek(), Some(i));
            q.pop_front();
        }
        assert_eq!(q.peek(), None);
    }
    #[test]
    fn stack() {
        let mut q: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let n = 10;
        for i in 0..n {
            q.push_front(i);
        }
        for i in (0..n).rev() {
            let popped = q.pop_front();
            dbg!(&popped);
            assert_eq!(popped, Some(i));
        }
    }
    #[test]
    fn test_insertion_sort() {
        let mut v = vec![35, 5, 3, 2, 1, 5, 3, 2];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 2, 3, 3, 5, 5, 35]);
        assert_eq!(Vec::<i32>::new(), Vec::<i32>::new());
    }
    #[test]
    fn test_ring_buffer() {
        let mut rb = RingBuffer::new(100);
        for i in 0..10 {
            for j in 0..57 {
                rb.push(57 * i + j).unwrap();
            }
            dbg!(&rb);
            for j in 0..57 {
                assert_eq!(rb.pop(), Some(57 * i + j));
            }
        }
        dbg!(&rb);
    }

    #[test]
    fn test_merge_sort() {
        let mut v = vec![35, 5, 3, 2, 1, 5, 3, 2];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 2, 3, 3, 5, 5, 35]);
    }
}
