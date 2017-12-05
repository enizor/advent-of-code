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
    let f = File::open(Path::new("input/day5.txt"))?;
    let file = BufReader::new(&f);
    let mut sum1 = 0;
    let mut sum2 = 0;
    let v: Vec<isize> = file.lines().filter_map( |x| x.unwrap().parse().ok()).collect();
    sum1 = solve1(v.clone());
    Ok((sum1, sum2))
}

fn solve1(mut v: Vec<isize>) -> usize {
   let mut steps = 0;
   let mut pos: isize = 0;
   let n = v.len();
   while pos >= 0 && pos < n as isize {
       let x = v[pos as usize];
       v[pos as usize] += 1;
       pos += x;
       steps += 1;
   }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve1_test() {
        let inputs = [vec![1], vec![-1], vec![0], vec![0,-1], vec![0,3,0,1,-3]];
        let solutions = [1, 1, 2, 4, 5];
        for (input, solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(solve1(input.clone()), *solution)
        }
    }
}
