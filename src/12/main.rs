use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug,PartialEq,Hash,Eq,Clone,Copy)]
struct Position{
    x: usize,
    y: usize
}

impl Position {
    fn next_to(&self, other: &Position) -> bool{
        if self.x.abs_diff(other.x) == 1 && self.y.abs_diff(other.y) == 0{
            return true;
        };
        if self.x.abs_diff(other.x) == 0 && self.y.abs_diff(other.y) == 1{
            return true;
        };
        false
    }
}

fn main(){
    let data = include_str!("input");

    let mut grains: Vec<Vec<char>> = Vec::new();
    let mut field_types: HashMap<char, HashSet<Position>> = HashMap::new();
    let mut field_borders: HashMap<Position, u8> = HashMap::new();
    let mut fields: Vec<Vec<Position>> = Vec::new();

    for (x, line) in data.lines().enumerate(){
        let mut tmp: Vec<char> = Vec::new();
        for (y, char) in line.chars().enumerate(){
            let pos = Position{x, y};
            field_types
                .entry(char)
                .or_default()
                .insert(pos);
            tmp.push(char);
        }
        grains.push(tmp);
    }

    for same_fields in field_types.values_mut(){
        while same_fields.len() > 0 {
            let mut new_field: Vec<Position> = Vec::new();
            let seed = match same_fields.iter().next() {
                Some(T) => T,
                None => continue
            };
            let mut untested: VecDeque<Position> = VecDeque::new();
            untested.push_back(*seed);
            while untested.len() > 0 {
                let test = untested.pop_front().expect("While loop");
                if new_field.contains(&test){
                    continue;
                }
                for remaining_unorganized_field in same_fields.iter() {
                    if test.next_to(remaining_unorganized_field) {
                        untested.push_back(*remaining_unorganized_field);
                    }
                }
                same_fields.remove(&test);
                new_field.push(test);
            }

            fields.push(new_field);
        }
    }

    for (x, line) in grains.iter().enumerate(){
        for (y, char) in line.iter().enumerate(){
            let pos = Position{x,y };
            let mut num_borders = field_borders.entry(pos).or_insert(0);
            if y!=0{
                if line[y-1] != *char {
                    *num_borders += 1;
                }
            }else{
                *num_borders += 1;
            }

            if let Some(char_r) = line.get(y+1){
                if *char_r != *char {
                    *num_borders += 1;
                }
            }else{
                *num_borders += 1;
            }

            if x!=0 {
                if grains[x -1][y] != *char {
                    *num_borders += 1;
                }
            }else{
                *num_borders += 1;
            }


            if let Some(bot_row) = grains.get(x+1){
                if bot_row[y] != *char {
                    *num_borders += 1;
                }
            }else{
                *num_borders += 1;
            }
        }
    }

    let mut price = 0;
    for field in fields{
        let mut borders = 0;
        for pos in &field{
            let tmp = field_borders[&pos];
            borders += tmp;
        }
        price += field.len() * borders as usize;
    }
    println!("{}", price);

}