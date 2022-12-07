use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Context, Result};

#[derive(Debug)]
enum Command {
    Ls(Vec<LsEntry>),
    Cd(String),
}

#[derive(Debug)]
enum LsEntry {
    File(String, usize),
    Dir(String),
}

fn parse_commands(input: &str) -> Result<Vec<Command>> {
    Ok(input
        .split("$ ")
        .filter(|exec| !exec.is_empty())
        .map(|exec| {
            println!("{}", exec);

            let (command, response) = exec.split_once("\n").context("Failed to parse command.")?;

            let args = command.split(" ").collect::<Vec<&str>>();

            match args[0] {
                "ls" => {
                    let entries = response
                        .split("\n")
                        .filter(|entry| !entry.is_empty())
                        .map(|entry| entry.split(" ").collect::<Vec<&str>>())
                        .map(|entry| {
                            let base = entry.get(0).context("Missing ls entry base")?.to_owned();
                            let name = entry.get(1).context("Missing ls entry name")?.to_owned();

                            match base {
                                "dir" => Ok(LsEntry::Dir(name.to_string())),
                                _ => Ok(LsEntry::File(
                                    name.to_string(),
                                    usize::from_str_radix(base, 10)?,
                                )),
                            }
                        })
                        .collect::<Result<Vec<LsEntry>>>()?;

                    Ok(Command::Ls(entries))
                }
                "cd" => {
                    let dir = args.get(1).context("Missing required arg for cd command")?;

                    Ok(Command::Cd(dir.to_string()))
                }
                _ => bail!("Unknown command: {}", args[0]),
            }
        })
        .collect::<Result<Vec<Command>>>()?)
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct FileTree {
    files: Vec<File>,
    directories: HashMap<String, Box<FileTree>>,
}

impl FileTree {
    fn new() -> FileTree {
        FileTree {
            files: Vec::new(),
            directories: HashMap::new(),
        }
    }

    fn size(&self) -> usize {
        let file_size: usize = self.files.iter().map(|file| file.size).sum();
        let dir_size: usize = self.directories.iter().map(|(_, tree)| tree.size()).sum();
        let total_size = file_size + dir_size;

        return file_size + dir_size;
    }
}

fn build_file_tree(commands: Vec<Command>) -> FileTree {
    let mut tree = FileTree::new();

    let mut pwd: Vec<String> = Vec::new();
    for cmd in commands {
        match cmd {
            Command::Cd(path) => match path.as_str() {
                ".." => {
                    pwd.pop();
                }
                "/" => {
                    pwd.clear();
                }
                _ => {
                    pwd.push(path.to_string());
                }
            },
            Command::Ls(entries) => {
                let mut curr = &mut tree;

                for part in &pwd {
                    let node = curr
                        .directories
                        .entry(part.to_string())
                        .or_insert_with(|| Box::new(FileTree::new()));

                    curr = node.as_mut();
                }

                entries.iter().for_each(|entry| match entry {
                    LsEntry::Dir(name) => {
                        curr.directories
                            .insert(name.to_string(), Box::new(FileTree::new()));
                    }
                    LsEntry::File(name, size) => curr.files.push(File {
                        name: name.to_string(),
                        size: *size,
                    }),
                })
            }
        }
    }

    tree
}

fn part1(input: &str) -> Result<usize> {
    let commands = parse_commands(input)?;
    let tree = build_file_tree(commands);

    let mut total = 0;
    let mut queue = VecDeque::new();
    queue.push_back(&tree);

    while let Some(node) = queue.pop_front() {
        for dir in node.directories.values() {
            queue.push_back(&*dir);
        }

        let size = node.size();
        if size <= 100_000 {
            total += size;
        }
    }

    Ok(total)
}

const DISK_SIZE: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;
const REQ_SIZE: usize = DISK_SIZE - UPDATE_SIZE;

fn part2(input: &str) -> Result<usize> {
    let commands = parse_commands(input)?;
    let tree = build_file_tree(commands);

    let curr_size = tree.size();
    let min_size_remove = curr_size - REQ_SIZE;

    let mut queue = VecDeque::new();
    queue.push_back(&tree);

    let mut curr_remove_size = DISK_SIZE;

    while let Some(node) = queue.pop_front() {
        for dir in node.directories.values() {
            queue.push_back(&*dir);
        }

        let size = node.size();
        if size >= min_size_remove && size < curr_remove_size {
            curr_remove_size = size;
        }
    }

    Ok(curr_remove_size)
}

#[cfg(test)]
mod tests_example {
    use anyhow::Result;
    use indoc::indoc;

    use crate::util;

    const INPUT: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn test_part1() -> Result<()> {
        let result = super::part1(util::format_input(INPUT))?;

        assert_eq!(result, 95437);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let result = super::part2(util::format_input(INPUT))?;

        assert_eq!(result, 24933642);

        Ok(())
    }
}
