fn main() {
    // Arrays
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let bytes = [0; 5];

    let first_day = days[0];
    let last_day = days[days.len() - 1];

    println!("{:?}", bytes);
    println!("{}", first_day);
    println!("{}", last_day);

    // Vectors
    let three_numbers = vec![1, 2, 3];
    let zeros = vec![0; 5];

    println!("{:?}", three_numbers);
    println!("{:?}", zeros);

    let mut fruit = Vec::new();

    fruit.push("apple");
    fruit.push("Banana");
    fruit.push("Cherry");

    println!("Fruits: {:?}", fruit);

    println!("Last fruit: {:?}", fruit.pop());

    fruit[1] = "Blueberry";

    println!("Fruits: {:?}", fruit);
}
