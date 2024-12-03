use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::Matrix;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for i in 0.. {
        match input.chars().skip(i).take(4).collect_tuple() {
            Some(('m', 'u', 'l', '(')) => {}
            None => break,
            _ => continue,
        }
        let a = input
            .chars()
            .skip(i + 4)
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        if !matches!(input.chars().skip(i + 4 + a.len()).next(), Some(',')) {
            continue;
        }
        let b = input
            .chars()
            .skip(i + 4 + a.len() + 1)
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        if !matches!(
            input.chars().skip(i + 4 + a.len() + 1 + b.len()).next(),
            Some(')')
        ) {
            continue;
        }
        if a.len() < 1 || a.len() > 3 || b.len() < 1 || b.len() > 3 {
            continue;
        }
        sum += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    let mut skip = false;
    for i in 0.. {
        match input.chars().skip(i).take(7).collect_tuple() {
            Some(('d', 'o', '(', ')', _, _, _)) => skip = false,
            Some(('d', 'o', 'n', '\'', 't', '(', ')')) => skip = true,
            Some(('m', 'u', 'l', '(', _, _, _)) => {}

            None => break,
            _ => continue,
        }
        let a = input
            .chars()
            .skip(i + 4)
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        if !matches!(input.chars().skip(i + 4 + a.len()).next(), Some(',')) {
            continue;
        }
        let b = input
            .chars()
            .skip(i + 4 + a.len() + 1)
            .take_while(|c| c.is_numeric())
            .collect::<String>();
        if !matches!(
            input.chars().skip(i + 4 + a.len() + 1 + b.len()).next(),
            Some(')')
        ) {
            continue;
        }
        if a.len() < 1 || a.len() > 3 || b.len() < 1 || b.len() > 3 {
            continue;
        }
        if !skip {
            sum += a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap();
        }
    }
    Some(sum)
}

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
