use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::Matrix;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let m = input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut i = 0;
    for row in m {
        if row
            .iter()
            .tuple_windows()
            .map(|(a, b)| (*a as i32) - (*b as i32))
            .tuple_windows()
            .all(|(a, b)| a.is_negative() == b.is_negative())
            && row
                .iter()
                .tuple_windows()
                .map(|(a, b)| a.abs_diff(*b))
                .all(|v| v <= 3 && v >= 1)
        {
            i += 1;
        }
    }
    Some(i)
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut i = 0;
    for row_d in m {
        let mut o = vec![row_d.clone()];
        for i in 0..row_d.len() {
            let mut b = row_d.clone();
            b.remove(i);
            o.push(b);
        }
        let mut safe = false;
        for row in o {
            if row
                .iter()
                .tuple_windows()
                .map(|(a, b)| (*a as i32) - (*b as i32))
                .tuple_windows()
                .all(|(a, b)| a.is_negative() == b.is_negative())
                && row
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| a.abs_diff(*b))
                    .all(|v| v <= 3 && v >= 1)
            {
                safe = true;
                break;
            }
        }
        if safe {
            i += 1;
        }
    }
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
