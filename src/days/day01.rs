use anyhow::Result;

use crate::input_tests;

fn part1(input: &str) -> Result<u32> {
    let packs = input.split("\n\n");

    let mut max = 0;
    for pack in packs {
        let items = pack.split("\n");
        let total = items
            .map(|item| u32::from_str_radix(item, 10).unwrap())
            .sum();

        if total > max {
            max = total;
        }
    }

    Ok(max)
}

fn part2(input: &str) -> Result<u32> {
    let packs = input.split("\n\n");

    let mut values = Vec::<u32>::new();
    for pack in packs {
        let items = pack.split("\n");
        let total = items
            .map(|item| u32::from_str_radix(item, 10).unwrap())
            .sum();

        values.push(total);
    }

    values.sort();
    values.reverse();

    Ok(values[0] + values[1] + values[2])
}

input_tests!(1, part1, part2);

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 24000);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, 45000);

        Ok(())
    }
}
