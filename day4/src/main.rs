use std::cmp::{max, min};
use std::str::FromStr;

struct Range {
    min: u64,
    max: u64,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once("-").unwrap();
        Ok(Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        })
    }
}

fn does_range_include_other(a: &Range, b: &Range) -> bool {
    (a.min <= b.min && a.max >= b.max) || (b.min <= a.min && b.max >= a.max)
}

fn do_ranges_overlap(a: &Range, b: &Range) -> bool {
    max(a.min, b.min) <= min(a.max, b.max)
}

fn main() {
    println!("Hello, world!");

    let val: u64 = include_str!("../input.txt")
        .lines()
        .map(|pair| {
            let (one, two) = pair.split_once(",").unwrap();
            let o = one.parse().unwrap();
            let t = two.parse().unwrap();
            does_range_include_other(&o, &t)
        })
        .filter(|f| *f)
        .count() as u64;

    println!("part1 {}", val);

    let val2: u64 = include_str!("../input.txt")
        .lines()
        .map(|pair| {
            let (one, two) = pair.split_once(",").unwrap();
            let o = one.parse().unwrap();
            let t = two.parse().unwrap();
            do_ranges_overlap(&o, &t)
        })
        .filter(|f| *f)
        .count() as u64;

    println!("part2 {}", val2)
}
