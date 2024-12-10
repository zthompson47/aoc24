fn main() {
    let disk_map = DiskMap::from(
        include_str!("input.txt")
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>(),
    );

    //println!("disk map: {:?}", disk_map.inner);
    //if disk_map.inner.len() % 2 == 0 {
    //    println!("..even");
    //} else {
    //    println!("..odd");
    //}
    //for c in disk_map.tail_chunks() {
    //    println!("{c:?}");
    //}

    let mut tail = disk_map.tail_chunks();
    let mut chunk = tail.next().unwrap();
    let mut position: usize = 0;
    let mut result: u128 = 0;

    // NOT 6449015880027
    'out: for (i, x) in disk_map.inner.iter().enumerate() {
        for _ in 0..*x {
            //println!("i: {i}, chunk.index: {}, x: {x}", chunk.index);
            if i >= chunk.index {
                for _ in 0..chunk.count {
                    //println!("__drain: {} * {}", position, chunk.file_id);
                    result += position as u128 * chunk.file_id as u128;
                    position += 1;
                }
                break 'out;
            }
            if i % 2 == 0 {
                //println!("__filled: {} * {}", position, i / 2);
                result += position as u128 * i as u128 / 2;
            } else {
                if chunk.count == 0 {
                    //println!("RESET TAIL");
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
                //println!("__empty: {} * {}", position, chunk.file_id);
                result += position as u128 * chunk.file_id as u128;
                chunk.count -= 1;
            }
            position += 1;
        }
    }

    println!("Part 1: {result}");

    /*
    let mut disk: Vec<Block> = disk_map
        .inner
        .iter()
        .enumerate()
        .map(|(i, size)| {
            if i % 2 == 0 {
                Block::Occupied(Chunk {
                    index: i,
                    file_id: i / 2,
                    count: *size,
                    visited: false,
                })
            } else {
                Block::Vacant(*size)
            }
        })
        .collect();

    let mut xx = 0;
    //display_disk(&disk);
    let mut chunk_holder = first_unvisited(&mut disk);
    while let Some(chunk) = chunk_holder {
        //println!("trying: {chunk:?}");
        chunk_holder = first_unvisited(&mut disk);
        for i in 0..chunk.index {
            if let Block::Vacant(size) = disk[i] {
                if chunk.count <= size {
                    //println!("___at {i} for {}", chunk.file_id);
                    disk[i] = Block::Occupied(chunk);
                    let size_diff = size - chunk.count;
                    disk[chunk.index] = Block::Vacant(chunk.count);
                    if size_diff > 0 {
                        //println!("** add space **");
                        disk.insert(i + 1, Block::Vacant(size_diff));
                        if disk.len() > i + 2 {
                            //println!("*** updating offsets ***");
                            for j in i + 2..disk.len() {
                                let block = disk[j];
                                if let Block::Occupied(mut chunk) = block {
                                    chunk.index += 1;
                                    disk[j] = Block::Occupied(chunk);
                                }
                            }
                        }
                    }
                    break;
                }
            }
        }
        /*if xx == 10 {
            break;
        }
        xx += 1;*/
        //display_disk(&disk);
    }

    let mut checksum = 0;
    let mut position = 0;
    for block in disk {
        if let Block::Occupied(chunk) = block {
            for _ in 0..chunk.count {
                checksum += position * chunk.file_id;
                position += 1;
            }
        } else if let Block::Vacant(size) = block {
            position += size;
        }
    }

    println!("Part 2: {checksum}");
    */

    let mut disk = Vec::new();
    let mut files = Vec::new();
    //println!("{files:?}");
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
    //print_disk(&disk);
    let mut count = 0;
    for file in files.iter().rev() {
        println!("{count}/{}", files.len());
        count += 1;
        //println!("try file: {file:?}");
        for i in 0..file.index {
            if i + file.size < disk.len() && disk[i..i + file.size] == vec![Thing::Empty; file.size]
            {
                //println!("__found space at {i}");
                for j in i..i + file.size {
                    disk[j] = Thing::Number(file.id);
                }
                for j in file.index..file.index + file.size {
                    disk[j] = Thing::Empty;
                }
                //print_disk(&disk);
                break;
            }
        }
    }
    //print_disk(&disk);

    let mut checksum = 0;
    for (i, thing) in disk.iter().enumerate() {
        if let Thing::Number(n) = thing {
            checksum += i * n;
        }
    }

    println!("Part 2: {checksum}");
}

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

fn first_unvisited(disk: &mut [Block]) -> Option<Chunk> {
    for i in (0..disk.len()).rev() {
        if let Block::Occupied(mut chunk) = disk[i] {
            if !chunk.visited {
                chunk.visited = true;
                disk[i] = Block::Occupied(chunk);
                return Some(chunk);
            }
        }
    }

    None
}

fn display_disk(disk: &[Block]) {
    for block in disk {
        if let Block::Occupied(chunk) = block {
            for _ in 0..chunk.count {
                print!("{}", chunk.file_id);
            }
        } else if let Block::Vacant(size) = block {
            for _ in 0..*size {
                print!(".");
            }
        }
    }
    //println!();
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
                visited: false,
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
    visited: bool,
}

#[derive(Debug, Clone, Copy)]
enum Block {
    Vacant(usize),
    Occupied(Chunk),
}
