use advent_of_code::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(7);

pub fn permute(test: &usize, current: usize, vals: &[usize]) -> bool {
    if vals.is_empty() {
        return test == &current;
    }
    let val = &vals[0];
    let vals = &vals[1..];

    if permute(test, current + val, vals) || permute(test, current * val, vals) {
        return true;
    }

    return false;
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
            .filter(|(a, b)| permute(a, 0, b))
            .map(|(a, _)| a)
            .sum(),
    )
}

pub fn permute2(test: &usize, current: usize, vals: &[usize]) -> bool {
    if current > *test || vals.is_empty() {
        return test == &current;
    }
    let val = &vals[0];
    let vals = &vals[1..];
    if permute2(test, current + val, vals)
        || permute2(test, current * val, vals)
        || permute2(
            test,
            current * (10_u64.pow(val.checked_ilog10().unwrap() + 1) as usize) + val,
            vals,
        )
    {
        return true;
    }

    return false;
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
            .filter(|(a, b)| permute2(a, *&b[0], &b[1..]))
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
