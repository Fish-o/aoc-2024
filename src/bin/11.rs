use advent_of_code::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, LinkedList},
    fmt::{Debug, Display},
};
advent_of_code::solution!(11);

pub fn compute_next(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    } else if stone.ilog10() % 2 == 1 {
        let c = (stone.ilog10() + 1) / 2;
        let a = stone / (10 as usize).pow(c);
        let b = stone % (10 as usize).pow(c);
        return vec![a, b];
    } else {
        vec![stone * 2024]
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    input
        .trim()
        .split_whitespace()
        .map(|p| p.parse::<usize>().unwrap())
        .for_each(|p| {
            *counts.entry(p).or_insert(0) += 1;
        });

    for _ in 0..25 {
        let mut new_counts = HashMap::new();
        for (stone, count) in counts {
            for new_stone in compute_next(stone) {
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }
        counts = new_counts;
    }

    Some(counts.values().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    input
        .trim()
        .split_whitespace()
        .map(|p| p.parse::<usize>().unwrap())
        .for_each(|p| {
            *counts.entry(p).or_insert(0) += 1;
        });

    for _ in 0..75 {
        let mut new_counts = HashMap::new();
        for (stone, count) in counts {
            for new_stone in compute_next(stone) {
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }
        counts = new_counts;
    }

    Some(counts.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
