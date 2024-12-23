use regex::Regex;

#[derive(Debug)]
struct Robot{
    px: i64,
    py: i64,
    vx: i64,
    vy: i64,
}

impl Robot {
    fn step(& mut self, n: u64, wide: u64, height: u64){
        let mod_x = (wide as i64);
        let mod_y = (height as i64);


        self.px = (((self.px + self.vx * n as i64) % mod_x) + mod_x) % mod_x;
        self.py = (((self.py + self.vy * n as i64) % mod_y) + mod_y) % mod_y
    }
}

fn display_robots(robots: &Vec<Robot>, wide: u64, height: u64){
    for y in 0..height{
        for x in 0..wide{
            let mut res = 0;
            for robot in robots{
                if robot.px == x as i64 && robot.py == y as i64{
                    res += 1;
                }
            }
            print!("{}", res);
        }
        println!();
    }

}

fn main(){
    let re = Regex::new(r"p=(.*),(.*) v=(.*),(.*)").unwrap();

    let testing = false;

    let wide: u64 = if testing {11} else {101};
    let height: u64 = if testing {7} else {103};

    let data;
    if testing {
        data = include_str!("example");
    }else{
        data = include_str!("input");
    };

    let mut robots: Vec<Robot> = Vec::new();
    for line in data.lines(){

        let caps = re.captures(line).unwrap();
        // Accessing match groups using indices
        let px = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let py = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let vx = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let vy = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

        let mut tmp = Robot{px, py, vx, vy};
        tmp.step(100, wide, height);
        robots.push(tmp);
    }

    //display_robots(&robots, wide, height);
    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    let v = wide / 2;
    let h = height / 2;

    for robot in robots{
        if robot.px == v as i64 || robot.py == h as i64{
            continue
        }
        if robot.px < v as i64{
            if robot.py < h as i64 {
                bl += 1;
            }else{
                br += 1;
            }
        } else {
            if robot.py < h as i64 {
                tl += 1;
            }else{
                tr += 1;
            }
        }
    }
    println!("{}", bl * br * tr * tl);

}
