use std::fs::File;
use std::io::Read;
use std::path::Path;

fn solve(input: &str) -> (String, usize) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut string = "".to_string();
    let mut steps = 0;
    let mut i = 0;
    let mut j = 0;
    let mut direction: (isize, isize) = (1, 0);
    // Find entrance
    while grid[i][j].is_whitespace() {
        j += 1;
    }
    loop {
        match grid[i][j] {
            ' ' => break,
            '|' => (),
            '-' => (),
            '+' => {
                if direction.0 == 0 {
                    if grid[i - 1][j] != ' ' {
                        direction = (-1, 0);
                    } else {
                        direction = (1, 0);
                    }
                } else {
                    if j != 0 && grid[i][j - 1] != ' ' {
                        direction = (0, -1);
                    } else {
                        direction = (0, 1);
                    }
                }
            }
            c => string.push(c),
        }
        i = (i as isize + direction.0) as usize;
        j = (j as isize + direction.1) as usize;
        steps += 1;

    }
    (string, steps)
}

fn main() {
    let mut f = File::open(Path::new("input/day19.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let (p1, p2) = solve(&s);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {
        let input = "     |         
     |  +--+   
     A  |  C   
 F---|----E|--+
     |  |  |  D
     +B-+  +--+ ";
        assert_eq!(solve(&input), ("ABCDEF".to_string(), 38));
    }

}
