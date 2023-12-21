pub mod linked_list {
    use std::rc::Rc;
    use std::cell::RefCell;

    // Implemented by any linked list node that has a next pointer.
    pub trait NodeHasNext {
        //type T: Copy;

        fn get_next(&self) -> Option<Rc<RefCell<Self>>>;
        //fn get_value(&self) -> Self::T;
    }

    // Implemented by any linked list that supports forward traversal.
    pub trait List {
        type ListNode: NodeHasNext;

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
                let tmp = Rc::clone(curr.borrow().get_next().as_ref().unwrap());
                curr = Rc::clone(&tmp);
                curr_pos += 1;
            }
            Some(curr)
        }

        // TODO: reuse this method for both lists.
        // fn get_at_index(&self, i: usize) -> Option<Self::ListNode::T> {
        //     match self.go_to_element(i) {
        //         None => None,
        //         Some(node) => {
        //             Some(node.borrow().val)
        //         },
        //     }
        // }
    }

    // Singly linked list
    type Pointer<T> = Rc<RefCell<Node<T>>>;

    #[derive(Debug)]
    pub struct Node<T> {
        val: T,
        next: Option<Pointer<T>>,
    }

    impl<T> NodeHasNext for Node<T> {
        fn get_next(&self) -> Option<Rc<RefCell<Node<T>>>> {
            match &self.next {
                Some(x) => Some(Rc::clone(x)),
                None => None,
            }
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

    impl<T> List for LinkedList<T> {
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

    impl<T> NodeHasNext for DoublyNode<T> {
        fn get_next(&self) -> Option<Rc<RefCell<DoublyNode<T>>>> {
            match &self.next {
                Some(x) => Some(Rc::clone(x)),
                None => None,
            }
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

    impl<T> List for DoublyLinkedList<T> {
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

    // Get the payload of the element at a given index.
    // Only work when the type of the payload is copyable.
    impl<T: Copy + std::fmt::Debug> DoublyLinkedList<T> {
        pub fn get_at_index(&self, i: usize) -> Option<T> {
            match self.go_to_element(i) {
                None => None,
                Some(node) => {
                    Some(node.borrow().val)
                },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::linked_list::*;

    #[test]
    fn insert_head() {
        // Singly linked list
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

        // Doubly linked list
        let mut doubly_list = DoublyLinkedList::<i32>::new();
        doubly_list.insert_head(7);
        doubly_list.insert_head(6);
        doubly_list.insert_head(5);
        assert_eq!(doubly_list.get_size(), 3);
        assert_eq!(doubly_list.get_at_index(1).unwrap(), 6);
        assert_eq!(doubly_list.get_at_index(2).unwrap(), 7);
        assert_eq!(doubly_list.get_at_index(0).unwrap(), 5);
        assert!(doubly_list.get_at_index(3).is_none());
        doubly_list.delete_at_index(1);
        assert_eq!(doubly_list.get_size(), 2);
        assert_eq!(doubly_list.get_at_index(0).unwrap(), 5);
        assert_eq!(doubly_list.get_at_index(1).unwrap(), 7);
    }

    #[test]
    fn insert_tail() {
        // Singly linked list
        let mut list = LinkedList::<u32>::new();
        list.insert_tail(5);
        list.insert_tail(6);
        assert_eq!(list.get_size(), 2);
        assert_eq!(list.get_at_index(0).unwrap(), 5);
        assert_eq!(list.get_at_index(1).unwrap(), 6);
        assert!(list.go_to_element(4).is_none());

        // Doubly linked list
        let mut doubly_list = DoublyLinkedList::<char>::new();
        doubly_list.insert_tail('a');
        doubly_list.insert_tail('b');
        doubly_list.insert_tail('c');
        assert_eq!(doubly_list.get_size(), 3);
        assert_eq!(doubly_list.get_at_index(2).unwrap(), 'c');
        assert_eq!(doubly_list.get_at_index(1).unwrap(), 'b');
        assert_eq!(doubly_list.get_at_index(0).unwrap(), 'a');
        doubly_list.delete_at_index(2);
        assert_eq!(doubly_list.get_size(), 2);
        assert_eq!(doubly_list.get_at_index(0).unwrap(), 'a');
        assert_eq!(doubly_list.get_at_index(1).unwrap(), 'b');
    }

    #[test]
    fn delete_head() {
        // Singly linked list
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

        // Doubly linked list
        let mut doubly_list = DoublyLinkedList::<u32>::new();
        doubly_list.insert_at_index(0, 10);
        doubly_list.insert_at_index(1, 12);
        doubly_list.insert_at_index(1, 11);
        assert_eq!(doubly_list.get_size(), 3);
        doubly_list.delete_head();
        assert_eq!(doubly_list.get_size(), 2);
    }

    #[test]
    fn delete_tail() {
        // Singly linked list
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

        // Doubly linked list
        let mut doubly_list = DoublyLinkedList::<u16>::new();
        doubly_list.insert_head(100);
        doubly_list.insert_head(200);
        assert_eq!(doubly_list.get_size(), 2);
        doubly_list.delete_tail();
        assert_eq!(doubly_list.get_size(), 1);
    }

    #[test]
    fn insert_at_index() {
        // Singly linked list
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

        // Doubly linked list
        let mut doubly_list = DoublyLinkedList::<char>::new();
        doubly_list.insert_at_index(0, 'a');
        doubly_list.insert_at_index(1, 'b');
        assert_eq!(doubly_list.get_size(), 2);
        assert_eq!(doubly_list.get_at_index(1).unwrap(), 'b');
        doubly_list.delete_tail();
        assert_eq!(doubly_list.get_at_index(0).unwrap(), 'a');
        assert!(doubly_list.get_at_index(1).is_none());
    }
}
