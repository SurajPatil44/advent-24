use std::collections::{HashMap, HashSet};
use utils::Content;

fn valid(grid: &Vec<Vec<char>>, pos: (isize, isize)) -> bool {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    (pos.0 >= 0 && pos.0 < n) && (pos.1 >= 0 && pos.1 < m)
}

fn get_antinodes(
    grid: &Vec<Vec<char>>,
    first: &(usize, usize),
    second: &(usize, usize),
) -> HashSet<(usize, usize)> {
    let mut ret = HashSet::new();
    let del_i: isize = second.0 as isize - first.0 as isize;
    let del_j: isize = second.1 as isize - first.1 as isize;

    let mut neig1: (isize, isize) = (first.0 as isize, first.1 as isize);
    loop {
        if valid(grid, neig1) {
            ret.insert((neig1.0 as usize, neig1.1 as usize));
        } else {
            break;
        }
        neig1 = (neig1.0 as isize - del_i, neig1.1 as isize - del_j);
    }

    let mut neig2: (isize, isize) = (second.0 as isize, second.1 as isize);
    loop {
        if valid(grid, neig2) {
            ret.insert((neig2.0 as usize, neig2.1 as usize));
        } else {
            break;
        }
        neig2 = (neig2.0 as isize + del_i, neig2.1 as isize + del_j);
    }
    ret
}

fn get_all_antinodes(
    grid: &Vec<Vec<char>>,
    points: &Vec<(usize, usize)>,
    antinodes: &mut HashSet<(usize, usize)>,
) {
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            antinodes.extend(get_antinodes(grid, &points[i], &points[j]));
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);

    let mut grid: Vec<Vec<char>> = vec![];

    for l in reader {
        grid.push(l.chars().collect());
    }

    let mut data: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                continue;
            }
            data.entry(grid[i][j])
                .or_insert(Vec::<_>::new())
                .push((i, j));
        }
    }

    let mut antinodes = HashSet::new();

    for k in data {
        get_all_antinodes(&grid, &k.1, &mut antinodes);
        //break;
    }

    dbg!(antinodes.len());
}
