use crate::rectangle::Rectangle;
use rand::Rng;
use std::fs::File;
use std::{cmp::Ordering, io};
//use crate::guess::guess::Guess;
//use crate::collections_exercises::collections_exercises::foo;
//use crate::hash_maps_vectors_exercises::hash_maps_vectors_exercises::foo;
use crate::generics_exercises::generics_exercises::foo;

pub mod collections_exercises;
pub mod generics_exercises;
pub mod guess;
pub mod hash_maps_vectors_exercises;
pub mod rectangle;

fn main() {
    foo();
}

fn _open_file() {
    let _greeting_file = File::open("bonjour.txt")
        .expect("bonjour.txt should be included in the project root folder!");

    let greeting_file_result = File::open("hello.txt");

    let greeting_file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    println!("{:?}", greeting_file_result.metadata());
}

fn _rectangle() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    let rect2: Rectangle = rect1.times(2);

    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("The area of the rectangle is: {}", rect1.area());

    println!("The area of the rectangle times 2 is: {}", rect2.area());

    println!("Rect2 fit in rect1: {}", rect1.can_hold(&rect2));

    println!("Rect3 fit in rect1: {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(21);

    println!("Square area is: {}", square1.area());
}

fn _guess_game() {
    println!("Guess the number!");

    let random_number: i32 = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();
        let msg: &str = "No guess";

        io::stdin().read_line(&mut guess).expect(msg);

        println!("You guessed: {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            }
        }
    }
}

fn _truthy() -> bool {
    return true;
}

#[cfg(test)]
mod test {
    use super::_truthy;

    #[test]
    fn test_truthy() {
        assert_eq!(true, _truthy());
    }

    #[test]
    fn test_untruthy() {
        assert_ne!(false, _truthy());
    }
}
