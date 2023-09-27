use std::rc::Rc;
use std::cell::RefCell;

type Pointer<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Pointer<T>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node::<T> {
            val: t,
            next: None,
        }
    }
}

struct LinkedList<T> {
    head: Option<Pointer<T>>,
    tail: Option<Pointer<T>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
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

    // Go to the element at the given index.
    fn go_to_element(&self, i: usize) -> Option<Pointer<T>> {
        if i >= self.size {
            eprintln!("Invalid index: {}. List size: {}", i, self.size);
            return None;
        }
        if i == 0 {
            return Some(Rc::clone(self.head.as_ref().unwrap()));
        }
        if i == self.size - 1 {
            return Some(Rc::clone(self.tail.as_ref().unwrap()));
        }

        let mut curr_idx = 0;
        let mut curr = Rc::clone(self.head.as_ref().unwrap());
        while curr_idx <= i {
            let tmp = Rc::clone(curr.borrow().next.as_ref().unwrap());
            curr = Rc::clone(&tmp);
            curr_idx += 1;
        }
        Some(curr)
    }

    // Insert an element to the given index.
    pub fn insert_at_index(&mut self, i: usize, obj: T) {
        if i > self.size {
            eprintln!("Invalid index: {}. List size: {}", i, self.size);
            return;
        }

        if i == 0 {
            return self.insert_head(obj);
        }
        if i == self.size {
            return self.insert_tail(obj);
        }

        let curr = self.go_to_element(i - 1).unwrap();
        let node = Rc::new(RefCell::new(Node::new(obj)));
        node.borrow_mut().next = Some(Rc::clone(curr.borrow().next.as_ref().unwrap()));
        curr.borrow_mut().next = Some(Rc::clone(&node));
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
    }

    pub fn delete_tail(&mut self) {
        // TODO
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
        // TODO: Delete element in the middle.
    }
}

impl<T: Copy> LinkedList<T> {
    // Get the payload of the element at the given index.
    pub fn get_at_index(&self, i: usize) -> Option<T> {
        match self.go_to_element(i) {
            None => None,
            Some(node) => {
                Some(node.borrow().val)
            }
        }
    }
}

fn main() {
}
