fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let sum = 0;
    let (mut map, moves, mut robot) = input();

    print_map(&map);
    //println!("{moves:?}");
    println!("{robot:?}");

    /*
    println!("direction N");
    for c in CellsInDirection::new(robot, Direction::N, &map) {
        print!("{c:?} ");
    }
    println!();
    println!("direction E");
    for c in CellsInDirection::new(robot, Direction::E, &map) {
        print!("{c:?} ");
    }
    println!();
    println!("direction S");
    for c in CellsInDirection::new(robot, Direction::S, &map) {
        print!("{c:?} ");
    }
    println!();
    println!("direction W");
    for c in CellsInDirection::new(robot, Direction::W, &map) {
        print!("{c:?} ");
    }
    println!();
    */

    for direction in moves {
        //println!("__________Trying: {direction:?}");
        if let Some(moved_to) = make_move(&mut map, robot, direction) {
            robot = moved_to;
        }
        //print_map(&map);
        /*
        if let Some((i, Cell::Blank)) = CellsInDirection::new(robot, *direction, &map)
            .enumerate()
            .find(|(_, x)| *x == Cell::Blank || *x == Cell::Wall)
        {
            println!("   Found i: {i}");
            robot = shift(&mut map, robot, *direction, i);
            println!("  ***  Robot now at {:?}", robot);
        }
        */
    }

    print_map(&map);

    score(&map)
}

fn score(map: &[Vec<Cell>]) -> usize {
    let mut result = 0;
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column] == Cell::Box {
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
    //println!(".. make_move start={start:?}, direction={direction:?}");
    let next_position = match direction {
        Direction::N => (start.0 - 1, start.1),
        Direction::E => (start.0, start.1 + 1),
        Direction::S => (start.0 + 1, start.1),
        Direction::W => (start.0, start.1 - 1),
    };
    //println!(
    //    "next_position={next_position:?} is {:?}",
    //    map[next_position.0][next_position.1]
    //);
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
    } else {
        return None;
    }
    Some(next_position)
}

/*
fn shift(
    map: &mut [Vec<Cell>],
    robot: (usize, usize),
    direction: Direction,
    blank: usize,
) -> (usize, usize) {
    println!("---- robot: {robot:?}, blank: {blank:?}");
    let mut position = match direction {
        Direction::N => (robot.0 - blank, robot.1),
        Direction::E => (robot.0, robot.1 + blank),
        Direction::S => (robot.0 + blank, robot.1),
        Direction::W => (robot.0, robot.1 - blank),
    };
    println!("---- position: {position:?}");
    let mut new_robot = robot;
    while position != robot {
        let new_position = match direction {
            Direction::N => (position.0 + 1, position.1),
            Direction::E => (position.0, position.1 - 1),
            Direction::S => (position.0 - 1, position.1),
            Direction::W => (position.0, position.1 + 1),
        };
        println!("---- new_position: {new_position:?}");
        map[position.0][position.1] = map[new_position.0][new_position.1];
        new_robot = position;
        position = new_position;
    }
    println!("---- new_robot: {new_robot:?}");
    map[position.0][position.1] = Cell::Blank;
    new_robot
}
*/

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
                }
            );
        }
        println!()
    }
}

/*
struct CellsInDirection<'a> {
    direction: Direction,
    map: &'a [Vec<Cell>],
    position: (usize, usize),
}

impl<'a> CellsInDirection<'a> {
    fn new(position: (usize, usize), direction: Direction, map: &'a [Vec<Cell>]) -> Self {
        assert_eq!(map[position.0][position.1], Cell::Robot);
        CellsInDirection {
            direction,
            map,
            position,
        }
    }
}

impl<'a> Iterator for CellsInDirection<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let (r, c) = self.position;
        if r == 0 || c == 0 || r == self.map.len() - 1 || c == self.map[0].len() - 1 {
            return None;
        }
        let p = match self.direction {
            Direction::N => (r - 1, c),
            Direction::E => (r, c + 1),
            Direction::S => (r + 1, c),
            Direction::W => (r, c - 1),
        };
        self.position = p;
        Some(self.map[p.0][p.1])
    }
}
*/

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

#[derive(Debug, Clone, Copy)]
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
