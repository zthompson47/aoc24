use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut patterns: Vec<&str> = lines.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = lines.skip(1).collect();

    patterns.sort();
    //println!("Patterns: {patterns:#?}");

    let part1: usize = designs
        .iter()
        .filter(|design| is_possible(design, &patterns))
        .count();

    println!("Part 1: {part1}");
}

fn is_possible(design: &str, patterns: &[&str]) -> bool {
    println!("design: {design}");

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
    //println!("    matches {:?}", matches);

    //tail_matches(design, &matches)
    let mut visited = Vec::new();
    matches_fit(design.len(), &matches, 0, &mut visited)
}

fn matches_fit(
    design_len: usize,
    matches: &[HashMap<usize, Vec<&str>>],
    level: usize,
    visited: &mut Vec<usize>,
) -> bool {
    //println!("___________________");
    if visited.contains(&level) {
        return false;
    }
    if level < 20 {
        println!(
            "level: {level} design_len: {design_len} {:?}",
            matches[level]
        );
    }
    if level > matches.len() {
        //println!("REACED END");
        return false;
    }
    let mut keys = matches[level].keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| b.cmp(a));
    for len in keys {
        //println!("try len:{len}");
        if design_len == *len {
            println!("MATCHED: {len}");
            return true;
        }
        if design_len > *len && matches_fit(design_len - len, matches, level + len, visited) {
            println!("NEXT LEVEL: {}", level + len);
            return true;
        }
    }
    //println!("FALSE");
    visited.push(level);
    false
}

/*
fn tail_matches(design: &str, matches: &[HashMap<usize, Vec<&str>>]) -> bool {
    println!("  tail_matches: {design}");
    //println!("    matches {:?}", matches);
    println!("    index: {}", design.len() - 1);
    println!("    should try {:?}", matches[design.len() - 1].keys());
    for len in matches[design.len() - 1].keys() {
        println!("   trying len:{len}");
        if *len == design.len() - 1 || tail_matches(&design[0..design.len() - len], matches) {
            println!("----------TRUE----------------");
            return true;
        }
    }
    false
}
*/

/*
fn is_possible(design: &str, patterns: &[&str]) -> bool {
    for pattern in patterns {
        if design == *pattern {
            return true;
        }
    }
    for pattern in patterns {
        //println!("trying pattern {pattern}");
        if let Some(design) = design.strip_prefix(*pattern) {
            //println!("found prefix {pattern}");
            println!("found prefix:{pattern} for {design}");
            if is_possible(design, patterns) {
                return true;
            }
        }
    }
    println!("nope");
    false
}
*/
