use std::cmp::PartialEq;
use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::path::Path;


fn main() {
    match solve() {
        Ok((sum1, sum2)) => println!("Part 1: {} | Part 2: {}", sum1, sum2),
        Err(err) => println!("/!\\ Error! {}", err.to_string()),
    }
}

fn solve() -> Result<(usize, usize), Error> {
    let f = File::open(Path::new("input/day6.txt"))?;
    let mut file = BufReader::new(&f);
    let mut input = "".to_string();
    file.read_line(&mut input)?;
    let v: Vec<usize> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    let sum1 = solve1(v.clone());
    let sum2 = 0;
    Ok((sum1, sum2))
}

fn solve1(mut v: Vec<usize>) -> usize {
    let w = v.clone();
    let mut states = vec![w];
    let mut res = 1;
    distribute(&mut v);
    while !contains(&states, v.clone()) {
        states.push(v.clone());
        distribute(&mut v);
        res += 1;
    }
    res
}

fn index_of_max(v: &[usize]) -> usize {
    let mut res = 0;
    let mut max = v[0];
    for (i, &x) in v.iter().enumerate() {
        if x > max {
            res = i;
            max = x;
        }
    }
    res
}

fn distribute(v: &mut Vec<usize>) {
    let mut i = index_of_max(&v);
    let len = v.len();
    let n = v[i];
    v[i] = 0;
    for _ in 0..n {
        v[(i + 1) % len] += 1;
        i += 1;
    }
}

fn contains<T: PartialEq>(v: &[T], x: T) -> bool {
    let mut res = false;
    let mut i = 0;
    while i < v.len() && !res {
        res = v[i] == x;
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_test() {
        let inputs = [vec![1, 2, 3, 4], vec![1, 3, 2, 1], vec![3, 1, 2, 3]];
        let solutions = [3, 1, 0];
        for (input, solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(index_of_max(input), *solution)
        }
    }

    #[test]
    fn distribute_test() {
        let mut input = vec![0, 2, 7, 0];
        distribute(&mut input);
        assert_eq!(&input, &[2, 4, 1, 2]);
        distribute(&mut input);
        assert_eq!(&input, &[3, 1, 2, 3]);
        distribute(&mut input);
        assert_eq!(&input, &[0, 2, 3, 4]);
        distribute(&mut input);
        assert_eq!(&input, &[1, 3, 4, 1]);
        distribute(&mut input);
        assert_eq!(&input, &[2, 4, 1, 2]);
    }

    #[test]
    fn contains_test() {
        assert_eq!(contains(&[], "a"), false);
        assert_eq!(contains(&["a"], "a"), true);
        assert_eq!(contains(&["b"], "a"), false);
        assert_eq!(contains(&[1, 2, 3], 2), true);
    }

    #[test]
    fn solve1_test() {
        assert_eq!(solve1(vec![0, 2, 7, 0]), 5);
        assert_eq!(solve1(vec![1, 2, 1]), 3);
        assert_eq!(solve1(vec![0, 1, 0]), 3);
    }
}
