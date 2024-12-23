/*
A value of a recursive type can have another value of the same type as part of itself.

Recursive types pose an issue because, at compile time, Rust needs to know how much space a type takes up.
However, the nesting of values of recursive types could theoretically continue infinitely,

so Rust can’t know how much space the value needs. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition
*/
#[derive(Debug)]
pub enum List<T: PartialEq + Sized> {
    // Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    // We define the variable b_int to have the value of a Box that points to the value 10, which is allocated on the heap
    let b_int = Box::new(10);
    println!("boxed i32: {}", b_int);

    // The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references
    let b_str = Box::new("Hello, World!");
    println!("boxed str: {}", b_str);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("list: {:?}", list);

    let list1 = List::Cons(
        "Hi",
        Box::new(List::Cons("Good Morning", Box::new(List::Nil))),
    );
    println!("list1: {:?}", list1);
}
