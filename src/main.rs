use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let random_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        let msg = "No guess";

        io::stdin()
            .read_line(&mut guess)
            .expect(msg);

        println!("You guessed: {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Equal!");
                break;
            }
        }
    }
}

fn truthy() -> bool {
    return true;
}

#[cfg(test)]
mod test {
    use super::truthy;

    #[test]
    fn test_truthy() {
        assert_eq!(true, truthy());
    }

    #[test]
    fn test_untruthy() {
        assert_ne!(false, truthy());
    }
}
