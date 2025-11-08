fn main() {
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });

    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });

    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });

    for composer in composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}
