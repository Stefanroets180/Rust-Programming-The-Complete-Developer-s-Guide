// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let sum = 2 + 2;
    println!("Sum: {}", sum);
    let value = 10 - 5;
    println!("Value: {}", value);
    let division = 10 / 2;
    println!("Division: {}", division);
    let multiplication = 5 * 5;
    println!("Multiplication: {}", multiplication);

    let five = sub(8,3);
    println!("Five: {}", five);

    let rem = 6 % 3;
    println!("Rem: {}", rem);
    let rem2 = 6 % 4;
    println!("Rem2: {}", rem2);
}
