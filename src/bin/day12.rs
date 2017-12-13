use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let mut f = File::open(Path::new("input/day12.txt")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).ok();
    let input = parse_input(&s.trim());
    let (p1, p2) = solve(input);
    println!("Part 1: {}, Part 2: {}", p1, p2)
}

fn solve(input: Vec<Vec<usize>>) -> (usize, usize) {
    let mut groups = vec![];
    for pipe in input {
        let x = pipe[0];
        for &y in &pipe[1..] {
            add_link(x, y, &mut groups);
        }
    }
    (groups[0].len(), groups.len())
}


fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.trim_matches(',').parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn find_item(k: usize, v: &Vec<Vec<usize>>) -> Option<usize> {
    let mut res: Option<usize> = None;
    for (pos, ve) in v.iter().enumerate() {
        if find_item_2(k, &ve) {
            res = Some(pos)
        };
    }
    res
}

fn find_item_2(k: usize, v: &Vec<usize>) -> bool {
    let mut res = false;
    for i in v {
        res = res || (k == *i);
    }
    res
}

fn add_link(x: usize, y: usize, groups: &mut Vec<Vec<usize>>) {
    match (find_item(x, &groups), find_item(y, &groups)) {
        (Some(i), Some(j)) if i == j => (),
        (Some(i), Some(j)) => merge(i, j, groups),
        (None, Some(j)) => groups[j].push(x),
        (Some(i), None) => groups[i].push(y),
        (None, None) if x == y => groups.push(vec![x]),
        (None, None) => groups.push(vec![x, y]),
    };
}

fn merge(i: usize, j: usize, groups: &mut Vec<Vec<usize>>) {
    if i < j {
        let mut v = groups.swap_remove(j);
        groups[i].append(&mut v);
    } else {
        merge(j, i, groups)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn link_test() {
        let mut groups = vec![vec![0], vec![1], vec![2, 3], vec![4]];
        add_link(1, 4, &mut groups);
        assert_eq!(groups, vec![vec![0], vec![1, 4], vec![2, 3]]);
        add_link(5, 6, &mut groups);
        assert_eq!(groups, [vec![0], vec![1, 4], vec![2, 3], vec![5, 6]]);
    }

    #[test]
    fn solve_test() {
        let input = "0 <-> 2
                      1 <-> 1
                      2 <-> 0, 3, 4
                      3 <-> 2, 4
                      4 <-> 2, 3, 6
                      5 <-> 6
                      6 <-> 4, 5";
        assert_eq!(solve(parse_input(input)), (6, 2))
    }
}
