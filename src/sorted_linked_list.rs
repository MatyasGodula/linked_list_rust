use std::fmt::Debug;

pub struct SortedList<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T: Ord> Node<T> { 
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        Node {
            value,
            next
        }
    }

}

impl<T: Ord> SortedList<T> { // the implementation for the linked list requires a type that supports ordering
    pub fn new() -> Self {
        SortedList { head: None }
    }

    pub fn insert(&mut self, value: T) { // 
        let mut new_node = Box::new(Node::new(value, None));

        if self.head.is_none() || self.head.as_ref().unwrap().value > new_node.value {
            new_node.next = self.head.take();
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if node.next.is_none() || node.next.as_ref().unwrap().value > new_node.value {
                // Insert new node here
                new_node.next = node.next.take(); // Link new node to the next node
                node.next = Some(new_node); // Update current node's next pointer
                return;
            }
            current = node.next.as_mut(); // Move to the next node
        }
    }

    pub fn delete(&mut self, value: T) {
        if self.head.is_none() {
            return; // nothing to delete
        }

        if self.head.as_ref().unwrap().value == value {
            self.head = self.head.as_mut().unwrap().next.take();
            return;
        }

        let mut current = self.head.as_mut();
        while let Some(node) = current { // node == &mut Box<Node<T>>
            if let Some(next_node) = node.next.as_mut() {
                if next_node.value == value {
                    node.next = next_node.next.take();
                }
            }
            current = node.next.as_mut();
        }
    }

    pub fn print_list(&self) where T: Debug, {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            print!("{:?}, ", node.value);
            current = node.next.as_ref();
        }
        print!("\n");
    }

    pub fn clear(&mut self) {
        self.head = None; // drops the list
    }

}