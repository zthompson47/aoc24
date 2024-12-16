fn main() {
    //println!("Part 1: {}", part1());
    println!();
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    let machines = input();
    let mut cost = 0;

    for machine in machines {
        println!("{machine:?}");
        let mut costs: Vec<usize> = Vec::new();

        for a in 0..=100 {
            for b in 0..=100 {
                let x = machine.a.0 * a + machine.b.0 * b;
                let y = machine.a.1 * a + machine.b.1 * b;
                if x == machine.prize.0 && y == machine.prize.1 {
                    //println!("{machine:?}");
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
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
        println!("_____________________________");
        println!("{machine:?}");
        let Machine { a, b, prize } = machine;

        let max_a_x = prize.0 / a.0;
        let max_a_y = prize.1 / a.1;
        let max_b_x = prize.0 / b.0;
        let max_b_y = prize.1 / b.1;
        //println!("max_a_x={max_a_x} max_a_y={max_a_y} max_b_x={max_b_x} max_b_y={max_b_y}");

        let mut x_fits = Vec::new();
        for i in (0..=max_b_x).rev() {
            let delta = prize.0 - b.0 * i;
            //println!("try delta x {delta}, i={i}, {}%", delta as f64 * 100.0 / prize.0 as f64);
            if delta > a.0 * b.0 * 2 {
                break;
            }
            if delta % a.0 == 0 {
                let distance = delta / a.0;
                //println!("GOT ONE X - b.x={i} a.x={distance}");
                x_fits.push((i, distance));
            }
            if x_fits.len() > 1 {
                break;
            }
        }

        //println!("----");

        let mut y_fits = Vec::new();
        for i in (0..=max_b_y).rev() {
            let delta = prize.1 - b.1 * i;
            //println!("try delta y {delta}, i={i}");
            if delta > a.1 * b.1 * 2 {
                break;
            }
            if delta % a.1 == 0 {
                let distance = delta / a.1;
                //println!("GOT ONE Y - b.y={i} a.y={distance}");
                y_fits.push((i, distance));
            }
            if y_fits.len() > 1 {
                break;
            }
        }

        println!("___FITS: x_len: {}, y_len: {}", x_fits.len(), y_fits.len());
        println!("___FITS: x: {:?}, y: {:?}", x_fits, y_fits);

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
                println!("...... {x1} {x2} {x3} {x4} {y1} {y2} {y3} {y4}");
                let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
                if denominator == 0 {
                    println!("...... Parallel or coincident!");
                } else {
                    let x_numerator =
                        (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
                    let y_numerator =
                        (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
                    let x = x_numerator / denominator;
                    let y = y_numerator / denominator;
                    if x >= 0 && y >= 0 {
                        let new_cost = x as usize + 3 * y as usize;
                        println!("------ Intersects! x={x} y={y} cost={new_cost}");
                        if x as usize * b.0 + y as usize * a.0 == prize.0
                            && x as usize * b.1 + y as usize * a.1 == prize.1
                        {
                            println!("------ and in rnage! cost={new_cost}");
                            cost += new_cost;
                        }
                    } else {
                        println!("------ negative IMPOSSIBLE");
                    }
                }
            }
        } else if x_fits.is_empty() || y_fits.is_empty() {
            println!("------ zero IMPOSSIBLE");
            continue;
        } else if x_fits.len() == 1 && y_fits.len() == 1 && x_fits[0] == y_fits[0] {
            let (x, y) = y_fits[0];
            let new_cost = x + 3 * y;
            println!("------ single match x={x} y={y} cost={new_cost}");
            cost += new_cost;
        } else if x_fits.len() == 1 && y_fits.len() == 2 {
            println!("-------------------------x1");
            let (x, y) = x_fits[0];
            if x * b.0 + y * a.0 == prize.0 && x * b.1 + y * a.1 == prize.1 {
                let new_cost = x + 3 * y;
                println!("---- x1 YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY cost={new_cost}");
                cost += new_cost;
            }
        } else if x_fits.len() == 2 && y_fits.len() == 1 {
            println!("-------------------------y1");
            let (x, y) = y_fits[0];
            if x * b.0 + y * a.0 == prize.0 && x * b.1 + y * a.1 == prize.1 {
                let new_cost = x + 3 * y;
                println!("---- y1 YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY cost={new_cost}");
                cost += new_cost;
            }
        }

        /*
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
        */

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
