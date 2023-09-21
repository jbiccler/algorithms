use algos::data_structures::doubly_linked_list::*;
use algos::data_structures::singly_linked_list::*;
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
