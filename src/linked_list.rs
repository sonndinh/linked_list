use std::rc::Rc;
use std::cell::RefCell;

use crate::common_traits::NodeHasNext;
use crate::common_traits::List;

type Pointer<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Option<Pointer<T>>,
}

impl<T: Copy> NodeHasNext for Node<T> {
    type ValueType = T;

    fn get_next(&self) -> Option<Rc<RefCell<Node<T>>>> {
        match &self.next {
            Some(x) => Some(Rc::clone(x)),
            None => None,
        }
    }

    fn get_value(&self) -> T {
        self.val
    }
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node::<T> {
            val: t,
            next: None,
        }
    }
}

pub struct LinkedList<T> {
    head: Option<Pointer<T>>,
    tail: Option<Pointer<T>>,
    size: usize,
}

impl<T: Copy> List for LinkedList<T> {
    type ListNode = Node<T>;

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_head(&self) -> Option<Pointer<T>> {
        match &self.head {
            Some(x) => Some(Rc::clone(x)),
            None => None,
        }
    }

    fn get_tail(&self) -> Option<Pointer<T>> {
        match &self.tail {
            Some(x) => Some(Rc::clone(x)),
            None => None,
        }
    }
}

impl<T: Copy + std::fmt::Debug> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn insert_head(&mut self, obj: T) {
        let node = Rc::new(RefCell::new(Node::new(obj)));
        if self.size == 0 {
            self.tail = Some(Rc::clone(&node));
        } else {
            node.borrow_mut().next = Some(Rc::clone(self.head.as_ref().unwrap()));
        }
        self.head = Some(Rc::clone(&node));
        self.size += 1;
    }

    pub fn insert_tail(&mut self, obj: T) {
        let node = Rc::new(RefCell::new(Node::new(obj)));
        if self.size == 0 {
            self.head = Some(Rc::clone(&node));
        } else {
            self.tail.as_mut().unwrap().borrow_mut().next = Some(Rc::clone(&node));
        }
        self.tail = Some(Rc::clone(&node));
        self.size += 1;
    }

    // Insert an element at the given index.
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
        let node = Rc::new(RefCell::new(Node::new(obj)));
        let next = Rc::clone(prev.borrow().next.as_ref().unwrap());
        node.borrow_mut().next = Some(Rc::clone(&next));
        prev.borrow_mut().next = Some(Rc::clone(&node));
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
            },
        };
        self.size -= 1;
    }

    pub fn delete_tail(&mut self) {
        if self.size <= 1 {
            self.head = None;
            self.tail = None;
        } else {
            self.tail = self.go_to_element(self.size - 2);
            self.tail.as_ref().unwrap().borrow_mut().next = None;
        }
        if self.size > 0 {
            self.size -= 1;
        }
    }

    // Delete an element at the given index.
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
        let to_remove = self.go_to_element(i).unwrap();
        let prior = self.go_to_element(i - 1).unwrap();
        prior.borrow_mut().next = Some(Rc::clone(to_remove.borrow().next.as_ref().unwrap()));
        self.size -= 1;
    }
}
