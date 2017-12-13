use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, PartialEq)]
struct Layer {
    depth: usize,
    range: usize,
}

impl Layer {
    fn caught(&self, time: usize) -> bool {
        time % (2 * self.range - 2) == 0
    }

    fn severity(&self, time: usize) -> Option<usize> {
        if self.caught(time) {
            Some(self.depth * self.range)
        } else {
            None
        }
    }
}

fn main() {
    let mut f = File::open(Path::new("input/day13.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let (p1, p2) = solve(&s);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

fn solve(input: &str) -> (usize, usize) {
    let layers = parse_input(input);
    let mut total = None;
    let mut delay = 0;
    for layer in layers.iter() {
        if let Some(s) = layer.severity(layer.depth) {
            total = Some(total.unwrap_or(0) + s);
        }
    }
    let part_1 = total.unwrap_or(0);
    while total.is_some() {
        total = None;
        delay += 1;
        for layer in layers.iter() {
            if let Some(s) = layer.severity(layer.depth + delay) {
                total = Some(total.unwrap_or(0) + s);
            }
        }
    }
    (part_1, delay)
}


fn parse_input(input: &str) -> Vec<Layer> {
    input
        .lines()
        .map(|line| {
            let nums = line.split_whitespace()
                .filter_map(|n| n.trim_matches(':').parse::<usize>().ok())
                .collect::<Vec<usize>>();
            Layer {
                depth: nums[0],
                range: nums[1],
            }
        })
        .collect::<Vec<Layer>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "0: 3
                     1: 2
                     4: 4
                     6: 4";
        assert_eq!(
            parse_input(input),
            vec![
                Layer { depth: 0, range: 3 },
                Layer { depth: 1, range: 2 },
                Layer { depth: 4, range: 4 },
                Layer { depth: 6, range: 4 },
            ]
        )
    }

    #[test]
    fn solve_test() {
        let input = "0: 3
                     1: 2
                     4: 4
                     6: 4";
        assert_eq!(solve(input), (24, 10))
    }
}
