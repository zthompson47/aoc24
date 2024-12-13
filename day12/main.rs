use std::collections::HashMap;

fn main() {
    let (part1, part2) = run();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn run() -> (usize, usize) {
    let mut garden: Vec<Vec<Plot>> = include_str!("input.txt")
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, garden)| {
            let new_row = garden
                .iter()
                .enumerate()
                .map(|(column, x)| Plot {
                    label: *x,
                    row,
                    column,
                    ..Default::default()
                })
                .collect::<Vec<_>>();
            acc.push(new_row);
            acc
        });

    let mut group = 0;
    for row in 0..garden.len() {
        for column in 0..garden[0].len() {
            let neighbors = neighbors(row, column, &garden);

            if garden[row][column].group.is_none() {
                fill_group(row, column, &mut garden, group);
                group += 1;
            }

            let mut fences = 4 - neighbors.len();
            for neighbor in neighbors {
                if neighbor.label == garden[row][column].label {
                } else {
                    fences += 1;
                }
            }
            garden[row][column].fences = fences;
            garden[row][column].corners = corners(row, column, &garden);
        }
    }

    let groups = garden.iter().flatten().fold(
        HashMap::<usize, (usize, usize, usize)>::new(),
        |mut acc, plot| {
            acc.entry(plot.group.unwrap())
                .and_modify(|x| {
                    x.0 += 1;
                    x.1 += plot.fences;
                    x.2 += plot.corners;
                })
                .or_insert((1, plot.fences, plot.corners));
            acc
        },
    );

    let part1 = groups
        .values()
        .map(|(plots, fences, _)| plots * fences)
        .sum();
    let part2 = groups
        .values()
        .map(|(plots, _, corners)| plots * corners)
        .sum();

    (part1, part2)
}

fn corners(row: usize, column: usize, garden: &[Vec<Plot>]) -> usize {
    let plot = garden[row][column];
    let north = if row > 0 {
        garden[row - 1][column].label == plot.label
    } else {
        false
    };
    let northeast = if row > 0 && column < garden[0].len() - 1 {
        garden[row - 1][column + 1].label == plot.label
    } else {
        false
    };
    let east = if column < garden[0].len() - 1 {
        garden[row][column + 1].label == plot.label
    } else {
        false
    };
    let southeast = if row < garden.len() - 1 && column < garden[0].len() - 1 {
        garden[row + 1][column + 1].label == plot.label
    } else {
        false
    };
    let south = if row < garden.len() - 1 {
        garden[row + 1][column].label == plot.label
    } else {
        false
    };
    let southwest = if row < garden.len() - 1 && column > 0 {
        garden[row + 1][column - 1].label == plot.label
    } else {
        false
    };
    let west = if column > 0 {
        garden[row][column - 1].label == plot.label
    } else {
        false
    };
    let northwest = if row > 0 && column > 0 {
        garden[row - 1][column - 1].label == plot.label
    } else {
        false
    };
    [
        (west, northwest, north),
        (north, northeast, east),
        (east, southeast, south),
        (south, southwest, west),
    ]
    .iter()
    .map(|corners| match corners {
        (true, true, false)
        | (false, true, true)
        | (false, false, true)
        | (true, false, false)
        | (true, true, true) => 0,
        _ => 1,
    })
    .sum()
}

fn fill_group(row: usize, column: usize, garden: &mut Vec<Vec<Plot>>, group_id: usize) {
    for neighbor in neighbors(row, column, garden) {
        garden[row][column].group = Some(group_id);
        if neighbor.label == garden[row][column].label && neighbor.group.is_none() {
            fill_group(neighbor.row, neighbor.column, garden, group_id);
        }
    }
}

fn neighbors(row: usize, column: usize, garden: &[Vec<Plot>]) -> Vec<Plot> {
    let mut plots = Vec::new();
    if row > 0 {
        plots.push(garden[row - 1][column]);
    }
    if row < garden.len() - 1 {
        plots.push(garden[row + 1][column]);
    }
    if column > 0 {
        plots.push(garden[row][column - 1]);
    }
    if column < garden[0].len() - 1 {
        plots.push(garden[row][column + 1]);
    }
    plots
}

#[derive(Default, Debug, Clone, Copy)]
struct Plot {
    label: char,
    row: usize,
    column: usize,
    fences: usize,
    group: Option<usize>,
    corners: usize,
}
