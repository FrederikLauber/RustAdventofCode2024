use std::cmp::min;
use std::ops::Sub;
use itertools::Itertools;
use regex::Regex;

fn main(){
    let data = include_str!("input");

    let blocks = data.split("\n\n");
    let re = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X\=(\d+), Y\=(\d+)$").unwrap();

    let mut res = 0;
    for block in blocks{
        let caps = re.captures(block).unwrap();
        // Accessing match groups using indices
        let x1 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        let y1 = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
        let x2 = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
        let y2 = caps.get(4).unwrap().as_str().parse::<u64>().unwrap();
        let x = caps.get(5).unwrap().as_str().parse::<u64>().unwrap();
        let y = caps.get(6).unwrap().as_str().parse::<u64>().unwrap();

        let mut tokens = 2000;
        for a in 0..=100{
            for b in 0..=100{
                if y == a * y1 + b * y2 && x == a * x1 + b * x2{
                    tokens = min(tokens, 3*a+b);
                }
            }
        }

        if tokens < 2000 {
            res += tokens;
        }
    }
    println!("{}", res);
}