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

fn dfs(
    grid: &Vec<Vec<char>>,
    cur: (isize, isize),
    dir: Dir,
    places: &mut HashSet<(usize, usize)>,
    visited: &mut HashSet<((usize, usize), Dir)>,
    nl: &HashMap<Dir, (isize, isize)>,
    dl: &HashMap<Dir, Dir>,
) -> bool {
    if !valid(&grid, cur) {
        return true;
    }

    if visited.contains(&((cur.0 as usize, cur.1 as usize), dir)) {
        return false;
    }

    let mut res = false;

    if grid[cur.0 as usize][cur.1 as usize] == '#' {
        let del = nl[&dir];
        let cur = (cur.0 - del.0, cur.1 - del.1);
        let dir = dl[&dir];
        res |= dfs(&grid, cur, dir, places, visited, &nl, &dl);
    } else {
        visited.insert(((cur.0 as usize, cur.1 as usize), dir));
        places.insert((cur.0 as usize, cur.1 as usize));
        let del = nl[&dir];

        res |= dfs(
            &grid,
            (cur.0 + del.0, cur.1 + del.1),
            dir,
            places,
            visited,
            &nl,
            &dl,
        );
    }

    res
}

fn count_obstacles(
    grid: &Vec<Vec<char>>,
    places: &HashSet<(usize, usize)>,
    start: (usize, usize),
    nl: &HashMap<Dir, (isize, isize)>,
    dl: &HashMap<Dir, Dir>,
) -> u32 {
    let mut count = 0;

    let mut cpy: Vec<Vec<char>> = vec![];
    for v in grid {
        cpy.push(v.to_vec())
    }

    let mut dummy: HashSet<(usize, usize)> = HashSet::new();
    let mut dummy2: HashSet<((usize, usize), Dir)> = HashSet::new();
    for cur in places.iter() {
        //let cur = places.pop();
        if *cur == start {
            dbg!("here");
            continue;
        }

        cpy[cur.0 as usize][cur.1 as usize] = '#';

        let ans = dfs(
            &cpy,
            (start.0 as isize, start.1 as isize),
            Dir::North,
            &mut dummy,
            &mut dummy2,
            &nl,
            &dl,
        );
        if !ans {
            count += 1;
        }

        dummy.clear();
        dummy2.clear();
        cpy[cur.0 as usize][cur.1 as usize] = '.';
    }
    count
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
    let mut places: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<((usize, usize), Dir)> = HashSet::new();

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

    //grid[6][3] = '#';
    //let ans = bfs(&grid, start, &mut places);
    let ans = dfs(
        &grid,
        (start.0 as isize, start.1 as isize),
        Dir::North,
        &mut places,
        &mut visited,
        &neigh_lookup,
        &dir_lookup,
    );
    let count = count_obstacles(&grid, &places, start, &neigh_lookup, &dir_lookup);

    dbg!(places.len(), count);
}
