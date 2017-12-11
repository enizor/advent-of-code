use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve(input: &str) -> (isize, isize) {
    let mut north: isize = 0;
    let mut east: isize = 0;
    let mut steps = 0;
    let mut max_steps = 0;
    for step in input.split(",") {
        // Need to make sure NE + NW = N
        match step.trim() {
            "ne" => {
                north += 1;
                east += 1;
            }
            "n" => north += 2,
            "nw" => {
                north += 1;
                east -= 1;
            }
            "sw" => {
                north -= 1;
                east -= 1;
            }
            "s" => north -= 2,
            "se" => {
                north -= 1;
                east += 1;
            }
            _ => (),
        }
        // need to cover east distance by going NE, then finish full North
        steps = east.abs() + (north.abs() - east.abs()) / 2;
        max_steps = max_steps.max(steps);
    }
    (steps, max_steps)
}

fn main() {
    let mut f = File::open(Path::new("input/day11.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = s.trim();
    let (p1, p2) = solve(&input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {
        assert_eq!(solve("ne,ne,ne"), (3, 3));
        assert_eq!(solve("ne,ne,sw,sw"), (0, 2));
        assert_eq!(solve("ne,ne,s,s"), (2, 2));
        assert_eq!(solve("se,sw,se,sw,sw"), (3, 3));
    }
}
