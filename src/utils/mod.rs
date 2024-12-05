use std::{collections::HashMap, hash::Hash};

pub mod matrix;
pub mod matrix_utils;
pub mod parsing;
pub mod plane;
pub mod tree;

pub fn frequencies<E: Hash + Eq>(list: &Vec<E>) -> HashMap<&E, usize> {
    let mut h = HashMap::new();
    list.iter().for_each(|e| *h.entry(e).or_insert(0) += 1);
    h
}
