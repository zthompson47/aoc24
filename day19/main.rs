use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = lines.skip(1).collect();

    patterns.sort();

    let part1: usize = designs
        .iter()
        .filter(|design| is_possible(design, &patterns))
        .count();

    println!("Part 1: {part1}");

    let part2: usize = designs
        .iter()
        .map(|design| {
            let matches = matches(design, &patterns);
            let mut visited = HashMap::new();
            solutions(design.len(), &matches, 0, &mut visited)
        })
        .sum();

    println!("Part 2: {part2}");
}

fn solutions(
    remaining: usize,
    matches: &[HashMap<usize, Vec<&str>>],
    level: usize,
    visited: &mut HashMap<usize, usize>,
) -> usize {
    if level > matches.len() {
        return 0;
    }
    if visited.contains_key(&level) {
        return *visited.get(&level).unwrap();
    }
    // Start with longest substring matches.
    let mut keys = matches[level].keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| b.cmp(a));

    let mut result = 0;
    for len in keys {
        let match_count = matches[level].get(len).unwrap().len();

        #[allow(clippy::comparison_chain)]
        if *len == remaining {
            result += match_count;
        } else if *len < remaining {
            let to_add = match_count * solutions(remaining - *len, matches, level + *len, visited);
            result += to_add;
        }
    }

    visited.insert(level, result);

    result
}

fn is_possible(design: &str, patterns: &[&str]) -> bool {
    let matches = matches(design, patterns);
    let mut visited = Vec::new();
    matches_fit(design.len(), &matches, 0, &mut visited)
}

fn matches<'a>(design: &str, patterns: &'a [&str]) -> Vec<HashMap<usize, Vec<&'a str>>> {
    // Count possible matches by pattern length at each color in design.
    let mut matches: Vec<HashMap<usize, Vec<&str>>> = Vec::new();
    for i in 0..design.len() {
        let mut sizes = HashMap::new();
        for pattern in patterns {
            if pattern.len() <= design[i..].len() && &design[i..i + pattern.len()] == *pattern {
                sizes
                    .entry(pattern.len())
                    .and_modify(|x: &mut Vec<&str>| x.push(*pattern))
                    .or_insert(vec![*pattern]);
            }
        }
        matches.push(sizes);
    }
    matches
}

fn matches_fit(
    design_len: usize,
    matches: &[HashMap<usize, Vec<&str>>],
    level: usize,
    visited: &mut Vec<usize>,
) -> bool {
    if level > matches.len() {
        return false;
    }
    if visited.contains(&level) {
        return false;
    }
    let mut keys = matches[level].keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| b.cmp(a));
    for len in keys {
        if design_len == *len {
            return true;
        }
        if design_len > *len && matches_fit(design_len - len, matches, level + len, visited) {
            return true;
        }
    }
    visited.push(level);
    false
}
