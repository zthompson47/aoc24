use std::collections::HashMap;

const X: usize = 71;
const Y: usize = 71;
const BYTES: usize = 1024;
//const BYTES: usize = 0;

//const X: usize = 7;
//const Y: usize = 7;
//const BYTES: usize = 12;

fn main() {
    let mut space = [[Location::Safe; Y]; X];
    let bytes = include_str!("input.txt")
        //let bytes = include_str!("test.txt")
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect::<Vec<_>>();
    for i in 0..BYTES {
        space[bytes[i].0][bytes[i].1] = Location::Corrupted;
    }

    /*
    // Fill in pockets of empty space.
    fn is_empty_pocket(space: &[[Location; Y]; X], (x, y): (usize, usize)) -> bool {
        if x > 0 && y > 0 && x < X - 1 && y < Y - 1 {
            #[allow(clippy::needless_range_loop)]
            for ix in x - 1..x + 2 {
                for iy in y - 1..y + 2 {
                    if space[ix][iy] == Location::Corrupted {
                        return false;
                    }
                }
            }
            return true;
        }
        false
    }
    let mut space2 = space;
    #[allow(clippy::needless_range_loop)]
    for x in 1..space.len() - 1 {
        for y in 1..space[0].len() - 1 {
            if is_empty_pocket(&space, (x, y)) {
                space2[x][y] = Location::Corrupted;
            }
        }
    }
    space = space2;
    */

    // Convert to maze format from day 16.
    let mut space_as_maze = [[Location::Safe; Y + 2]; X + 2];
    space_as_maze[0] = [Location::Corrupted; Y + 2];
    space_as_maze[X + 1] = [Location::Corrupted; Y + 2];
    for (x, mut sx) in space_as_maze.iter_mut().skip(1).take(X).zip(space) {
        x[0] = Location::Corrupted;
        x[Y + 1] = Location::Corrupted;
        sx.reverse();
        for (i, y) in sx.iter_mut().enumerate() {
            x[i + 1] = *y;
        }
    }
    space_as_maze[X][1] = Location::End;
    let mut maze = String::new();
    for y in 0..space_as_maze[0].len() {
        for column in space_as_maze {
            let _ = std::fmt::write(
                &mut maze,
                format_args!(
                    "{}",
                    match column[y] {
                        Location::Corrupted => '#',
                        Location::Safe => '.',
                        Location::End => 'E',
                    }
                ),
            );
        }
        let _ = std::fmt::write(&mut maze, format_args!("\n"));
    }
    println!("{maze}");
    let part1 = solve_maze(&maze);
    println!("Part 1: {}", part1);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Location {
    Safe,
    Corrupted,
    End,
}

fn solve_maze(maze: &str) -> usize {
    let maze = input(maze);
    let solver = Solver::new(&maze);
    let start_position = Position::new(maze.len() - 2, 1);
    let start_direction = Direction::E;
    solver.solve(start_position, start_direction)
}

fn input(input: &str) -> Vec<Vec<Cell>> {
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().map(Cell::from).collect::<Vec<_>>());
        acc
    })
}

#[derive(Default)]
struct Solver<'a> {
    maze: &'a [Vec<Cell>],
    solutions: Vec<usize>,
    solution: Option<usize>,
    visited: HashMap<PosDir, usize>,
}

impl<'a> Solver<'a> {
    fn new(maze: &'a [Vec<Cell>]) -> Self {
        Solver {
            maze,
            ..Default::default()
        }
    }

    fn solve(mut self, start_position: Position, start_direction: Direction) -> usize {
        self.follow_path(start_position, start_direction, 0);
        self.print();
        self.solutions.sort();
        self.solutions[0]
    }

    fn print(&self) {
        let visited = self.visited.keys().map(|x| x.position).collect::<Vec<_>>();
        for (row_i, row) in self.maze.iter().enumerate() {
            for (col_i, cell) in row.iter().enumerate() {
                print!(
                    "{}",
                    match cell {
                        Cell::End => 'E',
                        Cell::Wall => '#',
                        Cell::Space => {
                            if visited.contains(&Position {
                                row: row_i,
                                column: col_i,
                            }) {
                                'O'
                            } else {
                                '.'
                            }
                        }
                    }
                );
            }
            println!();
        }
    }

