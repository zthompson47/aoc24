use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> u32 {
    let mut no_follow: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut manuals: Vec<Vec<u32>> = Vec::new();
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

    //println!("{no_follow:#?}");
    //println!("{manuals:#?}");

    'manual: for manual in manuals {
        let mut illegal: Vec<u32> = Vec::new();
        //println!("Trying: {manual:?}");

        for page in &manual {
            //println!("{illegal:?} {page}");
            if illegal.contains(page) {
                //println!("continuing...");
                continue 'manual;
            }
            if let Some(pages) = no_follow.get(page) {
                illegal.append(&mut pages.clone());
            }
        }

        //println!("illegal: {illegal:?}");
        //println!("good: {manual:?}");
        //println!();

        let middle = manual[manual.len() / 2];
        result += middle;
    }

    result
}
