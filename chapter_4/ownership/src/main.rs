// What is Ownership?: https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html
// References and Borrowing: https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html

// The expression &m1 uses the ampersand operator to create a reference to (or "borrow") m1. 
// The type of the greet parameter g1 is changed to &String, meaning "a reference to a String".

// References are non-owning pointers, because they do not own the data they point to.
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
    println!("{}!",s)
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2);
}

// Dereferencing uses *

fn main(){
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
                             // so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;        // so only one dereference is needed to read it
}