// exploring the cons list
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use std::ops::Deref;
mod pointers;
// use std::rc::Rc;

// use pointers::List::{Cons, Nil};
// use pointers::ListTwo::{Cons as otherCons, Nil as otherNil};
fn main() {
    // let b = Box::new(8);
    // println!("b = {b}");
    // use crate::List::{Cons, Nil};
    // // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // let x = 9;
    // let y = &x;
    // // println!("x= {}",*y);
    // assert_eq!(9, x);
    // assert_eq!(9, *y);

    // // using box
    // let x: i128 = 567890986755;
    // let y = Box::new(x);
    // println!("The size of  = {}", std::mem::size_of_val(&y));

    // //  Definig our own Smart pointer
    // let x = 6;
    // let y = MyBox::new(x);
    // println!("The value of x = {}", *y);
    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);

    // implmenting drop traits

    // let c = CustomeSmartPointer {
    //     data: String::from("My Rust Book"),
    // };
    // let d = CustomeSmartPointer {
    //     data: String::from("My Rust Book2"),
    // };

    // println!("CustomSmartPointers created.");
    // drop(d);
    // println!("The CustomSmartPointer dropped before the end of main");

    // Exploring CountedSmartPointer
    // let a = Cons(5, Box::new(&Nil));
    // let b = Cons(3, Box::new(&a));
    // let c = Cons(4, Box::new(&a));
    // // counting the increment in refrence count;
    // let a1 = Rc::new(otherCons(5, Rc::new(otherCons(10, Rc::new(otherNil)))));
    // println!("Count after creating  a1 = {}", Rc::strong_count(&a1));
    // let b1 = otherCons(3, Rc::clone(&a1));
    // println!("Count after creating b1 = {}", Rc::strong_count(&a1));
    // {
    //     let c1 = otherCons(4, Rc::clone(&a1));
    //     println!("Count after creating c = {}", Rc::strong_count(&a1));
    // }
    // println!(
    //     "Count after c goes out of scope = {}",
    //     Rc::strong_count(&a1)
    // );



    
}

// struct CustomeSmartPointer {
//     data: String,
// }

// impl Drop for CustomeSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomeSmartPointer with data `{}`!", self.data);
//     }
// }
// struct MyBox<T>(T);

// // here we have not implemented the traits which enables us to use derefrencing on mybox
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// fn hello(name: &str) {
//     println!("Hello, {name}!")
// }
