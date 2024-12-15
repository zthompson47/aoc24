fn main() {
    println!("Part 1: {}", part1());
    println!();
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
                    println!("{machine:?}");
                    println!("a={a} b={b} cost={}", a * 3 + b);
                    costs.push(a * 3 + b);
                }
            }
        }

        if !costs.is_empty() {
            costs.sort();
            println!(
                "****************** asdf 1. ADDING COST: {} *****************",
                costs[0]
            );
            cost += costs[0];
        }
    }

    cost
}

fn part2() -> usize {
    let mut machines = input();
    let mut cost = 0;

    for machine in machines.iter_mut() {
        //machine.prize.0 += 10000000000000;
        //machine.prize.1 += 10000000000000;
        println!("_____________________________");
        println!("{machine:?}");
        let Machine { a, b, prize } = machine;

        let max_a_x = prize.0 / a.0;
        let max_a_y = prize.1 / a.1;
        let max_b_x = prize.0 / b.0;
        let max_b_y = prize.1 / b.1;
        println!("max_a_x={max_a_x} max_a_y={max_a_y} max_b_x={max_b_x} max_b_y={max_b_y}");

        let mut x_fits = Vec::new();
        for i in (0..=max_b_x).rev() {
            let delta = prize.0 - b.0 * i;
            //println!("try delta x {delta}, i={i}");
            if delta > a.0 * b.0 * 2 {
                //break;
            }
            if delta % a.0 == 0 {
                let distance = delta / a.0;
                println!("GOT ONE X - b.x={i} a.x={distance}");
                x_fits.push((i, distance));
            }
        }

        println!("----");

        let mut y_fits = Vec::new();
        for i in (0..=max_b_y).rev() {
            let delta = prize.1 - b.1 * i;
            //println!("try delta y {delta}, i={i}");
            if delta > a.1 * b.1 * 2 {
                break;
            }
            if delta % a.1 == 0 {
                let distance = delta / a.1;
                println!("GOT ONE Y - b.y={i} a.y={distance}");
                y_fits.push((i, distance));
            }
        }

        println!("___FITS: {x_fits:?}, {y_fits:?}");

        let mut step = None;
        if x_fits.len() > 0 && y_fits.len() > 0 {
            let x_step = if x_fits.len() == 1 {
                2
            } else {
                x_fits[1].1 - x_fits[0].1
            };
            let y_step = if y_fits.len() == 1 {
                2
            } else {
                y_fits[1].1 - y_fits[0].1
            };
            step = Some((x_step, y_step));
        }

        println!("___STEP: {step:?}");

        if x_fits == y_fits && !x_fits.is_empty() {
            cost += x_fits[0].0 + x_fits[0].1 * 3;
            println!(
                "****************** asdf 2. ADDING COST: {} *****************",
                x_fits[0].0 + x_fits[0].1 * 3
            );
        } else if let Some(step) = step {
            for i in (0..=max_b_x).rev() {
                if i > step.0 * step.1 * 2 {
                    break;
                }
                let delta = prize.0 - b.0 * i;
                //println!("try delta x {delta}, i={i}");
                if delta > a.0 * b.0 * 2 {
                    //break;
                }
                if delta % a.0 == 0 {
                    let distance = delta / a.0;
                    println!("GOT ONE AGAIN X - b.x={i} a.x={distance}");
                    if b.1 * i + a.1 * distance == prize.1 {
                        println!("FOUND COST: b={i} a={distance}");
                        println!(
                            "****************** asdf 2. ADDING COST: {} *****************",
                            i + distance * 3
                        );
                        cost += i + distance * 3;
                        break;
                    }
                }
            }
        }

        //if i == 0 {
        //    break;
        //}
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
