use std::io;
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

fn get_display(player: Option<Player>) -> String {
    match player {
        Some(player) => player.to_string(),
        None => String::from(" "),
    }
}

#[derive(Copy, Clone)]
struct Board {
    grid: [[Option<Player>; 3]; 3],
    turn: Player,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} | {} | {} \n {} | {} | {} \n {} | {} | {}", 
          get_display(self.grid[0][0]), get_display(self.grid[0][1]), get_display(self.grid[0][2]),
          get_display(self.grid[1][0]), get_display(self.grid[1][1]), get_display(self.grid[1][2]),
          get_display(self.grid[2][0]), get_display(self.grid[2][1]), get_display(self.grid[2][2]))
    }
}

impl Board {
    fn check(&self) -> Option<Player> {
        let grid = self.grid;

        if grid[0][0] == grid[0][1] && grid[0][1] == grid[0][2] {
            return grid[0][0];
        } else if grid[1][0] == grid[1][1] && grid[1][1] == grid[1][2] {
            return grid[1][0];
        } else if grid[2][0] == grid[2][1] && grid[2][1] == grid[2][2] {
            return grid[2][0];
        } else if grid[0][0] == grid[1][0] && grid[1][0] == grid[2][0] {
            return grid[0][0];
        } else if grid[0][1] == grid[1][1] && grid[1][1] == grid[2][1] {
            return grid[0][1];
        } else if grid[0][2] == grid[1][2] && grid[1][2] == grid[2][2] {
            return grid[0][2];
        } else if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
            return grid[0][0];
        } else if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] {
            return grid[0][2];
        } else {
            return None;
        }
    }
}

fn main() {

    let mut board = Board {
        grid: [
                [None, None, None],
                [None, None, None], 
                [None, None, None],
        ],
        turn: Player::X,
    };

    println!("Tic Tac Toe");

    loop {
        println!("{}", board.to_string());

        println!("It is {}'s turn.", board.turn);

        println!("Enter your x position (1, 2 or 3):");
        let mut x_string = String::new();
        io::stdin()
            .read_line(&mut x_string)
            .expect("Failed to read line");
        
        let x: usize = match x_string.trim().parse() {
            Ok(ok) => {
                if ok < 1 || ok > 3 {
                    println!("Please enter 1, 2 or 3!");
                    continue;
                }
                ok
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("Enter your y position (1, 2 or 3):");
        let mut y_string = String::new();
        io::stdin()
            .read_line(&mut y_string)
            .expect("Failed to read line");
        let y: usize = match y_string.trim().parse() {
            Ok(ok) => {
                if ok < 1 || ok > 3 {
                    println!("Please enter 1, 2 or 3!");
                    continue;
                }
                ok
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        
        if board.grid[y - 1][x - 1] != None {
            println!("That space is occupied already!");
        } else {
            board.grid[y - 1][x - 1] = Some(board.turn);
            if board.turn == Player::X {
                board.turn = Player::O;
            } else {
                board.turn = Player::X;
            }
        }

        let winner = board.check();
        if winner != None {
            println!("Player {} has won!", get_display(winner));
            break;
        }
    }
}
