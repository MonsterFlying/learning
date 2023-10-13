//use std::io;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!(" secret number {}", secret_number);
    println!("Guess the number!");
    println!("Please input your guess.");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入正确的类型!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Equal => {
                println!("恭喜你猜中了!");
                break;
            }
            Ordering::Greater => println!("太大了！"),
        }
    }
    println!("game is over!");
}
