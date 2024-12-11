use std::env;
use std::fs;

fn main() {
    let reports = parse_input();

    let x = reports.iter().filter(|x| report_is_safe(x.clone()));
    println!("{:?}", x.count());
}

fn report_is_safe(inp: &Vec<i32>) -> bool {
    let deltas = inp
        .windows(2)
        .map(|v| {
            let [x, y] = v else { unreachable!() };
            y - x
        })
        .collect::<Vec<i32>>();
    let first_step = deltas[0];
    if first_step == 0 {
        return false;
    } else if first_step < 0 {
        return deltas.iter().all(|x| (-3..0).contains(x));
    } else {
        return deltas.iter().all(|x| (1..=3).contains(x));
    }
}

fn parse_input() -> Vec<Vec<i32>> {
    let path = if std::env::var("USE_EXAMPLE").is_ok() {
        "../example.txt"
    } else {
        "../input.txt"
    };
    let lines = fs::read_to_string(path).unwrap();
    let mut out = Vec::new();
    for line in lines.lines() {
        let line = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        out.push(line);
    }
    out
}
