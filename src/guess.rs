pub mod guess {
    pub struct Guess {
        value:i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            const MIN: i32 = 1;
            const MAX: i32 = 100;
            if value < MIN || value > MAX {
                panic!("Guess must be between {MIN} and {MAX}, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
