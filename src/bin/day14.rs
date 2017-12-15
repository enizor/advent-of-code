use std::fs::File;
use std::io::Read;
use std::path::Path;

fn knot_hash(input: &str) -> Vec<usize> {
    let mut v = (0..256).collect::<Vec<usize>>();
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
    dense_hash
    // format in hex notation isn't needed
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

fn reverse(v: &mut Vec<usize>, pos: usize, len: usize) {
    let n = v.len();
    for i in 0..(len / 2) {
        let x = v[(pos + i) % n];
        v[(pos + i) % n] = v[(pos + len - 1 - i) % n];
        v[(pos + len - 1 - i) % n] = x;
    }
}

fn solve1(input: &str) -> usize {
    let mut line: String = input.to_string();
    line.push('-');
    let mut res: usize = 0;
    for x in 0..128 {
        res += knot_hash(&(line.clone() + &format!("{}", x))).iter().fold(
            0,
            |x,
             y| {
                x + y.count_ones()
            },
        ) as usize;
    }
    res
}


fn find_item(k: (usize, usize), v: &Vec<Vec<(usize, usize)>>) -> Option<usize> {
    v.iter().position(|group| group.iter().any(|&x| x == k))
}

fn add_link(x: (usize, usize), y: (usize, usize), groups: &mut Vec<Vec<(usize, usize)>>) {
    match (find_item(x, groups), find_item(y, groups)) {
        (Some(i), Some(j)) if i == j => (),
        (Some(i), Some(j)) => merge(i, j, groups),
        _ => (),
    };
}

fn merge(i: usize, j: usize, groups: &mut Vec<Vec<(usize, usize)>>) {
    if i < j {
        let mut v = groups.swap_remove(j);
        groups[i].append(&mut v);
    } else {
        merge(j, i, groups)
    }
}

fn nth_bit(x: usize, n: usize) -> bool {
    ((x >> n) & 1) == 1
}

fn solve2(input: &str) -> usize {
    let mut line: String = input.to_string();
    line.push('-');
    let mut groups = vec![];
    let mut grid = [[false; 130]; 130];
    for i in 1..129 {
        let mut j = 1;
        for x in knot_hash(&(line.clone() + &format!("{}", i - 1))).iter() {
            for n in (0..8).rev() {
                if nth_bit(*x, n) {
                    grid[i][j] = true;
                    groups.push(vec![(i, j)]);
                }
                j += 1;
            }
        }
    }
    for (i, line) in grid.iter().enumerate() {
        for (j, x) in line.iter().enumerate() {
            if *x {
                if grid[i + 1][j] {
                    add_link((i, j), (i + 1, j), &mut groups)
                }

                if grid[i][j + 1] {
                    add_link((i, j), (i, j + 1), &mut groups)
                }
            }
        }
    }
    groups.len()
}

fn main() {
    let mut f = File::open(Path::new("input/day14.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = s.trim();
    let p1 = solve1(&input);
    let p2 = solve2(&input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
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
        assert_eq!(solve1("flqrgnkx"), 8108)
    }

    #[test]
    fn solve2_test() {
        assert_eq!(solve2("flqrgnkx"), 1242)
    }
}
