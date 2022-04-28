use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
struct A {
    a: i32,
    b: u32,
}

fn jo(s: &String) -> String {
    let a = *s;

    "ho".to_string() + &a
}

fn main() {
    let  a = "1".to_string();
    jo(&a);


    println!("{}", a);
}
