use std::io;
use rand::Rng;

fn main() {
    let mut name = "".to_string();

    let master_number = rand::thread_rng().gen_range(1,101);

    println!("Master number is {}", master_number);

    get_input(&mut name);

    println!("Hello {}", name);

    println!("Let's play a game..A guessing game");
    println!("I am thinking of a number between 1 and 100");
    println!("Let's see if you can guess it right...");

    let mut guessed = false;
    for i in 1..6 {
        let mut guess = String::from("");

        println!("Enter your Guess.");

        get_input(&mut guess);

        println!("Your guess is {}", guess);

        let j = guess.trim().parse().expect("Please enter a number!!");

        if master_number == j {
            println!("Which is right!  Hurray");
            guessed = true;
            break;
        } else {
            println!("Which is wrong....You have {} chances", {5-i});
        }
    }

    if !guessed {
        println!("You ran out of chances, the correct number was {}", master_number);
    }


}

fn get_input(mut name: &mut String) {
    io::stdin().read_line(&mut name)
        .expect("Failed to read line");
}
