use std::collections::VecDeque;

#[derive(PartialEq,Debug)]
enum Direction{
    East,
    West,
    North,
    South
}

#[derive(Debug)]
struct Seed{
    x: usize,
    y: usize,
}

fn main(){
    let data = include_str!("input");

    let mut heights: Vec<Vec<u8>> = Vec::new();
    let mut search_points: VecDeque<Seed> = VecDeque::new();


    for (x, line) in data.lines().enumerate(){
        let mut tmp: Vec<u8> = Vec::new();
        for (y, char) in line.chars().enumerate(){
            let height = char.to_digit(10).expect("sdds") as u8;
            if height == 0{
                search_points.push_back(Seed{x, y});
            }
            tmp.push(height);
        }
        heights.push(tmp);
    }

    let heights = heights;
    let max_x = heights.len();
    let max_y = heights.first().expect("We have at least one line").len();

    let mut res = 0;

    while search_points.len() > 0 {
        let search_point = search_points.pop_front().expect("We only loop on len>0");
        let mut x = search_point.x as i32;
        let mut y = search_point.y as i32;
        let current_height = heights[search_point.x][search_point.y] + 1;

        if current_height == 10{
            res += 1;
            continue;
        }

        for new_seed in vec!(
            Seed{x: (x + 1) as usize, y: y as usize},
            Seed{x: (x - 1) as usize, y: y as usize},
            Seed{x: x as usize, y: (y + 1) as usize},
            Seed{x: x as usize, y: (y - 1) as usize}
        ){
            if new_seed.x>= max_x || new_seed.y>= max_y{
                // out of bounds
                continue;
            }

            if current_height != heights[new_seed.x][new_seed.y]{
                // not 1 heigher
                continue;
            }

            search_points.push_front(new_seed);
        }
    }

    println!("{}", res);
}