use std::collections::HashMap;

fn main() {
    let (part1, bad_manuals, no_follow) = part1();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(bad_manuals, no_follow));
}

fn part1() -> (u32, Vec<Vec<u32>>, HashMap<u32, Vec<u32>>) {
    let mut no_follow: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut manuals: Vec<Vec<u32>> = Vec::new();
    let mut bad_manuals: Vec<Vec<u32>> = Vec::new();
    let mut reached_manuals = false;
    let mut result = 0;

    include_str!("input.txt").lines().for_each(|line| {
        if line.is_empty() {
            reached_manuals = true;
        } else if reached_manuals {
            manuals.push(line.split(',').map(|d| d.parse::<u32>().unwrap()).collect());
        } else {
            let (l, r) = line.split_once('|').unwrap();
            let (l, r) = (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap());
            no_follow
                .entry(r)
                .and_modify(|pages| pages.push(l))
                .or_insert(vec![l]);
        }
    });

    'manual: for manual in manuals {
        let mut illegal: Vec<u32> = Vec::new();

        for page in &manual {
            if illegal.contains(page) {
                bad_manuals.push(manual);
                continue 'manual;
            }
            if let Some(pages) = no_follow.get(page) {
                illegal.append(&mut pages.clone());
            }
        }

        let middle = manual[manual.len() / 2];
        result += middle;
    }

    (result, bad_manuals, no_follow)
}

fn part2(bad_manuals: Vec<Vec<u32>>, no_follow: HashMap<u32, Vec<u32>>) -> u32 {
    let mut result = 0;

    bad_manuals.iter().for_each(|manual| {
        let mut fixed: Vec<u32> = Vec::new();

        for page in manual {
            let mut insert_at = fixed.len();
            for i in (0..fixed.len()).rev() {
                if let Some(illegal) = no_follow.get(&fixed[i]) {
                    if illegal.contains(page) {
                        insert_at = i;
                    }
                }
            }
            fixed.insert(insert_at, *page);
        }

        result += fixed[fixed.len() / 2];
    });

    result
}
