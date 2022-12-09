use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::{Context, Result};
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    dir: Direction,
    amount: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    fn move_dir(&mut self, dir: Direction) -> Self {
        let mut x = 0;
        let mut y = 0;

        match dir {
            Direction::UP => y += 1,
            Direction::DOWN => y -= 1,
            Direction::LEFT => x -= 1,
            Direction::RIGHT => x += 1,
        }

        return Point::new(self.x + x, self.y + y);
    }

    fn add(&self, other: &Self) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

// All the positions that would cause a tail movement:
//
// (-2,  2) (-1,  2) ( 0,  2) ( 1,  2) ( 2,  2)
// (-2,  1)                            ( 2,  1)
// (-2,  0)                            ( 2,  0)
// (-2, -1)                            ( 2, -1)
// (-2, -2) (-1, -2) ( 0, -2) ( 1, -2) ( 2, -2)

lazy_static! {
    static ref MOVE_MAP: HashMap<Point, Point> = HashMap::from([
        // Up Moves
        (Point::new(0, 2), Point::new(0, 1)),

        // Down Moves
        (Point::new(0, -2), Point::new(0, -1)),

        // Left Moves
        (Point::new(-2, 0), Point::new(-1, 0)),

        // Right Moves
        (Point::new(2, 0), Point::new(1, 0)),

        // Up-Left Diagonal Moves
        (Point::new(-2, 2), Point::new(-1, 1)),
        (Point::new(-1, 2), Point::new(-1, 1)),
        (Point::new(-2, 1), Point::new(-1, 1)),

        // Up-Right Diagonal Moves
        (Point::new(1, 2), Point::new(1, 1)),
        (Point::new(2, 2), Point::new(1, 1)),
        (Point::new(2, 1), Point::new(1, 1)),

        // Down-Left Diagonal Moves
        (Point::new(-2, -1), Point::new(-1, -1)),
        (Point::new(-2, -2), Point::new(-1, -1)),
        (Point::new(-1, -2), Point::new(-1, -1)),

        // Down-Right Diagonal Moves
        (Point::new(2, -1), Point::new(1, -1)),
        (Point::new(1, -2), Point::new(1, -1)),
        (Point::new(2, -2), Point::new(1, -1)),
    ]);
}

#[derive(Debug)]
struct Grid {
    parts: Vec<Point>,

    tail_pos: HashSet<Point>,
}

impl Grid {
    fn new(tails: usize) -> Grid {
        Grid {
            parts: vec![Point::new(0, 0); tails + 1],
            tail_pos: HashSet::from([Point::new(0, 0)]),
        }
    }

    fn move_head(&mut self, dir: Direction) {
        self.parts[0] = self.parts[0].move_dir(dir);
    }

    fn move_tails(&mut self) {
        for i in 1..self.parts.len() {
            let part = self.parts[i];

            for check_add in MOVE_MAP.keys() {
                let point = part.add(check_add);

                if point == self.parts[i - 1] {
                    let new_add = MOVE_MAP.get(check_add).unwrap();
                    let new_tail = part.add(new_add);

                    self.parts[i] = new_tail;

                    if i == self.parts.len() - 1 {
                        self.tail_pos.insert(new_tail);
                    }

                    break;
                }
            }
        }
    }
}

fn part1(input: &str) -> Result<i32> {
    let movements = input
        .split("\n")
        .map(|line| {
            let chars = line.split(" ").collect::<Vec<&str>>();

            let dir = chars
                .get(0)
                .map_or(None, |dir| match *dir {
                    "U" => Some(Direction::UP),
                    "D" => Some(Direction::DOWN),
                    "L" => Some(Direction::LEFT),
                    "R" => Some(Direction::RIGHT),
                    _ => None,
                })
                .context("Invalid or missing direction")?;
            let amount = chars
                .get(1)
                .map_or(None, |amount| i32::from_str_radix(amount, 10).ok())
                .context("Missing or invalid amount")?;

            Ok(Movement { dir, amount })
        })
        .collect::<Result<Vec<Movement>>>()?;

    let mut grid = Grid::new(1);

    for m in movements {
        for i in 0..m.amount {
            grid.move_head(m.dir);
            grid.move_tails();
        }
    }

    Ok(grid.tail_pos.len() as i32)
}

fn part2(input: &str) -> Result<i32> {
    let movements = input
        .split("\n")
        .map(|line| {
            let chars = line.split(" ").collect::<Vec<&str>>();

            let dir = chars
                .get(0)
                .map_or(None, |dir| match *dir {
                    "U" => Some(Direction::UP),
                    "D" => Some(Direction::DOWN),
                    "L" => Some(Direction::LEFT),
                    "R" => Some(Direction::RIGHT),
                    _ => None,
                })
                .context("Invalid or missing direction")?;
            let amount = chars
                .get(1)
                .map_or(None, |amount| i32::from_str_radix(amount, 10).ok())
                .context("Missing or invalid amount")?;

            Ok(Movement { dir, amount })
        })
        .collect::<Result<Vec<Movement>>>()?;

    let mut grid = Grid::new(9);

    for m in movements {
        for i in 0..m.amount {
            grid.move_head(m.dir);
            grid.move_tails();
        }
    }

    Ok(grid.tail_pos.len() as i32)
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 13);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, 1);

        Ok(())
    }
}
