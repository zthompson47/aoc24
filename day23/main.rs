use std::collections::{HashMap, HashSet};

type Network<'a> = HashMap<&'a str, Vec<&'a str>>;

fn main() {
    let network: Network =
        include_str!("input.txt")
            .lines()
            .fold(HashMap::new(), |mut network, connection| {
                let (left, right) = connection.split_once('-').unwrap();
                network
                    .entry(left)
                    .and_modify(|x| {
                        if !x.contains(&right) {
                            x.push(right)
                        }
                    })
                    .or_insert(vec![right]);
                network
                    .entry(right)
                    .and_modify(|x| {
                        if !x.contains(&left) {
                            x.push(left)
                        }
                    })
                    .or_insert(vec![left]);
                network
            });

    let triplets: HashSet<Triplet> =
        network
            .iter()
            .fold(HashSet::new(), |mut triplets, (host, peers)| {
                for pair in pairs(peers) {
                    if network.get(pair.0).unwrap().contains(&pair.1) {
                        triplets.insert(Triplet::new(host, pair));
                    }
                }
                triplets
            });

    let part1 = triplets.iter().filter(|x| x.has_t()).count();
    println!("Part 1: {part1}");

    let mut max = network
        .keys()
        .map(|host| largest_network(host, &network))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();
    max.sort();
    println!("Part 2: {}", max.join(","));
}

fn largest_network<'a>(host: &'a str, network: &'a Network) -> Vec<&'a str> {
    let mut result = vec![host];
    'out: for peer in network.get(host).unwrap() {
        let peer_network = network.get(peer).unwrap();
        for host in &result {
            if !peer_network.contains(host) {
                continue 'out;
            }
        }
        result.push(peer);
    }
    result
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Triplet<'a>(&'a str, &'a str, &'a str);

impl<'a> Triplet<'a> {
    fn new(host: &'a str, pair: (&'a str, &'a str)) -> Self {
        let mut triplet = [host, pair.0, pair.1];
        triplet.sort();
        Triplet(triplet[0], triplet[1], triplet[2])
    }

    fn has_t(&self) -> bool {
        self.0.starts_with('t') || self.1.starts_with('t') || self.2.starts_with('t')
    }
}

fn pairs<'a>(set: &'a [&str]) -> Vec<(&'a str, &'a str)> {
    assert!(set.len() >= 2);
    if set.len() == 2 {
        vec![(set[0], set[1])]
    } else {
        let mut result = Vec::new();
        for i in 1..set.len() {
            result.push((set[0], set[i]));
        }
        result.append(&mut pairs(&set[1..]));
        result
    }
}
