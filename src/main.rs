use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let random_number: i32 = rand::thread_rng().gen_range(1, 3);
    // println!("rand: {}", random_number);

    loop {
        println!("Guess a number: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Nummer: {}", input);

        match input.cmp(&random_number) {
            Ordering::Equal => {
                println!("Wuhuu guessed!");
                break;
            }
            Ordering::Less => println!("Too less"),
            Ordering::Greater => println!("Too big")
        }
    }
}
