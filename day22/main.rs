use std::collections::{HashMap, VecDeque};

fn main() {
    let buyers: Vec<Vec<isize>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut num = line.parse::<isize>().unwrap();
            let mut secrets = vec![num];
            for _ in 0..2000 {
                num = step(num);
                secrets.push(num);
            }
            secrets
        })
        .collect();

    //println!("{buyers:#?}");

    let part1: isize = buyers.iter().map(|secrets| secrets.last().unwrap()).sum();
    println!("Part 1: {part1}");

    let prices: Vec<Vec<(Price, Diff)>> = buyers
        .iter()
        .map(|secrets| {
            secrets
                .iter()
                .zip(secrets.iter().skip(1))
                .map(|(prior, next)| (next % 10, next % 10 - prior % 10))
                .collect()
        })
        .collect();

    //println!("{prices:#?}");

    let predictions: Vec<HashMap<Sequence, Price>> = prices
        .iter()
        .map(|price_list| {
            let mut sequence = VecDeque::<Diff>::new();
            price_list
                .iter()
                .fold(HashMap::new(), |mut acc, (price, diff)| {
                    sequence.push_back(*diff);
                    if sequence.len() == 5 {
                        sequence.pop_front();
                    }
                    if sequence.len() == 4 {
                        let sequence_key = Sequence::from(&sequence);
                        acc.entry(sequence_key).or_insert(*price);
                    }
                    acc
                })
        })
        .collect();

    //println!("{predictions:?}");

    let mut master_price_list: HashMap<Sequence, Price> = HashMap::new();

    for prediction_list in predictions {
        // Use with test0.txt..
        //if let Some(price) = prediction_list.get(&Sequence((-2, 1, -1, 3))) {
        //    println!("yeah: {price}");
        //}
        for prediction in prediction_list {
            master_price_list
                .entry(prediction.0)
                .and_modify(|x| *x += prediction.1)
                .or_insert(prediction.1);
        }
    }

    //println!("{master_price_list:?}");

    let part2 = master_price_list.values().max().unwrap();

    println!("Part 2: {part2}");
}

type Price = isize;
type Diff = isize;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Sequence((isize, isize, isize, isize));

impl From<&VecDeque<isize>> for Sequence {
    fn from(deque: &VecDeque<isize>) -> Self {
        Sequence((deque[0], deque[1], deque[2], deque[3]))
    }
}

fn step(mut num: isize) -> isize {
    num = num ^ (num * 64);
    num %= 16777216;
    num = num ^ (num / 32);
    num %= 16777216;
    num = num ^ (num * 2048);
    num %= 16777216;
    num
}
