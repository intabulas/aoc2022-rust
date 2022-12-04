use itertools::Itertools;

fn item_priority(item: char) -> u64 {
    const CHARS: [char; 52] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    CHARS.iter().position(|i| i == &item).unwrap() as u64 + 1
}

fn common_chars(a: &str, b: &str) -> Vec<char> {
    a.chars()
        .filter(|ac| b.chars().contains(ac))
        .unique()
        .collect_vec()
}

fn main() {
    let sum: u64 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let len = l.len();
            let (first, second) = l.split_at(len / 2);
            let common = common_chars(first, second);
            common.iter().map(|c| item_priority(*c)).sum::<u64>()
        })
        .sum();
    // .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
    // split on blank lines

    println!("part1: {}", sum);

    let sum2: u64 = include_str!("../input.txt")
        .lines()
        .tuples()
        .into_iter()
        .map(|(a, b, c)| {
            let ab = common_chars(a, b);
            let ac = common_chars(a, c);
            let abc = ab
                .iter()
                .filter(|ab_char| ac.contains(&ab_char))
                .unique()
                .collect_vec();
            item_priority(**abc.first().unwrap())
        })
        .sum();
    println!("{:?}", sum);
    println!("{:?}", sum2);
}
