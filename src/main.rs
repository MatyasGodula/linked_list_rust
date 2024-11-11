mod sorted_linked_list;

use sorted_linked_list::SortedList;

fn main() {
    let mut list: SortedList<i32> = SortedList::new(); // have not implemented new for SortedList yet
    list.insert(3);
    list.insert(4);
    list.insert(5);
    list.insert(3);

    list.delete(3);

    list.print_list(); // should print 4, 5

    list.clear();
}
