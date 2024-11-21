// Declare enum for Car transmission type
#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

// Declare enum for Car age type
#[derive(PartialEq, Debug)]
enum Age { New, Used }

// Declare Car struct to describe vehicle with four named fields
#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32)
}

fn main() {
}
