use std::collections::{HashMap, HashSet};

fn main() {
    //let (part1, part2) = part1();
    println!("Part 1: {}", part1());
    //println!("Part 2: {}", part2);
}

fn part1() -> usize {
    let trail_map: Vec<Vec<u32>> = include_str!("input.txt").lines().enumerate().fold(
        Default::default(),
        |mut acc, (i, row)| {
            acc.push(row.chars().map(|c| c.to_digit(10).unwrap()).collect());
            acc
        },
    );

    let mut total_score = 0;
    let mut trail_cache: HashMap<Position, HashSet<Position>> = Default::default();
    //let mut total_rating = 0;

    for row in 0..trail_map.len() {
        for column in 0..trail_map[0].len() {
            //if row == 5 && column == 5 && trail_map[row][column] == 0 {
            if trail_map[row][column] == 0 {
                let score =
                    trailhead_peaks(&trail_map, Position { row, column }, &mut trail_cache);
                //println!("trailhead ({row_index}, {column_index}) = {}", score.len());
                //println!("{trail_cache:?}");
                total_score += score.len();
                //total_rating += rating;
            }
        }
    }

    total_score
}

fn trailhead_peaks(
    trail_map: &[Vec<u32>],
    position: Position,
    _cache: &mut HashMap<Position, HashSet<Position>>,
) -> HashSet<Position> {
    //println!("START: {position:?}");
    //let mut score = 0;
    let peaks: Vec<HashSet<Position>> = adjacent(trail_map, position)
        .iter()
        .map(|(adjacent_position, height)| {
            if *height == trail_map[position.row][position.column] + 1 {
                let peaks = if *height == 9 {
                    //println!("___TOP!!!!!: {adjacent_position:?}");
                    //score += 1;
                    HashSet::from([*adjacent_position])
                } else {
                    //println!("adjacent: {adjacent_position:?}");
                    trailhead_peaks(trail_map, *adjacent_position, _cache)
                };
                //cache.insert(position, peaks.clone());
                #[allow(clippy::let_and_return)]
                peaks
            } else {
                //println!("dead end: {adjacent_position:?}");
                HashSet::from([])
            }
        })
        .collect();

    // println!("--------{peaks:?}");
    let mut result = HashSet::new();
    //let mut score = 0;
    for set in peaks {
        for i in set {
            //println!("{i:?}, {added_score}");
            result.insert(i);
            //score += added_score;
            //if added_score == 1 {
            //    score += 1;
            //    //score = 1;
            //}
        }
    }
    //println!("___score: {score}");
    //(result, score)
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
