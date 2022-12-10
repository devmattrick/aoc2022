use std::fmt::Display;

use anyhow::{bail, Context, Result};

enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    cycle: usize,
    register: i32,

    // List of instructions
    instructions: Vec<Instruction>,

    // The index of current instruction
    current: usize,

    // Counter keeps track of which "stage" of a multicycle instruction we're on
    counter: u8,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Cpu {
            cycle: 1,
            register: 1,
            instructions,
            current: 0,
            counter: 0,
        }
    }

    fn cycle(&mut self) -> bool {
        let instruction = self.instructions.get(self.current);

        if instruction.is_none() {
            return false;
        }

        self.cycle += 1;

        let instruction = instruction.unwrap();

        match instruction {
            Instruction::Noop => {
                self.current += 1;
            }
            Instruction::Addx(v) => {
                if self.counter == 0 {
                    self.counter = 2;
                }

                if self.counter == 1 {
                    self.register += v;
                    self.current += 1;
                }

                self.counter -= 1;
            }
        }

        true
    }
}

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

struct Screen {
    pixels: [bool; SCREEN_SIZE],
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: [false; SCREEN_SIZE],
        }
    }

    fn index(x: usize, y: usize) -> usize {
        x * SCREEN_HEIGHT + y
    }

    fn set(&mut self, x: usize, y: usize, on: bool) {
        self.pixels[Self::index(x, y)] = on
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[Self::index(x, y)]
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if self.get(x, y) {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }

            output.push('\n');
        }

        f.write_fmt(format_args!("{}", output))
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>> {
    Ok(input
        .split("\n")
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();

            let command = parts.get(0).context("Missing required command part")?;
            let arg = parts.get(1);

            match *command {
                "noop" => Ok(Instruction::Noop),
                "addx" => {
                    let v = arg
                        .map_or(None, |s| i32::from_str_radix(s, 10).ok())
                        .context("Missing or invalid value argument")?;

                    Ok(Instruction::Addx(v))
                }
                _ => bail!("unknown instruction: {}", command),
            }
        })
        .collect::<Result<Vec<Instruction>>>()?)
}

fn part1(input: &str) -> Result<i32> {
    let instructions = parse_instructions(input)?;

    let mut cpu = Cpu::new(instructions);

    let mut result = 0;

    while cpu.cycle() {
        if (cpu.cycle + 20) % 40 == 0 {
            let signal = cpu.register * cpu.cycle as i32;

            result += signal;
        }
    }

    Ok(result)
}

fn part2(input: &str) -> Result<String> {
    let instructions = parse_instructions(input)?;

    let mut cpu = Cpu::new(instructions);
    let mut screen = Screen::new();

    loop {
        let ray_x = (cpu.cycle - 1) % 40;
        let ray_y = (cpu.cycle - 1) / 40;

        let spirte_pos = (cpu.register - 1)..=(cpu.register + 1);

        if spirte_pos.contains(&(ray_x as i32)) {
            screen.set(ray_x, ray_y, true);
        }

        println!("{}", screen);
        println!("");

        if !cpu.cycle() {
            break;
        }
    }

    Ok(format!("{}", screen).trim().to_owned())
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 13140);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let expected = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "}
        .trim();

        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, expected);

        Ok(())
    }
}
