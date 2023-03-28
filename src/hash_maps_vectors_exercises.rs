pub mod hash_maps_vectors_exercises {
    use std::collections::HashMap;

    pub fn foo() {
        let mut company = HashMap::new();

        company.insert("Titouan", "Dev");
        company.insert("Se", "Marketing");
        company.insert("Nicolas", "Boss");

        for (key, value) in &company {
            println!("{key}: {value}");
        }

        println!("{:?}", company);
    }
}
