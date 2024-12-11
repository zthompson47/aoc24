use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let trail_map: Vec<Vec<u32>> =
        include_str!("input.txt")
            .lines()
            .fold(Vec::new(), |mut acc, row| {
                acc.push(row.chars().map(|c| c.to_digit(10).unwrap()).collect());
                acc
            });
    let mut total_score = 0;
    for row in 0..trail_map.len() {
        for column in 0..trail_map[0].len() {
            if trail_map[row][column] == 0 {
                let score = trailhead_peaks(&trail_map, Position { row, column });
                total_score += score.len();
            }
        }
    }
    total_score
}

fn trailhead_peaks(
    trail_map: &[Vec<u32>],
    position: Position,
    //_cache: &mut HashMap<Position, HashSet<Position>>,
) -> HashSet<Position> {
    let peaks: Vec<HashSet<Position>> = adjacent(trail_map, position)
        .iter()
        .map(|(adjacent_position, height)| {
            if *height == trail_map[position.row][position.column] + 1 {
                if *height == 9 {
                    HashSet::from([*adjacent_position])
                } else {
                    trailhead_peaks(trail_map, *adjacent_position)
                }
            } else {
                HashSet::from([])
            }
        })
        .collect();

    let mut result = HashSet::new();
    for set in peaks {
        for i in set {
            result.insert(i);
        }
    }
    result
}

fn adjacent(trail_map: &[Vec<u32>], position: Position) -> Vec<(Position, u32)> {
    let mut result = Vec::new();
    if position.row > 0 {
        let r = position.row - 1;
        let c = position.column;
        result.push((Position { row: r, column: c }, trail_map[r][c]));
    }
    if position.row < trail_map.len() - 1 {
        let r = position.row + 1;
        let c = position.column;
        result.push((Position { row: r, column: c }, trail_map[r][c]));
    }
    if position.column > 0 {
        let r = position.row;
        let c = position.column - 1;
        result.push((Position { row: r, column: c }, trail_map[r][c]));
    }
    if position.column < trail_map[0].len() - 1 {
        let r = position.row;
        let c = position.column + 1;
        result.push((Position { row: r, column: c }, trail_map[r][c]));
    }
    result
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    row: usize,
    column: usize,
}
