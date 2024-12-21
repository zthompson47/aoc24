use std::collections::{hash_map::Entry, HashMap};

type Track = Vec<Vec<Cell>>;

fn main() {
    let (mut start, mut end) = (None, None);
    let track: Track = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, c)| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Space,
                    'S' => {
                        start = Some((row, column));
                        Cell::Space
                    }
                    'E' => {
                        end = Some((row, column));
                        Cell::Space
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let (start, end) = (Position::from(start.unwrap()), Position::from(end.unwrap()));
    let steps = count_steps(start, end, &track);
    //println!("{}", steps.len());

    let cheats = find_cheats(&steps, &track);
    //println!("{:#?}", cheats.keys());

    let part1 = cheats.iter().fold(0, |mut acc, (key, val)| {
        if *key >= 100 {
            acc += val.len();
        }
        acc
    });

    println!("Part 1: {part1}");
}

fn find_cheats(steps: &HashMap<Position, usize>, track: &Track) -> HashMap<usize, Vec<Cheat>> {
    let mut result = HashMap::new();
    for (position, count) in steps {
        for target in neighboring_cheats(*position, track) {
            if steps.contains_key(&target) {
                let new_count = steps.get(&target).unwrap();
                if new_count > count {
                    let saved = new_count - count - 2;
                    let cheat = Cheat {
                        start: *position,
                        end: target,
                        saved,
                    };
                    result
                        .entry(saved)
                        .and_modify(|e: &mut Vec<Cheat>| e.push(cheat))
                        .or_insert(vec![cheat]);
                }
            }
        }
    }
    result
}

fn neighboring_cheats(Position { row, column }: Position, track: &Track) -> Vec<Position> {
    let mut result = Vec::new();
    if row > 2 && track[row - 1][column] == Cell::Wall && track[row - 2][column] == Cell::Space {
        result.push(Position {
            row: row - 2,
            column,
        });
    }
    if row < track.len() - 2
        && track[row + 1][column] == Cell::Wall
        && track[row + 2][column] == Cell::Space
    {
        result.push(Position {
            row: row + 2,
            column,
        });
    }

    if column > 2 && track[row][column - 1] == Cell::Wall && track[row][column - 2] == Cell::Space {
        result.push(Position {
            row,
            column: column - 2,
        });
    }
    if column < track.len() - 2
        && track[row][column + 1] == Cell::Wall
        && track[row][column + 2] == Cell::Space
    {
        result.push(Position {
            row,
            column: column + 2,
        });
    }
    result
}

#[derive(Debug, Clone, Copy)]
struct Cheat {
    start: Position,
    end: Position,
    saved: usize,
}

fn count_steps(start: Position, end: Position, track: &Track) -> HashMap<Position, usize> {
    let mut result = HashMap::new();
    result.insert(start, 0);
    let mut space = start;
    let mut i = 1;
    while space != end {
        for next_space in neighboring_space(space, track) {
            if let Entry::Vacant(e) = result.entry(next_space) {
                e.insert(i);
                space = next_space;
                i += 1;
            }
        }
    }
    result
}

fn neighboring_space(Position { row, column }: Position, track: &Track) -> Vec<Position> {
    let mut result = Vec::new();
    if row > 0 && track[row - 1][column] == Cell::Space {
        result.push(Position {
            row: row - 1,
            column,
        });
    }
    if row < track.len() - 1 && track[row + 1][column] == Cell::Space {
        result.push(Position {
            row: row + 1,
            column,
        });
    }

    if column > 0 && track[row][column - 1] == Cell::Space {
        result.push(Position {
            row,
            column: column - 1,
        });
    }
    if column < track[0].len() - 1 && track[row][column + 1] == Cell::Space {
        result.push(Position {
            row,
            column: column + 1,
        });
    }
    result
}

#[derive(Debug, PartialEq)]
enum Cell {
    Space,
    Wall,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    row: usize,
    column: usize,
}

impl From<(usize, usize)> for Position {
    fn from((row, column): (usize, usize)) -> Self {
        Position { row, column }
    }
}

/*
#[derive(Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

impl From<(Position, Position)> for Direction {
    fn from((start, end): (Position, Position)) -> Self {
        if start.row < end.row {
            Direction::S
        } else if start.column < end.column {
            Direction::E
        } else if start.row > end.row {
            Direction::N
        } else if start.column > end.column {
            Direction::W
        } else {
            unreachable!()
        }
    }
}
*/
