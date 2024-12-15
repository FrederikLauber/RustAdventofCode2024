use std::fs::File;
use std::io::Write;

fn main() {
    let data = include_str!("input");
    let mut file = File::create("input_second").expect("sdds");
    let mut res = 0;

    'outer: for line in data.lines(){
        let (target_str, value_str) = line.trim().split_once(":").expect("That is the form of the line");
        let target = target_str.parse::<u128>().expect("Format");
        let values: Vec<u128> = value_str.trim().split(" ").map(|a| a.trim().parse::<u128>().expect("Format!")).collect();

        println!("Target: {}", target);
        println!("Values: {:?}", values);

        'trial_loop: for trial in 0..2_u128.pow(values.len() as u32){
            let mut tmp = *values.first().expect("There should be at least one");
            for value_index in 1..values.len(){
                let value = values[value_index];
                let exp = 2_u128.pow(value_index as u32 - 1);
                if trial & exp == 0 {
                    tmp += value;
                }else{
                    tmp *= value;
                }
                if tmp > target {
                    continue 'trial_loop
                }
            }
            if tmp == target{
                res+=target;
                continue 'outer
            }
        }
        // save lines which we cannot use
        writeln!(file, "{}", line);
    }
    println!("{}", res);
}