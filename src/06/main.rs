use std::collections::HashSet;
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Block{
    x: i32,
    y: i32,
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Position{
    block: Block,
    dir: Direction,
}

impl Position {
    pub fn x(&self) -> i32 {
        self.block.x
    }
    pub fn y(&self) -> i32 {
        self.block.y
    }
}


#[derive(Clone, Debug)]
struct SimulationState {
    floor: Vec<Vec<char>>,
    floor_x: i32,
    floor_y: i32,
    position: Position,
    previous_position: HashSet<Position>
}

impl SimulationState {
    fn add_current(&mut self) {
        self.previous_position.insert(self.position.clone());
    }

    fn mark(&mut self, position: &Position, char: char) {
        self.floor[position.x() as usize][position.y() as usize] = char;
    }

    fn mark_current_visited(&mut self) {
        self.mark(&self.position.clone(), 'x');
    }

    fn next_position(&self) -> Option<Position> {
        let mut new_x = self.position.x();
        let mut new_y = self.position.y();

        match self.position.dir {
            Direction::Up => { new_x = new_x - 1; }
            Direction::Right => { new_y = new_y + 1; }
            Direction::Down => { new_x = new_x + 1; }
            Direction::Left => { new_y = new_y - 1; }
        };

        //left grid?
        if new_x < 0 || new_x >= self.floor_x || new_y < 0 || new_y >= self.floor_y{
            return None
        }

        Some(Position {
            block: Block { x: new_x, y: new_y },
            dir: self.position.dir.clone(),
        })
    }

    fn blocked(&self, testposition: &Position) -> bool {
        matches!(self.floor[testposition.x() as usize][testposition.y() as usize], '#'|'O')
    }

    fn next_direction(&self) -> Direction {
        match self.position.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn count_x(&self) -> usize {
        self.floor.iter().flat_map(|line| line.iter()).filter(|&&c| c == 'x').count()
    }

    fn rotate(&mut self){
        self.position.dir = self.next_direction();
    }

    fn move_to(&mut self, position: Position){
        self.position = position;
    }

    fn step(&mut self) -> Option<bool>{
        loop {
            if let Some(new_position) = self.next_position() {
                // we are in a loop?
                if self.previous_position.contains(&new_position) {
                    return Some(true)
                }
                self.add_current();
                if self.blocked(&new_position) {
                    self.rotate();
                } else {
                    self.move_to(new_position);
                    self.mark_current_visited();
                    return None
                }
            } else {
                // left grid
                return Some(false)
            }
        }
    }
}

fn simulate_is_loop(simulation_state: &mut SimulationState) -> bool{
    loop {
        if let Some(loop_status) = simulation_state.step(){
            return loop_status
        }
    }
}


fn main() {
    let data = include_str!("input");
    let mut data_map: Vec<Vec<char>> = Vec::new();

    for line in data.lines(){
        data_map.push(line.trim().chars().collect());
    }

    let len_x = data_map.len() as i32;
    let len_y = data_map.first().expect("We expect at least one line").len() as i32;

    let mut pos_x =None;
    let mut pos_y=None;
    let mut dir=None;

    'outer: for (x, line) in data_map.iter().enumerate() {
        for (y, char) in line.iter().enumerate() {
            if matches!(char, 'v' | '<' | '>' | '^') {
                pos_x = Some(x as i32);
                pos_y = Some(y as i32);
                dir = Some(match char {
                    'v' => Direction::Down,
                    '^' => Direction::Up,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("Cannot happen as we already matched on on the char before")
                });
                break 'outer;
            }
        }
    }

    let mut simstate = SimulationState{
        floor: data_map,
        floor_x: len_x,
        floor_y: len_y,
        position: Position {
            block: Block { x: pos_x.expect("Failure to find start"), y: pos_y.expect("Failure to find start") },
            dir: dir.expect("Failure to find start"),
        },
        previous_position: HashSet::new(),
    };
    // start point should be marked as x
    simstate.mark_current_visited();
    simstate.add_current();
    let start_position = simstate.position.clone();

    let mut simstate_looping = simstate.clone();

    // first task
    simulate_is_loop(&mut simstate);
    println!("First: {}", simstate.count_x());

    let mut res = 0;

    let mut already_tested : HashSet<Block> = HashSet::new();

    loop{
        let mut loop_simstate = simstate_looping.clone();

        if let Some(left) = simstate_looping.step(){
            break
        }
        if simstate_looping.position == start_position{
            continue;
        }
        if already_tested.contains(&simstate_looping.position.block){
            continue;
        }
        loop_simstate.mark(&simstate_looping.position, 'O');
        already_tested.insert(simstate_looping.position.block.clone());
        if simulate_is_loop(&mut loop_simstate){
            res += 1;
        }
    }
    println!("Second: {}", res);
}
