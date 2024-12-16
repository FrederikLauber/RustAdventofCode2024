use std::collections::{HashMap, HashSet};
use itertools::Itertools; // 0.8.2

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn add(&self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

}

fn main(){
    let data = include_str!("input");
    let mut antennas_map : HashMap<char, HashSet<Position>> = HashMap::new();
    let mut antinodes: HashSet<Position> = HashSet::new();
    let mut antinodes_multiples: HashSet<Position> = HashSet::new();
    let mut max_x=0;
    let mut max_y=0;

    for (x, line) in data.lines().enumerate(){
        max_y = line.len() - 1;
        max_x = x;
        for (y, c) in line.chars().enumerate(){
            if c == '.' {
                continue
            }
            antennas_map.entry(c).or_insert(HashSet::new()).insert(Position{x: x as i64, y: y as i64});
        }
    }

    for antennas in antennas_map.values(){
        for antenna_perm in antennas.iter().permutations(2).unique(){

            let antenna1 = antenna_perm[0];
            let antenna2 = antenna_perm[1];

            if antenna1 == antenna2{
                continue
            }
            let diff1 = antenna1.sub(antenna2);
            antinodes.insert(antenna1.add(&diff1));
            antinodes.insert(antenna2.sub(&diff1));

            let mut pos = antenna1.clone();
            loop {
                pos = pos.add(&diff1).clone();
                if pos.x < 0 || pos.x > max_x as i64 || pos.y < 0 || pos.x > max_y as i64 {
                    break
                }
                antinodes_multiples.insert(pos.clone());
            }

            let mut pos = antenna1.clone();
            loop {
                pos = pos.sub(&diff1).clone();
                if pos.x < 0 || pos.x > max_x as i64 || pos.y < 0 || pos.x > max_y as i64 {
                    break
                }
                antinodes_multiples.insert(pos.clone());
            }
        }
    }

    let mut antinodes_in_map: HashSet<Position> = HashSet::new();
    for antinode in antinodes.iter(){
        if antinode.x >= 0 && antinode.x <= max_x as i64 && antinode.y >= 0 && antinode.y <= max_y as i64 {
            antinodes_in_map.insert(antinode.clone());
        }
    }

    let mut antinodes_multipol_in_map: HashSet<Position> = HashSet::new();
    for antinode in antinodes_multiples.iter(){
        if antinode.x >= 0 && antinode.x <= max_x as i64 && antinode.y >= 0 && antinode.y <= max_y as i64 {
            antinodes_multipol_in_map.insert(antinode.clone());
        }
    }


    //println!("{:?}", antinodes_in_map);
    println!("{}", antinodes_in_map.len());
    println!("{}", antinodes_multipol_in_map.len())
}