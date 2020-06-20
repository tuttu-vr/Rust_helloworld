use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let random_number = rand::thread_rng().gen_range(1, 100);
    println!("generated: {}", random_number);

    loop {
        println!("Please enter guessing number!");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read");

        let guess: i32 = guess.trim().parse()
            .expect("Failed to parse");
        
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("Clear!");
                break;
            },
        }
    }
    println!("Finish");
}
