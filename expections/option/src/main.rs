fn main() {
    excercise();

    let fruits =vec!["banana", "apple", "coconut", "orange", "strawberry"];

    for &index in [0, 2, 99].iter() {
        // Like switch-case
        match fruits.get(index) {
            Some(&"banana") => println!("Bananas are gay!"),
            Some(fruit_name) => println!("Fruit name: {}", fruit_name),
            None => println!("No fruit at index {}", index),
        }
    }

    let a_number = Some(7);

    // only for one value
    if let Some(7) = a_number {
        println!("That's a seven!");
    }

    // unwrap
    // this access to Some value
    let gift = Some("Candy");

    assert_eq!(gift.unwrap(), "Candy");

    // let empty_gift: Option<&str> = None;

    // if unwrap value is Option::None, this will panic!
    // assert_eq!(empty_gift.unwrap(), "Candy");

    let a = Some("value");

    assert_eq!(a.expect("a Fruits are healthy"), "value");

    let b: Option<&str> = None;

    b.expect("b Fruits are healthy");

    // in case that value is Option::None, this will return default value
    // Cat in this case
    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "dog");

    panic!("FIRE IN THE HOLEEE!!!!");
}

struct Person {
    first_name: String,
    middle_name: Option<String>,
    last_name: String
}

fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();

    full_name.push_str(&person.first_name);
    full_name.push(' ');

    if let Some(middle) = &person.middle_name {
        full_name.push_str(&middle);
        full_name.push(' ');
    };

    full_name.push_str(&person.last_name);

    full_name
}

fn excercise() {
    let john = Person {
        first_name: String::from("James"),
        middle_name: Some(String::from("Oliver")),
        last_name: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");

    let alice = Person {
        first_name: String::from("Alice"),
        middle_name: None,
        last_name: String::from("Stevens"),
    };
    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first_name: String::from("Robert"),
        middle_name: Some(String::from("Murdock")),
        last_name: String::from("Jones"),
    };
    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
}
