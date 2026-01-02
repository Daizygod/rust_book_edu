use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {

    // TASK print lyrics of song "The Twelve Days of Christmas"

    // println!("The Twelve Days of Christmas");
    // println!();
    //
    // let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];
    // let sent_by_true_love = [
    //     "Twelve drummers drumming",
    //     "Eleven pipers piping",
    //     "Ten lords a-leaping",
    //     "Nine ladies dancing",
    //     "Eight maids a-milking",
    //     "Seven swans a-swimming",
    //     "Six geese a-laying",
    //     "Five golden rings",
    //     "Four calling birds",
    //     "Three french hens",
    //     "Two turtle doves and",
    //     "A partridge in a pear tree"
    // ];
    //
    // let mut day = 1;
    //
    // while day <= 12 {
    //     println!("On the {} day of Christmas, my true love sent to me", days[day - 1]);
    //
    //     for day in (1..day+1).rev()
    //     {
    //         println!("{}", sent_by_true_love[12-day]);
    //     }
    //
    //     println!();
    //     day += 1;
    // }

    // TASK get fibonacci number

    // println!("Convert farenheit to celsius");
    // let max_index = 47;
    // loop {
    //     println!("Please input index of fibonacci number 0-n");
    //
    //     let mut index = String::new();
    //     io::stdin()
    //         .read_line(&mut index)
    //         .expect("Failed to read line");
    //
    //     let index: u32 = match index.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please input a valid number");
    //             continue
    //         }
    //     };
    //
    //     if index > max_index {
    //         println!("This program not support calculate more than unsigned integer 32, max index is {max_index} (I`m to lazy to use arrays sorry)");
    //         continue
    //     }
    //
    //     println!("Fibonacci number in index {index} is {}", get_fibonacci_by_index(index));
    //
    //     break;
    // }
    //
    // fn get_fibonacci_by_index(index: u32) -> u64
    // {
    //     if index == 0 {
    //         0
    //     } else if index == 1 {
    //         1
    //     } else {
    //         let mut iteration = index - 1;
    //         let mut first = 0;
    //         let mut second = 1;
    //         let mut temp;
    //
    //         while iteration > 0
    //         {
    //             temp = second;
    //             second = first + second;
    //             first = temp;
    //             iteration -= 1;
    //         }
    //
    //         second
    //     }
    // }

    // TASK celsius from fahrenheit

    // println!("Convert farenheit to celsius");
    //
    // loop {
    //     println!("Please input temp in farnheit ℉");
    //
    //     let mut fahrenheit = String::new();
    //     io::stdin()
    //         .read_line(&mut fahrenheit)
    //         .expect("Failed to read line");
    //
    //     let fahrenheit: f32 = match fahrenheit.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please input a valid number");
    //             continue
    //         }
    //     };
    //
    //     let celsius: f32 = (fahrenheit - 32.0) / 1.8;
    //
    //     println!("Temp in celsius: {celsius} °C");
    //
    //     break;
    // }


    // Guess number

    // println!("Guess the number");
    //
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    //
    // // println!("The secret number is: {}", secret_number);
    //
    // loop {
    //     println!("Please input ur guess");
    //
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please input a valid number");
    //             continue
    //         }
    //     };
    //
    //     println!("You guessed: {guess}");
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => { println!("You win!"); break;}
    //     }
    // }
}
