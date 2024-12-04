fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> u32 {
    let grid = include_str!("test.txt")
        .lines()
        .map(|line| line.bytes().map(Letter::from).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let result = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, letter) in row.iter().enumerate() {
            if *letter == Letter::X {
                println!("Possible: {} {} {}", r, c, letter);
            }
        }
    }

    result
}

#[derive(PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl From<u8> for Letter {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Self::X, // 88
            b'M' => Self::M, // 77
            b'A' => Self::A, // 65
            b'S' => Self::S, // 83
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => 'X',
                Self::M => 'M',
                Self::A => 'A',
                Self::S => 'S',
            }
        )
    }
}
