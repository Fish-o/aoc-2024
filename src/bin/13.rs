use advent_of_code::utils::*;
use itertools::Itertools;
use nalgebra::{Cholesky, ComplexField, Matrix2, Vector2};
use rayon::prelude::*;
use rsmt2::Solver;
use std::fmt::{Debug, Display};
advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, false)
}
pub fn part_two(input: &str) -> Option<usize> {
    solve(input, true)
}

pub fn parse(input: &str) -> Vec<(f64, f64, f64, f64, f64, f64)> {
    input
        .trim()
        .split("\n\n")
        .map(|cm| {
            let mut lines = cm.lines();
            let a = lines.next().unwrap().split_once(":").unwrap().1.trim();
            let b = lines.next().unwrap().split_once(":").unwrap().1.trim();
            let xy = lines.next().unwrap().split_once(":").unwrap().1.trim();
            let (ax, ay) = a.split_once(", ").unwrap();
            let (bx, by) = b.split_once(", ").unwrap();
            let (x, y) = xy.split_once(", ").unwrap();

            type T = f64;
            let (ax, ay) = (
                &ax[2..].parse::<T>().unwrap(),
                &ay[2..].parse::<T>().unwrap(),
            );
            let (bx, by) = (
                &bx[2..].parse::<T>().unwrap(),
                &by[2..].parse::<T>().unwrap(),
            );
            let (x, y) = (&x[2..].parse::<T>().unwrap(), &y[2..].parse::<T>().unwrap());
            (*ax, *ay, *bx, *by, *x, *y)
        })
        .collect_vec()
}
pub fn solve(input: &str, two: bool) -> Option<usize> {
    Some(
        parse(input)
            .into_iter()
            .map(|(ax, bx, ay, by, x, y)| {
                let a = Matrix2::new(ax, bx, ay, by);
                let ai = a.try_inverse().unwrap();

                let x = if two {
                    Vector2::new(x + 10000000000000.0, y + 10000000000000.0)
                } else {
                    Vector2::new(x, y)
                };
                let b = ai * x;
                if floating_point_bullshit(b[(0, 0)]) && floating_point_bullshit(b[(1, 0)]) {
                    Some(b)
                } else {
                    None
                }
            })
            .filter(|v| v.is_some())
            .map(|v| v.unwrap())
            .fold(0 as isize, |tokens, v| {
                tokens + (3 * v[(0, 0)].round() as isize) + (v[(1, 0)].round() as isize)
            }) as usize,
    )
}

const DELTA: f64 = 0.0005;
fn floating_point_bullshit(num: f64) -> bool {
    (num as isize as f64 - num).abs() < DELTA || (num as isize as f64 - num).abs() > (1.0 - DELTA)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
