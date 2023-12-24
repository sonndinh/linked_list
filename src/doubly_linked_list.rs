use std::rc::Rc;
use std::cell::RefCell;

use crate::common_traits::NodeHasNext;
use crate::common_traits::List;

type DoublyPointer<T> = Rc<RefCell<DoublyNode<T>>>;

pub struct DoublyNode<T> {
    val: T,
    next: Option<DoublyPointer<T>>,
    prev: Option<DoublyPointer<T>>,
}

impl<T: Copy> NodeHasNext for DoublyNode<T> {
    type ValueType = T;

    fn get_next(&self) -> Option<Rc<RefCell<DoublyNode<T>>>> {
        match &self.next {
            Some(x) => Some(Rc::clone(x)),
            None => None,
        }
    }

    fn get_value(&self) -> T {
        self.val
    }
}

impl<T> DoublyNode<T> {
    fn new(t: T) -> DoublyNode<T> {
        DoublyNode::<T> {
            val: t,
            next: None,
            prev: None,
        }
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<DoublyPointer<T>>,
    tail: Option<DoublyPointer<T>>,
    size: usize,
}

impl<T: Copy> List for DoublyLinkedList<T> {
    type ListNode = DoublyNode<T>;

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_head(&self) -> Option<DoublyPointer<T>> {
        match &self.head {
            Some(x) => Some(Rc::clone(x)),
            None => None,
        }
    }

    fn get_tail(&self) -> Option<DoublyPointer<T>> {
        match &self.tail {
            Some(x) => Some(Rc::clone(x)),
            None => None,
        }
    }
}

impl<T: Copy + std::fmt::Debug> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList::<T> {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn insert_head(&mut self, obj: T) {
        let node = Rc::new(RefCell::new(DoublyNode::new(obj)));
        if self.size == 0 {
            self.tail = Some(Rc::clone(&node));
        } else {
            // Preprend the new node to the current head
            node.borrow_mut().next = Some(Rc::clone(self.head.as_ref().unwrap()));
            self.head.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(&node));
        }
        self.head = Some(Rc::clone(&node));
        self.size += 1;
    }

    pub fn insert_tail(&mut self, obj: T) {
        let node = Rc::new(RefCell::new(DoublyNode::new(obj)));
        if self.size == 0 {
            self.head = Some(Rc::clone(&node));
        } else {
            // Append the new node to the current tail
            node.borrow_mut().prev = Some(Rc::clone(self.tail.as_ref().unwrap()));
            self.tail.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&node));
        }
        self.tail = Some(Rc::clone(&node));
        self.size += 1;
    }

    // Insert an element at the given index
    pub fn insert_at_index(&mut self, index: usize, obj: T) {
        if index > self.size {
            eprintln!("Invalid index: {}. List size: {}", index, self.size);
            return;
        }

        if index == 0 {
            return self.insert_head(obj);
        }
        if index == self.size {
            return self.insert_tail(obj);
        }

        let prev = self.go_to_element(index - 1).unwrap();
        let node = Rc::new(RefCell::new(DoublyNode::new(obj)));
        let next = Rc::clone(prev.borrow().next.as_ref().unwrap());
        node.borrow_mut().next = Some(Rc::clone(&next));
        prev.borrow_mut().next = Some(Rc::clone(&node));
        node.borrow_mut().prev = Some(Rc::clone(&prev));
        next.borrow_mut().prev = Some(Rc::clone(&node));
        self.size += 1;
    }

    pub fn delete_head(&mut self) {
        if self.size == 0 {
            return;
        }

        let head_node = Rc::clone(self.head.as_ref().unwrap());
        match head_node.borrow().next.as_ref() {
            None => {
                self.head = None;
                self.tail = None;
            },
            Some(next_node) => {
                self.head = Some(Rc::clone(next_node));
                next_node.borrow_mut().prev = None;
            },
        };
        self.size -= 1;
    }

    pub fn delete_tail(&mut self) {
        if self.size == 0 {
            return;
        }

        let tail_node = Rc::clone(self.tail.as_ref().unwrap());
        match tail_node.borrow().prev.as_ref() {
            None => {
                self.head = None;
                self.tail = None;
            },
            Some(prev_node) => {
                self.tail = Some(Rc::clone(prev_node));
                prev_node.borrow_mut().next = None;
            },
        };
        self.size -= 1;
    }

    pub fn delete_at_index(&mut self, i: usize) {
        if i >= self.size {
            eprintln!("Invalid index: {}. List size: {}", i, self.size);
            return;
        }

        if i == 0 {
            return self.delete_head();
        }
        if i == self.size - 1 {
            return self.delete_tail();
        }

        // The list has at least 3 elements.
        let prev = self.go_to_element(i-1).unwrap();
        let next = self.go_to_element(i+1).unwrap();
        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(&prev));
        self.size -= 1;
    }
}
