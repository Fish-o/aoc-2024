use advent_of_code::utils::*;
use core::panic;
use itertools::Itertools;
use matrix::Matrix;
use num::{integer::Average, Integer};
use rayon::prelude::*;
use std::{
    fmt::{Debug, Display},
    thread,
    time::Duration,
};
advent_of_code::solution!(14);

struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

// const X_MAX: isize = 11;
// const Y_MAX: isize = 7;

const X_MAX: isize = 101;
const Y_MAX: isize = 103;

impl Robot {
    pub fn iterate(&mut self, n: isize) {
        self.p = (
            (self.p.0 + (self.v.0 * n)).mod_floor(&X_MAX),
            (self.p.1 + (self.v.1 * n)).mod_floor(&Y_MAX),
        );
    }
    pub fn top(&self) -> bool {
        self.p.1 < (Y_MAX / 2)
    }
    pub fn bottom(&self) -> bool {
        self.p.1 >= ((Y_MAX + 1) / 2)
    }
    pub fn left(&self) -> bool {
        self.p.0 < (X_MAX / 2)
    }
    pub fn right(&self) -> bool {
        self.p.0 >= ((X_MAX + 1) / 2)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut r = input
        .trim()
        .lines()
        .map(|l| l.trim().split_once(" ").unwrap())
        .map(|(p, v)| (&p[2..], &v[2..]))
        .map(|(p, v)| (p.split_once(",").unwrap(), v.split_once(",").unwrap()))
        .map(|((px, py), (vx, vy))| Robot {
            p: (px.parse().unwrap(), py.parse().unwrap()),
            v: (vx.parse().unwrap(), vy.parse().unwrap()),
        })
        .collect_vec();
    r.iter_mut().for_each(|r| r.iterate(100));
    let tl = r.iter().filter(|r| r.left() && r.top()).count();
    let bl = r.iter().filter(|r| r.left() && r.bottom()).count();
    let tr = r.iter().filter(|r| r.right() && r.top()).count();
    let br = r.iter().filter(|r| r.right() && r.bottom()).count();

    let mut m = Matrix::<usize>::new_empty(Y_MAX as usize, X_MAX as usize);
    m.enumerate()
        .iter()
        .map(|(p, _)| p.clone())
        .collect_vec()
        .iter()
        .for_each(|p| *m.get_pos_mut(&p).unwrap() = Some(0));

    let mut m = m.sequence().unwrap().to_owned_values();
    r.iter().for_each(|r| {
        let (r, c) = (r.p.1 as usize, r.p.0 as usize);
        *m.get_mut(r, c) = m.get(r, c) + 1;
    });

    println!("{m:?}");

    println!("{tl}, {bl}, {tr}, {br}");
    Some(tl * tr * bl * br)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut m = Matrix::<usize>::new_empty(Y_MAX as usize, X_MAX as usize);
    m.enumerate()
        .iter()
        .map(|(p, _)| p.clone())
        .collect_vec()
        .iter()
        .for_each(|p| *m.get_pos_mut(&p).unwrap() = Some(0));
    let base_m = m.sequence().unwrap().to_owned_values();

    let mut r = input
        .trim()
        .lines()
        .map(|l| l.trim().split_once(" ").unwrap())
        .map(|(p, v)| (&p[2..], &v[2..]))
        .map(|(p, v)| (p.split_once(",").unwrap(), v.split_once(",").unwrap()))
        .map(|((px, py), (vx, vy))| Robot {
            p: (px.parse().unwrap(), py.parse().unwrap()),
            v: (vx.parse().unwrap(), vy.parse().unwrap()),
        })
        .collect_vec();
    // r.iter_mut().for_each(|r| r.iterate(p));
    for i in 0.. {
        r.iter_mut().for_each(|r| r.iterate(1));

        let mut m = base_m.clone();
        r.iter().for_each(|r| {
            let (r, c) = (r.p.1 as usize, r.p.0 as usize);
            *m.get_mut(r, c) = m.get(r, c) + 1;
        });
        if m.enumerate().into_iter().all(|(_, e)| e <= &(1 as usize)) {
            println!("{m}");
            println!("Iteration: {i}");
            thread::sleep(Duration::from_millis(3000));
        }
    }

    // println!("{tl}, {bl}, {tr}, {br}");
    // Some(tl * tr * bl * br)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
