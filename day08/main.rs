use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> u32 {
    let map: Map = include_str!("test.txt")
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
        });

    println!("{map:?}");

    0
}

#[derive(Default, Debug)]
struct Map {
    dimensions: (usize, usize),
    antennas: HashMap<char, Vec<(usize, usize)>>,
}
