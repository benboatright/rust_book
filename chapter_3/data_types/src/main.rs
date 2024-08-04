//https://rust-book.cs.brown.edu/ch03-02-data-types.html
// fn main() {

//     //Floats:
//     let _x = 2.0; // f64
//     let _y: f32 = 3.0; // f32

//     //Numeric Operations:
//     // addition
//     let _sum = 5 + 10;
//     // subtraction
//     let _difference = 95.5 - 4.3;
//     // multiplication
//     let _product = 4 * 30;
//     // division
//     let _quotient = 56.7 / 32.2;
//     let _truncated = -5 / 3; // Results in -1
//     // remainder
//     let _remainder = 43 % 5;

//     // Boolean
//     let _t = true;
//     let _f: bool = false; // with explicit type annotation

//     // Characters (single quotes)
//     let _c = 'z';
//     let _z: char = 'ℤ'; // with explicit type annotation
//     let _heart_eyed_cat = '😻';
    
//     //Tuple
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (_x, y, _z) = tup;
//     println!("The value of y is: {y}");
//     let five_hundred = tup.0;
//     println!("Value 1: {five_hundred}");
//     let six_point_four = tup.1;
//     println!("Value 2: {six_point_four}");
//     let one = tup.2;
//     println!("Value 3: {one}");

//     let mut x: (i32, i32) = (1, 2);
//     x.0 = 0;
//     x.1 += 5;

//     //Array
//     let a = [1, 2, 3, 4, 5];
//     let _first = a[0];
//     let _second = a[1];
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
