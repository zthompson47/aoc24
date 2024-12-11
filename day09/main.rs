fn main() {
    let disk_map = DiskMap::from(
        include_str!("input.txt")
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>(),
    );

    let mut tail = disk_map.tail_chunks();
    let mut chunk = tail.next().unwrap();
    let mut position: usize = 0;
    let mut result: u128 = 0;

    'out: for (i, x) in disk_map.inner.iter().enumerate() {
        for _ in 0..*x {
            if i >= chunk.index {
                for _ in 0..chunk.count {
                    result += position as u128 * chunk.file_id as u128;
                    position += 1;
                }
                break 'out;
            }
            if i % 2 == 0 {
                result += position as u128 * i as u128 / 2;
            } else {
                if chunk.count == 0 {
                    if let Some(new_chunk) = tail.next() {
                        if new_chunk.index < i {
                            break 'out;
                        } else {
                            chunk = new_chunk;
                        }
                    } else {
                        break 'out;
                    }
                }
                result += position as u128 * chunk.file_id as u128;
                chunk.count -= 1;
            }
            position += 1;
        }
    }

    println!("Part 1: {result}");

    let mut disk = Vec::new();
    let mut files = Vec::new();
    for (i, size) in disk_map.inner.iter().enumerate() {
        if i % 2 == 0 {
            files.push(File {
                id: i / 2,
                size: *size,
                index: disk.len(),
            });
            for _ in 0..*size {
                disk.push(Thing::Number(i / 2));
            }
        } else {
            for _ in 0..*size {
                disk.push(Thing::Empty);
            }
        }
    }

    for file in files.iter().rev() {
        for i in 0..file.index {
            if i + file.size < disk.len() && disk[i..i + file.size] == vec![Thing::Empty; file.size]
            {
                #[allow(clippy::needless_range_loop)]
                for j in i..i + file.size {
                    disk[j] = Thing::Number(file.id);
                }
                #[allow(clippy::needless_range_loop)]
                for j in file.index..file.index + file.size {
                    disk[j] = Thing::Empty;
                }
                break;
            }
        }
    }

    let mut checksum = 0;
    for (i, thing) in disk.iter().enumerate() {
        if let Thing::Number(n) = thing {
            checksum += i * n;
        }
    }

    println!("Part 2: {checksum}");
}

#[allow(unused)]
fn print_disk(disk: &[Thing]) {
    for thing in disk {
        match thing {
            Thing::Number(n) => print!("{n}"),
            Thing::Empty => print!("."),
        }
    }
    println!();
}

#[derive(Debug, Clone, PartialEq)]
enum Thing {
    Number(usize),
    Empty,
}

#[derive(Debug)]
struct File {
    id: usize,
    size: usize,
    index: usize,
}

#[derive(Debug)]
struct DiskMap {
    inner: Vec<usize>,
}

impl From<Vec<usize>> for DiskMap {
    fn from(inner: Vec<usize>) -> Self {
        DiskMap { inner }
    }
}

impl DiskMap {
    fn tail_chunks(&self) -> TailChunks {
        let index = if self.inner.len() % 2 == 0 {
            self.inner.len() - 2
        } else {
            self.inner.len() - 1
        };
        let file_id = self.inner.len() / 2;
        TailChunks {
            index,
            file_id,
            map: self,
        }
    }
}

struct TailChunks<'a> {
    index: usize,
    file_id: usize,
    map: &'a DiskMap,
}

impl Iterator for TailChunks<'_> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            None
        } else {
            let chunk = Chunk {
                file_id: self.file_id,
                index: self.index,
                count: self.map.inner[self.index],
            };
            self.index -= 2;
            self.file_id -= 1;
            Some(chunk)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Chunk {
    index: usize,
    file_id: usize,
    count: usize,
}
