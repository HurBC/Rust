use regex::Regex;

mod authentication;
mod text_processing;

mod car_factory {
    pub fn build_car() {
        println!("Building a car");
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text);

    (number_of_letters, number_of_numbers)
}

fn main() {
    let user = authentication::User::new("Franco", "Amogus124");

    println!("The username is: {}", user.get_username());

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    println!("Did our date match with the regex? {}", re.is_match("2024-11-23"));

    car_factory::build_car();

    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
}
