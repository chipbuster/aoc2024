use std::env;
use std::fs;

fn main() {
    let reports = parse_input();

    let x = reports.iter().filter(|x| report_is_safe(x.clone()));
    println!("{:?}", x.count());
}

fn report_is_safe(inp: &Vec<i32>) -> bool {
    if report_is_safe_nr(inp) {
        return true;
    } else {
        for i in 0..inp.len() {
            let mut inp1 = inp.clone();
            inp1.remove(i);
            if report_is_safe_nr(&inp1) {
                println!("Found safe on removing {}", i);
                return true;
            }
        }
        return false;
    }
}

fn report_is_safe_nr(inp: &Vec<i32>) -> bool {
    if inp.len() == 1 {
        return true;
    }
    let sign = (inp[1] - inp[0]).signum();

    if sign == 0 {
        return false;
    }

    let step_okay = |i, j| {
        let diff: i32 = inp[j] - inp[i];
        diff.signum() == sign && diff.abs() <= 3 && diff.abs() > 0
    };

    for i in 0..inp.len() - 1 {
        let ok = step_okay(i, i + 1);
        if !ok {
            return false;
        }
    }

    return true;
}

// fn report_is_safe(inp: &Vec<i32>, can_remove: bool) -> bool {
//     let mut sign = (inp[1] - inp[0]).signum();

//     let remove_elem = |i| {
//         let mut inp1 = inp.clone();
//         inp1.remove(i);
//         inp1
//     };

//     if sign == 0 && can_remove {
//         return report_is_safe(&remove_elem(0), false) || report_is_safe(&remove_elem(1), false);
//     }

//     let step_okay = |i, j| {
//         let diff: i32 = inp[j] - inp[i];
//         diff.signum() == sign && diff.abs() <= 3 && diff.abs() > 0
//     };

//     for i in 0..inp.len() - 1 {
//         let ok = step_okay(i, i + 1);
//         if !ok && can_remove {
//             return report_is_safe(&remove_elem(i), false)
//                 || report_is_safe(&remove_elem(i + 1), false);
//         } else if !ok {
//             return false;
//         }
//     }

//     return true;
// }

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
