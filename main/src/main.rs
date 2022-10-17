use std::io;
use rand::Rng;

fn main() {
    let mut guess = String::new();
    let luckynum = rand::thread_rng().gen_range(1..=10);

    println!("input text here:");

    println!("the secret number was {}", luckynum);

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read input.");

    println!("you wrote {}", guess);
}
