fn main() {
    let p: Person = Methodes::new("Musah".to_string(), 8);
    p.show();
}
#[derive(Debug)]
#[allow(unused)]
struct Person {
    name: String,
    age: u8,
}
impl Methodes for Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}
use std::fmt::Debug;
trait Methodes {
    fn new(name: String, age: u8) -> Self;
    fn show(&self)
    where
        Self: Debug,
    {
        println!("{:?}", self);
    }
}
