enum Direction {
    Left,
    Up,
    Right,
}
fn main() {
    let go = Direction::Up;
    match go {
        Direction::Left => println!("Go left"),
        Direction::Right => println!("Go right"),
        Direction::Up => println!("Go up"),
    }

}