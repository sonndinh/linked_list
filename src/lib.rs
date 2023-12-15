pub mod linked_list {
    use std::rc::Rc;
    use std::cell::RefCell;

    // Implemented by LinkedList and DoublyLinkedList
    pub trait List {
        type ListNode;

        fn get_size(&self) -> usize;
        fn get_head(&self) -> Option<Rc<RefCell<Self::ListNode>>>;
        fn get_tail(&self) -> Option<Rc<RefCell<Self::ListNode>>>;

        // Go to the element at the given position (zero based).
        fn go_to_element(&self, pos: usize) -> Option<Rc<RefCell<Self::ListNode>>> {
            if pos >= self.get_size() {
                eprintln!("Invalid index: {}. List size: {}", pos, self.get_size());
                return None;
            }
            if pos == 0 {
                return Some(Rc::clone(self.get_head().as_ref().unwrap()));
            }
            if pos == self.get_size() - 1 {
                return Some(Rc::clone(self.get_tail().as_ref().unwrap()));
            }

            let mut curr_pos = 0;
            let mut curr = Rc::clone(self.get_head().as_ref().unwrap());
            while curr_pos < pos {
                let tmp = Rc::clone(curr.borrow().next.as_ref().unwrap());
                curr = Rc::clone(&tmp);
                curr_pos += 1;
            }
            Some(curr)
        }
    }

    // Singly linked list
    type Pointer<T> = Rc<RefCell<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T> {
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

    pub struct LinkedList<T> {
        head: Option<Pointer<T>>,
        tail: Option<Pointer<T>>,
        size: usize,
    }

    impl<T> List for LinkedList<T> {
        //type ListNode = Pointer<T>;
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

    impl<T: std::fmt::Debug> LinkedList<T> {
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

    // Doubly linked list
    type DoublyPointer<T> = Rc<RefCell<DoublyNode<T>>>;

    pub struct DoublyNode<T> {
        val: T,
        next: Option<DoublyPointer<T>>,
        prev: Option<DoublyPointer<T>>,
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

    impl<T: std::fmt::Debug> DoublyLinkedList<T> {
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

        pub fn insert_at_index(&mut self, _i: usize, _obj: T) {

        }

        pub fn delete_head(&mut self) {
        }

        pub fn delete_tail(&mut self) {
        }

        pub fn delete_at_index(&mut self, _i: usize) {
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::linked_list::*;

    #[test]
    fn insert_head() {
        let mut list = LinkedList::<i32>::new();
        list.insert_head(3);
        let content = list.get_at_index(0);
        assert_eq!(content.unwrap(), 3);
        assert_eq!(list.get_size(), 1);

        list.insert_head(2);
        assert_eq!(list.get_size(), 2);
        assert_eq!(list.get_at_index(0).unwrap(), 2);
        assert_eq!(list.get_at_index(1).unwrap(), 3);
        assert!(list.go_to_element(2).is_none());
    }

    #[test]
    fn insert_tail() {
        let mut list = LinkedList::<u32>::new();
        list.insert_tail(5);
        list.insert_tail(6);
        assert_eq!(list.get_size(), 2);
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
        assert_eq!(list.get_size(), 3);
        list.delete_head();
        assert_eq!(list.get_size(), 2);
        list.delete_head();
        assert_eq!(list.get_size(), 1);
        assert_eq!(list.get_at_index(0).unwrap(), 3);
    }

    #[test]
    fn delete_tail() {
        let mut list = LinkedList::<i16>::new();
        list.insert_tail(7);
        list.insert_tail(8);
        assert_eq!(list.get_size(), 2);
        list.delete_tail();
        assert_eq!(list.get_size(), 1);
        list.delete_tail();
        assert_eq!(list.get_size(), 0);
        list.delete_tail();
        assert_eq!(list.get_size(), 0);
    }

    #[test]
    fn insert_at_index() {
        let mut list = LinkedList::<i64>::new();
        list.insert_at_index(1, 10); // No-op since index is invalid
        assert_eq!(list.get_size(), 0);
        list.insert_at_index(0, 10);
        list.insert_at_index(0, 8);
        list.insert_at_index(1, 9);
        assert_eq!(list.get_size(), 3);
        assert_eq!(list.get_at_index(0).unwrap(), 8);
        assert_eq!(list.get_at_index(1).unwrap(), 9);
        assert_eq!(list.get_at_index(2).unwrap(), 10);
    }
}
