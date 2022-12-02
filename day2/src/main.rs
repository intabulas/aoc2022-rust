use std::collections::HashMap;

fn main() {
    let mut score: i32 = 0;

    let score_map = HashMap::<&str, i32>::from([
        ("A X", 4), // 1
        ("A Y", 8), // 2
        ("A Z", 3), // 3
        ("B X", 1), // 1
        ("B Y", 5), // 2
        ("B Z", 9), // 3
        ("C X", 7), // 1
        ("C Y", 2), // 2
        ("C Z", 6), // 3
    ]);

    let lines = include_str!("../input.txt").split("\n");
    // split on blank lines
    //
    for line in lines.clone().into_iter() {
        let s = score_map.get(line);
        if s.is_some() {
            score += s.unwrap();
        }
    }

    println!("Part 1 {:?}", score);
    score = 0;

    let score_mapb = HashMap::<&str, i32>::from([
        ("A X", 3), // 3
        ("A Y", 4), // 2
        ("A Z", 8), // 3
        ("B X", 1), // 1
        ("B Y", 5), // 2
        ("B Z", 9), // 3
        ("C X", 2), // 1
        ("C Y", 6), // 2
        ("C Z", 7), // 3
    ]);

    for line in lines.clone().into_iter() {
        let s = score_mapb.get(line);
        if s.is_some() {
            score += s.unwrap();
        }
    }
    println!("Part 2 {:?}", score);
}
