fn main() {
    let input = include_str!("../input.txt");
    // split on blank lines
    let lines = input.split("\n\n");

    // split chunks lines on \n and sum
    let mut lines_parsed: Vec<u32> = lines
        .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
        .collect();

    // sort descending
    lines_parsed.sort_by(|a, b| b.cmp(a));

    // print max
    println!("Part 1 {:?}", lines_parsed[0]);
    // print top 3
    println!("Part 1 {:?}", lines_parsed.iter().take(3).sum::<u32>())
}
