use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let mut f = File::open(Path::new("input/day9.txt")).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).ok();
    let (p1, p2) = solve(&input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

fn solve(input: &str) -> (usize, usize) {
    let mut score = 0;
    let mut depth = 0;
    let mut in_garbage = false;
    let mut ignore_next = false;
    let mut count_garbage = 0;
    for c in input.chars() {
        if !ignore_next {
            if in_garbage {
                match c {
                    '!' => ignore_next = true,
                    '>' => in_garbage = false,
                    _ => count_garbage += 1,
                }
            } else {
                match c {
                    '{' => {
                        depth += 1;
                        score += depth;
                    }
                    '}' => depth -= 1,
                    '<' => in_garbage = true,
                    '!' => ignore_next = true,
                    _ => (),
                }
            }
        } else {
            ignore_next = false
        }
    }
    (score, count_garbage)
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &str = "{}
    {{{}}}
    {{},{}}
    {{{},{},{{}}}}
    {<a>,<a>,<a>,<a>}
    {{<ab>},{<ab>},{<ab>},{<ab>}}
    {{<!!>},{<!!>},{<!!>},{<!!>}}
    {{<a!>},{<a!>},{<a!>},{<ab>}}";

    #[test]
    fn solve_test() {
        let sol = [
            (1, 0),
            (6, 0),
            (5, 0),
            (16, 0),
            (1, 4),
            (9, 8),
            (9, 0),
            (3, 17),
        ];
        for (input, solution) in INPUTS.lines().zip(sol.iter()) {
            assert_eq!(solve(input), *solution);
        }
    }

}
