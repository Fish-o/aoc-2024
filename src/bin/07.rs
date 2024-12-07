use advent_of_code::utils::*;
use itertools::Itertools;
use num::{integer::Average, Integer};
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(7);

pub fn permute_rev(current: usize, vals: &[usize]) -> bool {
    if vals.is_empty() {
        return current == 0;
    }
    let len = vals.len();
    let val = &vals[len - 1];
    let vals = &vals[0..(len - 1)];
    if current % val == 0 {
        if permute_rev(current / val, vals) {
            return true;
        }
    }
    if *val > current {
        return false;
    }
    return permute_rev(current - val, vals);
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .map(|(a, b)| {
                (
                    a.trim().parse::<usize>().unwrap(),
                    b.trim()
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_vec(),
                )
            })
            .filter(|(a, b)| permute_rev(*a, b))
            .map(|(a, _)| a)
            .sum(),
    )
}

pub fn permute2_rev(current: usize, vals: &[usize]) -> bool {
    if vals.is_empty() {
        return current == 0;
    }
    let len = vals.len();
    let val = &vals[len - 1];
    let vals = &vals[0..(len - 1)];
    if current % val == 0 {
        if permute2_rev(current / val, vals) {
            return true;
        }
    }
    let power = 10_u64.pow(val.ilog10() + 1) as usize;
    if current % power == *val {
        if permute2_rev(current / power, vals) {
            return true;
        }
    }
    if *val > current {
        return false;
    }
    return permute2_rev(current - val, vals);
}
pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .trim()
            .lines()
            .map(|l| l.split_once(": ").unwrap())
            .map(|(a, b)| {
                (
                    a.trim().parse::<usize>().unwrap(),
                    b.trim()
                        .split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_vec(),
                )
            })
            .filter(|(a, b)| permute2_rev(*a, b))
            .map(|(a, _)| a)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
