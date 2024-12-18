use std::collections::{HashMap, HashSet};

const LOWEST_SCORE: usize = 91464;

fn main() {
    let (part1, part2) = solve_maze();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn input() -> Vec<Vec<Cell>> {
    let input = include_str!("input.txt");
    //println!("{input}");
    input.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(line.chars().map(Cell::from).collect::<Vec<_>>());
        acc
    })
}

fn solve_maze() -> (usize, usize) {
    let maze = input();
    let solver = Solver::new(&maze);
    let start_position = Position::new(maze.len() - 2, 1);
    let start_direction = Direction::E;
    solver.solve(start_position, start_direction)
}

#[derive(Default)]
struct Solver<'a> {
    maze: &'a [Vec<Cell>],
    solutions: Vec<usize>,
    solution: Option<usize>,
    visited: HashMap<PosDir, usize>,
    lowest_solution_tiles: HashSet<Position>,
}

impl<'a> Solver<'a> {
    fn new(maze: &'a [Vec<Cell>]) -> Self {
        Solver {
            maze,
            ..Default::default()
        }
    }

    fn solve(mut self, start_position: Position, start_direction: Direction) -> (usize, usize) {
        self.follow_path(start_position, start_direction, 0, HashSet::new());
        self.solutions.sort();
        (self.solutions[0], self.lowest_solution_tiles.len())
    }

    fn follow_path(
        &mut self,
        position: Position,
        direction: Direction,
        score: usize,
        mut visited_tiles: HashSet<Position>,
    ) {
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
                < score
            {
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
                        // Store each tile visited.
                        for tile in position.tile_span(corner.position) {
                            visited_tiles.insert(tile);
                        }

                        let running_score =
                            score + 1000 + position.distance_from(corner.position) + 1;
                        self.follow_path(
                            corner.position.step_direction(direction),
                            direction,
                            running_score,
                            visited_tiles.clone(),
                        )
                    }
                }
            }
            Path::DeadEnd => (),
            Path::Solution(solution) => {
                let final_score = score + position.distance_from(solution);

                // Keep track of tiles used in lowest score solutions from part 1.
                if final_score == LOWEST_SCORE {
                    for tile in position.tile_span(solution) {
                        visited_tiles.insert(tile);
                    }
                    let union = self
                        .lowest_solution_tiles
                        .union(&visited_tiles)
                        .copied()
                        .collect::<HashSet<_>>();
                    self.lowest_solution_tiles = union;
                }

                if let Some(current) = self.solution {
                    if final_score < current {
                        self.solution = Some(final_score);
                    }
                }
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

    fn tile_span(&self, other: Self) -> Vec<Position> {
        let mut result = vec![];

        if *self == other {
            return vec![*self];
        } else if self.row == other.row {
            let range = if self.column > other.column {
                other.column..=self.column
            } else {
                self.column..=other.column
            };
            for i in range {
                result.push(Position {
                    row: self.row,
                    column: i,
                });
            }
        } else if self.column == other.column {
            let range = if self.row > other.row {
                other.row..=self.row
            } else {
                self.row..=other.row
            };
            for i in range {
                result.push(Position {
                    row: i,
                    column: self.column,
                });
            }
        }

        result
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