    fn follow_path(&mut self, position: Position, direction: Direction, score: usize) {
        if let Some(current) = self.solution {
            if score > current {
                return;
            }
        }
        if self.visited.contains_key(&PosDir {
            position,
            direction,
        }) {
            if *self
                .visited
                .get(&PosDir {
                    position,
                    direction,
                })
                .unwrap()
                <= score
            {
                // Ignore any path less OR equal score - we don't need to keep track
                // of all tiles visited (unlike day 16).
                return;
            } else {
                self.visited.insert(
                    PosDir {
                        position,
                        direction,
                    },
                    score,
                );
            }
        } else {
            self.visited.insert(
                PosDir {
                    position,
                    direction,
                },
                score,
            );
        }

        match Path::new(position, direction, self.maze) {
            Path::Corners(corners) => {
                for corner in corners {
                    for direction in corner.directions {
                        let running_score = score + position.distance_from(corner.position) + 1;
                        self.follow_path(
                            corner.position.step_direction(direction),
                            direction,
                            running_score,
                        )
                    }
                }
            }
            Path::DeadEnd => (),
            Path::Solution(solution) => {
                let final_score = score + position.distance_from(solution);
                if let Some(current) = self.solution {
                    if final_score < current {
                        self.solution = Some(final_score);
                    }
                }
                //println!("SOLUTION: {final_score}");
                self.solutions.push(final_score);
            }
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn distance_from(&self, other: Self) -> usize {
        self.row.abs_diff(other.row) + self.column.abs_diff(other.column)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct PosDir {
    position: Position,
    direction: Direction,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Position { row, column }
    }

    fn step_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::N => Position {
                row: self.row - 1,
                column: self.column,
            },
            Direction::E => Position {
                row: self.row,
                column: self.column + 1,
            },
            Direction::S => Position {
                row: self.row + 1,
                column: self.column,
            },
            Direction::W => Position {
                row: self.row,
                column: self.column - 1,
            },
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, column): (usize, usize)) -> Self {
        Position { row, column }
    }
}

#[derive(Debug)]
enum Path {
    Corners(Vec<Corner>),
    DeadEnd,
    Solution(Position),
}

impl Path {
    fn new(mut position: Position, direction: Direction, maze: &[Vec<Cell>]) -> Self {
        let mut corners = Vec::new();
        while maze[position.row][position.column] != Cell::Wall {
            match maze[position.row][position.column] {
                Cell::End => return Path::Solution(position),
                Cell::Space => {
                    if let Some(corner) = Corner::find(maze, position, direction) {
                        corners.push(corner)
                    }
                }
                _ => unreachable!(),
            }
            position = Position::from(match direction {
                Direction::N => (position.row - 1, position.column),
                Direction::E => (position.row, position.column + 1),
                Direction::S => (position.row + 1, position.column),
                Direction::W => (position.row, position.column - 1),
            });
        }
        //println!("{corners:?}");
        if corners.is_empty() {
            Path::DeadEnd
        } else {
            Path::Corners(corners)
        }
    }
}

#[derive(Debug)]
struct Corner {
    position: Position,
    directions: Vec<Direction>,
}

impl Corner {
    fn find(
        maze: &[Vec<Cell>],
        position @ Position { row, column }: Position,
        direction: Direction,
    ) -> Option<Self> {
        let directions = match direction {
            Direction::N | Direction::S => {
                let mut directions = Vec::new();
                if maze[row][column - 1] == Cell::Space {
                    directions.push(Direction::W);
                }
                if maze[row][column + 1] == Cell::Space {
                    directions.push(Direction::E);
                }
                directions
            }
            Direction::E | Direction::W => {
                let mut directions = Vec::new();
                if maze[row - 1][column] == Cell::Space {
                    directions.push(Direction::N);
                }
                if maze[row + 1][column] == Cell::Space {
                    directions.push(Direction::S);
                }
                directions
            }
        };

        if directions.is_empty() {
            None
        } else {
            Some(Corner {
                position,
                directions,
            })
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq)]
enum Cell {
    Wall,
    Space,
    End,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Wall,
            '.' | 'S' => Cell::Space,
            'E' => Cell::End,
            _ => unreachable!(),
        }
    }
}
