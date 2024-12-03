fn main() {
    let data = include_str!("input");

    let mut res = 0;
    for line in data.lines(){
        let orig_report: Vec<i32> = line.split(" ")
            .map(|a| a.trim().parse::<i32>().expect("Number could not be parsed")).collect();

        let mut report;
        for i in 0..orig_report.len() + 1{
            report = orig_report.clone();
            if i < orig_report.len() {
                report.remove(i);
            }

            if report.windows(2).all(|w| w[0] < w[1] && w[1]-w[0] <= 3){
                res += 1;
                break;
            }

            if report.windows(2).all(|w| w[1] < w[0] && w[0]-w[1] <= 3){
                res += 1;
                break;
            }
        }


    }

    println!("{}", res);

}