mod basket;
mod stack;
mod container;

use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::new(String::from("Basket1"));
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    let s1 = Stack::new(vec![String::from("One")]);
    let s2 = Stack::new(vec![1,2,3]);
}
