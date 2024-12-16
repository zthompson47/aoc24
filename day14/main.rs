fn main() {
    //println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

//const X: usize = 11;
//const Y: usize = 7;
const X: usize = 101;
const Y: usize = 103;
const T: isize = 100;

type Map = Vec<Vec<i32>>;

fn part1() -> usize {
    let mut map: Map = vec![vec![0; Y]; X];
    let mut robots = input();

    for r in robots.iter_mut() {
        r.make_moves(&map, T);
        map[r.position.0 as usize][r.position.1 as usize] += 1;
    }

    println!("{robots:?}");

    print_map(&robots, &map);

    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    let x_mid = map.len() / 2;
    let y_mid = map[0].len() / 2;
    for y in 0..Y {
        for x in 0..X {
            if x < x_mid && y < y_mid {
                q1 += map[x][y];
            } else if x > x_mid && y < y_mid {
                q2 += map[x][y];
            } else if x < x_mid && y > y_mid {
                q3 += map[x][y];
            } else if x > x_mid && y > y_mid {
                q4 += map[x][y];
            }
        }
        println!();
    }

    println!("q1:{q1} q2:{q2} q3:{q3} q4:{q4}");

    (q1 * q2 * q3 * q4).try_into().unwrap()
}

fn part2() -> usize {
    let mut result = 0;
    let mut robots = input();

    for i in 1..10000 {
        //println!("{robots:?}");
        let mut map = vec![vec![0; Y]; X];
        for r in robots.iter_mut() {
            r.make_moves(&map, 1);
            map[r.position.0 as usize][r.position.1 as usize] += 1;
        }
        if vertical_symmetry(&map) > 92.0 {
            println!("{}", vertical_symmetry(&map));
            print_map(&robots, &map);
            println!();
            result = i;
            break;
        }

        // Top left blank..
        /*
        let mut got_one = true;
        'out: for x in 0..10 {
            for y in 0..10 {
                if map[x][y] != 0 {
                    got_one = false;
                    break 'out;
                }
            }
        }
        if got_one {
            print_map(&robots, &map);
            println!();
        }
        */

        //println!("{robots:?}");
        //print_map(&robots, &map);
        //println!();
        //println!("{}", percent_blank(&map));
    }

    result
}

fn vertical_symmetry(map: &Map) -> f32 {
    let mut count = 0;
    for x in 0..map.len() / 2 {
        for y in 0..map[0].len() {
            let left = map[x][y];
            let right = map[map.len() - 1 - x][y];
            match (left, right) {
                (0, 0) => count += 1,
                (0, _) | (_, 0) => {}
                _ => count += 1,
            }
        }
    }

    (count as f32 * 100.0) / ((map.len() as f32 / 2.0) * map[0].len() as f32)
}

fn percent_blank(map: &Map) -> f32 {
    map.iter().flatten().filter(|x| **x == 0).count() as f32 * 100.0
        / (map.len() as f32 * map[0].len() as f32)
}

fn print_map(robots: &[Robot], map: &Map) {
    for y in 0..Y {
        for x in 0..X {
            match map[x][y] {
                0 => print!(". "),
                x => print!("{x} "),
            }
        }
        println!();
    }
}

fn input() -> Vec<Robot> {
    include_str!("input.txt")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            let (p, v) = line.split_once(' ').unwrap();
            let p = p.split_once('=').unwrap().1.split_once(',').unwrap();
            let v = v.split_once('=').unwrap().1.split_once(',').unwrap();
            let position = (p.0.parse::<isize>().unwrap(), p.1.parse::<isize>().unwrap());
            let velocity = (v.0.parse::<isize>().unwrap(), v.1.parse::<isize>().unwrap());
            acc.push(Robot { position, velocity });
            acc
        })
}

#[derive(Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn make_moves(&mut self, map: &Map, count: isize) {
        let mut new_x = (self.position.0 + self.velocity.0 * count) % map.len() as isize;
        if new_x < 0 {
            new_x += map.len() as isize;
        }

        let mut new_y = (self.position.1 + self.velocity.1 * count) % map[0].len() as isize;
        if new_y < 0 {
            new_y += map[0].len() as isize;
        }

        self.position = (new_x, new_y);
    }
}
