use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve1(input: &str) -> isize {
    let mut north: isize = 0;
    let mut east: isize = 0;
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
    }
    // need to cover east distance by going NE, then finish full North
    east.abs() + (north.abs() - east.abs()) / 2
}


fn solve2(input: &str) -> usize {
    0
}

fn main() {
    let mut f = File::open(Path::new("input/day11.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = s.trim();
    let p1 = solve1(&input);
    let p2 = solve2(&input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve1_test() {
        assert_eq!(solve1("ne,ne,ne"), 3);
        assert_eq!(solve1("ne,ne,sw,sw"), 0);
        assert_eq!(solve1("ne,ne,s,s"), 2);
        assert_eq!(solve1("se,sw,se,sw,sw"), 3);
    }
}
