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
        let x1 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y1 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let x2 = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let y2 = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let x = caps.get(5).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;
        let y = caps.get(6).unwrap().as_str().parse::<i64>().unwrap() + 10000000000000;

        let top = y * x1 - x * y1;
        let bot = y2 * x1 - x2 * y1;

        if top % bot == 0 {
            let b = top / bot;
            let topa = x - b * x2;
            if topa % x1 == 0{
                let a = topa / x1;
                res += 3*a + b;
            }
        }

    }
    println!("{}", res);
}