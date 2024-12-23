use std::rc::Rc;

#[derive(Debug)]
enum List<T: Sized> {
    Cons(T, Rc<List<T>>),
    Nil,
}

/*
Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules: multiple mutable borrows to the same place can cause data races and inconsistencies
*/

fn main() {
    let list1 = Rc::new(List::Cons("hub", Rc::new(List::Nil))); // ref count 1
    println!("count: {}", Rc::strong_count(&list1));

    let list2 = List::Cons("palce1", Rc::clone(&list1)); // ref count 2
    let list3 = List::Cons("palce2", Rc::clone(&list1)); // ref count 3

    println!("count: {}", Rc::strong_count(&list1));

    println!("list2: {:?}", list2);
    println!("list3: {:?}", list3);
}
