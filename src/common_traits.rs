use std::rc::Rc;
use std::cell::RefCell;

// Implemented by any linked list node that has a next pointer.
pub trait NodeHasNext {
    type ValueType: Copy;

    fn get_next(&self) -> Option<Rc<RefCell<Self>>>;
    fn get_value(&self) -> Self::ValueType;
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

    fn get_at_index(&self, i: usize) -> Option<<Self::ListNode as NodeHasNext>::ValueType> {
        match self.go_to_element(i) {
            None => None,
            Some(node) => {
                Some(node.borrow().get_value())
            },
        }
    }
}
