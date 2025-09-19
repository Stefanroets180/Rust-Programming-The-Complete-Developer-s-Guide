fn main() {
    let my_name = "Jayson";
    match my_name{
        "Jayson" => println!("that is my name"),
        "Bob" => println!("that not name"),
        "Alice" => println!("hello alice"),
        _ => println!("nice to meet you!"),
    }

}