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
pub fn solve(input: &str, two: bool) -> Option<usize> {
    // 3 tokens for A
    // 1 token for B
    // x = a*xa + b*xb
    // y = a*ya + b*yb
    // (x) = (xa, xb)(a)
    // (y) = (ya, yb)(b)

    Some(
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
                let a = Matrix2::new(*ax, *bx, *ay, *by);
                let ai = a.try_inverse().unwrap();

                // Ab = x
                // b = Ax
                let x = if two {
                    Vector2::new(*x + 10000000000000.0, *y + 10000000000000.0)
                } else {
                    Vector2::new(*x, *y)
                };
                // println!("{a}");
                // println!("{ai}");
                // println!("{x}");
                let b = ai * x;
                if check_fraction(b[(0, 0)]) && check_fraction(b[(1, 0)]) {
                    println!("some: {b:?}");
                    Some(b)
                } else {
                    println!("NONE: {b:?}");
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

const ACC: f64 = 0.0005;
fn check_fraction(num: f64) -> bool {
    (num as isize as f64 - num).abs() < ACC || (num as isize as f64 - num).abs() > (1.0 - ACC)
}
pub fn part_two(input: &str) -> Option<usize> {
    solve(input, true)
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
