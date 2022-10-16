use std::io;

fn main() {
    let mut test = String::new();

    println!("input text here:");

    io::stdin()
        .read_line(&mut test)
        .expect("failed to read input.");

    println!("you wrote {}", test);
}
