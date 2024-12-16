fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let sum = 0;
    let (map, moves) = input();

    println!("{map:?}");
    println!();
    println!("{moves:?}");

    sum
}

fn input() -> (Vec<Vec<Cell>>, Vec<Direction>) {
    let mut on_map = true;
    include_str!("test0.txt")
        .lines()
        .fold((Vec::new(), Vec::new()), |mut acc, line| {
            if line.is_empty() {
                on_map = false;
            } else if on_map {
                acc.0.push(line.chars().map(Cell::from).collect::<Vec<_>>());
            } else {
                acc.1
                    .append(&mut line.chars().map(Direction::from).collect::<Vec<_>>());
            }
            acc
        })
}

#[derive(Debug)]
enum Cell {
    Wall,
    Box,
    Robot,
    Blank,
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

#[derive(Debug)]
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
