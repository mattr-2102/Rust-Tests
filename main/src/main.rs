use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    //declare random gen number
    let luckynum = rand::thread_rng().gen_range(1..=100);

    //guess loop
    loop {
        let mut guess = String::new();
        println!("input a guess 1-100 here:");

        //gather input
        io::stdin().read_line(&mut guess).expect("failed to read input.");
    
        //input => int var
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //check guess against random num
        match guess.cmp(&luckynum) {
            Ordering::Less => println!("Guess was too low."),
            Ordering::Equal => {
                println!("You guessed correctly!");
                println!("the lucky number was {}, and you guessed {}", luckynum, guess);
                break;
            },
            Ordering::Greater => println!("Guess was too high."),
        }
    }
}
