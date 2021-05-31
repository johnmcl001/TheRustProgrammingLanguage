use std::io;

fn main() {
    let mut board = [[0; 3]; 3];
    display_board(&board);
    let (x, y) = get_player_input();
    board[x][y] = 4;
    display_board(&board);
}

fn display_board(board: &[[u32; 3]; 3]) {
    for row in board {
        println!("{:?}", row);
    }
}

fn get_player_input() -> (usize, usize) {
    let mut x_y: (usize, usize) = (0, 0);
    let mut input = String::new();
    println!("Enter X:");
    std::io::stdin().read_line(&mut input).expect("Cannot read line");
    x_y.0 = input.trim().parse().expect("error");
    input.clear();
    println!("Enter Y:");
    std::io::stdin().read_line(&mut input).expect("Cannot read line");
    x_y.1 = input.trim().parse().expect("error");
    input.clear();
    x_y
}
