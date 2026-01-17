mod chapter_two;
use crate::chapter_two::add_with_lifetimes;

fn main() {
    let a = 2;
    let b= 3;

    let sum = add_with_lifetimes(&a, &b);
    println!("Sum: {}", sum);
}
