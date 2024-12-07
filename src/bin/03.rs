use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::Matrix;
use rayon::prelude::*;
use regex::Regex;
use std::fmt::{Debug, Display};
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_, [n1, n2]) in r.captures_iter(input).map(|c| c.extract()) {
        if n1.len() > 3 || n2.len() > 3 {
            continue;
        }
        sum += n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let r1 = Regex::new(r"(?ms)do\(\)").unwrap();
    let r2 = Regex::new(r"(?ms)don't\(\)").unwrap();
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = r1
        .split(input)
        .into_iter()
        .par_bridge()
        .map(|e| {
            let mut sum = 0;
            let v = r2.split(e).next().unwrap();
            for (_, [n1, n2]) in r.captures_iter(v).map(|c| c.extract()) {
                if n1.len() > 3 || n2.len() > 3 {
                    continue;
                }
                sum += n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap();
            }
            sum
        })
        .sum();

    Some(sum)
}
//84893551
//11696166

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
