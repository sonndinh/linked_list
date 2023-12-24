// Implement a singly linked list and a doubly linked list.
// For now the type of the payload in the nodes must be Copy.
pub mod common_traits;
pub mod linked_list;
pub mod doubly_linked_list;

#[cfg(test)]
mod tests {
    use crate::common_traits::*;
    use crate::linked_list::*;
    use crate::doubly_linked_list::*;

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
