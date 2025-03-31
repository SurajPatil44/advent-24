use std::collections::BinaryHeap;
use std::iter::zip;
use utils::Content;

/*
Problem  : https://adventofcode.com/2024/day/1
*/

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    let mut sum = 0;
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
        distance += i1.abs_diff(i2);
    }

    println!("Solution 1 {}", distance);

    //assert_eq!(distance, 11);
}
