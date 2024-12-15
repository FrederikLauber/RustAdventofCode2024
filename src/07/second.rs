use std::collections::HashMap;

fn num_digits(n: u32) -> u32 {
    if n == 0 {
        1 // Special case for zero
    } else {
        (n as f32).log10().floor() as u32 + 1
    }
}
fn main() {
    let previous = 6231007345478_u128;
    let previous_example = 3749;

    let data = include_str!("input_second");
    let mut res = 0;

    'outer: for line in data.lines(){
        let (target_str, value_str) = line.trim().split_once(":").expect("That is the form of the line");
        let target = target_str.parse::<u128>().expect("Format");
        let values: Vec<u128> = value_str.trim().split(" ").map(|a| a.trim().parse::<u128>().expect("Format!")).collect();

        println!("Target: {}", target);
        println!("Values: {:?}", values);

        let mut operators = HashMap::new();
        for i in 1..values.len() + 1{
            operators.insert(i, 0);
        }

        'trial_loop: loop{
            let mut tmp = *values.first().expect("There should be at least one");

            for value_index in 1..values.len(){
                let value = values[value_index];
                let opt = operators[&value_index];

                match opt{
                    0 => {tmp += value},
                    1 => {tmp *= value},
                    2 => {tmp *= 10_u128.pow(num_digits(value as u32)); tmp += value}
                    _ => {}
                }

                if tmp > target {
                    break
                }
            }
            if tmp == target{
                res+=target;
                continue 'outer
            }

            let mut i: usize = 1;
            loop {
                if i >= values.len(){
                    break 'trial_loop
                }

                let opt = operators[&i];
                if opt < 2{
                    operators.insert(i, opt +1);
                    continue 'trial_loop
                } else{
                    operators.insert(i, 0);
                    i += 1;
                }
            }




        }
    }
    println!("{}", res + previous);
}