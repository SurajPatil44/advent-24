use std::collections::BinaryHeap;
use std::iter::zip;
use utils::Content;

/*
Problem  : https://adventofcode.com/2024/day/1
*/

fn lower_bound(sv: &Vec<u32>, n: u32) -> Option<usize> {
    let mut l = 0;
    let mut h = sv.len() - 1;
    let mut ans: Option<usize> = None;
    while l <= h {
        let m = (l + h) / 2;

        if sv[m] == n {
            ans = Some(m);
            h = m - 1;
        } else if (sv[m] > n) {
            l = m + 1;
        } else {
            h = m - 1;
        }
    }

    return ans;
}

fn upper_bound(sv: &Vec<u32>, n: u32) -> Option<usize> {
    let mut l = 0;
    let mut h = sv.len() - 1;
    let mut ans: Option<usize> = None;
    while l <= h {
        let m = (l + h) / 2;
        //dbg!(m, sv[m]);
        if sv[m] == n {
            ans = Some(m);
            l = m + 1;
        } else if sv[m] > n {
            l = m + 1;
        } else {
            h = m - 1;
        }
    }

    return ans;
}

fn get_count(sv: &Vec<u32>, n: u32) -> usize {
    let lb = lower_bound(sv, n);
    //dbg!(lb);
    if lb.is_none() {
        return 0;
    }
    let rb = upper_bound(sv, n);
    //dbg!(n, rb);
    return rb.unwrap() - lb.unwrap() + 1;
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);

    let mut list_1 = BinaryHeap::new();
    let mut list_2 = BinaryHeap::new();
    for l in reader {
        let mut parts = l.split_whitespace();
        let p1 = parts.next().unwrap().parse::<u32>().unwrap();
        list_1.push(p1);
        let p2 = parts.next().unwrap().parse::<u32>().unwrap();
        list_2.push(p2);
    }

    let mut distance = 0;
    let mut vector_list1 = Vec::new();
    let mut vector_list2 = Vec::new();

    loop {
        if (list_1.is_empty() || list_2.is_empty()) {
            break;
        }
        let (i1, i2) = (list_1.pop().unwrap(), list_2.pop().unwrap());
        vector_list1.push(i1);
        vector_list2.push(i2);
        //dbg!(i1, i2);
        distance += i1.abs_diff(i2);
    }

    println!("Solution 1 {}", distance);

    //let vector_list1 = list_1.into_vec();
    //let vector_list2 = list_2.into_vec();
    //assert_eq!(distance, 11);
    //dbg!(&vector_list1, &vector_list2);
    let mut distance2 = 0;
    for num in vector_list1 {
        let d = get_count(&vector_list2, num);
        distance2 += (num * d as u32);
    }

    println!("Solution 2 {}", distance2);
}
