use std::collections::{HashMap, HashSet};

fn main() {
    println!("Part 1: {}", Map::new().antinodes().len());
    println!("Part 2: {}", Map::new().antinodes_with_harmonics().len());
}

#[derive(Default, Debug)]
struct Map {
    dimensions: (usize, usize),
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Position {
    row: usize,
    column: usize,
}

impl Map {
    fn new() -> Self {
        include_str!("input.txt")
            .lines()
            .enumerate()
            .fold(Map::default(), |mut map, (row, line)| {
                map.dimensions.0 = row + 1;
                line.chars()
                    .enumerate()
                    .fold(map, |mut map, (column, char)| {
                        map.dimensions.1 = column + 1;
                        match char {
                            '.' => {}
                            char => {
                                map.antennas
                                    .entry(char)
                                    .and_modify(|x| x.push((row, column)))
                                    .or_insert(vec![(row, column)]);
                            }
                        };
                        map
                    })
            })
    }

    fn antinodes(&self) -> HashSet<Position> {
        self.antennas
            .values()
            .fold(HashSet::new(), |mut acc, group| {
                for l in group.iter().enumerate() {
                    if group.len() > l.0 + 1 {
                        for r in group[l.0 + 1..].iter() {
                            let l = ((l.1).0 as i32, (l.1).1 as i32);
                            let r = (r.0 as i32, r.1 as i32);
                            let delta = (l.0 - r.0, l.1 - r.1);
                            let antinode_l = (l.0 + delta.0, l.1 + delta.1);
                            let antinode_r = (r.0 - delta.0, r.1 - delta.1);
                            if self.in_bounds(antinode_l) {
                                acc.insert(Position {
                                    row: antinode_l.0 as usize,
                                    column: antinode_l.1 as usize,
                                });
                            }
                            if self.in_bounds(antinode_r) {
                                acc.insert(Position {
                                    row: antinode_r.0 as usize,
                                    column: antinode_r.1 as usize,
                                });
                            }
                        }
                    }
                }
                acc
            })
    }

    fn antinodes_with_harmonics(&self) -> HashSet<Position> {
        self.antennas
            .values()
            .fold(HashSet::new(), |mut acc, group| {
                for (l_index, l_position) in group.iter().enumerate() {
                    if group.len() > l_index + 1 {
                        for r_position in group[l_index + 1..].iter() {
                            let mut l = (l_position.0 as i32, l_position.1 as i32);
                            let mut r = (r_position.0 as i32, r_position.1 as i32);
                            let slope = simplify((l.0 - r.0, l.1 - r.1));
                            loop {
                                acc.insert(Position {
                                    row: l.0 as usize,
                                    column: l.1 as usize,
                                });
                                l.0 += slope.0;
                                l.1 += slope.1;
                                if !self.in_bounds(l) {
                                    break;
                                }
                            }
                            loop {
                                acc.insert(Position {
                                    row: r.0 as usize,
                                    column: r.1 as usize,
                                });
                                r.0 -= slope.0;
                                r.1 -= slope.1;
                                if !self.in_bounds(r) {
                                    break;
                                }
                            }
                        }
                    }
                }
                acc
            })
    }

    fn in_bounds(&self, antinode: (i32, i32)) -> bool {
        if antinode.0 >= 0
            && antinode.0 < self.dimensions.0 as i32
            && antinode.1 >= 0
            && antinode.1 < self.dimensions.1 as i32
        {
            return true;
        }
        false
    }
}

fn simplify(fraction: (i32, i32)) -> (i32, i32) {
    let mut fraction = fraction;
    loop {
        if fraction.0 % 2 != 0 || fraction.1 % 2 != 0 {
            return fraction;
        }
        fraction = (fraction.0 / 2, fraction.1 / 2);
    }
}
