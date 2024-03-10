use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");
    let _x = "squidface";
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

      println!("You guessed: {guess}");
      println!("The secret number is: {secret_number}");
}


fn _add_one(x: i32) -> i32 {
    let result: i32 = x + 1;
    println!("The sum is: {x}");
    result + 10
}



fn _add_two(x: i32) -> i32 {
    let result: i32 = x + 2;
    result
}



fn _add_three(x: i32) -> i32 {
    x + 3
}