use anyhow::{Context, Result};

type Stack = Vec<char>;

fn parse_header(input: &str) -> Result<Vec<Stack>> {
    // Extract the header from our input
    let header = input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    // Get the columns
    let columns = header
        .last()
        .context("Could not find column base line")?
        .chars()
        .enumerate()
        .filter(|(i, c)| c.is_numeric())
        .map(|(i, c)| {
            Ok((
                i,
                usize::from_str_radix(&c.to_string(), 10)
                    .context("Failed to parse column number")?,
            ))
        })
        .collect::<Result<Vec<(usize, usize)>>>()?;

    let mut stacks = Vec::<Stack>::with_capacity(columns.len());

    let rows = header
        .iter()
        .rev()
        .skip(1)
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for (i, col) in columns {
        let mut stack = Vec::new();

        for row in &rows {
            // Handle if lines that end in spaces get trimmed
            if row.len() <= i {
                continue;
            }

            let c = row[i];

            // Don't add whitespace chars
            if !c.is_alphabetic() {
                continue;
            }

            stack.push(c);
        }

        stacks.push(stack);
    }

    Ok(stacks)
}

#[derive(Debug)]

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>> {
    Ok(input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            let mut part = line.split(" ");
            let count = usize::from_str_radix(part.nth(1).context("Could not get count")?, 10)?;
            let from = usize::from_str_radix(part.nth(1).context("Could not get from")?, 10)? - 1;
            let to = usize::from_str_radix(part.nth(1).context("Could not get tos")?, 10)? - 1;

            Ok(Instruction { count, from, to })
        })
        .collect::<Result<Vec<Instruction>>>()?)
}

fn print_instruction(instruction: &Instruction) {
    println!(
        "move {} from {} to {}",
        instruction.count,
        instruction.from + 1,
        instruction.to + 1
    );
}

fn print_stacks(stacks: &Vec<Stack>) {
    let mut max_height = 0;
    for col in stacks {
        let len = col.len();

        if len > max_height {
            max_height = len;
        }
    }

    for i in 0..stacks.len() {
        print!(" {}  ", i + 1);
    }
    println!("");

    for row in 0..max_height {
        for col in stacks.iter() {
            if col.len() > row {
                print!("[{}] ", col[row]);
            } else {
                print!("    ");
            }
        }
        println!("");
    }
}

fn part1(input: &str) -> Result<String> {
    let mut stacks = parse_header(input)?;
    let instructions = parse_instructions(input)?;

    for instruction in instructions {
        for _ in 0..instruction.count {
            let c = stacks[instruction.from]
                .pop()
                .context("Attempted to move crate from empty stack")?;
            stacks[instruction.to].push(c);
        }
    }

    let top = stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();

    Ok(top)
}

fn part2(input: &str) -> Result<String> {
    let mut stacks = parse_header(input)?;
    let instructions = parse_instructions(input)?;

    for instruction in instructions {
        let end = stacks[instruction.from].len();
        let mut grabbed = stacks[instruction.from].split_off(end - instruction.count);
        stacks[instruction.to].append(&mut grabbed);
    }

    let top = stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();

    Ok(top)
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(INPUT)?;

        assert_eq!(result, "CMZ");

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(INPUT)?;

        assert_eq!(result, "MCD");

        Ok(())
    }
}
