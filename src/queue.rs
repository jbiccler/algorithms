use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T: Clone> {
    val: T,
    next: Link<T>,
}

#[derive(Debug)]
pub struct Queue<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn dequeue(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(h) => {
                // Take the current head's next
                let mut h = RefCell::borrow_mut(&h);
                self.len -= 1;
                self.head = h.next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(h.val.clone())
            }
        }
    }
    pub fn queue(&mut self, val: T) {
        match self.tail.take() {
            None => {
                // No nodes yet in the list
                // Initialize list that doens't have a next node and set it as head and tail
                let node = Rc::new(RefCell::new(Node { val, next: None }));
                self.tail = Some(Rc::clone(&node));
                self.head = Some(Rc::clone(&node));
                self.len += 1;
            }
            Some(t) => {
                let mut t = RefCell::borrow_mut(&t);
                let node = Rc::new(RefCell::new(Node { val, next: None }));
                // set tail to new latest node
                self.tail = Some(Rc::clone(&node));
                // change previous latest node to point to new latest node
                t.next = Some(Rc::clone(&node));
                self.len += 1;
            }
        }
    }
    pub fn peek(&self) -> Option<T> {
        self.head.as_ref().map(|h| RefCell::borrow(h).val.clone())
    }
}
