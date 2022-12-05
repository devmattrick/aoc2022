use anyhow::Result;

fn item_value(item: char) -> i32 {
    // Convert to an uppercase char here so that a and A = 1
    let mut c_value = (item.to_ascii_uppercase() as i32) - 64;

    // If it's an uppercase char, we then add 24
    if item.is_ascii_uppercase() {
        c_value += 26;
    }

    c_value
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .split("\n")
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, second)| {
            for c in second.chars() {
                if first.contains(c) {
                    return item_value(c);
                }
            }

            return 0;
        })
        .sum())
}

fn part2(input: &str) -> Result<i32> {
    Ok(input
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    return item_value(c);
                }
            }
            return 0;
        })
        .sum())
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 157);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, 70);

        Ok(())
    }
}
