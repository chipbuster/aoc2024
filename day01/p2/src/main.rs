use std::collections::HashMap;

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

    let mut counts = HashMap::new();
    for i in a2 {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
    }

    let mut out = 0;
    for i in a1 {
        out += counts.get(&i).unwrap_or(&0) * i;
    }

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
