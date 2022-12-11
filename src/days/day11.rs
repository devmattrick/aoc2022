use std::collections::VecDeque;

use anyhow::{Context, Result};

peg::parser! {
    grammar monkey_parser() for str {
        pub rule monkey_list() -> Vec<Monkey>
            = l:(monkey() ** "\n\n") { l }

        rule monkey() -> Monkey
            = "Monkey " n:number() ":\n" items:item_list() "\n" op:operation() "\n" test:test() {
                Monkey::new(items, op, test)
            }

        rule number() -> u128
            = n:$(['0'..='9']+) { ? n.parse().or(Err("Failed to parse item"))}

        rule item_list() -> Vec<u128>
            = "Starting items: " l:(number() ** ", ") { l }

        rule operation() -> Operation
            = "Operation: " op:(operation_square() / operation_arith()) { op }

        rule operation_square() -> Operation
             = "new = old * old" { Operation::Square }

        rule operation_arith() -> Operation
             = "new = old " op:$("+" / "*") " " val:number() {
                ? match op {
                    "*" => Ok(Operation::Multiply(val)),
                    "+" => Ok(Operation::Add(val)),
                    _ => Err("Invalid operation")
                }
             }

        rule test() -> Test
             = "Test: " div:divisible() "\n    " tr:if_true() "\n    " fa:if_false() {
                Test {
                    div_by: div,
                    if_true: tr,
                    if_false: fa
                }
             }

        rule divisible() -> u128
             = "divisible by " n:number() { n }

        rule if_true() -> usize
             = "If true: throw to monkey " n:number() { ? n.try_into().or(Err("Failed to convert")) }

        rule if_false() -> usize
             = "If false: throw to monkey " n:number() { ? n.try_into().or(Err("Failed to convert")) }
    }
}

#[derive(Debug)]
enum Operation {
    Add(u128),
    Multiply(u128),
    Square,
}

#[derive(Debug)]
struct Test {
    div_by: u128,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    test: Test,

    inspects: usize,
}

impl Monkey {
    fn new(items: Vec<u128>, operation: Operation, test: Test) -> Self {
        Self {
            items: VecDeque::from(items),
            operation,
            test,
            inspects: 0,
        }
    }

    fn inspect(&self) -> Vec<u128> {
        self.items
            .iter()
            .map(|item| match self.operation {
                Operation::Add(n) => item + n,
                Operation::Multiply(n) => item * n,
                Operation::Square => item * item,
            })
            .collect::<Vec<u128>>()
    }
}

// A group of monkeys are called a barrel :)
struct Barrel {
    monkeys: Vec<Monkey>,
    worrying: bool,
    product: u128,
}

impl Barrel {
    fn new(monkeys: Vec<Monkey>, worrying: bool) -> Self {
        // I should probably be using LCM here but I'm lazy
        let product = monkeys.iter().map(|m| m.test.div_by).product();

        Self {
            monkeys,
            worrying,
            product,
        }
    }

    fn round(&mut self) -> Result<()> {
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get(i).context("Unable to get monkey")?;

            let items = monkey.inspect();

            let div_by = monkey.test.div_by;
            let if_true = monkey.test.if_true;
            let if_false = monkey.test.if_false;

            for item in &items {
                let item = if self.worrying {
                    item % self.product
                } else {
                    item / 3
                };

                let to = if item % div_by == 0 {
                    if_true
                } else {
                    if_false
                };

                self.monkeys
                    .get_mut(to)
                    .context("Invalid monkey id")?
                    .items
                    .push_back(item);
            }

            let monkey = self.monkeys.get_mut(i).context("Failed to get monkey")?;
            monkey.items = VecDeque::new();
            monkey.inspects += items.len();
        }

        Ok(())
    }
}

fn part1(input: &str) -> Result<usize> {
    let monkeys = monkey_parser::monkey_list(input)?;

    let mut barrel = Barrel::new(monkeys, false);

    for i in 0..20 {
        barrel.round();
    }

    let mut inspects = barrel
        .monkeys
        .iter()
        .map(|monkey| monkey.inspects)
        .collect::<Vec<usize>>();
    inspects.sort();

    let total = inspects[inspects.len() - 2] * inspects[inspects.len() - 1];

    Ok(total)
}

fn part2(input: &str) -> Result<usize> {
    let monkeys = monkey_parser::monkey_list(input)?;

    let mut barrel = Barrel::new(monkeys, true);

    for i in 0..10000 {
        barrel.round()?;

        let round = i + 1;

        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("== After round {} ==", round);

            for (j, m) in barrel.monkeys.iter().enumerate() {
                println!("Monkey {} inspected items {} times", j, m.inspects);
            }
        }
    }

    let mut inspects = barrel
        .monkeys
        .iter()
        .map(|monkey| monkey.inspects)
        .collect::<Vec<usize>>();
    inspects.sort();

    let total = inspects[inspects.len() - 2] * inspects[inspects.len() - 1];

    Ok(total)
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        Monkey 0:
        Starting items: 61
        Operation: new = old * 11
        Test: divisible by 5
            If true: throw to monkey 7
            If false: throw to monkey 4

        Monkey 1:
        Starting items: 76, 92, 53, 93, 79, 86, 81
        Operation: new = old + 4
        Test: divisible by 2
            If true: throw to monkey 2
            If false: throw to monkey 6

        Monkey 2:
        Starting items: 91, 99
        Operation: new = old * 19
        Test: divisible by 13
            If true: throw to monkey 5
            If false: throw to monkey 0

        Monkey 3:
        Starting items: 58, 67, 66
        Operation: new = old * old
        Test: divisible by 7
            If true: throw to monkey 6
            If false: throw to monkey 1

        Monkey 4:
        Starting items: 94, 54, 62, 73
        Operation: new = old + 1
        Test: divisible by 19
            If true: throw to monkey 3
            If false: throw to monkey 7

        Monkey 5:
        Starting items: 59, 95, 51, 58, 58
        Operation: new = old + 3
        Test: divisible by 11
            If true: throw to monkey 0
            If false: throw to monkey 4

        Monkey 6:
        Starting items: 87, 69, 92, 56, 91, 93, 88, 73
        Operation: new = old + 8
        Test: divisible by 3
            If true: throw to monkey 5
            If false: throw to monkey 2

        Monkey 7:
        Starting items: 71, 57, 86, 67, 96, 95
        Operation: new = old + 7
        Test: divisible by 17
            If true: throw to monkey 3
            If false: throw to monkey 1
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 10605);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, 2713310158);

        Ok(())
    }
}
