
enum Meme<T> {
    Pepe,
    Surreal(T),
}

use Meme::*;
use rand::Rng;

fn main() {
    let mut meme = get_meme(rand::thread_rng().gen_range(0, 2));
    if let Surreal(x) = meme {
        println!("meme matched as Surreal({})", x);
    }

    meme = get_meme(rand::thread_rng().gen_range(0, 2));
    match meme {
        Pepe => println!("meme matched as Pepe"),
        Surreal(x) => println!("meme matched as Surreal({})", x),
    }

    let x = match meme {
        Pepe => 0,
        Surreal(x) => x,
    };

    println!("digital representation of meme: {}", x)
}

fn get_meme(i: u8) -> Meme<u8> {
    if i > 0 {
        return Surreal(i)
    }

    Pepe
}
