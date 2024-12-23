# Rust Smart Pointers

## 1st: Box

> The most straightforward smart pointer is a box, whose type is written `Box<T>`. Boxes allow you to store data on the heap rather than the stack.

* When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size.

* When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.

* When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

* `Box<T>` can help in scenarios where you need to explicitly manage ownership or when you want to move ownership of data to another part of your program without unnecessary copying.


## 2nd: `Rc<T>`: Reference Count

In the majority of cases, ownership is clear: you know exactly which variable owns a given value. However, there are cases when a single value might have multiple owners.

Rust enable multiple ownership explicitly by using the Rust type `Rc<T>`, The `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last



## 3rd: `RefCell<T>` and the Interior Mutability Pattern

Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.

We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime.

The RefCell<T> type is useful **when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that**.


## Summary

* Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.

* Box<T> allows immutable or mutable borrows checked at compile time; 

* Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
* RefCell<T> 

* Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
