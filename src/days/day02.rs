use anyhow::Result;

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const LOSE_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

#[derive(Clone, Copy)]
enum Move {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

fn map_move(code: &str) -> Move {
    match code {
        "A" | "X" => Move::ROCK,
        "B" | "Y" => Move::PAPER,
        "C" | "Z" => Move::SCISSORS,
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy)]
enum MatchResult {
    WIN = 6,
    DRAW = 3,
    LOSS = 0,
}

fn calculate_result(me: Move, opponent: Move) -> MatchResult {
    match me {
        Move::ROCK => match opponent {
            Move::ROCK => MatchResult::DRAW,
            Move::PAPER => MatchResult::LOSS,
            Move::SCISSORS => MatchResult::WIN,
        },
        Move::PAPER => match opponent {
            Move::ROCK => MatchResult::WIN,
            Move::PAPER => MatchResult::DRAW,
            Move::SCISSORS => MatchResult::LOSS,
        },
        Move::SCISSORS => match opponent {
            Move::ROCK => MatchResult::LOSS,
            Move::PAPER => MatchResult::WIN,
            Move::SCISSORS => MatchResult::DRAW,
        },
    }
}

fn calculate_score(me: Move, result: MatchResult) -> u32 {
    return me as u32 + result as u32;
}

fn part1(input: &str) -> Result<u32> {
    let lines = input.split("\n");

    let score = lines
        .map(|line| {
            let mut plays = line.split(" ");
            let opponent = map_move(plays.next().unwrap());
            let me = map_move(plays.next().unwrap());

            let result = calculate_result(me, opponent);
            calculate_score(me, result)
        })
        .sum();

    Ok(score)
}

fn map_result(code: &str) -> MatchResult {
    match code {
        "X" => MatchResult::LOSS,
        "Y" => MatchResult::DRAW,
        "Z" => MatchResult::WIN,
        _ => unreachable!(),
    }
}

fn calculate_me(opponent: Move, result: MatchResult) -> Move {
    match result {
        MatchResult::WIN => match opponent {
            Move::ROCK => Move::PAPER,
            Move::PAPER => Move::SCISSORS,
            Move::SCISSORS => Move::ROCK,
        },
        MatchResult::DRAW => opponent,
        MatchResult::LOSS => match opponent {
            Move::ROCK => Move::SCISSORS,
            Move::PAPER => Move::ROCK,
            Move::SCISSORS => Move::PAPER,
        },
    }
}

fn part2(input: &str) -> Result<u32> {
    let lines = input.split("\n");

    let score = lines
        .map(|line| {
            let mut plays = line.split(" ");
            let opponent = map_move(plays.next().unwrap());
            let result = map_result(plays.next().unwrap());
            let me = calculate_me(opponent, result);

            calculate_score(me, result)
        })
        .sum();

    Ok(score)
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 15);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, 12);

        Ok(())
    }
}
