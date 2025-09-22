// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop stat ement
// * Use break to exit the loop

fn main() {
    let mut s = 1;
    loop {
        println!("{:?}", s);
        if s == 4 {
            break;
        }
        s = s + 1;
    }
}