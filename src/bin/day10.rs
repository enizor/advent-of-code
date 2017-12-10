use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve1(mut v: &mut Vec<usize>, lengths: &[usize]) -> usize {
    knot(&mut v, lengths, 0, 0);
    v[0] * v[1]
}

fn knot(mut v: &mut Vec<usize>, lengths: &[usize], pos: usize, skip: usize) -> (usize, usize) {
    let mut pos_res = pos;
    let mut skip_res = skip;
    let n = v.len();
    for &len in lengths {
        reverse(&mut v, pos_res, len);
        pos_res = (pos_res + len + skip_res) % n;
        skip_res += 1;
    }
    (pos_res, skip_res)
}

fn main() {
    let mut f = File::open(Path::new("input/day10.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = s.trim();
    let lengths: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut v = (0..256).collect::<Vec<usize>>();
    let p1 = solve1(&mut v, &lengths);
    let p2 = 0;
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
    fn solve1_test() {
        let mut v: Vec<usize> = (0..5).collect();
        let input = &[3, 4, 1, 5];
        assert_eq!(solve1(&mut v, input), 12)
    }

}
