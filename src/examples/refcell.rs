use std::cell::RefCell;

#[derive(Debug, Clone)]
enum List<T: PartialEq> {
    Cons(T, Box<RefCell<List<T>>>),
    Nil,
}

impl<T: PartialEq> List<T> {
    pub fn new() -> Self {
        // Self::Cons(v, Box::new(RefCell::new(Self::Nil)))
        Self::Nil
    }

    pub fn add(&mut self, v: T) {
        match &self {
            List::Cons(_, next) => {
                next.borrow_mut().add(v);
            }
            List::Nil => {
                *self = List::Cons(v, Box::new(RefCell::new(List::Nil)));
            }
        }
    }
}

fn main() {
    let mut list: List<i32> = List::new(); // List::Nil
    list.add(1); // List::Cons(1, List::Nil)
    list.add(2); // List::Cons(1, List::Cons(2, List::Nil))
    list.add(3); // List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)))
    list.add(100); // List::Cons(1, List::Cons(2, List::Cons(3, List::Cons(100, List::Nil))))

    let mut cur = list;
    while let List::Cons(v, next) = cur {
        println!("{}", v);
        let borrowed = next.borrow();
        cur = borrowed.clone();
    }
}
