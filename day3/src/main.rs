use regex::Regex;
use utils::Content;

fn multi(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut ans = 0;
    for caps in re.captures_iter(input) {
        let x = &caps[1].parse::<u32>().unwrap();
        let y = &caps[2].parse::<u32>().unwrap();
        //dbg!(x, y);
        ans += x * y;
    }
    ans
}

fn partition<'a>(input: &'a str, pattern: &'a str) -> (&'a str, &'a str) {
    let len = input.len();
    let pos = input.find(pattern).unwrap_or(len);
    let offset = std::cmp::min(pos, pos + pattern.len());

    return (&input[..pos], &input[offset..]);
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);
    //println!("Hello, world!");
    let mut ans = 0;
    let mut inp = String::new();
    for l in reader {
        ans += multi(&l);
        inp.push_str(&l);
    }
    println!("Solution {}", ans);
    ans = 0;
    let mut rest = inp.as_str();
    loop {
        //dbg!(&rest);
        if rest.len() == 0 {
            break;
        }
        let (part1, inp) = partition(&rest, "don't()");
        rest = inp;
        //dbg!(part1);
        ans += multi(part1);
        //dbg!(&ans);
        let (_, inp) = partition(&rest, "do()");
        rest = inp;
        //dbg!(&inp);
    }
    println!("Solution part 2 {}", ans)
}
