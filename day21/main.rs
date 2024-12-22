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

    for code in codes.iter() {
        println!("{code:?}");
    }

    let part1: usize = codes
        .iter()
        .zip(numbers)
        .map(|(code, number)| {
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

    /*
    let part1: usize = codes
        .iter()
        .zip(numbers)
        .map(|(code, number)| {
            let mut steps = directions(directions(directions(code.to_vec())));
            steps.remove(0);
            steps.len() * number
        })
        .sum();
    */

    //println!("Part 1: {part1}");
}

/// Find shortest path between keys on numeric keyboard.  Assume corner paths will always
/// be shorter than zig-zag through the center (?).
fn shortest_numeric(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    let mut paths = Vec::new();
    if from == to {
        return Vec::new();
    } else if from.0 != to.0 && from.1 != to.1 {
        let corners = [(from.0, to.1), (to.0, from.1)];
        println!("CORNER: from:{from:?} to:{to:?} corners:{corners:?}");
        for corner in corners {
            let mut path = Vec::new();
            if corner != (3, 0) {
                path.append(&mut straight_directional_line(from, corner));
                path.append(&mut straight_directional_line(corner, to));
            }
            println!(" corner path:{path:?}");
        }
    } else {
        println!("STRAIGHT: from:{from:?} to:{to:?}");
        paths = straight_directional_line(from, to);
    }
    println!("    RESULT:{paths:?}");

    Vec::new()
}

fn straight_directional_line(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    assert!(from.0 == to.0 || from.1 == to.1);
    //println!("    line: from:{from:?} to:{to:?}");
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

/*
fn directions(mut positions: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = vec![position_from_directional('A')];
    let mut from = positions.remove(0);
    for to in positions {
        for _ in 0..from.0.abs_diff(to.0) {
            result.push(position_from_directional(if from.0 < to.0 {
                'v'
            } else {
                '^'
            }));
        }
        for _ in 0..from.1.abs_diff(to.1) {
            result.push(position_from_directional(if from.1 < to.1 {
                '>'
            } else {
                '<'
            }));
        }
        result.push(position_from_directional('A'));
        from = to;
    }
    result
}
*/

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

fn directions_from_positions(positions: Vec<(usize, usize)>) -> String {
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
