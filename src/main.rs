use std::io;
use std::process;

#[derive(Copy, Clone)]
struct Position {
    has_mine: bool,
    known: bool,
}

fn print_board(board: &Vec<Vec<Position>>) {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if !board[x][y].known {
                print!(".")
            }
            else if board[x][y].has_mine{
                print!("!")
            }
            else {
                let neighbors = count_neighbors(board, x, y);
                if neighbors == 0 {
                    print!(" ");
                }
                else {
                    print!("{}", neighbors);
                }
            }
        }
        println!("");
    }
}

fn done(board: &Vec<Vec<Position>>) -> bool {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if !board[x][y].known && !board[x][y].has_mine {
                return false;
            }
        }
    }
    true
}

fn count_neighbors(board: &Vec<Vec<Position>>, x: usize, y: usize) -> i32 {
    let offsets: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut result = 0;
    for &(ox, oy) in offsets.iter() {
        let px = x as i32 + ox;
        let py = y as i32 + oy;
        if px >= 0 && (px as usize) < board[0].len() && py >= 0 && (py as usize) < board.len() && board[px as usize][py as usize].has_mine {
            result += 1;
        }
    }
    result
}

fn main() {
    let mut board = vec![vec![Position{has_mine: false, known: false}; 4]; 4];

    board[2][2].has_mine = true;
    board[3][3].has_mine = true;

    loop {
        print_board(&board);

        println!("<x> <y>");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let split = input.trim().split(" ").collect::<Vec<&str>>();
        let x = split[0].parse::<usize>().unwrap();
        let y = split[1].parse::<usize>().unwrap();
        board[x][y].known = true;
        if board[x][y].has_mine {
            println!("You lose");
            print_board(&board);
            process::exit(1);
        }
        if done(&board) {
            println!("You win");
            print_board(&board);
            process::exit(0);
        }
    }
}
