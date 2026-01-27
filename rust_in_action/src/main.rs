use std::path::Path;

mod chapter_two;
mod chapter_three;
use crate::chapter_two::chapter_two::add_with_lifetimes;
use crate::chapter_two::chapter_two::add;
use crate::chapter_two::chapter_two::search_string;
use crate::chapter_two::chapter_two::search_string_v2;
use crate::chapter_two::chapter_two::read_a_file;
use crate::chapter_three::chapter_three::*;

fn main() {

    // Chapter 2
    // let path = Path::new("./src/chapter_two/file.txt");
    // let a = 2;
    // let b= 3;

    // let sum = add_with_lifetimes(&a, &b);
    // println!("Sum: {}", sum);

    // let sum = add(a, b);
    // println!("Sum: {}", sum);

    // let quote = "\
    //     Every face, every shop, bedroom window, public-house, and
    //     dark square is a picture feverishly turned--in search of what?
    //     It is the same with books.
    //     What do we seek through millions of pages?";
    // search_string(quote, "books");
    // search_string_v2(quote, "seek");
    // read_a_file(&path);

    // Chapter 3
    let mut f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_data = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_data);
}
