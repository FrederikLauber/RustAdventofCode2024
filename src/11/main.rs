use std::collections::{BTreeMap, HashMap};
use std::ops::Add;

fn main(){
    let data = include_str!("input");
    let mut stones: HashMap<u128, u64> = HashMap::new();

    for num_str in data.split(" "){
        let tmp = num_str.parse::<u128>().expect("File format");
        stones.entry(tmp)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for blink in 0..75{ //25 for first
        println!("{blink}");
        // println!("{}: {:?}", blink, line);
        let mut new_stones: HashMap<u128, u64> = HashMap::new();
        for (stone, num) in stones.iter(){
            // rule 1
            if *stone == 0{
                new_stones.entry(1)
                    .and_modify(|e| *e += *num)
                    .or_insert(*num);

                continue;
            }
            // rule 2
            {
                let chars = stone.to_string();
                if chars.len() % 2 == 0{
                    let first_half = chars.chars().take(chars.len() / 2).collect::<String>();
                    let second_half = chars.chars().skip(chars.len() / 2).collect::<String>();

                    let num1 = first_half.parse::<u128>().expect("foo");
                    let num2 = second_half.parse::<u128>().expect("foo");

                    new_stones.entry(num1)
                        .and_modify(|e| *e += *num)
                        .or_insert(*num);

                    new_stones.entry(num2)
                        .and_modify(|e| *e += *num)
                        .or_insert(*num);
                    continue;
                }
            }
            // rule 3
            new_stones.entry(2024 * *stone)
                .and_modify(|e| *e += *num)
                .or_insert(*num);
        }
        stones = new_stones;
    }

    println!("{:?}", stones);

    let mut res = 0;
    for val in stones.values(){
        res += *val;
    }

    println!("Result: {}", res);
}