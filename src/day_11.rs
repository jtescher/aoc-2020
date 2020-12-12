pub fn part_one(input: &str) -> anyhow::Result<usize> {
    let mut grid = build_grid(input)?;

    while let Some(next) = part_one_step(&grid) {
        grid = next;
    }

    Ok(grid
        .iter()
        .map(|row| {
            row.iter()
                .filter(|col| matches!(col, Tile::Occupied))
                .count()
        })
        .sum())
}

pub fn part_two(input: &str) -> anyhow::Result<usize> {
    let mut grid = build_grid(input)?;

    while let Some(next) = part_two_step(&grid) {
        grid = next;
    }

    Ok(grid
        .iter()
        .map(|row| {
            row.iter()
                .filter(|col| matches!(col, Tile::Occupied))
                .count()
        })
        .sum())
}

fn build_grid(input: &str) -> anyhow::Result<Vec<Vec<Tile>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'L' => Ok(Tile::Empty),
                    '#' => Ok(Tile::Occupied),
                    '.' => Ok(Tile::Floor),
                    c => anyhow::bail!("expected tile, got: {}", c),
                })
                .collect()
        })
        .collect()
}

fn part_one_step(previous: &Vec<Vec<Tile>>) -> Option<Vec<Vec<Tile>>> {
    let mut changed = false;
    let next = previous
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, tile)| match tile {
                    Tile::Empty => {
                        if adjacent_occupied_count(previous, row_idx, col_idx) == 0 {
                            changed = true;
                            Tile::Occupied
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Occupied => {
                        if adjacent_occupied_count(previous, row_idx, col_idx) >= 4 {
                            changed = true;
                            Tile::Empty
                        } else {
                            Tile::Occupied
                        }
                    }
                    Tile::Floor => Tile::Floor,
                })
                .collect()
        })
        .collect();

    if changed {
        Some(next)
    } else {
        None
    }
}

fn part_two_step(previous: &Vec<Vec<Tile>>) -> Option<Vec<Vec<Tile>>> {
    let mut changed = false;
    let next = previous
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, tile)| match tile {
                    Tile::Empty => {
                        if seen_occupied_count(previous, row_idx, col_idx) == 0 {
                            changed = true;
                            Tile::Occupied
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Occupied => {
                        if seen_occupied_count(previous, row_idx, col_idx) >= 5 {
                            changed = true;
                            Tile::Empty
                        } else {
                            Tile::Occupied
                        }
                    }
                    Tile::Floor => Tile::Floor,
                })
                .collect()
        })
        .collect();

    if changed {
        Some(next)
    } else {
        None
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn adjacent_occupied_count(grid: &Vec<Vec<Tile>>, row_idx: usize, col_idx: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter(|(dx, dy)| {
            let row_idx = if dx.is_negative() {
                row_idx.checked_sub(dx.abs() as usize).unwrap_or(usize::MAX)
            } else {
                row_idx + (*dx as usize)
            };
            let col_idx = if dy.is_negative() {
                col_idx.checked_sub(dy.abs() as usize).unwrap_or(usize::MAX)
            } else {
                col_idx + (*dy as usize)
            };
            grid.get(row_idx)
                .and_then(|row| row.get(col_idx).map(|tile| matches!(tile, Tile::Occupied)))
                .unwrap_or(false)
        })
        .count()
}

fn look_dir(row_idx: isize, col_idx: isize, dx: isize, dy: isize, grid: &Vec<Vec<Tile>>) -> usize {
    if !(0..grid.len() as isize).contains(&row_idx)
        || !(0..grid[0].len() as isize).contains(&col_idx)
    {
        return 0;
    }

    match grid[row_idx as usize][col_idx as usize] {
        Tile::Floor => look_dir(row_idx + dx, col_idx + dy, dx, dy, grid),
        Tile::Occupied => 1,
        Tile::Empty => 0,
    }
}

fn seen_occupied_count(grid: &Vec<Vec<Tile>>, row_idx: usize, col_idx: usize) -> usize {
    DIRECTIONS
        .iter()
        .map(|&(dx, dy)| look_dir(row_idx as isize + dx, col_idx as isize + dy, dx, dy, grid))
        .sum()
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Occupied,
    Floor,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(part_one(input).unwrap(), 37);
        assert_eq!(part_two(input).unwrap(), 26);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../data/day_11.txt");

        assert_eq!(part_one(input).unwrap(), 2368);
        assert_eq!(part_two(input).unwrap(), 2124);
    }
}
