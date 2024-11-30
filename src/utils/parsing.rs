use itertools::Itertools;

pub fn to_int_vec(string: &str) -> Vec<i64> {
    string.trim().lines().map(|s| {
        s.trim()
            .parse::<i64>()
            .expect("Could not parse a line as an integer")
    }).collect::<Vec<_>>()
}
