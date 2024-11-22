fn main() {
    let mut counter = 1;

    let stop_loop = loop {
        counter *= 2;

        if counter > 100 {
            break counter;
        }
    };

    println!("Loop stopped at {}", stop_loop);

    let big_birds = vec!["Eagle", "Ostrich", "Emu"];

    for bird in big_birds {
        println!("{}", bird);
    }

    for number in 0..5 {
        println!("{}", number);
    }

}
