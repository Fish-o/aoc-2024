use advent_of_code::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Debug, Display},
};
advent_of_code::solution!(5);
fn unique_permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Ord,
{
    if items.len() == 1 {
        vec![items]
    } else {
        let mut output: Vec<Vec<T>> = vec![];

        // Obtain a list of the unique elements.
        // Sorting and deduping should be faster than using a hashset for most small n.
        let mut unique_items = items.clone();
        unique_items.sort();
        unique_items.dedup();
        for first in unique_items {
            let mut remaining_elements = items.clone();

            // this feature is unstable
            // remaining_elements.remove_item(first);

            let index = remaining_elements.iter().position(|x| *x == first).unwrap();
            remaining_elements.remove(index);

            for mut permutation in unique_permutations(remaining_elements) {
                permutation.insert(0, first.clone());
                output.push(permutation);
            }
        }
        output
    }
}

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
                // 'perms: for p in unique_permutations(parts) {
                for (a, b) in &r {
                    if p.iter().position(|e| e == a).unwrap_or(0)
                        > p.iter().position(|e| e == b).unwrap_or(p.len())
                    {
                        // continue 'perms;
                        return false;
                    }
                }
                return true;
                // return p;
                // }
                panic!("No solution found :(")
            })
            .map(|l| {
                // println!("{}", l.len());
                let len = l.len().clone();
                l.get(len / 2).unwrap().clone()
            })
            .sum(),
    )
}

pub fn sort(v: &mut Vec<usize>, r2: &HashMap<usize, usize>) {
    let r_len = r2.keys().count();
    for _ in 0..(v.len() * 2) {
        for i in 0..(v.len() - 1) {
            let a = v[i];
            let b = v[i + 1];
            let mut a_before = a;
            let ordering = 'getord: {
                'loopp: for _ in 0..r_len {
                    if a_before == b {
                        break 'getord Some(Ordering::Less);
                    }
                    match r2.get(&a_before) {
                        None => break 'loopp,
                        Some(v) => a_before = *v,
                    }
                }
                let mut b_before = b;
                'loopp: for _ in 0..r_len {
                    if b_before == a {
                        break 'getord Some(Ordering::Greater);
                    }
                    match r2.get(&b_before) {
                        None => break 'loopp,
                        Some(v) => b_before = *v,
                    }
                }
                None
            };
            println!("A {:?}", ordering);
            match ordering {
                Some(Ordering::Greater) => {
                    v[i] = b;
                    v[i + 1] = a;
                }
                _ => {}
            }
        }
    }
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
    let r2 = r.clone().into_iter().collect::<HashMap<_, _>>();
    Some(
        p.lines()
            .map(|l| {
                l.split(",")
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .filter(|p| {
                // 'perms: for p in unique_permutations(parts) {
                for (a, b) in &r {
                    if p.iter().position(|e| e == a).unwrap_or(0)
                        > p.iter().position(|e| e == b).unwrap_or(p.len())
                    {
                        // continue 'perms;
                        return true;
                    }
                }
                return false;
                // return p;
                // }
            })
            // .par_bridge()
            .map(|mut parts| {
                sort2(&mut parts, &r);
                // unreachable!();
                parts
            })
            .map(|l| {
                // println!("{}", l.len());
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
