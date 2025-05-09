use utils::Content;

fn check_level(sv: &Vec<u32>) -> bool {
    /*
    1. The levels are either all increasing or all decreasing.
    2. Any two adjacent levels differ by at least one and at most three.
    */
    let mut increasing = false;
    if sv[1] > sv[0] {
        increasing = true;
    }

    for i in 0..sv.len() - 1 {
        //dbg!(i);
        let diff: i32 = sv[i + 1] as i32 - sv[i] as i32;
        if diff <= 0 && increasing {
            return false;
        } else if diff >= 0 && !increasing {
            return false;
        }

        if diff.abs() > 3 {
            return false;
        }
    }

    return true;
}

fn check_level2(sv: &Vec<u32>) -> bool {
    // 0 --> pdiff was negative
    // 1 --> pdiff was positive
    if check_level(sv) {
        return true;
    }

    for i in 0..sv.len() {
        let mut temp: Vec<u32> = vec![];
        for j in 0..sv.len() {
            if i == j {
                continue;
            }
            temp.push(sv[j]);
        }
        assert!(sv.len() - temp.len() == 1);
        if check_level(&temp) {
            //dbg!(&temp);
            return true;
        }
    }
    false
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);
    let mut grid: Vec<Vec<u32>> = Vec::new();

    for l in reader {
        let parts = l.split_whitespace();
        let mut temp: Vec<u32> = Vec::new();
        for p in parts {
            temp.push(p.parse::<u32>().unwrap());
        }
        grid.push(temp);
    }

    let mut count = 0;

    for v in grid {
        if check_level2(&v) {
            count += 1;
        }
    }

    //dbg!(check_level2(&grid[1]));
    //dbg!(check_level2(&grid[2]));
    println!("Solution {}", count);
}
