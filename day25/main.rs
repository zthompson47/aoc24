fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    loop {
        let next = lines.next();
        if let Some("#####") = next {
            let mut lock = [0; 5];
            for i in 1..=5 {
                let line = lines.next().unwrap();
                for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock[j] = i;
                    }
                }
            }
            lines.next();
            locks.push(lock);
        } else if let Some(".....") = next {
            let mut key = [5; 5];
            for i in 1..=5 {
                let line = lines.next().unwrap();
                for (j, c) in line.chars().enumerate() {
                    if c == '.' {
                        key[j] = 5 - i;
                    }
                }
            }
            lines.next();
            keys.push(key);
        } else if next.is_none() {
            break;
        }
    }
    //println!("locks:");
    for lock in &locks {
        //println!("{lock:?}");
    }
    //println!("keys:");
    for key in &keys {
        //println!("{key:?}");
    }
    //println!("locks: {}, keys: {}", &locks.len(), &keys.len());

    let mut matches = 0;
    for lock in &locks {
        'out: for key in &keys {
            //println!("compare lock: {lock:?} to key: {key:?}");
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    //println!("no match");
                    continue 'out;
                }
            }
            //println!("match");
            matches += 1;
        }
    }

    println!("Part 1: {}", matches);
}
