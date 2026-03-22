use rand::Rng;
//use std::cmp::Ordering;
use std::io;

fn main() {
    banner();

    let secret_number: i32 = rand::thread_rng().gen_range(1, 10);

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("unable to read from stdin");

        let user_input: i32 = user_input.trim().parse().expect("invalid input");

        match user_input.cmp(&secret_number) {
            std::cmp::Ordering::Greater => {
                println!("too high");
                continue;
            }
            std::cmp::Ordering::Equal => {
                println!("you win!");
                break;
            }

            std::cmp::Ordering::Less => {
                println!("too low");
                continue;
            }
        }
    }
}

fn banner() {
    println!("WELCOME TO");
    println!("________________");
    println!("|guess the number|");
    println!("________________");
}
