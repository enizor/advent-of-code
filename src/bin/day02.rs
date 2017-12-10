use std::fs::File;
use std::io::{BufRead, Error, BufReader};
use std::path::Path;

fn main() {
    match solve() {
        Ok((sum1, sum2)) => println!("Part 1: {}, Part 2: {}", sum1, sum2),
        Err(err) => println!("/!\\ Error! {}", err.to_string()),
    }
}

fn solve() -> Result<(u32, u32), Error> {
    let v = read_ints(Path::new("input/day2.txt"))?;
    let mut sum1 = 0u32;
    let mut sum2 = 0u32;
    for line in &v {
        sum1 += checksum_line(line);
        sum2 += division_line(line);
    }
    Ok((sum1, sum2))
}

fn read_ints(p: &Path) -> Result<Vec<Vec<u32>>, Error> {
    let f = File::open(p)?;
    let buffer = BufReader::new(f);
    let lines = buffer.lines();
    let res = lines
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect()
        })
        .collect();
    Ok(res)
}

fn checksum_line(v: &[u32]) -> u32 {
    let mut min = v[0];
    let mut max = min;
    for x in v {
        if *x > max {
            max = *x
        };
        if *x < min {
            min = *x
        };
    }
    max - min
}

fn division_line(v: &[u32]) -> u32 {
    let mut res = 0;
    for x in v {
        for y in v {
            if x != y && x % y == 0 {
                res = x / y
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn checksum_test() {
        let inputs: [&[u32]; 3] = [&[5, 1, 9, 5], &[7, 5, 3], &[2, 4, 6, 8]];
        let solutions = [8, 4, 6];
        for (input, solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(checksum_line(*input), *solution)
        }
    }

    #[test]
    fn division_test() {
        let inputs: [&[u32]; 3] = [&[5, 9, 2, 8], &[9, 4, 7, 3], &[3, 8, 6, 5]];
        let solutions = [4, 3, 2];
        for (input, solution) in inputs.iter().zip(solutions.iter()) {
            assert_eq!(division_line(*input), *solution)
        }
    }

}
