mod linked_list;
use linked_list::LinkedList;
fn main() {
    let mut lst = LinkedList::new();
    lst.append(5);
    lst.append(2);
    lst.append(2);
    lst.append(3);
    lst.append(4);
    println!("{:?}",lst);
    lst.list();
}
