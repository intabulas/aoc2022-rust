fn get_union(line: &str) -> u32 {
    let size = line.chars().count();
    let anchor = size / 2;
    let first = line[0..anchor].to_string();
    let second = line[anchor..size].to_string();

    let letters: Vec<char> = first.chars().collect();
    let mut value: u32 = 0;
    for letter in letters {
        if second.contains(letter) {
            if letter.is_lowercase() {
                value = letter as u32 - 96;
            } else {
                value = letter as u32 - 64 + 26;
            }
        }
    }
    return value;
}

fn main() {
    let lines = include_str!("../input.txt").split("\n");
    // .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
    // split on blank lines
    let mut sum: u32 = 0;
    for line in lines.clone().into_iter() {
        sum += get_union(&line);
    }

    println!("part1: {}", sum);
}
