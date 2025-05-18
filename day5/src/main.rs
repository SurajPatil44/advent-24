use std::collections::{HashMap, HashSet};
use utils::{partition, Content};

fn check_record(record: &Vec<u32>, after: &HashMap<u32, HashSet<u32>>) -> (bool, isize) {
    let mut cpy: Vec<_> = record.iter().map(|x| x).collect();
    let mut banned: HashSet<u32> = HashSet::new();
    let mut prev = cpy.pop().unwrap();
    let mut ind = record.len() - 1;
    if after.contains_key(&prev) {
        banned.extend(after[prev].clone());
    }
    while !cpy.is_empty() {
        let cur = cpy.pop().unwrap();
        ind = ind - 1;
        if banned.contains(cur) {
            return (false, ind as isize);
        }
        prev = cur;
        if after.contains_key(&prev) {
            banned.extend(after[prev].clone());
        }
    }
    (true, -1)
}

fn fix_record(record: &Vec<u32>, after: &HashMap<u32, HashSet<u32>>, swap_ind: usize) -> u32 {
    let mut cpy: Vec<_> = record.iter().map(|x| *x).collect();

    cpy.swap(swap_ind, swap_ind + 1);

    loop {
        let (v, swap_ind) = check_record(&cpy, &after);
        //dbg!(v, &cpy);
        if v {
            break;
        }

        cpy.swap(swap_ind as usize, swap_ind as usize + 1);
    }

    cpy[cpy.len() / 2]
}
fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);

    // the set of pages that should come after key
    let mut after: HashMap<u32, HashSet<u32>> = HashMap::new();
    loop {
        let l = reader.next().unwrap();
        if l == "" {
            break;
        }
        let (first, second) = partition(&l, '|');
        let first = first.parse::<u32>().unwrap();
        let second = second.parse::<u32>().unwrap();

        /*
         *dict.entry(key).or_insert(Vec::new()).push(token);
        }*/

        after.entry(first).or_insert(HashSet::new()).insert(second);
    }
    let mut records: Vec<Vec<u32>> = vec![];

    for l in reader {
        records.push(l.split(',').map(|x| x.parse::<u32>().unwrap()).collect())
    }
    //dbg!(records);
    let mut ret = 0;
    let mut fixed = 0;
    for r in records {
        let (v, i) = check_record(&r, &after);
        if v {
            let mid = r[r.len() / 2];
            ret += mid;
        } else {
            fixed += fix_record(&r, &after, i as usize);
        }
    }
    dbg!(ret);
    dbg!(fixed);
}
