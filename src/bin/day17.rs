use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve1(input: usize) -> usize {
    let mut buffer = vec![0];
    let mut position = 0;
    for i in 1..2018 {
        position = (position + input) % buffer.len() + 1;
        if position == buffer.len() {
            buffer.push(i)
        } else {
            buffer.insert(position, i)
        }
    }
    buffer[(position + 1) % buffer.len()]
}

fn solve2(input: usize) -> usize {
    // no need to create the whole buffer
    // Only nned to check when we insert at position 1
    let mut len = 1;
    let mut position = 0;
    let mut res = 0;
    for i in 1..50_000_000 {
        position = (position + input) % len + 1;
        len += 1;
        if position == 1 {
            res = i
        }
    }
    res
}

fn main() {
    let mut f = File::open(Path::new("input/day17.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = s.trim().parse::<usize>().unwrap();
    let p1 = solve1(input);
    let p2 = solve2(input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {
        assert_eq!(solve1(3), 638);
    }

}
