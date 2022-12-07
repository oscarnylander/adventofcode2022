use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default)]
struct TreeNode {
    size: u32,
    children: HashMap<String, TreeNode>,
}

#[derive(PartialEq, Debug)]
pub enum InputLine {
    Command(Command),
    LsOutput(LsOutput),
}

#[derive(PartialEq, Debug)]
pub enum Command {
    CD(String),
    LS,
}

#[derive(PartialEq, Debug)]
pub enum LsOutput {
    Directory(String),
    File(u32, String),
}

#[aoc_generator(day7)]
pub fn generate(input: &str) -> Vec<InputLine> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(' ').peekable();
            match split.peek().unwrap() {
                &"$" => {
                    split.next().unwrap();
                    match split.peek().unwrap() {
                        &"cd" => {
                            split.next().unwrap();
                            InputLine::Command(Command::CD(split.peek().unwrap().to_string()))
                        }
                        &"ls" => InputLine::Command(Command::LS),
                        _ => unreachable!(),
                    }
                }
                &"dir" => {
                    split.next().unwrap();
                    InputLine::LsOutput(LsOutput::Directory(split.peek().unwrap().to_string()))
                }
                _ => {
                    let size = split.next().unwrap().parse::<u32>().unwrap();
                    InputLine::LsOutput(LsOutput::File(size, split.next().unwrap().to_string()))
                }
            }
        })
        .collect()
}

fn dfs_1(node: &TreeNode) -> (u32, u32) {
    if node.children.is_empty() {
        return (node.size, 0);
    }
    let mut child_size_under_threshold = 0;
    let mut size = 0;
    for child in node.children.values() {
        let (child_size, child_children_under_threshold) = dfs_1(child);
        child_size_under_threshold += child_children_under_threshold;
        size += child_size;
    }
    (
        size,
        if size <= 100_000 {
            child_size_under_threshold + size
        } else {
            child_size_under_threshold
        },
    )
}

fn dfs_2(current_free_size: u32, node: &TreeNode) -> (u32, u32) {
    if node.children.is_empty() {
        return (node.size, u32::MAX);
    }
    let mut size = 0;
    let mut smallest_removal_size = u32::MAX;
    for child in node.children.values() {
        let (child_size, smallest_child_removal_size) = dfs_2(current_free_size, child);
        size += child_size;
        smallest_removal_size = std::cmp::min(smallest_removal_size, smallest_child_removal_size);
    }
    (
        size,
        if current_free_size + size >= 30000000 {
            std::cmp::min(smallest_removal_size, size)
        } else {
            smallest_removal_size
        },
    )
}

fn parse_subtree(input_stream: &mut dyn Iterator<Item = &InputLine>, root: &mut TreeNode) {
    loop {
        let next = input_stream.next();
        if next.is_none() {
            return;
        }
        match next.unwrap() {
            InputLine::Command(command) => match command {
                Command::CD(path) => match path.as_str() {
                    ".." => return,
                    _ => {
                        let subtree = root.children.get_mut(path.as_str()).unwrap();
                        parse_subtree(input_stream, subtree);
                    }
                },
                Command::LS => {}
            },
            InputLine::LsOutput(ls_output) => match ls_output {
                LsOutput::Directory(name) => {
                    let new_node = TreeNode::default();
                    root.children.insert(name.clone(), new_node);
                }
                LsOutput::File(size, name) => {
                    let mut new_node = TreeNode::default();
                    new_node.size = *size;
                    root.children.insert(name.clone(), new_node);
                }
            },
        }
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[InputLine]) -> u32 {
    let mut root = TreeNode::default();
    parse_subtree(&mut input.iter().skip(1), &mut root);
    let (_, ret) = dfs_1(&root);
    ret
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[InputLine]) -> u32 {
    let mut root = TreeNode::default();
    parse_subtree(&mut input.iter().skip(1), &mut root);
    let (total_size, _) = dfs_1(&root);
    let (_, ret) = dfs_2(70000000 - total_size, &root);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "$ cd /
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
7214296 k";

    #[test]
    fn test1() {
        let expected = vec![
            InputLine::Command(Command::CD("/".to_string())),
            InputLine::Command(Command::LS),
            InputLine::LsOutput(LsOutput::Directory("a".to_string())),
            InputLine::LsOutput(LsOutput::File(14848514, "b.txt".to_string())),
            InputLine::LsOutput(LsOutput::File(8504156, "c.dat".to_string())),
            InputLine::LsOutput(LsOutput::Directory("d".to_string())),
            InputLine::Command(Command::CD("a".to_string())),
            InputLine::Command(Command::LS),
            InputLine::LsOutput(LsOutput::Directory("e".to_string())),
            InputLine::LsOutput(LsOutput::File(29116, "f".to_string())),
            InputLine::LsOutput(LsOutput::File(2557, "g".to_string())),
            InputLine::LsOutput(LsOutput::File(62596, "h.lst".to_string())),
            InputLine::Command(Command::CD("e".to_string())),
            InputLine::Command(Command::LS),
            InputLine::LsOutput(LsOutput::File(584, "i".to_string())),
            InputLine::Command(Command::CD("..".to_string())),
            InputLine::Command(Command::CD("..".to_string())),
            InputLine::Command(Command::CD("d".to_string())),
            InputLine::Command(Command::LS),
            InputLine::LsOutput(LsOutput::File(4060174, "j".to_string())),
            InputLine::LsOutput(LsOutput::File(8033020, "d.log".to_string())),
            InputLine::LsOutput(LsOutput::File(5626152, "d.ext".to_string())),
            InputLine::LsOutput(LsOutput::File(7214296, "k".to_string())),
        ];

        let actual = generate(EXAMPLE);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 95437;
        let actual = solve_part1(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }

    #[test]
    fn test3() {
        let expected = 24933642;
        let actual = solve_part2(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }
}
