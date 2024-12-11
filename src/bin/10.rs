use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::{Matrix, Pos};
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let m = Matrix::from_ugrid(input);
    let start_positions = m
        .enumerate()
        .into_iter()
        .filter(|(_, c)| **c == 0)
        .map(|(p, _)| p)
        .collect_vec();
    let mut sum = 0;
    for pos in start_positions {
        let res = m.flood_regions(
            &vec![pos],
            |m, p, _| {
                return m
                    .touching_positions(p)
                    .into_iter()
                    .filter(|p2| *m.get_pos(p2).unwrap() == m.get_pos(p).unwrap() + 1)
                    .collect_vec();
            },
            true,
            true,
        );
        assert_eq!(res.len(), 1);
        sum += res
            .first()
            .unwrap()
            .iter()
            .filter(|p| m.get_pos(p).unwrap() == &9)
            .count();
    }

    Some(sum)
}

pub fn count_paths(m: &Matrix<usize>, path: &Vec<Pos>) -> Vec<Vec<Pos>> {
    let p = path.iter().last().unwrap();
    let v = m.get_pos(p).unwrap();
    if v == &9 {
        return vec![path.to_vec()];
    }
    let mut res = vec![];
    for touching in m.touching_positions(p) {
        if *m.get_pos(&touching).unwrap() == v + 1 {
            let mut new_path = path.clone();
            new_path.push(touching);
            res.append(&mut count_paths(m, &new_path));
        }
    }
    res
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = Matrix::from_ugrid(input);
    let start_positions = m
        .enumerate()
        .into_iter()
        .filter(|(_, c)| **c == 0)
        .map(|(p, _)| p)
        .collect_vec();
    let mut sum = 0;
    for pos in start_positions {
        let paths = count_paths(&m, &vec![pos]);
        sum += paths.len();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
