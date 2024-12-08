use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::{Matrix, Pos};
use plane::{Line, Point};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let m = Matrix::from_grid(input.trim());
    let mut points = HashMap::new();
    m.enumerate()
        .into_iter()
        .filter(|(_, c)| c != &".")
        .for_each(|(p, c)| points.entry(c).or_insert(vec![]).push(p));

    let mut antinodes = HashSet::new();
    for (_, px) in points {
        px.iter().tuple_combinations::<(_, _)>().for_each(|(a, b)| {
            let (ar, ac) = a.get_rc();
            let (br, bc) = b.get_rc();
            let (dr, dc) = (br as isize - ar as isize, bc as isize - ac as isize);
            let (br2, bc2) = (br as isize + dr, bc as isize + dc);
            let (ar2, ac2) = (ar as isize - dr, ac as isize - dc);
            if !(br2 < 0 || br2 >= m.height() as isize || bc2 < 0 || bc2 >= m.width() as isize) {
                antinodes.insert((br2, bc2));
            }
            if !(ar2 < 0 || ar2 >= m.height() as isize || ac2 < 0 || ac2 >= m.width() as isize) {
                antinodes.insert((ar2, ac2));
            }
        });
    }

    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = Matrix::from_grid(input.trim());
    let mut points = HashMap::new();
    m.enumerate()
        .into_iter()
        .filter(|(_, c)| c != &".")
        .for_each(|(p, c)| points.entry(c).or_insert(vec![]).push(p));

    let mut anti_nodes = HashSet::new();
    points.into_values().for_each(|px| {
        px.into_iter()
            .tuple_combinations::<(_, _)>()
            .for_each(|(Pos(ar, ac), Pos(br, bc))| {
                let (dr, dc) = (br as isize - ar as isize, bc as isize - ac as isize);
                let (mut br2, mut bc2) = (br as isize, bc as isize);
                let (mut ar2, mut ac2) = (ar as isize, ac as isize);
                loop {
                    let a = !(ar2 < 0
                        || ar2 >= m.height() as isize
                        || ac2 < 0
                        || ac2 >= m.width() as isize);
                    let b = !(br2 < 0
                        || br2 >= m.height() as isize
                        || bc2 < 0
                        || bc2 >= m.width() as isize);
                    if a {
                        anti_nodes.insert((ar2, ac2));
                        (ar2, ac2) = (ar2 as isize - dr, ac2 as isize - dc);
                    }
                    if b {
                        anti_nodes.insert((br2, bc2));
                        (br2, bc2) = (br2 as isize + dr, bc2 as isize + dc);
                    }
                    if !a && !b {
                        break;
                    }
                }
            });
    });

    Some(anti_nodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
