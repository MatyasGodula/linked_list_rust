mod sorted_linked_list;

use sorted_linked_list::SortedList;

fn main() {

    let mut list = SortedList::new();
    // Insert elements
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.insert(4);

    println!("Original list:");
    list.print_list(); // Output: 1, 2, 3, 4

    // Delete the head
    list.delete(1);
    println!("After deleting 1 (head):");
    list.print_list(); // Output: 2, 3, 4

    // Delete a middle element
    list.delete(3);
    println!("After deleting 3:");
    list.print_list(); // Output: 2, 4

    // Delete the tail
    list.delete(4);
    println!("After deleting 4 (tail):");
    list.print_list(); // Output: 2

    // Try to delete a non-existent element
    list.delete(42);
    println!("After trying to delete 42:");
    list.print_list(); // Output: 2

    list.clear();
}
