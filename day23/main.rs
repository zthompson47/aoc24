use std::collections::{HashMap, HashSet};

fn main() {
    let network: HashMap<&str, Vec<&str>> =
        include_str!("input.txt")
            .lines()
            .fold(HashMap::new(), |mut network, connection| {
                let (left, right) = connection.split_once('-').unwrap();
                network
                    .entry(left)
                    .and_modify(|x| {
                        //if !x.contains(&right) {
                        x.push(right)
                        //}
                    })
                    .or_insert(vec![right]);
                network
                    .entry(right)
                    .and_modify(|x| {
                        //if !x.contains(&left) {
                        x.push(left)
                        //}
                    })
                    .or_insert(vec![left]);
                network
            });
    println!("{network:?}");
    //println!("network count: {}", network.keys().len());
    //for v in network.values() {
    //    println!("{}", v.len());
    //}

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
    println!("{triplets:?}");
    let mut triplets = triplets.into_iter().collect::<Vec<_>>();
    triplets.sort_by(|a, b| a.0.cmp(b.0));
    //for t in triplets.iter() {
    //    println!("{t:?}");
    //}
    //println!("triplet count: {}", triplets.len());

    let part1 = triplets.iter().filter(|x| x.has_t()).count();
    println!("Part 1: {part1}");

    let mut max = Vec::new();
    for triplet in triplets.into_iter() {
        let group = triplet.max_network(&network);
        //println!("len: {} {group:?}", group.len());
        if group.len() > max.len() {
            max = group;
        }
    }
    //println!("{} {max:?}", max.len());
    max.sort();
    let part2 = max.join(",");
    println!("Part 2: {part2}");
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Cache(String, Vec<String>);

fn is_connected(
    host: &str,
    group: &[&str],
    network: &HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<Cache, bool>,
) -> bool {
    let mut new_group = Vec::new();
    for g in group {
        new_group.push(g.to_string());
    }
    if let Some(result) = cache.get(&Cache(host.to_string(), new_group.clone())) {
        return *result;
    }
    let host_group = network.get(host).unwrap();
    for peer in group.iter() {
        if !host_group.contains(peer) {
            cache.insert(Cache(host.to_string(), new_group), false);
            return false;
        }
    }
    true
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

    fn max_network(self, network: &'a HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
        let mut result = HashSet::from([self.0, self.1, self.2]);
        let peers0 = network.get(self.0).unwrap();
        let peers1 = network.get(self.1).unwrap();
        let peers2 = network.get(self.2).unwrap();
        for peer in peers0 {
            if peers1.contains(peer) && peers2.contains(peer) {
                let mut good = true;
                for h in &result {
                    let p = network.get(peer).unwrap();
                    if !p.contains(h) {
                        good = false;
                    }
                }
                if good {
                    result.insert(peer);
                }
            }
        }
        for peer in peers1 {
            if peers0.contains(peer) && peers2.contains(peer) {
                let mut good = true;
                for h in &result {
                    let p = network.get(peer).unwrap();
                    if !p.contains(h) {
                        good = false;
                    }
                }
                if good {
                    result.insert(peer);
                }
            }
        }
        for peer in peers2 {
            if peers0.contains(peer) && peers1.contains(peer) {
                let mut good = true;
                for h in &result {
                    let p = network.get(peer).unwrap();
                    if !p.contains(h) {
                        good = false;
                    }
                }
                if good {
                    result.insert(peer);
                }
            }
        }
        let mut result = result.iter().copied().collect::<Vec<_>>();
        result.sort();
        result
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
