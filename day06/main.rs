use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", Maze::new().count_positions());
    println!("Part 2: {}", Maze::new().count_possible_loops());
}

#[derive(Default, Debug, Clone)]
struct Maze {
    rows: Vec<Vec<Object>>,
    guard: Guard,
}

impl Maze {
    fn new() -> Self {
        include_str!("input.txt")
            .lines()
            .map(|line| line.chars().map(Object::from).collect::<Vec<Object>>())
            .enumerate()
            .fold(Maze::default(), |mut maze, (row_index, mut line)| {
                // Look for guard position.
                if let Some(column_index) = line.iter().position(|&x| x == Object::Guard) {
                    line[column_index] = Object::Nothing;
                    maze.guard.position.column = column_index;
                    maze.guard.position.row = row_index;
                }
                maze.rows.push(line);
                maze
            })
    }

    fn count_positions(mut self) -> usize {
        let mut visited: HashSet<Position> = HashSet::from([self.guard.position]);
        while let Some(Guard {
            position,
            direction: _direction,
        }) = self.next_position()
        {
            visited.insert(position);
        }
        visited.len()
    }

    fn count_possible_loops(&self) -> usize {
        self.rows
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                use rayon::prelude::*;
                row.par_iter()
                    .enumerate()
                    .map(|(column_index, position)| {
                        if *position == Object::Nothing {
                            let mut maze = self.clone();
                            let mut visited: HashSet<Guard> = HashSet::from([maze.guard]);
                            maze.rows[row_index][column_index] = Object::Something;
                            while let Some(guard) = maze.next_position() {
                                if !visited.insert(guard) {
                                    return 1;
                                }
                            }
                            0
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn next_position(&mut self) -> Option<Guard> {
        loop {
            let mut r = self.guard.position.row as i32;
            let mut c = self.guard.position.column as i32;
            match self.guard.direction {
                Direction::N => r -= 1,
                Direction::E => c += 1,
                Direction::S => r += 1,
                Direction::W => c -= 1,
            }
            if r < 0 || c < 0 || r >= self.rows.len() as i32 || c >= self.rows[0].len() as i32 {
                return None;
            } else if self.rows[r as usize][c as usize] == Object::Nothing {
                let new_position = Position {
                    row: r as usize,
                    column: c as usize,
                };
                self.guard.position = new_position;
                return Some(Guard {
                    position: new_position,
                    direction: self.guard.direction,
                });
            } else {
                self.guard.direction = self.guard.direction.turn_right();
            }
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    #[default]
    N,
    E,
    S,
    W,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Object {
    Nothing,
    Something,
    Guard,
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '.' => Object::Nothing,
            '#' => Object::Something,
            '^' => Object::Guard,
            _ => unreachable!(),
        }
    }
}
