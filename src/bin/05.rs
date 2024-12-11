use advent_of_code::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Debug, Display},
};
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (r, p) = input.trim().split_once("\n\n").unwrap();
    let r = r
        .trim()
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect_vec();
    Some(
        p.lines()
            .map(|l| {
                l.split(",")
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .filter(|p| {
                for (a, b) in &r {
                    if p.iter().position(|e| e == a).unwrap_or(0)
                        > p.iter().position(|e| e == b).unwrap_or(p.len())
                    {
                        return false;
                    }
                }
                return true;
            })
            .map(|l| {
                let len = l.len().clone();
                l.get(len / 2).unwrap().clone()
            })
            .sum(),
    )
}
pub fn sort2(v: &mut Vec<usize>, r: &Vec<(usize, usize)>) {
    let mut changed = true;
    while changed {
        changed = false;
        for (a, b) in r {
            let (ai, bi) = match (v.iter().position(|v| v == a), v.iter().position(|v| v == b)) {
                (Some(ai), Some(bi)) => (ai, bi),
                _ => continue,
            };
            if ai > bi {
                v.swap(ai, bi);
                changed = true;
            }
        }
    }
}
pub fn part_two(input: &str) -> Option<usize> {
    let (r, p) = input.trim().split_once("\n\n").unwrap();
    let r = r
        .trim()
        .lines()
        .map(|l| l.split_once("|").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .collect_vec();
    Some(
        p.lines()
            .map(|l| {
                l.split(",")
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .filter(|p| {
                for (a, b) in &r {
                    if p.iter().position(|e| e == a).unwrap_or(0)
                        > p.iter().position(|e| e == b).unwrap_or(p.len())
                    {
                        return true;
                    }
                }
                return false;
            })
            .par_bridge()
            .map(|mut parts| {
                sort2(&mut parts, &r);
                parts
            })
            .map(|l| {
                let len = l.len().clone();
                l.get(len / 2).unwrap().clone()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
