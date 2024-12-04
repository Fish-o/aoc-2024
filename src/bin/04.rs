use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::Matrix;
use rayon::prelude::*;
use std::fmt::{Debug, Display};
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let mut m = Matrix::from_grid(input);
    let r = m
        .rows()
        .iter()
        .flat_map(|r| {
            r.into_iter()
                .map(|e| e.as_ref())
                .tuple_windows::<(_, _, _, _)>()
                .filter(|xmas| match xmas {
                    ("X", "M", "A", "S") | ("S", "A", "M", "X") => true,
                    _ => false,
                })
                .collect_vec()
        })
        .count();
    let c = m
        .columns()
        .iter()
        .flat_map(|c| {
            c.into_iter()
                .map(|e| e.as_ref())
                .tuple_windows::<(_, _, _, _)>()
                .filter(|xmas| match xmas {
                    ("X", "M", "A", "S") | ("S", "A", "M", "X") => true,
                    _ => false,
                })
                .collect_vec()
        })
        .count();
    let mut diags = m.diagonals();
    println!("{diags:?}");
    diags.0.append(&mut diags.1);
    let d = diags
        .0
        .iter()
        .flat_map(|d| {
            d.into_iter()
                .map(|e| e.as_ref())
                .tuple_windows::<(_, _, _, _)>()
                .filter(|xmas| match xmas {
                    ("X", "M", "A", "S") | ("S", "A", "M", "X") => true,
                    _ => false,
                })
                .collect_vec()
        })
        .count();

    println!("{r} {c} {d}");
    Some(d + c + r)
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = Matrix::from_grid(input);
    let r = m
        .enumerate()
        .iter()
        .filter(|(p, c)| {
            if c != &&"A".to_owned() {
                return false;
            }
            let (r, c) = p.get_rc();
            if r < 1 || c < 1 || r + 1 >= m.height() || c + 1 >= m.width() {
                return false;
            }
            let tl = m.get(r - 1, c - 1).as_str();
            let tr = m.get(r - 1, c + 1).as_str();
            let bl = m.get(r + 1, c - 1).as_str();
            let br = m.get(r + 1, c + 1).as_str();
            let a = match (tl, br) {
                ("M", "S") | ("S", "M") => true,
                _ => false,
            };
            let b = match (tr, bl) {
                ("M", "S") | ("S", "M") => true,
                _ => false,
            };
            return a && b;
        })
        .count();
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
