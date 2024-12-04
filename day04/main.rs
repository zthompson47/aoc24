fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.bytes().map(Letter::from).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut result = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            result += grid.solutions_from(r, c);
        }
    }

    result
}

fn part2() -> u32 {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.bytes().map(Letter::from).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let mut result = 0;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid.is_xmas_cross(r, c) {
                result += 1;
            }
        }
    }

    result
}

trait Grid {
    fn solutions_from(&self, r: usize, c: usize) -> u32;
    fn is_xmas_cross(&self, r: usize, c: usize) -> bool;
}

impl Grid for Vec<Vec<Letter>> {
    fn is_xmas_cross(&self, r: usize, c: usize) -> bool {
        // No room to make cross with center Letter::A on edges.
        if r < 1 || r > self.len() - 2 || c < 1 || c > self.len() - 2 {
            return false;
        }

        if self[r][c] == Letter::A {
            let top_left = &self[r - 1][c - 1];
            let top_right = &self[r - 1][c + 1];
            let bottom_left = &self[r + 1][c - 1];
            let bottom_right = &self[r + 1][c + 1];
            if ((*top_left == Letter::M && *bottom_right == Letter::S)
                || (*top_left == Letter::S && *bottom_right == Letter::M))
                && ((*top_right == Letter::M && *bottom_left == Letter::S)
                    || (*top_right == Letter::S && *bottom_left == Letter::M))
            {
                return true;
            }
        }

        false
    }

    fn solutions_from(&self, r: usize, c: usize) -> u32 {
        if self[r][c] != Letter::X {
            return 0;
        }

        let mut result = 0;
        for direction in DIRECTIONS {
            // Check that there is room for the word and it won't go off-grid.
            let r_end = r as i32 + direction.0 * 3;
            let c_end = c as i32 + direction.1 * 3;
            let r_len = self.len() as i32;
            let c_len = self[0].len() as i32;

            if (0..r_len).contains(&r_end)
                && (0..c_len).contains(&c_end)
                && self[(r as i32 + direction.0) as usize][(c as i32 + direction.1) as usize]
                    == Letter::M
                && self[(r as i32 + direction.0 * 2) as usize]
                    [(c as i32 + direction.1 * 2) as usize]
                    == Letter::A
                && self[(r as i32 + direction.0 * 3) as usize]
                    [(c as i32 + direction.1 * 3) as usize]
                    == Letter::S
            {
                result += 1;
            }
        }

        result
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

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
