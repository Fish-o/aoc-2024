use advent_of_code::utils::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::Index,
};
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let chars = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect_vec();
    let skipped = chars.first().unwrap().clone();
    let chars = chars.into_iter().skip(1).collect_vec();
    let len = chars.iter().sum::<usize>();
    let mut data = vec![0; len];
    let mut pointer = 0;
    let mut write = false;
    let mut file = 1;
    for c in chars.iter() {
        if write {
            for p in pointer..(pointer + c) {
                data[p] = file;
            }
            file += 1;
        }
        write = !write;
        pointer += c;
    }

    let mut empty_pointer = 0;
    for to_move in (0..data.len()).rev() {
        println!("{empty_pointer} {to_move}");
        if to_move <= empty_pointer {
            break;
        }
        let d = data[to_move];
        if d == 0 {
            continue;
        }
        data[empty_pointer] = d;
        data[to_move] = 0;
        for new_empty_pointer in (empty_pointer + 1).. {
            if new_empty_pointer > to_move || data[new_empty_pointer] == 0 {
                empty_pointer = new_empty_pointer;
                break;
            }
        }
    }
    println!("Len {len}");
    println!("{}", data.iter().map(|c| c.to_string()).join(""));

    let mut sum = 0;
    for (i, d) in data.iter().enumerate() {
        sum += d * (skipped + i)
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sizes = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect_vec();
    let mut files = vec![];
    let mut max_count = 0;
    for i in 0..sizes.len() {
        if i % 2 == 0 {
            files.push(Some(max_count));
            max_count += 1;
        } else {
            files.push(None);
        }
    }

    for first_empty in 0.. {
        println!("{first_empty} {}", files.len());
        if first_empty >= files.len() {
            break;
        }
        if files[first_empty].is_some() {
            continue;
        }
        let first_empty_size = sizes[first_empty];
        for count in (0..max_count).rev() {
            let fi = files
                .iter()
                .position(|f| f.is_some() && f.unwrap() == count)
                .unwrap();
            assert!(fi != first_empty);
            if fi < first_empty {
                continue;
            }
            let f = files[fi];
            let s = sizes[fi];
            if f.is_none() || s > first_empty_size {
                continue;
            }

            let remaining = first_empty_size - s;
            files.swap(fi, first_empty);
            if remaining > 0 {
                files.insert(first_empty + 1, None);
                sizes.insert(first_empty + 1, remaining);
                sizes[first_empty] = s;
            }
            break;
        }
        // println!("{}", files.iter().map(|f| f.unwrap_or(0)).join(""));
        // println!("{}", sizes.iter().map(|f| f).join(""));
    }
    let mut sum = 0;
    let mut count = 0;
    for i in 0..files.len() {
        let c = sizes[i];
        if files[i].is_some() {
            let f = files[i].unwrap();
            for m in count..(count + c) {
                sum += m * f;
            }
        }
        count += c;
    }
    println!("{}", files.iter().map(|f| f.unwrap_or(0)).join(""));
    println!("{}", sizes.iter().map(|f| f).join(""));
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
