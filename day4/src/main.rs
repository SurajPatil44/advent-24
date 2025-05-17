use utils::Content;

fn valid(grid: &Vec<Vec<char>>, i: isize, j: isize) -> bool {
    let n = grid.len() as isize;
    let m = grid[0].len() as isize;
    return (i >= 0 && i < n) && (j >= 0 && j < m);
}

fn get_next(cur: char) -> Option<char> {
    match cur {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        'S' => None,
        _ => unreachable!(),
    }
}

fn dfs(
    grid: &Vec<Vec<char>>,
    i: isize,
    j: isize,
    dir: &(isize, isize),
    next: Option<char>,
) -> bool {
    if next.is_none() {
        return true;
    }
    let mut res = false;
    let next = next.unwrap();
    if valid(grid, i, j) {
        if grid[i as usize][j as usize] == next {
            let nnext = get_next(next);
            let next_i = i + dir.0;
            let next_j = j + dir.1;
            res |= dfs(grid, next_i, next_j, dir, nnext);
        }
    }
    res
}

fn check_X(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let n = grid.len();
    let m = grid[0].len();

    if i > n - 3 || j > m - 3 {
        return false;
    }

    let mut roi: Vec<Vec<char>> = vec![];

    for r in i..i + 3 {
        let mut temp: Vec<char> = vec![];
        for c in j..j + 3 {
            temp.push(grid[r][c]);
        }
        roi.push(temp);
    }
    let mut res = false;
    if roi[1][1] == 'A' {
        let mut check = 0;
        if roi[0][0] == 'M' {
            let ret = dfs(&roi, 0, 0, &(1, 1), Some('M'));
            if ret {
                check += 1;
            }
        }
        if roi[0][2] == 'M' {
            let ret = dfs(&roi, 0, 2, &(1, -1), Some('M'));
            if ret {
                check += 1;
            }
        }
        if roi[2][2] == 'M' {
            let ret = dfs(&roi, 2, 2, &(-1, -1), Some('M'));
            if ret {
                check += 1;
            }
        }
        if roi[2][0] == 'M' {
            let ret = dfs(&roi, 2, 0, &(-1, 1), Some('M'));
            if ret {
                check += 1;
            }
        }

        res = check == 2;
    }
    return res;
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);

    let mut grid: Vec<Vec<char>> = vec![];

    for l in reader {
        let temp: Vec<char> = l.chars().map(|x| x).collect();
        grid.push(temp);
    }

    /*
     *   up = (-1,0) , down = (1,0)
     *   right = (0,1), down = (0,-1)
     *   up_right = (-1,1), up_left = (-1,-1)
     *   down_right = (1,1), down_left = (1,-1)
     *
     */
    let dirs = vec![
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
    ];
    let mut count = 0;
    let n = grid.len();
    let m = grid[0].len();
    for i in 0..n {
        for j in 0..m {
            for dir in &dirs {
                if grid[i][j] == 'X' {
                    let ret = dfs(&grid, i as isize, j as isize, &dir, Some('X'));
                    if ret {
                        count += 1;
                    }
                }
            }
        }
    }
    dbg!(count);
    count = 0;
    for i in 0..n {
        for j in 0..m {
            if check_X(&grid, i, j) {
                count += 1;
            }
        }
    }
    dbg!(count);
}
