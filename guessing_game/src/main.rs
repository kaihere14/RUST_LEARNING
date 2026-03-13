use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello doston");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
    println!("Enter your number for guess : ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read");
    let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            },
        };

    println!("You guessed : {}",guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
    }

}
