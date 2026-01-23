mod chapter_two;
use crate::chapter_two::add_with_lifetimes;
use crate::chapter_two::add;
use crate::chapter_two::search_string;
use crate::chapter_two::search_string_v2;

fn main() {
    let a = 2;
    let b= 3;

    let sum = add_with_lifetimes(&a, &b);
    println!("Sum: {}", sum);

    let sum = add(a, b);
    println!("Sum: {}", sum);

    let quote = "\
        Every face, every shop, bedroom window, public-house, and
        dark square is a picture feverishly turned--in search of what?
        It is the same with books.
        What do we seek through millions of pages?";
    search_string(quote, "books");
    search_string_v2(quote, "seek");
}
