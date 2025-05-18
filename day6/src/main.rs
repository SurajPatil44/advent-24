use std::collections::{HashMap, HashSet, VecDeque};
use utils::Content;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn valid(grid: &Vec<Vec<char>>, pos: (isize, isize)) -> bool {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;

    (pos.0 >= 0 && pos.0 < n) && (pos.1 >= 0 && pos.1 < m)
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut deq: VecDeque<(usize, usize)> = VecDeque::new();
    deq.push_back(start);
    let mut dir = Dir::North;

    let neigh_lookup: HashMap<Dir, (isize, isize)> = HashMap::from([
        (Dir::North, (-1, 0)),
        (Dir::East, (0, 1)),
        (Dir::South, (1, 0)),
        (Dir::West, (0, -1)),
    ]);

    let dir_lookup: HashMap<Dir, Dir> = HashMap::from([
        (Dir::North, Dir::East),
        (Dir::East, Dir::South),
        (Dir::South, Dir::West),
        (Dir::West, Dir::North),
    ]);

    while !deq.is_empty() {
        let cur = deq.pop_front().unwrap();
        //dbg!(cur);
        visited.insert(cur);
        let neigh: (isize, isize) = (
            cur.0 as isize + neigh_lookup[&dir].0,
            cur.1 as isize + neigh_lookup[&dir].1,
        );
        if valid(&grid, neigh) {
            let val = grid[neigh.0 as usize][neigh.1 as usize];
            if val == '#' {
                dir = dir_lookup[&dir];
                //dbg!(&dir);
                let neigh: (isize, isize) = (
                    cur.0 as isize + neigh_lookup[&dir].0,
                    cur.1 as isize + neigh_lookup[&dir].1,
                );
                if valid(&grid, neigh) {
                    deq.push_back((neigh.0 as usize, neigh.1 as usize));
                }
            } else {
                deq.push_back((neigh.0 as usize, neigh.1 as usize));
            }
        }
    }
    visited.len()
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

    let mut start: (usize, usize) = (0, 0);
    let n = grid.len();
    let m = grid[0].len();

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '^' {
                start = (i, j);
                break;
            }
        }
    }
    //dbg!(start);
    let ans = bfs(&grid, start);
    dbg!(ans);
}
