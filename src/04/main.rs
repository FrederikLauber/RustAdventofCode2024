use std::collections::{HashMap, VecDeque};
use regex::Regex;

fn main() {
    let data = include_str!("input");
    let re1 = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();



    let mut lines: VecDeque<Vec<char>> = VecDeque::new();

    let mut res1 = 0;
    let mut res2 = 0;
    let mut res3 = 0;
    let mut res4 = 0;

    let mut res = 0;

    for line in  data.lines(){
        res1 += re1.find_iter(&line).count();
        res1 += re2.find_iter(&line).count();


        lines.push_back(line.chars().collect());
        if lines.len() < 4 {
            continue;
        } else if lines.len() > 4 {
            lines.pop_front();
        }

        for i in 0..line.len(){
            // Vertical XMAS SAMX
            if ((lines[0][i] == 'X') && (lines[1][i] == 'M') && (lines[2][i] == 'A') && (lines[3][i] == 'S')) ||
               ((lines[0][i] == 'S') && (lines[1][i] == 'A') && (lines[2][i] == 'M') && (lines[3][i] == 'X')){
                res2 += 1;
            }
        }

        for i in 0..line.len() - 3 {
            // diagonal XMAS SAMX
            if ((lines[0][i] == 'X') && (lines[1][i + 1] == 'M') && (lines[2][i + 2] == 'A') && (lines[3][i + 3] == 'S')) ||
                ((lines[0][i] == 'S') && (lines[1][i + 1] == 'A') && (lines[2][i + 2] == 'M') && (lines[3][i + 3] == 'X')) {
                res3 += 1;
            }
        }
        for i in 3..line.len() {

            if ((lines[0][i] == 'X') && (lines[1][i-1] == 'M') && (lines[2][i-2] == 'A') && (lines[3][i-3] == 'S')) ||
                ((lines[0][i] == 'S') && (lines[1][i-1] == 'A') && (lines[2][i-2] == 'M') && (lines[3][i-3] == 'X')) {
                res4 += 1;
            }
        }
    }

    println!("{}", res1);
    println!("{}", res2);
    println!("{}", res3);
    println!("{}", res4);

    println!("{}", res1+res2+res3+res4);

}