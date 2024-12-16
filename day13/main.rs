fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let machines = input();
    let mut cost = 0;

    for machine in machines {
        let mut costs: Vec<usize> = Vec::new();

        for a in 0..=100 {
            for b in 0..=100 {
                let x = machine.a.0 * a + machine.b.0 * b;
                let y = machine.a.1 * a + machine.b.1 * b;
                if x == machine.prize.0 && y == machine.prize.1 {
                    costs.push(a * 3 + b);
                }
            }
        }

        if !costs.is_empty() {
            costs.sort();
            cost += costs[0];
        }
    }

    cost
}

fn part2() -> usize {
    let mut machines = input();
    let mut cost = 0;

    for machine in machines.iter_mut() {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
        let Machine { a, b, prize } = machine;
        let max_b_x = prize.0 / b.0;
        let max_b_y = prize.1 / b.1;

        let mut x_fits = Vec::new();
        for i in (0..=max_b_x).rev() {
            let delta = prize.0 - b.0 * i;
            if delta > a.0 * b.0 * 2 {
                break;
            }
            if delta % a.0 == 0 {
                let distance = delta / a.0;
                x_fits.push((i, distance));
            }
            if x_fits.len() > 1 {
                break;
            }
        }

        let mut y_fits = Vec::new();
        for i in (0..=max_b_y).rev() {
            let delta = prize.1 - b.1 * i;
            if delta > a.1 * b.1 * 2 {
                break;
            }
            if delta % a.1 == 0 {
                let distance = delta / a.1;
                y_fits.push((i, distance));
            }
            if y_fits.len() > 1 {
                break;
            }
        }

        if x_fits.len() == 2 && y_fits.len() == 2 {
            let x_fits = x_fits
                .iter()
                .map(|(x, y)| (*x as i64, *y as i64))
                .collect::<Vec<_>>();
            let y_fits = y_fits
                .iter()
                .map(|(x, y)| (*x as i64, *y as i64))
                .collect::<Vec<_>>();
            if let (&[(x1, y1), (x2, y2)], &[(x3, y3), (x4, y4)]) = (&*x_fits, &*y_fits) {
                let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
                if denominator != 0 {
                    let x_numerator =
                        (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
                    let y_numerator =
                        (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
                    let x = x_numerator / denominator;
                    let y = y_numerator / denominator;
                    if x >= 0 && y >= 0 {
                        let new_cost = x as usize + 3 * y as usize;
                        if x as usize * b.0 + y as usize * a.0 == prize.0
                            && x as usize * b.1 + y as usize * a.1 == prize.1
                        {
                            cost += new_cost;
                        }
                    }
                }
            }
        } else if x_fits.len() == 1 && y_fits.len() == 1 && x_fits[0] == y_fits[0] {
            let (x, y) = y_fits[0];
            let new_cost = x + 3 * y;
            cost += new_cost;
        } else if x_fits.len() == 1 && y_fits.len() == 2 {
            let (x, y) = x_fits[0];
            if x * b.0 + y * a.0 == prize.0 && x * b.1 + y * a.1 == prize.1 {
                let new_cost = x + 3 * y;
                cost += new_cost;
            }
        } else if x_fits.len() == 2 && y_fits.len() == 1 {
            let (x, y) = y_fits[0];
            if x * b.0 + y * a.0 == prize.0 && x * b.1 + y * a.1 == prize.1 {
                let new_cost = x + 3 * y;
                cost += new_cost;
            }
        }
    }

    cost
}

fn input() -> Vec<Machine> {
    let mut lines = include_str!("input.txt").lines();
    let mut machines = Vec::new();
    fn parse_line(input: &str, delimiter: char) -> (usize, usize) {
        let x_y = input.split_once(':').unwrap().1.split_once(',').unwrap();
        (
            x_y.0
                .split_once(delimiter)
                .unwrap()
                .1
                .trim()
                .parse()
                .unwrap(),
            x_y.1
                .split_once(delimiter)
                .unwrap()
                .1
                .trim()
                .parse()
                .unwrap(),
        )
    }
    loop {
        machines.push(Machine {
            a: parse_line(lines.next().unwrap(), '+'),
            b: parse_line(lines.next().unwrap(), '+'),
            prize: parse_line(lines.next().unwrap(), '='),
        });
        if lines.next().is_none() {
            break;
        }
    }
    machines
}

#[derive(Default, Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}
