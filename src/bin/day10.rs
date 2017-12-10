use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve1(mut v: &mut Vec<usize>, lengths: &[usize]) -> usize {
    knot(&mut v, lengths, 0, 0);
    v[0] * v[1]
}


fn solve2(mut v: &mut Vec<usize>, input: &str) -> String {

    let mut lengths: Vec<usize> = input.chars().map(|c| c as usize).collect::<Vec<usize>>();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..64 {
        let res = knot(&mut v, &lengths, pos, skip);
        pos = res.0;
        skip = res.1;
    }
    // v is now the sparse hash, transform it in dense hash
    let mut dense_hash = vec![];
    for i in 0..16 {
        dense_hash.push(v[i * 16..(i + 1) * 16].iter().fold(0, |acc, &x| acc ^ x));
    }
    // format in hex notation
    dense_hash.iter().map(|&x| format(x)).collect()
}

fn format(n: usize) -> String {
    let mut s = format!("{:x}", n);
    if s.len() == 1 {
        s = String::from("0") + &s
    }
    s
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
    let mut v = (0..256).collect::<Vec<usize>>();
    let p2 = solve2(&mut v, &input);
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

    #[test]
    fn solve2_test() {
        assert_eq!(
            solve2(&mut (0..256).collect::<Vec<usize>>(), ""),
            "a2582a3a0e66e6e86e3812dcb672a272"
        );
        assert_eq!(
            solve2(&mut (0..256).collect::<Vec<usize>>(), "AoC 2017"),
            "33efeb34ea91902bb2f59c9920caa6cd"
        );
        assert_eq!(
            solve2(&mut (0..256).collect::<Vec<usize>>(), "1,2,3"),
            "3efbe78a8d82f29979031a4aa0b16a9d"
        );
        assert_eq!(
            solve2(&mut (0..256).collect::<Vec<usize>>(), "1,2,4"),
            "63960835bcdc130f0b66d7ff4f6a5a8e"
        );
    }
}
