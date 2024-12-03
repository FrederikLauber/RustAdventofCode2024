fn main() {
    let data = include_str!("input");

    let mut res = 0;
    for line in data.lines(){
        let report: Vec<i32> = line.split(" ")
            .map(|a| a.trim().parse::<i32>().expect("Number could not be parsed")).collect();


        if report.windows(2).all(|w| w[0] < w[1] && w[1]-w[0] <= 3){
            res += 1;
            continue;
        }

        if report.windows(2).all(|w| w[1] < w[0] && w[0]-w[1] <= 3){
            res += 1;
            continue;
        }
    }

    println!("{}", res);

}