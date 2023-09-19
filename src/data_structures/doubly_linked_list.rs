use std::cell::RefCell;
use std::rc::Rc;

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
                self.tail = Some(Rc::clone(&new_node));
                self.head = Some(Rc::clone(&new_node));
            }
            Some(prev_head) => {
                self.head = Some(Rc::clone(&new_node));
                RefCell::borrow_mut(&new_node).next = Some(Rc::clone(&prev_head));
                RefCell::borrow_mut(&prev_head).prev = Some(Rc::clone(&new_node));
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
                self.head = RefCell::borrow(&prev_head).next.as_ref().map(Rc::clone);
                if let Some(head) = self.head.as_ref() {
                    RefCell::borrow_mut(head).prev = None;
                }
                Some(RefCell::borrow(&prev_head).val.clone())
            }
        }
    }
}
