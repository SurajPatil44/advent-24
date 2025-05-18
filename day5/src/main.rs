use std::collections::{HashMap, HashSet};
use utils::{partition, Content};

fn check_record(records: &Vec<Vec<u32>>, index: usize, after: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut cpy: Vec<_> = records[index].iter().map(|x| x).collect();
    let mut banned: HashSet<u32> = HashSet::new();
    let mut prev = cpy.pop().unwrap();
    if after.contains_key(&prev) {
        banned.extend(after[prev].clone());
    }
    while !cpy.is_empty() {
        let cur = cpy.pop().unwrap();
        if banned.contains(cur) {
            return false;
        }
        prev = cur;
        if after.contains_key(&prev) {
            banned.extend(after[prev].clone());
        }
    }
    true
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
    for i in 0..records.len() {
        if check_record(&records, i, &after) {
            let mid = records[i][records[i].len() / 2];
            ret += mid;
        }
    }
    dbg!(ret);
}
