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

#[derive(Debug, Clone)]
struct Chunk {
    index: usize,
    file_id: usize,
    count: usize,
}
