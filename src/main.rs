use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn read_num(num: u16) -> Option<Ordering> {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    match input_text.trim().parse::<u16>() {
        Ok(n) => Some(n.cmp(&num)),
        Err(_) => None,
    }
}

fn main() {
    let num = rand::thread_rng().gen_range(1..101);
    println!("Guessing game!");
    println!("Guess a number between 1-100: ");
    loop {
        match read_num(num) {
            Some(Ordering::Less) => println!("The num is greater."),
            Some(Ordering::Greater) => println!("The num is smaller."),
            Some(Ordering::Equal) => {
                println!("Got It!");
                break;
            }
            _ => println!("Invalid number"),
        }
    }
}
