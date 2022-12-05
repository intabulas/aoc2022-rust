fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = input.replace("\r\n", "\n");
    let input = input.split("\n\n").collect::<Vec<_>>();

    let crates = input[0].lines().collect::<Vec<_>>();
    let instructions = input[1].lines().collect::<Vec<_>>();

    let cols = crates[crates.len() - 1]
        .split_whitespace()
        .map(|x| str::parse::<usize>(x).unwrap())
        .max()
        .unwrap();

    let rows = crates.len();

    let mut state = vec![vec![]; cols];

    for i in 0..cols {
        for r in crates[..rows - 1].iter() {
            let char = r.get(i * 4..i * 4 + 3).unwrap_or("").trim();
            if char.is_empty() {
                continue;
            } else {
                state[i].push(char.chars().nth(1).expect("not None"));
            }
        }
    }

    let mut parsed_instructions = vec![];
    for instruction in instructions {
        let mut instruction = instruction.split_whitespace();
        let count = get_next(&mut instruction);
        let from = get_next(&mut instruction);
        let to = get_next(&mut instruction);
        parsed_instructions.push((count, from, to));
    }

    (state, parsed_instructions)
}

fn get_next<'lt, T>(instruction: &mut T) -> usize
where
    T: Iterator<Item = &'lt str>,
{
    instruction.nth(1).unwrap().parse::<usize>().unwrap()
}

fn main() {
    let input = include_str!("../input.txt");

    let (mut state, parsed) = parse_input(input);
    for (count, from, to) in &parsed {
        for _ in 0..*count {
            let x = state[from - 1].remove(0);
            state[to - 1].insert(0, x);
        }
    }

    println!(
        "part 1 {:#?}",
        state
            .iter_mut()
            .filter_map(|c| c.first())
            .collect::<String>()
    );

    let (mut state, parsed) = parse_input(input);
    for (count, from, to) in &parsed {
        for i in 0..*count {
            let x = state[from - 1].remove(0);
            state[to - 1].insert(i, x);
        }
    }

    println!(
        "part 1 {:#?}",
        state
            .iter_mut()
            .filter_map(|c| c.first())
            .collect::<String>()
    );
}
