use std::collections::HashMap;
use utils::{partition, Content};

fn permut_ops(num: u8, ind: u32) -> Vec<char> {
    (0..num)
        .rev()
        .map(|b| if (ind & (1 << b)) != 0 { '*' } else { '+' })
        .collect()
}

fn calculate(nums: &Vec<u64>, ops: &mut Vec<char>) -> u64 {
    let mut cpy: Vec<u64> = nums.iter().rev().map(|x| *x).collect();

    //let dbgop = ops.clone();
    loop {
        if ops.is_empty() {
            break;
        }
        let first = cpy.pop().unwrap();
        let second = cpy.pop().unwrap();
        let op = ops.pop().unwrap();

        if op == '+' {
            cpy.push(first + second);
        } else if op == '*' {
            cpy.push(first * second);
        } else if op == '|' {
            let mut val = first.to_string();
            val.push_str(&second.to_string());
            cpy.push(val.parse::<u64>().unwrap());
        } else {
        }
    }

    //dbg!(&dbgop, &cpy);

    return cpy[0];
}

struct RepeatedPermutations {
    elements: Vec<char>,
    indices: Vec<usize>,
    done: bool,
}

impl RepeatedPermutations {
    fn new(elements: Vec<char>, length: usize) -> Self {
        Self {
            elements,
            indices: vec![0; length],
            done: false,
        }
    }
}

impl Iterator for RepeatedPermutations {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let combination = self
            .indices
            .iter()
            .map(|&i| self.elements[i])
            .collect::<_>();

        // Increment indices like a base-N counter
        for i in (0..self.indices.len()).rev() {
            if self.indices[i] + 1 < self.elements.len() {
                self.indices[i] += 1;
                for j in i + 1..self.indices.len() {
                    self.indices[j] = 0;
                }
                return Some(combination);
            }
        }

        self.done = true;
        Some(combination)
    }
}

fn target_achieved2(nums: &Vec<u64>, target: u64) -> bool {
    let n = nums.len() - 1;
    let ops = vec!['|', '*', '+'];
    let mut pmts = RepeatedPermutations::new(ops, n);

    loop {
        let op = pmts.next();
        if op.is_none() {
            break;
        }
        let mut op = op.unwrap();
        if calculate(nums, &mut op) == target {
            return true;
        }
    }

    false
}

fn target_achieved(nums: &Vec<u64>, target: u64) -> bool {
    let ops_size = nums.len() - 1;

    let mut l = 0;
    let r = 1 << ops_size;

    while l < r {
        let mut ops = permut_ops(ops_size as u8, l);
        if target == calculate(&nums, &mut ops) {
            return true;
        }

        l += 1;
    }

    false
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);

    let mut data: HashMap<u64, Vec<u64>> = HashMap::new();

    for l in reader {
        let parts = partition(&l, ':');
        let target = parts.0.parse::<u64>().unwrap();
        let vector: Vec<u64> = parts
            .1
            .trim()
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        data.insert(target, vector);
    }
    let mut res = 0;
    for item in data.iter() {
        if target_achieved(item.1, *item.0) {
            res += *item.0;
        } else {
            if target_achieved2(item.1, *item.0) {
                //dbg!(item.1);
                res += *item.0;
            }
        }
    }
    dbg!(res);
}
