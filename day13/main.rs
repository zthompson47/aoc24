fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    let machines = input();
    let mut cost = 0;

    for machine in machines {
        let mut costs: Vec<usize> = Vec::new();

        for a in 1..=100 {
            for b in 1..=100 {
                let x = machine.a.0 * a + machine.b.0 * b;
                let y = machine.a.1 * a + machine.b.1 * b;
                if x == machine.prize.0 && y == machine.prize.1 {
                    costs.push(a * 3 + b);
                }
            }
        }

        if !costs.is_empty() {
            costs.sort_by(|a, b| b.cmp(a));
            cost += costs[0];
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
