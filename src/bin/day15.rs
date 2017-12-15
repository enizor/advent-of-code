use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

fn judge(iter: usize, rx: mpsc::Receiver<u64>, ry: mpsc::Receiver<u64>) -> usize {
    let mut res = 0;
    for _ in 0..iter {
        let x = rx.recv().unwrap();
        let y = ry.recv().unwrap();
        if x == y {
            res += 1;
        }
    }
    // receivers are droppped so the channels are closed
    res
}

fn generator(mut x: u64, c: u64, tx: mpsc::Sender<u64>, modulo: usize) {
    const MOD: u64 = (1 << 31) - 1;
    const MASK: u64 = (1 << 16) - 1;
    let mut channel_up = true;
    while channel_up {
        x = (x * c) % MOD;
        // Stop the generator when the judge got enough data
        if x & ((1 << modulo) - 1) == 0 {
            channel_up = tx.send(x & MASK).is_ok();
        }
    }
}

fn solve(x: u64, y: u64, iter: usize, mod1: usize, mod2: usize) -> usize {
    // Part 1: mod = 0, iter = 40M
    // Part 2: mod1 = 2, mod2 = 3, iter = 5M
    let (tx, rx) = mpsc::channel();
    let (ty, ry) = mpsc::channel();
    thread::spawn(move || generator(x, 16807, tx, mod1));
    thread::spawn(move || generator(y, 48271, ty, mod2));
    judge(iter, rx, ry)
}

fn main() {
    let mut f = File::open(Path::new("input/day15.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let mut inputs = s.lines().map(|line| {
        line.split_whitespace()
            .filter_map(|x| x.parse::<u64>().ok())
            .next()
            .unwrap()
    });
    let x = inputs.next().unwrap();
    let y = inputs.next().unwrap();
    let p1 = thread::spawn(move || solve(x, y, 40_000_000, 0, 0));
    let p2 = thread::spawn(move || solve(x, y, 5_000_000, 2, 3));
    println!(
        "Part 1: {}, Part 2: {}",
        p1.join().unwrap(),
        p2.join().unwrap()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve(65, 8921, 4, 0, 0), 1)
    }

    #[test]
    fn part2() {
        assert_eq!(solve(65, 8921, 1057, 2, 3), 1)
    }

    #[test]
    #[ignore]
    fn part1_long() {
        assert_eq!(solve(65, 8921, 40_000_000, 0, 0), 588)
    }

    #[test]
    #[ignore]
    fn part2_long() {
        assert_eq!(solve(65, 8921, 5_000_000, 2, 3), 309)
    }
}
