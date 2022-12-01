fn main() {
    let mut elves = Vec::<u32>::new();
    let mut acc: u32 = 0;
    let lines = include_str!("../input.txt").lines();

    for line in lines {
        if line.is_empty() {
            elves.push(acc);
            acc = 0
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }

    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let top: Vec<u32> = elves[0..3].to_vec();
    println!("{}", top.iter().sum::<u32>());
}
