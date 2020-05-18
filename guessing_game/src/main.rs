use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut buf = String::with_capacity(8);
    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        buf.clear();

        io::stdin().read_line(&mut buf)
            .expect("Failed to read line");

        let guess: u32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
