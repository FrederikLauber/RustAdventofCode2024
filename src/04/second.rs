use std::collections::{HashMap, VecDeque};

fn main() {
    let data = include_str!("input");



    let mut lines: VecDeque<Vec<char>> = VecDeque::new();

    let mut res = 0;

    for line in  data.lines(){


        lines.push_back(line.chars().collect());
        if lines.len() < 3 {
            continue;
        } else if lines.len() > 3 {
            lines.pop_front();
        }

        for i in 0..line.len() - 2 {
            if lines[1][i+1] != 'A'{
                continue;
            }
            if !((lines[0][i] == 'M' && lines[2][i+2] == 'S') || (lines[0][i] == 'S' && lines[2][i+2] == 'M')){
                continue;
            }


            if !((lines[2][i] == 'M' && lines[0][i+2] == 'S') || (lines[2][i] == 'S' && lines[0][i+2] == 'M')){
                continue;
            }

            res += 1;
        }
    }

    println!("{}", res);

}