use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    let sercret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", sercret_number);
loop{
    println!("guessing game");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Please enter a valid number!".red());
            continue;
        }
    };
   
match guess.cmp(&sercret_number){
    Ordering::Less => {
        println!("{}", "Too small!".red());
    },
    Ordering::Greater => {
        println!("{}", "Too big!".red());
    },
    Ordering::Equal => {
        println!("{}", "You win!".green());
      break;
    },
}
 
}

    
}