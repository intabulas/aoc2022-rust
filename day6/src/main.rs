fn solve(input: &str, distinct: usize) -> Option<usize> {
    for i in 0..input.len() {
        let mut buf = 0_u32;
        let chars = &input[i..i + distinct];
        for c in chars.chars() {
            buf |= 1 << (c as u8 - b'a');
        }
        if buf.count_ones() == distinct as u32 {
            return Some(i + distinct);
        }
    }
    None
}

fn main() {
    let input = include_str!("../input.txt");
    let result = solve(input, 4);

    println!("Part 1: {}", result.unwrap());

    let result = solve(input, 14);
    println!("Part 1: {}", result.unwrap());
}
