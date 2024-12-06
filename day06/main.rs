fn main() {
    println!("Part 1: {}", part1());
}

#[derive(Debug, Default)]
enum Direction {
    #[default]
    N,
    E,
    S,
    W,
}

struct Phase {
    row: usize,
    column: usize,
    direction: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Position {
    Nothing,
    Something,
    Guard,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '.' => Position::Nothing,
            '#' => Position::Something,
            '^' => Position::Guard,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug)]
struct Maze {
    rows: Vec<Vec<Position>>,
    columns: Vec<Vec<Position>>,
    guard: (usize, usize, Direction),
}

fn part1() -> u32 {
    let maze = include_str!("test.txt")
        .lines()
        .map(|line| line.chars().map(Position::from).collect::<Vec<Position>>())
        .enumerate()
        .fold(Maze::default(), |mut maze, (row_index, mut line)| {
            // Look for guard position.
            if let Some(column_index) = line.iter().position(|&x| x == Position::Guard) {
                line[column_index] = Position::Nothing;
                maze.guard.0 = column_index;
                maze.guard.1 = row_index;
            }

            maze.rows.push(line.clone());

            for (column_index, position) in line.iter().enumerate() {
                if let Some(p) = maze.columns.get_mut(column_index) {
                    p.push(*position);
                } else {
                    maze.columns.insert(column_index, vec![*position]);
                }
            }
            maze
        });

    let result = 0;

    println!("{maze:#?}");

    result
}
