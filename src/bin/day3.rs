use std::env::args;
use std::cmp;

fn main() {
    match get_input() {
        Ok(n) => println!("Part 1: {} | Part 2: {}", solve_spiral(n), solve_2(n)),
        Err(err) => println!("/!\\ Error! {}", err.to_string()),
    }
}

fn get_input() -> Result<usize, std::num::ParseIntError> {
    let n = args().nth(1).expect("Please provide your input")
    .parse::<usize>()?;
    Ok(n)
}


fn solve_spiral(n: usize) -> usize {
/* The spiral can be seen as such

(x*x-2x+2)  (x*x-3x+3)
   \       /
    5  4  3
    6  1  2
    7  8  9
   /       \
(x*x-x+1)   (x*x)
(x is odd)
The distance to 1 for each of the corners is x-1
Finding odd x such that (x-1)**2 < n <= x**2
gives us the distance by calculating:
x-1 - min(|n-c|) for c in corners

*/
    let mut x = 1;
    while x*x < n {
        x = x + 2;
    }
    let mut min = x*x-n;
    for y in 1..5 {
        // y = 4 is the (x-2)**2 corner
        min = cmp::min(min, (n as i32 - (x*(x-y) + y)as i32 ).abs()as usize);
    }
    x-1 - min
}

fn solve_2(n: usize) -> usize {
    // After each turn, the number is more than 2 times the previous
    // Thus an acceptable size for the spirale is log2(n)/4 as an odd number

    let l: usize = ((32 - (n as u32).leading_zeros())/8 *2 +5) as usize;
    let mut m = vec![vec![0;l];l];
    // coordinates
    let mut x: usize = l/2;
    let mut y: usize = l/2+1;
    let mut direction = 0u8;
    m[x][y] = 1;
    m[x][y-1] = 1;
    while m[x][y] < n {
        let t = next_case(x, y, direction);
        x = t.0;
        y = t.1;
        m[x][y] = sum_neighbours(&m, x as isize, y as isize, l);
        // Direction change if at a corner
        if direction == 4 {
            direction = 0
        }
        if (x as i32 - (l/2) as i32).abs() == (y as i32 - (l/2) as i32).abs() {
            direction = direction + 1;
        }
    }
    // Lets print the spiral
    print_matrix(&m);
    m[x][y]
}

fn next_case(x:usize, y: usize, d: u8) -> (usize, usize) {
    match d {
        // dir = 4 is the bottom right corner
        // where we still need ine step before turning
        0 => (x-1, y),
        1 => (x, y-1),
        2 => (x+1, y),
        3 => (x, y+1),
        4 => (x, y+1),
        _ => (0,0)
    }

}

fn sum_neighbours(m: &Vec<Vec<usize>>, x: isize, y: isize, l:usize) -> usize {
    let mut sum = 0;
    // all the neighbours including (x,y) then filtered to have >=0 and <l
    let i_x = [x-1, x, x+1];
    let iter_x = i_x.into_iter()
    .filter( |&x| { *x >= 0 && *x < l as isize} );
    let i_y = [y-1, y, y+1];
    let iter_y: Vec<&isize> = i_y.into_iter()
    .filter( |&y| { *y >= 0 && *y < l as isize} ).collect();

    for x1 in iter_x {
        for y1 in &iter_y {
            sum += m[*x1 as usize][**y1 as usize];
        }
    }
    sum - m[x as usize][y as usize]
}

fn print_matrix(m: &Vec<Vec<usize>>) {
    for x in m {
        println!("{:?}", x)
    }
}
