use std::fmt::Debug;

use anyhow::{Context, Result};

#[derive(Debug, Clone, Copy)]
struct GridItem {
    height: i32,
}

#[derive(Debug)]
struct Grid {
    trees: Vec<Vec<GridItem>>,
}

impl Grid {
    fn new(input: &str) -> Result<Grid> {
        let mut trees = Vec::new();

        let lines = input.split("\n").map(|line| line.chars());

        for line in lines {
            let mut row = Vec::new();

            for char in line {
                let height = i32::from_str_radix(&char.to_string(), 10)?;

                row.push(GridItem { height });
            }

            trees.push(row);
        }

        return Ok(Grid { trees });
    }

    fn get(&self, x: i32, y: i32) -> Option<GridItem> {
        self.trees
            .get(y as usize)
            .map_or(None, |row| row.get(x as usize).copied())
    }

    fn width(&self) -> i32 {
        self.trees.get(0).map_or(0, |row| row.len() as i32)
    }

    fn height(&self) -> i32 {
        self.trees.len() as i32
    }
}

fn check_x_range<R: IntoIterator<Item = i32> + Debug>(
    grid: &Grid,
    range: R,
    y: i32,
    tree_height: i32,
) -> bool {
    for check_x in range {
        let check_height = grid.get(check_x, y).map_or(-1, |tree| tree.height);

        if check_height >= tree_height {
            return false;
        }
    }

    return true;
}

fn check_y_range<R: IntoIterator<Item = i32> + Debug>(
    grid: &Grid,
    range: R,
    x: i32,
    tree_height: i32,
) -> bool {
    for check_y in range {
        let check_height = grid.get(x, check_y).map_or(-1, |tree| tree.height);

        if check_height >= tree_height {
            return false;
        }
    }

    return true;
}

fn check_x_distance<R: IntoIterator<Item = i32> + Debug>(
    grid: &Grid,
    range: R,
    y: i32,
    tree_height: i32,
) -> i32 {
    let mut score = 0;

    for check_x in range {
        let check_height = grid.get(check_x, y).map_or(0, |tree| tree.height);

        if check_height < tree_height {
            score += 1;
        } else {
            if check_x >= 0 && check_x < grid.width() {
                score += 1;
            }
            break;
        }
    }

    return score;
}

fn check_y_distance<R: IntoIterator<Item = i32> + Debug>(
    grid: &Grid,
    range: R,
    x: i32,
    tree_height: i32,
) -> i32 {
    let mut score = 0;

    for check_y in range {
        let check_height = grid.get(x, check_y).map_or(0, |tree| tree.height);

        if check_height < tree_height {
            score += 1;
        } else {
            if check_y >= 0 && check_y < grid.height() {
                score += 1;
            }
            break;
        }
    }

    return score;
}

fn part1(input: &str) -> Result<usize> {
    let grid = Grid::new(input)?;

    let width = grid.width();
    let height = grid.height();

    let mut total = 0;

    for x in 0..width {
        for y in 0..height {
            let item = grid.get(x, y).context("Failed to get grid item.")?;
            let tree_height = item.height;

            let mut visible = false;

            visible = visible || check_x_range(&grid, 0..x, y, tree_height);
            visible = visible || check_x_range(&grid, (x + 1)..width, y, tree_height);

            visible = visible || check_y_range(&grid, 0..y, x, tree_height);
            visible = visible || check_y_range(&grid, (y + 1)..height, x, tree_height);

            if visible {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn part2(input: &str) -> Result<i32> {
    let grid = Grid::new(input)?;

    let width = grid.width();
    let height = grid.height();

    let mut score = 0;
    let mut selected = "".to_string();

    for x in 0..width {
        for y in 0..height {
            let item = grid.get(x, y).context("Failed to get grid item.")?;
            let tree_height = item.height;

            // Left
            let left = check_x_distance(&grid, (0..x).rev(), y, tree_height);

            // Right
            let right = check_x_distance(&grid, (x + 1)..width, y, tree_height);

            // Up
            let up = check_y_distance(&grid, (0..y).rev(), x, tree_height);

            // Down
            let down = check_y_distance(&grid, (y + 1)..height, x, tree_height);

            let total = left * right * up * down;

            if total > score {
                score = total;
                selected = format!("({}, {})", x, y);
            }
        }
    }

    Ok(score)
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 21);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, 8);

        Ok(())
    }
}
