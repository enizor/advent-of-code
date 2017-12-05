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
    let v: Vec<isize> = file.lines().filter_map( |x| x.unwrap().parse().ok()).collect();
    let sum1 = journey(v.clone(), |x| x+1);
    let sum2 = journey(v.clone(), |x| if x > 2 {x-1} else {x+1});
    Ok((sum1, sum2))
}

fn journey<F>(mut v: Vec<isize>, mut f: F) -> usize
where F: FnMut(isize) -> isize {
   let mut steps = 0;
   let mut pos: isize = 0;
   let n = v.len();
   while pos >= 0 && pos < n as isize {
       let x = v[pos as usize];
       v[pos as usize] = f(x);
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
            assert_eq!(journey(input.clone(), |x| x+1), *solution)
        }
    }

    #[test]
    fn solve2_test() {
        let inputs = [vec![1], vec![-1], vec![0], vec![0,-1], vec![0,3,0,1,-3]];
        let solutions = [1, 1, 2, 4, 10];
        for (input, solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(journey(input.clone(), |x| if x > 2 {x-1} else {x+1}), *solution)
        }
    }
}
