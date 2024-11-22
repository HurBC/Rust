fn main() {
    if 1 == 2 {
        println!("Numbers are equals");
    } else {
        println!("Numbers are not equals");
    }

    let formal = true;
    let greetings = if formal {
        "Good day"
    } else {
        "Hi"
    };

    println!("{}", greetings);

    let num = 500; // num variable can be set at some point in the program
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }

    println!("Is {} out of range? {}", num, out_of_range);
}
