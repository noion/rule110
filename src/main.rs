const BOARD_SIZE: usize = 100;

fn main() {
    let mask = [0, 1, 1, 1, 0, 1, 1, 0];

    let mut current_board = 1;
    let mut board = [[0; BOARD_SIZE]; 2];
    let sid = BOARD_SIZE - 2;

    board[current_board][sid] = 1;

    for _i in 1..sid {
        for j in 0..BOARD_SIZE {
            print!("{}", " *".chars().nth(board[current_board][j]).unwrap());
        }
        let next_board = 1 - current_board;
        for i in 1..BOARD_SIZE - 1 {
            let a = board[current_board][i - 1];
            let b = board[current_board][i];
            let c = board[current_board][i + 1];
            let pattern = (a << 2) | (b << 1) | (c << 0);
            board[next_board][i] = mask[pattern];
        }
        current_board = next_board;
        println!()
    }
}
