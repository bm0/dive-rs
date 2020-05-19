fn main() {
    let x = String::from("hello"); // structure types, such as String, store some of the data on the stack and some on the heap
    takes_ownership(x); // moving occurs here
    // println!("x: {}", x); // for this reason they cannot be used after moving

    let mut y = gives_ownership();
    y = takes_and_gives_back(y);
    println!("y: {}", y);

    let z = 5;  // scalar types don't store data on the heap, so data can be copied quickly
    makes_copy(z); // copying occurred here
    println!("{}", z); // so it can be used after moving (Copy trait)

    let j = String::from("hello");
    let len = calculate_length(&j); // borrowing occurred here
    println!("the len of '{}' is {}.", j, len);

    // let reference_to_nothing = dangle();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // some_string going in this scope

    // some_string leaves from this scope, 'drop' is called
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer); // some_string going in this scope
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.