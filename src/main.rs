use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::X => write!(f, "X "),
            Cell::O => write!(f, "O "),
            _ => write!(f, "_ "),
        }
    }
}

struct Field([[Cell; 3]; 3]);

#[derive(Debug)]
struct Coordinates {
    row: usize,
    column: usize,
}

impl FromStr for Coordinates {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbs: Vec<usize> = s
            .split_terminator(',')
            .map(|a| a.trim().parse::<usize>())
            .filter_map(|a| a.ok())
            .filter(|&a| a < 3)
            .collect();
        if numbs.len() >= 2 {
            Ok(Coordinates {
                row: numbs[0],
                column: numbs[1],
            })
        } else {
            Err("Некорректный ввод")
        }
    }
}

impl Field {
    fn new() -> Self {
        Field([
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
        ])
    }

    fn set(&mut self, coord: Coordinates, val: Cell) -> Result<State, &'static str> {
        match self.0[coord.row][coord.column] {
            Cell::Empty => {
                self.0[coord.row][coord.column] = val;
                Ok(self.check_win_draw(coord, val))
            }
            _ => Err("Ячейка уже занята"),
        }
    }

    fn check_win_draw(&self, coord: Coordinates, cell: Cell) -> State {
        let field_is_full = || {
            for i in self.0 {
                for j in i {
                    if let Cell::Empty = j {
                        return false;
                    }
                }
            }
            true
        };

        let check_row = || {
            for i in self.0[coord.row] {
                if i != cell {
                    return false;
                }
            }
            true
        };

        let check_column = || {
            for i in 0..3 {
                if self.0[i][coord.column] != cell {
                    return false;
                }
            }
            true
        };

        let check_dio = || match coord {
            Coordinates { row: 0, column: 0 }
            | Coordinates { row: 1, column: 1 }
            | Coordinates { row: 2, column: 2 } => {
                self.0[0][0] == cell && self.0[1][1] == cell && self.0[2][2] == cell
            }
            Coordinates { row: 0, column: 2 }
            | Coordinates { row: 1, column: 1 }
            | Coordinates { row: 2, column: 0 } => {
                self.0[0][2] == cell && self.0[1][1] == cell && self.0[2][0] == cell
            }
            _ => false,
        };

        if check_row() {
            return State::Win(cell);
        } else if check_column() {
            return State::Win(cell);
        } else if check_dio() {
            return State::Win(cell);
        } else if field_is_full() {
            return State::Draw;
        }
        State::InGame
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.0.iter() {
            for j in i {write!(f, "{} ", j)?;}
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(PartialEq)]
enum State {
    InGame,
    Win(Cell),
    Draw,
}

struct Game {
    field: Field,
    state: State,
}

impl Game {
    fn new() -> Self {
        Game {
            field: Field::new(),
            state: State::InGame,
        }
    }

    fn input(&mut self, cell: Cell) {
        let mut input = String::new();
        loop {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            match Coordinates::from_str(&input) {
                Ok(coord) => match self.field.set(coord, cell) {
                    Ok(state) => {self.state = state; break},
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            }
        }
    }

    fn game_loop(&mut self) {
        println!("Координаты вводятся через запятую 0,0");
        loop {
            println!("Ввод крестиков");
            println!("{}", &self.field);
            self.input(Cell::X);
            if self.state != State::InGame {
                break;
            }

            println!("Ввод ноликов");
            println!("{}", &self.field);
            self.input(Cell::O);
            if self.state != State::InGame {
                break;
            }
        }

        println!("{}", &self.field);
        match self.state {
            State::Draw => println!("Ничья"),
            State::Win(cell) => println!("Победа {}", cell),
            _ => panic!("Неизвестная ошибка"),
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.game_loop();
}