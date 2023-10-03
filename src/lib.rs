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

impl<T: std::fmt::Debug> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList::<T> {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
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
        while curr_idx < i {
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

        let prev = self.go_to_element(i - 1).unwrap();
        println!("prev node: {:?}", prev.borrow().val);
        let node = Rc::new(RefCell::new(Node::new(obj)));
        println!("insert node: {:?}", node.borrow().val);
        let next = Rc::clone(prev.borrow().next.as_ref().unwrap());
        println!("next node: {:?}", next.borrow().val);
        //node.borrow_mut().next = Some(Rc::clone(prev.borrow().next.as_ref().unwrap()));
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

        let to_remove = self.go_to_element(i).unwrap();
        let prior = self.go_to_element(i - 1).unwrap();
        prior.borrow_mut().next = Some(Rc::clone(to_remove.borrow().next.as_ref().unwrap()));
        self.size -= 1;
    }
}

impl<T: Copy + std::fmt::Debug> LinkedList<T> {
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn insert_head() {
        let mut list = LinkedList::<i32>::new();
        list.insert_head(3);
        let content = list.get_at_index(0);
        assert_eq!(content.unwrap(), 3);
        assert_eq!(list.size(), 1);

        list.insert_head(2);
        assert_eq!(list.size(), 2);
        assert_eq!(list.get_at_index(0).unwrap(), 2);
        assert_eq!(list.get_at_index(1).unwrap(), 3);
        assert!(list.go_to_element(2).is_none());
    }

    #[test]
    fn insert_tail() {
        let mut list = LinkedList::<u32>::new();
        list.insert_tail(5);
        list.insert_tail(6);
        assert_eq!(list.size(), 2);
        assert_eq!(list.get_at_index(0).unwrap(), 5);
        assert_eq!(list.get_at_index(1).unwrap(), 6);
        assert!(list.go_to_element(4).is_none());
    }

    #[test]
    fn delete_head() {
        let mut list = LinkedList::<i32>::new();
        list.insert_head(3);
        list.insert_head(2);
        list.insert_head(1);
        assert_eq!(list.size(), 3);
        list.delete_head();
        assert_eq!(list.size(), 2);
        list.delete_head();
        assert_eq!(list.size(), 1);
        assert_eq!(list.get_at_index(0).unwrap(), 3);
    }

    #[test]
    fn delete_tail() {
        let mut list = LinkedList::<i16>::new();
        list.insert_tail(7);
        list.insert_tail(8);
        assert_eq!(list.size(), 2);
        list.delete_tail();
        assert_eq!(list.size(), 1);
        list.delete_tail();
        assert_eq!(list.size(), 0);
        list.delete_tail();
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn insert_at_index() {
        let mut list = LinkedList::<i64>::new();
        list.insert_at_index(1, 10); // No-op since index is invalid
        assert_eq!(list.size(), 0);
        list.insert_at_index(0, 10);
        list.insert_at_index(0, 8);
        println!("{} {}", list.get_at_index(0).unwrap(), list.get_at_index(1).unwrap());
        list.insert_at_index(1, 9);
        assert_eq!(list.size(), 3);
        println!("{} {} {}", list.get_at_index(0).unwrap(),
                 list.get_at_index(1).unwrap(), list.get_at_index(2).unwrap());
        //assert!(false);
        assert_eq!(list.get_at_index(0).unwrap(), 8);
        assert_eq!(list.get_at_index(1).unwrap(), 9);
        assert_eq!(list.get_at_index(2).unwrap(), 10);
    }
}
