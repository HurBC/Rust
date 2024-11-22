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

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {  (Age::Used, miles) } else { (Age::New, miles) }
}

fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut motor = Transmission::Manual;
    let mut roof = true;
    
    let mut color = order as usize;

    color = color % 4;

    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }

    Car {
        color: colors[color].to_string(),
        motor,
        roof,
        age: car_quality(miles)
    }
}

fn main() {
    use std::collections::HashMap;

    let mut orders: HashMap<i32, Car> = HashMap::new();
    let mut car: Car;
    let mut miles = 0;

    let max_orders = 12;

    for order in 1..max_orders {
        car = car_factory(order, miles);

        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        miles = if miles == 2100 { 0 } else { miles + 700 };
    }
}
