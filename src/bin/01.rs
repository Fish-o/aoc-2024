use advent_of_code::utils::*;
use itertools::Itertools;
use num::Integer;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    let mut a = input
        .trim()
        .lines()
        .map(|e| e.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .fold((vec![], vec![]), |mut a, b| {
            a.0.push(b.0);
            a.1.push(b.1);
            (a.0, a.1)
        });

    a.0.sort();
    a.1.sort();

    Some(a.0.iter().zip(a.1).map(|(a, b)| a.abs_diff(b)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut a = input
        .trim()
        .lines()
        .map(|e| e.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .fold((vec![], vec![]), |mut a, b| {
            a.0.push(b.0);
            a.1.push(b.1);
            (a.0, a.1)
        });

    a.1.sort();
    let h: HashMap<usize, usize> = HashMap::new();
    let (v, c, mut h) = a.1.iter().fold((0, 0, h), |(x, mut count, mut h), v| {
        if &x == v {
            count += 1;
        } else {
            h.insert(x, count);
            count = 1;
        }
        (*v, count, h)
    });
    h.insert(v, c);
    Some(a.0.iter().map(|v| v * h.get(v).or(Some(&0)).unwrap()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
