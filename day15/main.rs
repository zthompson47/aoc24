fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let (mut map, moves, mut robot) = input();
    for direction in moves {
        if let Some(moved_to) = make_move(&mut map, robot, direction) {
            robot = moved_to;
        }
    }
    score(&map)
}

fn part2() -> usize {
    let (mut map, moves, mut robot) = input();
    //print_map(&map);
    //println!("{moves:?}");
    //println!();
    map = double_wide(&map);
    robot = (robot.0, robot.1 * 2);
    //print_map(&map);
    //println!();
    for direction in moves {
        //println!("direction={direction:?} robot: {robot:?}");
        if let Some(moved_to) = make_move(&mut map, robot, direction) {
            //println!("!!!!!!!!!!!GOTIT!!!!!! {moved_to:?}");
            robot = moved_to;
            //print_map(&map);
            //println!();
        }
    }
    print_map(&map);
    //println!();
    score(&map)
}

fn double_wide(map: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    map.iter().fold(Vec::new(), |mut acc, row| {
        let mut new_row = Vec::new();
        for cell in row {
            match cell {
                Cell::Wall => {
                    new_row.push(Cell::Wall);
                    new_row.push(Cell::Wall);
                }
                Cell::Box => {
                    new_row.push(Cell::BoxLeft);
                    new_row.push(Cell::BoxRight);
                }
                Cell::Robot => {
                    new_row.push(Cell::Robot);
                    new_row.push(Cell::Blank);
                }
                Cell::Blank => {
                    new_row.push(Cell::Blank);
                    new_row.push(Cell::Blank);
                }
                Cell::BoxLeft => unreachable!(),
                Cell::BoxRight => unreachable!(),
            }
        }
        acc.push(new_row);
        acc
    })
}

fn score(map: &[Vec<Cell>]) -> usize {
    let mut result = 0;
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column] == Cell::Box || map[row][column] == Cell::BoxLeft {
                result += 100 * row + column;
            }
        }
    }
    result
}

fn make_move(
    map: &mut [Vec<Cell>],
    start: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let next_position = match direction {
        Direction::N => (start.0 - 1, start.1),
        Direction::E => (start.0, start.1 + 1),
        Direction::S => (start.0 + 1, start.1),
        Direction::W => (start.0, start.1 - 1),
    };
    if map[next_position.0][next_position.1] == Cell::Blank {
        map[next_position.0][next_position.1] = map[start.0][start.1];
        map[start.0][start.1] = Cell::Blank;
    } else if map[next_position.0][next_position.1] == Cell::Wall {
        return None;
    } else if map[next_position.0][next_position.1] == Cell::Box {
        if make_move(map, next_position, direction).is_some() {
            map[next_position.0][next_position.1] = map[start.0][start.1];
            map[start.0][start.1] = Cell::Blank;
        } else {
            return None;
        }
    } else if map[next_position.0][next_position.1] == Cell::BoxLeft
        || map[next_position.0][next_position.1] == Cell::BoxRight
    {
        if direction == Direction::N || direction == Direction::S {
            let next_other = match map[next_position.0][next_position.1] {
                Cell::BoxLeft => (next_position.0, next_position.1 + 1),
                Cell::BoxRight => (next_position.0, next_position.1 - 1),
                _ => unreachable!(),
            };
            if can_move_up_down(map, next_position, direction)
                && can_move_up_down(map, next_other, direction)
            {
                make_move(map, next_position, direction);
                make_move(map, next_other, direction);
                map[next_position.0][next_position.1] = map[start.0][start.1];
                map[start.0][start.1] = Cell::Blank;
            } else {
                return None;
            }
        } else if direction == Direction::E || direction == Direction::W {
            if make_move(map, next_position, direction).is_some() {
                map[next_position.0][next_position.1] = map[start.0][start.1];
                map[start.0][start.1] = Cell::Blank;
            } else {
                return None;
            }
        }
    } else {
        return None;
    }
    Some(next_position)
}

fn can_move_up_down(map: &[Vec<Cell>], start: (usize, usize), direction: Direction) -> bool {
    let next_position = match direction {
        Direction::N => (start.0 - 1, start.1),
        Direction::S => (start.0 + 1, start.1),
        _ => unreachable!(),
    };
    match map[next_position.0][next_position.1] {
        Cell::Blank => true,
        Cell::Wall => false,
        Cell::Box => can_move_up_down(map, next_position, direction),
        wide_box => {
            let next_other = match wide_box {
                Cell::BoxLeft => (next_position.0, next_position.1 + 1),
                Cell::BoxRight => (next_position.0, next_position.1 - 1),
                _ => unreachable!(),
            };
            can_move_up_down(map, next_position, direction)
                && can_move_up_down(map, next_other, direction)
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &[Vec<Cell>]) {
    for row in map {
        for column in row {
            print!(
                "{}",
                match column {
                    Cell::Wall => '#',
                    Cell::Box => 'O',
                    Cell::Robot => '@',
                    Cell::Blank => '.',
                    Cell::BoxLeft => '[',
                    Cell::BoxRight => ']',
                }
            );
        }
        println!()
    }
}

fn input() -> (Vec<Vec<Cell>>, Vec<Direction>, (usize, usize)) {
    let mut on_map = true;
    include_str!("input.txt").lines().enumerate().fold(
        (Vec::new(), Vec::new(), (0, 0)),
        |mut acc, (row, line)| {
            if line.is_empty() {
                on_map = false;
            } else if on_map {
                let line = line.chars().map(Cell::from).collect::<Vec<_>>();
                for (column, cell) in line.iter().enumerate() {
                    if *cell == Cell::Robot {
                        acc.2 = (row, column);
                    }
                }
                acc.0.push(line);
            } else {
                acc.1
                    .append(&mut line.chars().map(Direction::from).collect::<Vec<_>>());
            }
            acc
        },
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Wall,
    Box,
    Robot,
    Blank,
    BoxLeft,
    BoxRight,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Cell::Wall,
            '.' => Cell::Blank,
            'O' => Cell::Box,
            '@' => Cell::Robot,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::W,
            '>' => Direction::E,
            '^' => Direction::N,
            'v' => Direction::S,
            _ => unreachable!(),
        }
    }
}
