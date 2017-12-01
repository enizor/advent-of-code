extern crate atoi;

use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;


fn main() {
    match solve() {
        Ok((sum1, sum2)) => println!("Part 1: {}, Part 2: {}", sum1, sum2),
        Err(err) => println!("/!\\ Error! {}", err.to_string()),
    }
}

fn solve() -> Result<(u32, u32), Error> {
    let v = read_ints(Path::new("input/day1.txt"))?;
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    let n = v.len();
    for i in 0..v.len() {
        // Part 1
        if v[i] == v[(i+1)%n] { sum1 += v[i] as u32 }
        // Part 2
        if v[i] == v[(i+(n/2))%n] { sum2 += v[i] as u32}
    }
    Ok((sum1, sum2))
}

fn read_ints(p: &Path) -> Result<Vec<u8>, Error> {
    let mut f = File::open(p)?;
    let mut res = vec![];
    let mut to_delete = vec![];
    f.read_to_end(&mut res)?;
    for (i, s) in res.iter_mut().enumerate() {
        match atoi::ascii_to_digit(*s) {
            Some(x) => {*s = x},
            _ => to_delete.push(i)
        }
    }
    let mut c = 0;
    for i in to_delete.iter() {
        res.remove(*i-c);
        c += 1;
    }
    Ok(res)
}
