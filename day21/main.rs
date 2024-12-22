fn main() {
    let mut numbers = Vec::new();
    let codes = include_str!("test.txt")
        .lines()
        .map(|line| {
            //let mut code = vec![position_from_numeric('A')];
            let mut code = Vec::new();
            numbers.push(line[..3].parse::<usize>().unwrap());
            for key in line.chars() {
                code.push(position_from_numeric(key));
            }
            code
        })
        .collect::<Vec<_>>();

    let part1: usize = codes
        .iter()
        .zip(numbers)
        .map(|(code, number)| {
            println!(
                "___________________________________________________\n{} {code:?}",
                code.iter()
                    .map(|x| numeric_from_position(*x))
                    .collect::<String>()
            );
            let mut from = position_from_numeric('A');
            code.iter()
                .map(|to| {
                    let result = shortest_numeric(from, *to);
                    from = *to;
                    result
                })
                .map(|x| x.len())
                .sum::<usize>()
                * number
        })
        .sum();

    println!("Part 1: {part1}");
}

/// Find shortest path between keys on numeric keyboard.  Assume corner paths will always
/// be shorter than zig-zag through the center (?).
fn shortest_numeric(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    println!(
        "shortest_numeric from:{from:?} to:{to:?}, {} to {}",
        numeric_from_position(from),
        numeric_from_position(to)
    );
    let mut paths = Vec::new();
    if from == to {
        // Same button, press it again.
        return vec![position_from_numeric('A')];
    } else if from.0 != to.0 && from.1 != to.1 {
        // Path has a corner.
        let corners = [(from.0, to.1), (to.0, from.1)];
        println!("  corners:{corners:?}");
        for corner in corners {
            if corner != (3, 0) {
                let mut path = Vec::new();
                path.append(&mut straight_directional_line(from, corner));
                path.append(&mut straight_directional_line(corner, to));
                println!("    corner path:{path:?}");
                paths.push(path);
            }
        }
    } else {
        // Straight path.
        println!("  straight");
        paths.push(straight_directional_line(from, to));
    }
    println!(
        "    PATHS:{:?}",
        paths
            .iter()
            .map(|x| directionals_from_positions((*x).clone()))
            .collect::<Vec<_>>()
    );

    Vec::new()
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

#[allow(dead_code)]
fn directionals_from_positions(positions: Vec<(usize, usize)>) -> String {
    let mut result = String::new();
    for p in positions {
        result.push(match p {
            (0, 1) => '^',
            (1, 1) => 'v',
            (1, 2) => '>',
            (1, 0) => '<',
            (0, 2) => 'A',
            _ => '?',
        });
    }
    result
}