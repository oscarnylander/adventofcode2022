use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn generate(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u32).collect())
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> usize {
    eprintln!("size is {}", input.len() * input[0].len());
    let mut visible = HashSet::<(usize, usize)>::default();

    let mut column_highs = input[0].iter().map(|x| *x).collect::<Vec<_>>();

    // Top + Left

    for (x, row) in input.iter().enumerate() {
        let mut highest_left = row[0];
        for (y, elem) in row.iter().enumerate() {
            if x == 0 {
                visible.insert((x, y));
            }
            if y == 0 {
                visible.insert((x, y));
            }
            if *elem > highest_left {
                highest_left = *elem;
                visible.insert((x, y));
            }
            if *elem > column_highs[y] {
                column_highs[y] = *elem;
                visible.insert((x, y));
            }
        }
    }

    let mut column_highs = input.last().unwrap().iter().map(|x| *x).collect::<Vec<_>>();

    // Bottom + Right

    for (x, row) in input.iter().enumerate().rev() {
        let mut highest_right = *row.last().unwrap();
        for (y, elem) in row.iter().enumerate().rev() {
            if x == row.len() - 1 {
                visible.insert((x, y));
            }
            if y == input.len() - 1 {
                visible.insert((x, y));
            }
            if *elem > highest_right {
                highest_right = *elem;
                visible.insert((x, y));
            }
            if *elem > column_highs[y] {
                column_highs[y] = *elem;
                visible.insert((x, y));
            }
        }
    }

    visible.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test1() {
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        let actual = generate(EXAMPLE);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test2() {
        let expected = 21;
        let actual = solve_part1(&generate(EXAMPLE));

        assert_eq!(expected, actual)
    }
}
