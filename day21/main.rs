#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let mut numbers = Vec::new();
    let codes = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut code = Vec::new();
            numbers.push(line[..3].parse::<usize>().unwrap());
            for key in line.chars() {
                code.push(position_from_numeric(key));
            }
            code
        })
        .collect::<Vec<_>>();

    let part1 = run(&codes, &numbers, 2);
    let part2 = run(&codes, &numbers, 25);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn run(codes: &[Vec<(usize, usize)>], numbers: &[usize], levels: usize) -> usize {
    let mut cache: Cache = Cache::default();
    codes
        .iter()
        .zip(numbers)
        .map(|(code, number)| {
            /*
            println!(
                "__{}",
                code.iter()
                    .map(|x| numeric_from_position(*x))
                    .collect::<String>()
            );
            */
            let mut from = position_from_numeric('A');
            code.iter()
                .map(|to| {
                    let result = shortest_numeric(from, *to, levels, &mut cache);
                    from = *to;
                    result
                })
                //.map(|x| x.len())
                .sum::<usize>()
                * number
        })
        .sum()
}

/// Find shortest path between keys on numeric keyboard.  Assume corner paths will always
/// be shorter than zig-zag through the center (?).
fn shortest_numeric(
    from: (usize, usize),
    to: (usize, usize),
    levels: usize,
    cache: &mut Cache,
) -> usize {
    /*
    println!(
        "{} to {}",
        numeric_from_position(from),
        numeric_from_position(to)
    );
    */
    let mut paths = Vec::new();
    if from == to {
        // Same button, press it again.
        //return vec![position_from_directional('A')];
        return 1;
    } else if from.0 != to.0 && from.1 != to.1 {
        // Path has a corner.
        let corners = [(from.0, to.1), (to.0, from.1)];
        for corner in corners {
            if corner != (3, 0) {
                let mut path = Vec::new();
                path.append(&mut straight_directional_line(from, corner));
                path.append(&mut straight_directional_line(corner, to));
                path.push(position_from_directional('A'));
                paths.push(path);
            }
        }
    } else {
        // Straight path.
        let mut path = straight_directional_line(from, to);
        path.push(position_from_directional('A'));
        paths.push(path);
    }

    let mut result = 0;
    for path in paths {
        let shortest_directional = shortest_directional(path, levels, cache);
        if result == 0 || shortest_directional < result {
            result = shortest_directional;
        }
    }

    result
}

type Cache = HashMap<CacheKey, usize>;

#[derive(Default, Hash, Eq, PartialEq)]
struct CacheKey {
    level: usize,
    path: Vec<(usize, usize)>,
}

fn shortest_directional(path: Vec<(usize, usize)>, level: usize, cache: &mut Cache) -> usize {
    if level == 0 {
        return path.len();
    }

    if let Some(count) = cache.get(&CacheKey {
        level,
        path: path.clone(),
    }) {
        return *count;
    }

    let mut result = 0;

    let mut from = position_from_directional('A');
    for to in &path {
        let mut result_paths = Vec::new();
        /*
        println!(
            "    {level:width$} shortest_directional from:{from:?} to:{to:?}, {} to {}",
            directional_from_position(from),
            directional_from_position(*to),
            width = level
        );
        */
        if from == *to {
            result_paths.push(vec![position_from_directional('A')]);
        } else if from.0 != to.0 && from.1 != to.1 {
            let corners = [(from.0, to.1), (to.0, from.1)];
            for corner in corners {
                if corner != (0, 0) {
                    let mut path = Vec::new();
                    path.append(&mut straight_directional_line(from, corner));
                    path.append(&mut straight_directional_line(corner, *to));
                    path.push(position_from_directional('A'));
                    result_paths.push(path);
                }
            }
        } else {
            let mut path = straight_directional_line(from, *to);
            path.push(position_from_directional('A'));
            result_paths.push(path);
        }
        from = *to;

        let result_paths_lens: Vec<usize> = result_paths
            .into_iter()
            .map(|x| shortest_directional(x, level - 1, cache))
            .collect();

        let mut smallest = 0;
        for count in result_paths_lens {
            if smallest == 0 || count < smallest {
                smallest = count;
            }
        }
        result += smallest;
        //result.append(&mut smallest);
    }

    cache.insert(
        CacheKey {
            level,
            path: path.clone(),
        },
        result,
    );

    result
}

fn straight_directional_line(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    assert!(from.0 == to.0 || from.1 == to.1);
    let mut result = Vec::new();
    if from.0 != to.0 {
        for _ in 0..from.0.abs_diff(to.0) {
            result.push(position_from_directional(if from.0 < to.0 {
                'v'
            } else {
                '^'
            }));
        }
    } else if from.1 != to.1 {
        for _ in 0..from.1.abs_diff(to.1) {
            result.push(position_from_directional(if from.1 < to.1 {
                '>'
            } else {
                '<'
            }));
        }
    } else {
        unreachable!()
    }
    result
}

fn position_from_numeric(key: char) -> (usize, usize) {
    match key {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}

fn position_from_directional(key: char) -> (usize, usize) {
    match key {
        '^' => (0, 1),
        'v' => (1, 1),
        '>' => (1, 2),
        '<' => (1, 0),
        'A' => (0, 2),
        _ => unreachable!(),
    }
}

fn numeric_from_position(position: (usize, usize)) -> char {
    match position {
        (0, 0) => '7',
        (0, 1) => '8',
        (0, 2) => '9',
        (1, 0) => '4',
        (1, 1) => '5',
        (1, 2) => '6',
        (2, 0) => '1',
        (2, 1) => '2',
        (2, 2) => '3',
        (3, 1) => '0',
        (3, 2) => 'A',
        _ => unreachable!(),
    }
}

fn directionals_from_positions(positions: Vec<(usize, usize)>) -> String {
    let mut result = String::new();
    for position in positions {
        result.push(directional_from_position(position));
    }
    result
}

fn directional_from_position(position: (usize, usize)) -> char {
    match position {
        (0, 1) => '^',
        (1, 1) => 'v',
        (1, 2) => '>',
        (1, 0) => '<',
        (0, 2) => 'A',
        _ => '?',
    }
}
