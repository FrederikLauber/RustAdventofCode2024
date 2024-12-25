extern crate core;

use core::fmt;
use std::cmp::{max, PartialEq};

type Coord = i64;

#[derive(Debug, PartialEq, Eq)]
enum Field{
    Empty,
    Box,
    Wall,
}

#[derive(Debug)]
enum Direction{
    East,
    West,
    North,
    South
}

struct Position{
    x: Coord,
    y: Coord
}

struct Map{
    map: Vec<Vec<Field>>,
    robot_position: Position,
    max_x: Coord,
    max_y: Coord,
}

impl Map {
    pub fn new() -> Self {
        Map {
            map: Vec::new(),
            robot_position: Position { x: 0, y: 0 },
            max_x: 0,
            max_y: 0,
        }
    }

    fn get(&self, x: Coord, y: Coord) -> Option<&Field>{
        // out of bounds?
        if x < 0 || y < 0 || x>= self.max_x || y >= self.max_y{
            return None
        }
        Some(&self.map[x as usize][y as usize])
    }

    pub fn add_row(&mut self, row: Vec<Field>){
        self.max_x += 1;
        self.max_y = row.len() as Coord;
        self.map.push(row);
    }

    pub fn set_robot(&mut self, robot: Position) -> Result<(), String>{
        if robot.x + 1 > self.max_x || robot.y + 1 > self.max_y{
            return Err("Robot position outside map".to_string());
        };
        self.robot_position = robot;
        Ok(())
    }

    pub fn get_gps(&mut self) -> u64{
        let mut res: u64 = 0;
        for (x, fields) in self.map.iter().enumerate(){
            for (y, field) in fields.iter().enumerate(){
                if *field == Field::Box{
                    res += 100_u64 * x as u64  + y as u64;
                }
            }
        }
        res
    }

    pub fn process(&mut self, commands: &Vec<Direction>){
        'outer: for direction in commands{
            let (x_change, y_change) = match direction {
                Direction::West => (0 as Coord, -1 as Coord),
                Direction::East => (0 as Coord, 1 as Coord),
                Direction::South => (1 as Coord, 0 as Coord),
                Direction::North => (-1 as Coord, 0 as Coord),
            };
            let currentx = self.robot_position.x;
            let currenty = self.robot_position.y;

            let mut shifts: Coord = 0;
            let mut movable = false;
            for i in 1..=max(self.max_x, self.max_y){
                let tmp = self.get(currentx + i * x_change, currenty + i * y_change);
                match tmp {
                    None => continue 'outer,
                    Some(t) =>
                        match t {
                            Field::Wall => break,
                            Field::Empty => {movable = true; break},
                            Field::Box => {shifts=i; continue},
                        }
                }
            }

            if !movable{
                continue;
            }

            self.robot_position.x = self.robot_position.x + x_change;
            self.robot_position.y = self.robot_position.y + y_change;

            self.map[self.robot_position.x as usize][self.robot_position.y as usize] = Field::Empty;
            for i in 2..shifts+2{
                self.map[(currentx + i * x_change) as usize][(currenty + i*y_change) as usize] = Field::Box;
            }
        };
    }
}



impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tmp: Vec<char> = Vec::new();
        for (x, fields) in self.map.iter().enumerate(){
            for (y, field) in fields.iter().enumerate(){
                if self.robot_position.x == x as Coord && self.robot_position.y == y as Coord {
                    tmp.push('@');
                    continue;
                }
                tmp.push(match field {
                    Field::Empty => '.',
                    Field::Wall => '#',
                    Field::Box => 'O'
                });
            }
            tmp.push('\n');
        }
        let s: String = tmp.into_iter().collect();
        write!(f, "{}", s)
    }
}



fn main(){
    let data = include_str!("input");

    let mut map: Map = Map::new();
    let mut commands: Vec<Direction> = Vec::new();

    let mut iter = data.lines();
    let mut robotPosition: Option<Position> = None;

    for (x, line) in iter.by_ref().enumerate(){
        if line == ""{
            break;
        }

        let mut tmp: Vec<Field> = Vec::new();
        for (y, char) in line.chars().enumerate(){
            match char {
                '#' => {tmp.push(Field::Wall);},
                'O' => {tmp.push(Field::Box);},
                '@' => {
                        robotPosition = Some(Position{x: x as Coord, y: y  as Coord});
                        tmp.push(Field::Empty);
                        },
                '.' => {tmp.push(Field::Empty)},
                _ => panic!("Should never happen! File format broken"),
            }
        }
        map.add_row(tmp);
    }

    map.set_robot(robotPosition.expect("No robot positon found")).expect("TODO: panic message");

    for line in iter{
        for char in line.chars(){
            commands.push(match char{
                '<' => Direction::West,
                '>' => Direction::East,
                '^' => Direction::North,
                'v' => Direction::South,
                _ => panic!("Should never happen, check file format"),

            });
        }
    }

    map.process(&commands);

    //for command in commands{
    //    println!("{}", map);
    //    println!("Move {:?}", command);
    //    map.process(&vec![command]);
    //}
    println!("{}", map.get_gps())
}