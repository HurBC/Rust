macro_rules! ternary {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    };
}

fn main() {
    goodbye("Franco");

    println!("Hello, world!");

    let number_to_divide_by_5 = 10;

    println!("{} divided by 5 is {}", number_to_divide_by_5, divide_by_5(number_to_divide_by_5));
}

fn goodbye(name: &str) {
    println!("Goodbye, {}", name);
}


fn divide_by_5(num: u32) -> u32 {
    return ternary!(num == 0 => 0; num / 5);
}
