use rust_iterators::{Incrementing, Take};

fn main() {
    let values = Take::new(Incrementing::new(0), 10);
    for value in values {
        println!("value: {value}");
    }
}
