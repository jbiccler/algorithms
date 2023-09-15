use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T: Clone> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T: Clone> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }))
    }
    /*
    fn get(&self, n: usize) -> Option<Rc<RefCell<Self>>> {
        let mut next = self.next;
        for i in 1..n {
            match next {
                Some(n) => next = n.borrow_mut().next,
                None => return None,
            }
        }
        return next;
    }
    */
}

pub struct DoublyLinkedList<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }
    pub fn push_front(&mut self, val: T) {
        let new_node = Node::new(val);
        match self.head.take() {
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node.clone());
            }
            Some(prev_head) => {
                self.head = Some(new_node.clone());
                new_node.as_ref().borrow_mut().next = Some(prev_head.clone());
                prev_head.as_ref().borrow_mut().prev = Some(new_node.clone());
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => {
                self.tail = None;
                None
            }
            Some(prev_head) => {
                let tmp = prev_head.as_ref().borrow_mut().val.clone();
                self.head = match prev_head.as_ref().borrow_mut().next.as_ref() {
                    Some(n) => Some(n.clone()),
                    None => None,
                };
                if let Some(head) = self.head.as_ref() {
                    head.as_ref().borrow_mut().prev = None;
                }
                Some(tmp)
            }
        }
    }
}
