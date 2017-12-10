use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve(mut v: Vec<usize>, lengths: &[usize]) -> (usize, usize) {
    let mut pos = 0;
    let mut skip = 0;
    let n = v.len();
    for &len in lengths {
        reverse(&mut v, pos, len);
        pos = (pos + len + skip) % n;
        skip += 1;
    }
    (v[0] * v[1], 0)
}

fn main() {
    let mut f = File::open(Path::new("input/day10.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = s.trim();
    let mut v: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let (p1, p2) = solve((0..256).collect::<Vec<usize>>(), &v);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

fn reverse(v: &mut Vec<usize>, pos: usize, len: usize) {
    let n = v.len();
    for i in 0..(len / 2) {
        let x = v[(pos + i) % n];
        v[(pos + i) % n] = v[(pos + len - 1 - i) % n];
        v[(pos + len - 1 - i) % n] = x;
    }
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn reverse_test() {
        let mut input = vec![0, 1, 2, 3, 4];
        reverse(&mut input, 0, 3);
        assert_eq!(input, vec![2, 1, 0, 3, 4]);
        reverse(&mut input, 3, 4);
        assert_eq!(input, vec![4, 3, 0, 1, 2]);
        reverse(&mut input, 1, 5);
        assert_eq!(input, vec![3, 4, 2, 1, 0]);
    }

    #[test]
    fn solve_test() {
        let v: Vec<usize> = (0..5).collect();
        let input = &[3, 4, 1, 5];
        assert_eq!(solve(v, input), (12, 0))
    }

}
