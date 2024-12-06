use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
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
        .run()
}

#[derive(Default, Debug)]
struct Maze {
    rows: Vec<Vec<Object>>,
    guard: Guard,
}

impl Maze {
    fn run(&mut self) -> usize {
        let mut visited: HashSet<Position> = HashSet::from([self.guard.position]);
        while let Some(position) = self.next_position() {
            visited.insert(position);
        }
        visited.len()
    }

    fn next_position(&mut self) -> Option<Position> {
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
                return Some(new_position);
            } else {
                self.guard.direction = self.guard.direction.turn_right();
            }
        }
    }
}

#[derive(Debug, Default)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug, Default)]
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
