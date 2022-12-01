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

    println!("{:?}", elves.iter().max())
}
