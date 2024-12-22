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
    let steps = find_steps(start, end, &track);
    let small_cheats = find_cheats(&steps, &track, CheatMode::Small);
    let big_cheats = find_cheats(&steps, &track, CheatMode::Big);
    fn count_cheats(cheats: &HashMap<usize, Vec<Cheat>>) -> usize {
        cheats.iter().fold(0, |mut acc, (key, val)| {
            if *key >= 100 {
                acc += val.len();
            }
            acc
        })
    }
    println!("Part 1: {}", count_cheats(&small_cheats));
    println!("Part 2: {}", count_cheats(&big_cheats));
}

enum CheatMode {
    Small,
    Big,
}

fn find_cheats(
    steps: &HashMap<Position, usize>,
    track: &Track,
    mode: CheatMode,
) -> HashMap<usize, Vec<Cheat>> {
    let mut result = HashMap::new();
    for (position, count) in steps {
        for (target, distance) in match mode {
            CheatMode::Small => neighboring_cheats(*position, track),
            CheatMode::Big => neighboring_big_cheats(*position, track),
        } {
            if steps.contains_key(&target) {
                let new_count = steps.get(&target).unwrap();
                if new_count > count {
                    let saved = new_count - count - distance;
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

fn neighboring_big_cheats(
    Position { row, column }: Position,
    track: &Track,
) -> HashMap<Position, usize> {
    let mut result = HashMap::new();
    let (row, column) = (row as isize, column as isize);
    for i in -20..=20 {
        let mid_column = column + i;
        if mid_column > 0 && mid_column < track[0].len() as isize - 1 {
            if track[row as usize][mid_column as usize] == Cell::Space {
                result.insert(
                    Position {
                        row: row as usize,
                        column: mid_column as usize,
                    },
                    i.unsigned_abs(),
                );
            }
            for j in -(20 - i.abs())..=20 - i.abs() {
                let mid_row = row + j;
                #[allow(clippy::collapsible_if)]
                if mid_row > 0 && mid_row < track.len() as isize - 1 {
                    if track[mid_row as usize][mid_column as usize] == Cell::Space {
                        result.insert(
                            Position {
                                row: mid_row as usize,
                                column: mid_column as usize,
                            },
                            i.unsigned_abs() + j.unsigned_abs(),
                        );
                    }
                }
            }
        }
    }
    result
}

fn neighboring_cheats(
    Position { row, column }: Position,
    track: &Track,
) -> HashMap<Position, usize> {
    let mut result = HashMap::new();
    if row > 2 && track[row - 1][column] == Cell::Wall && track[row - 2][column] == Cell::Space {
        result.insert(
            Position {
                row: row - 2,
                column,
            },
            2,
        );
    }
    if row < track.len() - 2
        && track[row + 1][column] == Cell::Wall
        && track[row + 2][column] == Cell::Space
    {
        result.insert(
            Position {
                row: row + 2,
                column,
            },
            2,
        );
    }

    if column > 2 && track[row][column - 1] == Cell::Wall && track[row][column - 2] == Cell::Space {
        result.insert(
            Position {
                row,
                column: column - 2,
            },
            2,
        );
    }
    if column < track.len() - 2
        && track[row][column + 1] == Cell::Wall
        && track[row][column + 2] == Cell::Space
    {
        result.insert(
            Position {
                row,
                column: column + 2,
            },
            2,
        );
    }
    result
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Cheat {
    start: Position,
    end: Position,
    saved: usize,
}

fn find_steps(start: Position, end: Position, track: &Track) -> HashMap<Position, usize> {
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
