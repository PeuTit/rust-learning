pub mod generics_exercises {
    use std::cmp::PartialOrd;
    use std::fmt::Display;

    pub fn foo() {
        println!("generics_exercises");
        let number_list = vec![34, 50, 25, 100, 65];
        let number_list_2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let res1 = largest(&number_list);
        println!("The largest number is {}", res1);
        let res2 = largest(&number_list_2);
        println!("The largest number is {}", res2);

        points();

        let x = "salut";
        let y = "au revoir";
        let ann = "C'est la fete!";

        let res3 = longest_with_anouncement(x, y, ann);
        println!("The largest is {}", res3);
    }

    fn largest<T: PartialOrd>(v: &[T]) -> &T {
        let mut largest = &v[0];

        for number in v {
            if number > largest {
                largest = number;
            }
        }

        largest
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &U {
            &self.y
        }
    }

    impl Point<f32, f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    fn points() {
        let integer = Point { x: 5, y: 1 };
        let float = Point { x: 5.0, y: 1.0 };

        let _integer_float = Point { x: 5, y: 1.0 };

        let p = integer;

        println!("p.x = {}", p.x());
        println!("p.y = {}", p.y());

        let dist = float.distance_from_origin();
        println!("distance from origin: {}", dist);

        // No method for other types
        // integer.distance_from_origin(); <- Does not compile
    }

    fn longest_with_anouncement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
