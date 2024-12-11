fn main() {
    let res = parse_input();
    let mut a1 = Vec::new();
    let mut a2 = Vec::new();
    for (a, b) in res {
        a1.push(a);
        a2.push(b);
    }

    a1.sort();
    a2.sort();

    let out = a1
        .iter()
        .zip(a2.iter())
        .map(|(a, b)| (b - a).abs())
        .sum::<i32>();

    println!("{}", out);
}

fn parse_input() -> Vec<(i32, i32)> {
    let contents = std::fs::read_to_string("input.txt").unwrap();
    let mut result = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        let a = parts.next().unwrap().parse().unwrap();
        let b = parts.next().unwrap().parse().unwrap();
        result.push((a, b));
    }
    result
}
