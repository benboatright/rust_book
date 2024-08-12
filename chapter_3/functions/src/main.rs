// fn main() {
//     another_function(5,'h');
// }

// fn another_function(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }


fn f(x: i32) -> i32 { x + 1 }
fn main() {
  println!("{}", f({let y = 1;
    y + 1
  }));
}