use advent_of_code::utils::*;
use itertools::Itertools;
use matrix::{Matrix, Pos};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};
advent_of_code::solution!(12);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Edge {
    // Above
    Horizontal(Pos, bool),
    // Left to
    Vertical(Pos, bool),
}

impl Edge {
    pub fn around(pos: &Pos) -> Vec<Edge> {
        let mut edges = vec![];
        edges.push(Edge::Horizontal(pos.clone(), true));
        edges.push(Edge::Vertical(pos.clone(), true));
        edges.push(Edge::Horizontal(Pos(pos.0 + 1, pos.1), false));
        edges.push(Edge::Vertical(Pos(pos.0, pos.1 + 1), false));
        edges
    }

    pub fn in_line_with(&self, other: &Edge) -> bool {
        match (self, other) {
            (Edge::Horizontal(p1, b1), Edge::Horizontal(p2, b2)) => {
                b1 == b2 && (p1.0 == p2.0) && p1.1.abs_diff(p2.1) == 1
            }
            (Edge::Vertical(p1, b1), Edge::Vertical(p2, b2)) => {
                b1 == b2 && (p1.1 == p2.1) && p1.0.abs_diff(p2.0) == 1
            }
            _ => false,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let m = Matrix::from_grid(input.trim());
    let positions = m.enumerate().into_iter().map(|(p, c)| p).collect_vec();
    let regions_list = m.flood_regions(
        &positions,
        |m, p, r| {
            let c = m.get_pos(p).unwrap();
            m.touching_positions(p)
                .into_iter()
                .filter(|tp| m.get_pos(tp).unwrap() == c)
                .collect_vec()
        },
        true,
        true,
    );
    let mut regions = vec![];
    for reg in regions_list {
        if reg.is_empty() {
            continue;
        }
        regions.push((m.get_pos(reg.first().unwrap()).unwrap(), reg));
    }

    Some(
        regions
            .into_iter()
            .map(|(region, mut positions)| {
                positions.sort();
                positions.dedup();
                let area = positions.len();
                let perimeter = positions
                    .iter()
                    .map(|p| {
                        4 - m
                            .touching_cells(p)
                            .into_iter()
                            .filter(|c| *c == region)
                            .count()
                    })
                    .sum::<usize>();
                area * perimeter
            })
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let m = Matrix::from_grid(input.trim());
    let positions = m.enumerate().into_iter().map(|(p, _)| p).collect_vec();
    let regions_list = m.flood_regions(
        &positions,
        |m, p, _| {
            let c = m.get_pos(p).unwrap();
            m.touching_positions(p)
                .into_iter()
                .filter(|tp| m.get_pos(tp).unwrap() == c)
                .collect_vec()
        },
        true,
        true,
    );
    let mut regions = vec![];
    for reg in regions_list {
        if reg.is_empty() {
            continue;
        }
        regions.push((m.get_pos(reg.first().unwrap()).unwrap(), reg));
    }

    Some(
        regions
            .into_iter()
            .map(|(region, mut positions)| {
                positions.sort();
                positions.dedup();
                let area = positions.len();
                let mut edges = positions
                    .iter()
                    .flat_map(|p| {
                        let mut edges = vec![];
                        let up = if p.0 == 0 {
                            None
                        } else {
                            m.get_pos(&Pos(p.0 - 1, p.1))
                        };
                        let down = m.get_pos(&Pos(p.0 + 1, p.1));
                        let left = if p.1 == 0 {
                            None
                        } else {
                            m.get_pos(&Pos(p.0, p.1 - 1))
                        };
                        let right = m.get_pos(&Pos(p.0, p.1 + 1));
                        if up.is_none() || up.unwrap() != region {
                            edges.push(Edge::Horizontal(p.clone(), true))
                        }
                        if down.is_none() || down.unwrap() != region {
                            edges.push(Edge::Horizontal(Pos(p.0 + 1, p.1), false))
                        }
                        if left.is_none() || left.unwrap() != region {
                            edges.push(Edge::Vertical(p.clone(), true))
                        }
                        if right.is_none() || right.unwrap() != region {
                            edges.push(Edge::Vertical(Pos(p.0, p.1 + 1), false))
                        }
                        edges
                    })
                    .collect_vec();

                let mut continuous = vec![];
                while let Some(starting_edge) = edges.pop() {
                    let mut stack = vec![starting_edge];
                    let mut line = vec![];
                    while let Some(edge) = stack.pop() {
                        line.push(edge.clone());

                        let (ix, ex) = &mut edges
                            .iter()
                            .enumerate()
                            .filter(|(_, e)| e.in_line_with(&edge))
                            .filter(|(_, e)| !line.contains(e) && !stack.contains(e))
                            .fold((vec![], vec![]), |(mut ix, mut ex), (i, e)| {
                                ix.push(i);
                                ex.push(e.clone());
                                (ix, ex)
                            });
                        ix.sort();
                        for i in ix.into_iter().rev() {
                            edges.remove(*i);
                        }
                        stack.append(ex);
                    }
                    continuous.push(stack);
                }
                area * continuous.len()
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
