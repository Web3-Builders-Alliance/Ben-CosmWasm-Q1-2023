// Enumerators in Rust

// Enums are types which have a few definite values
// In CosmWasm, we use enums to define the possible actions that can be performed by the contract

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

// define a function that uses the Movement enum as input
fn move_player(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Moving up"),
        Movement::Down => println!("Moving down"),
        Movement::Left => println!("Moving left"),
        Movement::Right => println!("Moving right"),
    }
}

pub fn run() {
    let player1 = Movement::Up;
    let player2 = Movement::Down;
    let player3 = Movement::Left;
    let player4 = Movement::Right;

    move_player(player1);
    move_player(player2);
    move_player(player3);
    move_player(player4);
}
