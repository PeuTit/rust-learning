pub mod collections_exercises {
    use ordered_float::OrderedFloat;

    pub fn foo() {
        let mut v: Vec<OrderedFloat<f64>> = vec![
            OrderedFloat(12.0),
            OrderedFloat(5.0),
            OrderedFloat(6.0),
            OrderedFloat(89.0),
            OrderedFloat(5.0),
            OrderedFloat(2390.0),
            OrderedFloat(1.0),
        ];
        let mut v2: Vec<OrderedFloat<f64>> = vec![
            OrderedFloat(12.0),
            OrderedFloat(5.0),
            OrderedFloat(6.0),
            OrderedFloat(89.0),
            OrderedFloat(5.0),
            OrderedFloat(1.0),
        ];

        for i in &mut v {
            println!("{i}");
        }

        let v_median = median_float(&mut v);
        let v_median2 = median_float(&mut v2);

        // println!("{:?}", v);
        println!("v_median: {:?}", v_median);

        // println!("{:?}", v2);
        println!("v_median2: {:?}", v_median2);
    }

    pub fn median_float(v: &mut Vec<OrderedFloat<f64>>) -> Option<&OrderedFloat<f64>> {
        let _ = &v.sort();

        // for i in v.into_iter() {
        //   println!("{i}");
        // }

        let v_len = &v.len();
        let mid_len = v_len / 2 + 1;

        let (x, y) = &v.split_at(mid_len);
        if v_len % 2 == 0 {
            //avg_val(&x.last().get_or_insert(&0.0).into(), &y.first());
            y.first()
        } else {
            x.last()
        }
    }

    pub fn avg_val_float(x: &f64, y: &f64) -> f64 {
        (x + y) / 2.0
    }

    pub fn median_int(v: &mut Vec<i64>) -> Option<&i64> {
        let _ = &v.sort();

        // for i in v.into_iter() {
        //   println!("{i}");
        // }

        let v_len = &v.len();
        let mid_len = v_len / 2 + 1;

        let (x, y) = v.split_at(mid_len);
        if v_len % 2 == 0 {
            //let xx = x.last().get_or_insert(&0);
            //let yy = y.first().get_or_insert(&0);

            //avg_val_int(&xx, &yy);

            //return *xx + *yy;
            //return Some(&avg_val_int(x.last().get_or_insert(&0), y.first().get_or_insert(&0)));
            //Some(&res)
            y.first()
        } else {
            x.last()
        }
    }

    pub fn avg_val_int(x: &i64, y: &i64) -> i64 {
        (*x + *y) / 2
    }
}

#[cfg(test)]
mod test {
    use super::collections_exercises::{median_float, avg_val_float, median_int, avg_val_int};
    use ordered_float::OrderedFloat;

    #[test]
    fn test_median_float() {
        let mut v: Vec<OrderedFloat<f64>> = vec![
            OrderedFloat(12.0),
            OrderedFloat(5.0),
            OrderedFloat(6.0),
            OrderedFloat(89.0),
            OrderedFloat(5.0),
            OrderedFloat(2390.0),
            OrderedFloat(1.0),
        ];

        let mut v2: Vec<OrderedFloat<f64>> = vec![
            OrderedFloat(12.0),
            OrderedFloat(5.0),
            OrderedFloat(7.0),
            OrderedFloat(89.0),
            OrderedFloat(5.0),
            OrderedFloat(1.0),
        ];

        assert_eq!(Some(&OrderedFloat(6.0)), median_float(&mut v));
        assert_eq!(Some(&OrderedFloat(6.0)), median_float(&mut v2));
    }
    
    #[test]
    fn test_avg_value_float() {
        let x: f64 = 5.0;
        let y: f64 = 6.0;

        assert_eq!(avg_val_float(&x, &y), 5.5);
    }

    #[test]
    fn test_median_int() {
        let mut v: Vec<i64> = vec![
            12,
            5,
            6,
            89,
            5,
            2390,
            1,
        ];

        let mut v2: Vec<i64> = vec![
            12,
            5,
            7,
            89,
            5,
            1,
        ];

        assert_eq!(Some(&6), median_int(&mut v));
        assert_eq!(Some(&6), median_int(&mut v2));
    }
    
    #[test]
    fn test_avg_value_int() {
        let x: i64 = 5;
        let y: i64 = 7;

        assert_eq!(avg_val_int(&x, &y), 6);
    }
}
