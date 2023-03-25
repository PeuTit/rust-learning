fn main() {
    println!("{} Hello, world! {}", "FDP", truthy());
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
