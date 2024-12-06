use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::{Matrix, Pos};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
};
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let m = Matrix::from_grid(input);
    let mut g_spot = m
        .enumerate()
        .into_iter()
        .find(|(p, c)| c == &"^")
        .map(|(p, c)| p)
        .unwrap();
    let mut step = (-1, 0);
    let mut positions = HashMap::new();
    positions.insert(g_spot.clone(), vec![step.clone()]);
    loop {
        let (r, c) = g_spot.get_rc();
        let (r, c) = (((r as isize) + step.0), ((c as isize) + step.1));
        if r < 0 || c < 0 || r >= m.height() as isize || c >= m.width() as isize {
            break;
        }
        let mut new_pos = Pos::from_rc(r as usize, c as usize);
        if let Some(c) = m.get_pos(&new_pos) {
            if c == &"#" {
                step = match step {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => unreachable!(),
                };
                new_pos = g_spot.clone();
            }
        }
        g_spot = new_pos;
        let res = positions.entry(g_spot.clone()).or_insert(vec![]);
        if res.contains(&step) {
            break;
        }
        res.push(step);
    }
    Some(positions.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = Matrix::from_grid(input);
    let mut count = 0;
    'outer: for (obstruct_pos, c) in m.enumerate().into_iter() {
        let mut g_spot = m
            .enumerate()
            .into_iter()
            .find(|(p, c)| c == &"^")
            .map(|(p, c)| p)
            .unwrap();
        let mut step = (-1, 0);
        if c != "." {
            continue;
        }
        let mut positions = HashMap::new();
        positions.insert(g_spot.clone(), vec![step.clone()]);
        loop {
            let (r, c) = g_spot.get_rc();
            let (r, c) = (((r as isize) + step.0), ((c as isize) + step.1));
            if r < 0 || c < 0 || r >= m.height() as isize || c >= m.width() as isize {
                break;
            }
            let mut new_pos = Pos::from_rc(r as usize, c as usize);
            if let Some(c) = m.get_pos(&new_pos) {
                if c == &"#" || obstruct_pos == new_pos {
                    step = match step {
                        (-1, 0) => (0, 1),
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        _ => unreachable!(),
                    };
                    new_pos = g_spot.clone();
                }
            }
            g_spot = new_pos;
            let res = positions.entry(g_spot.clone()).or_insert(vec![]);
            if res.contains(&step) {
                count += 1;
                continue 'outer;
            }
            res.push(step);
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
